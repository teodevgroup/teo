use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::{Date, DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use serde_json::{json, Map, Value as JsonValue};
use async_recursion::async_recursion;
use crate::core::argument::Argument;
use crate::core::field::{Field, Optionality, Store};
use crate::core::input::AtomicUpdateType::{Decrement, Divide, Increment, Multiply};
use crate::core::input::{AtomicUpdateType, Input};
use crate::core::input::Input::{AtomicUpdate, SetValue};
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
use crate::core::input_decoder::{decode_field_input, input_to_vec, one_length_json_obj};
use crate::core::model::Model;
use crate::core::relation::{Relation, RelationManipulation};
use crate::core::save_session::SaveSession;
use crate::core::stage::Stage;
use crate::core::value::Value;
use crate::error::{ActionError, ActionErrorType};


#[derive(Clone)]
pub struct Object {
    pub(crate) inner: Arc<ObjectInner>
}

impl Object {

    pub(crate) fn new<'g>(graph: &Graph, model: &Model) -> Object {
        Object { inner: Arc::new(ObjectInner {
            model,
            graph,
            is_initialized: AtomicBool::new(false),
            is_new: AtomicBool::new(true),
            is_modified: AtomicBool::new(false),
            is_partial: AtomicBool::new(false),
            is_deleted: AtomicBool::new(false),
            selected_fields: RefCell::new(HashSet::new()),
            modified_fields: RefCell::new(HashSet::new()),
            previous_values: RefCell::new(HashMap::new()),
            value_map: RefCell::new(HashMap::new()),
            atomic_updator_map: RefCell::new(HashMap::new()),
            queried_relation_map: RefCell::new(HashMap::new()),
            relation_map: RefCell::new(HashMap::new()),
            ignore_required_fields: RefCell::new(Vec::new()),
        }) }
    }

    #[async_recursion]
    pub async fn set_json(&self, json_value: &JsonValue) -> Result<(), ActionError> {
        self.set_or_update_json(json_value, true).await
    }

    pub async fn update_json(&self, json_value: &JsonValue) -> Result<(), ActionError> {
        self.set_or_update_json(json_value, false).await
    }

    pub fn set_value(&self, key: impl Into<String>, value: Value) -> Result<(), ActionError> {
        let key = key.into();
        let model_keys = self.model().save_keys();
        if !model_keys.contains(&key) {
            return Err(ActionError::keys_unallowed());
        }
        if value == Value::Null {
            self.inner.value_map.borrow_mut().remove(&key);
        } else {
            self.inner.value_map.borrow_mut().insert(key.to_string(), value);
        }
        if !self.inner.is_new.load(Ordering::SeqCst) {
            self.inner.is_modified.store(true, Ordering::SeqCst);
            self.inner.modified_fields.borrow_mut().insert(key.to_string());
        }
        Ok(())
    }

    pub fn get_value(&self, key: impl Into<String>) -> Result<Option<Value>, ActionError> {
        let key = key.into();
        let model_keys = &self.model().get_value_keys(); // TODO: should be all keys
        if !model_keys.contains(&key) {
            return Err(ActionError::keys_unallowed());
        }
        match self.inner.value_map.borrow().get(&key) {
            Some(value) => {
                Ok(Some(value.clone()))
            }
            None => {
                match self.inner.queried_relation_map.borrow().get(&key) {
                    Some(list) => {
                        let relation = self.model().relation(&key).unwrap();
                        if relation.is_vec {
                            let vec = list.iter().map(|o| Value::Object(o.clone())).collect();
                            Ok(Some(Value::Vec(vec)))
                        } else {
                            if list.is_empty() {
                                Ok(Some(Value::Null))
                            } else {
                                Ok(Some(Value::Object(list.get(0).unwrap().clone())))
                            }
                        }
                    }
                    None => {
                        Ok(None)
                    }
                }
            }
        }
    }

    pub fn select(&self, keys: HashSet<String>) -> Result<(), ActionError> {
        self.inner.selected_fields.borrow_mut().extend(keys);
        Ok(())
    }

    pub fn deselect(&self, keys: HashSet<String>) -> Result<(), ActionError> {
        if self.inner.selected_fields.borrow().len() == 0 {
            self.inner.selected_fields.borrow_mut().extend(self.model().output_keys().iter().map(|s| { s.to_string()}));
        }
        for key in keys {
            self.inner.selected_fields.borrow_mut().remove(&key);
        }
        return Ok(());
    }

    #[async_recursion(?Send)]
    pub(crate) async fn apply_on_save_pipeline_and_validate_required_fields(&self) -> Result<(), ActionError> {
        // apply on save pipeline first
        let model_keys = self.model().save_keys();
        for key in model_keys {
            let field = self.model().field(key);
            if field.is_none() {
                continue;
            }
            let field = field.unwrap();
            if field.needs_on_save_callback() {
                let mut stage = match self.inner.value_map.borrow().deref().get(&key.to_string()) {
                    Some(value) => {
                        Stage::Value(value.clone())
                    }
                    None => {
                        Stage::Value(Value::Null)
                    }
                };
                let new_stage = field.perform_on_save_callback(stage.clone(), self).await;
                match new_stage {
                    Stage::Invalid(s) => {
                        return Err(ActionError::invalid_input(key, s));
                    }
                    Stage::Value(v) | Stage::ConditionTrue(v) | Stage::ConditionFalse(v) => {
                        self.inner.value_map.borrow_mut().insert(key.to_string(), v);
                        if !self.inner.is_new.load(Ordering::SeqCst) {
                            self.inner.is_modified.store(true, Ordering::SeqCst);
                            self.inner.modified_fields.borrow_mut().insert(key.to_string());
                        }
                    }
                }
            }
        }
        // validate required fields
        for key in model_keys {
            let field = self.model().field(key);
            if field.is_none() {
                continue;
            }
            let field = field.unwrap();
            if self.inner.ignore_required_fields.borrow().contains(&field.name) {
                continue;
            }
            if field.auto || field.auto_increment {
                continue
            }
            match field.store {
                Store::ForeignKey(_) => continue,
                Store::LocalKey => continue,
                _ => ()
            }
            if field.optionality == Optionality::Required {
                let value = self.get_value(key).unwrap();
                if value.is_none() {
                    return Err(ActionError::value_required(key))
                }
            }
        }
        for relation in &self.model().relations_vec {
            let name = &relation.name;
            let map = self.inner.relation_map.borrow();
            let vec = map.get(name);
            match vec {
                None => {},
                Some(vec) => {
                    for manipulation in vec {
                        manipulation.object().apply_on_save_pipeline_and_validate_required_fields().await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn clear_state(&self) {
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
        *self.inner.modified_fields.borrow_mut() = HashSet::new();
    }

    #[async_recursion(?Send)]
    pub(crate) async fn delete_from_database(&self, session: Arc<dyn SaveSession>, no_recursive: bool) -> Result<(), ActionError> {
        Ok(())
    }

    #[async_recursion(?Send)]
    pub(crate) async fn save_to_database(&self, session: Arc<dyn SaveSession>, no_recursive: bool) -> Result<(), ActionError> {
        // handle relations and manipulations
        if !no_recursive {
            for relation in &self.model().relations_vec {
                let name = &relation.name;
                let map = self.inner.relation_map.borrow();
                let vec_option = map.get(name);
                match vec_option {
                    None => {},
                    Some(vec) => {
                        for manipulation in vec {
                            manipulation.object().save_to_database(session.clone(), false).await?;
                        }
                    }
                }
            }
        }
        // send to database to save self
        let connector = self.graph().connector();
        connector.save_object(self).await?;
        // links
        if !no_recursive {
            for relation in &self.model().relations_vec {
                let name = &relation.name;
                let map = self.inner.relation_map.borrow();
                let vec_option = map.get(name);
                match vec_option {
                    None => {},
                    Some(vec) => {
                        for manipulation in vec {
                            match manipulation {
                                RelationManipulation::Connect(obj) => {
                                    self.link_connect(obj, relation, session.clone()).await?;
                                }
                                RelationManipulation::Disconnect(obj) => {

                                    self.link_disconnect(obj, relation, session.clone()).await?;
                                }
                                RelationManipulation::Set(obj) => {
                                    self.link_connect(obj, relation, session.clone()).await?;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        // clear properties
        self.clear_state();
        Ok(())
    }

    pub(crate) async fn link_connect(&self, obj: &Object, relation: &Relation, session: Arc<dyn SaveSession>) -> Result<(), ActionError> {
        match &relation.through {
            Some(through) => { // with join table
                let relation_model = self.graph().model(through);
                let relation_object = self.graph().new_object(through);
                let local_relation_name = relation.fields.get(0).unwrap();
                let foreign_relation_name = relation.references.get(0).unwrap();
                let local_relation = relation_model.relation(local_relation_name).unwrap();
                let foreign_relation = relation_model.relation(foreign_relation_name).unwrap();
                for (index, field_name) in local_relation.fields.iter().enumerate() {
                    let local_field_name = local_relation.references.get(index).unwrap();
                    let val = self.get_value(local_field_name).unwrap().unwrap();
                    relation_object.set_value(field_name, val).unwrap();
                }
                for (index, field_name) in foreign_relation.fields.iter().enumerate() {
                    let foreign_field_name = foreign_relation.references.get(index).unwrap();
                    let val = obj.get_value(foreign_field_name).unwrap().unwrap();
                    relation_object.set_value(field_name, val).unwrap();
                }
                relation_object.save_to_database(session.clone(), true);
            }
            None => { // no join table
                for (index, reference) in relation.references.iter().enumerate() {
                    let field_name = relation.fields.get(index).unwrap();
                    let local_value = self.get_value(field_name)?;
                    let foreign_value = obj.get_value(reference)?;
                    if local_value.is_some() && foreign_value.is_none() {
                        obj.set_value(reference, local_value.unwrap())?;
                        obj.save_to_database(session.clone(), true).await?;
                    } else if foreign_value.is_some() && local_value.is_none() {
                        self.set_value(field_name, foreign_value.unwrap())?;
                        self.save_to_database(session.clone(), true).await?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn link_disconnect(&self, obj: &Object, relation: &Relation, session: Arc<dyn SaveSession>) -> Result<(), ActionError> {
        match &relation.through {
            Some(through) => { // with join table
                let relation_model = self.graph().model(through);
                let mut finder: Map<String, JsonValue> = Map::new();
                let local_relation_name = relation.fields.get(0).unwrap();
                let foreign_relation_name = relation.references.get(0).unwrap();
                let local_relation = relation_model.relation(local_relation_name).unwrap();
                let foreign_relation = relation_model.relation(foreign_relation_name).unwrap();
                for (index, field_name) in local_relation.fields.iter().enumerate() {
                    let local_field_name = local_relation.references.get(index).unwrap();
                    let val = self.get_value(local_field_name).unwrap().unwrap();
                    finder.insert(field_name.to_string(), val.to_json_value());
                }
                for (index, field_name) in foreign_relation.fields.iter().enumerate() {
                    let foreign_field_name = foreign_relation.references.get(index).unwrap();
                    let val = obj.get_value(foreign_field_name).unwrap().unwrap();
                    finder.insert(field_name.to_string(), val.to_json_value());
                }
                let relation_object = self.graph().find_unique(relation_model, &JsonValue::Object(finder), true).await?;
                relation_object.delete_from_database(session.clone(), false).await?;
            }
            None => { // no join table
                for (index, reference) in relation.references.iter().enumerate() {
                    let field_name = relation.fields.get(index).unwrap();
                    let local_value = self.get_value(field_name)?;
                    let foreign_value = obj.get_value(reference)?;
                    let local_field = self.model().field(field_name).unwrap();
                    let foreign_field = obj.model().field(reference).unwrap();
                    if local_value.is_some() && foreign_value.is_some() {
                        if local_field.optionality == Optionality::Optional {
                            self.set_value(field_name, Value::Null)?;
                            self.save_to_database(session.clone(), true);
                        } else if foreign_field.optionality == Optionality::Optional {
                            obj.set_value(reference, Value::Null)?;
                            obj.save_to_database(session.clone(), true);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn save(&self) -> Result<(), ActionError> {
        self.apply_on_save_pipeline_and_validate_required_fields().await?;
        let connector = self.graph().connector();
        let session = connector.new_save_session();
        self.save_to_database(session, false).await?;
        Ok(())
    }

    pub async fn delete(&self) -> Result<(), ActionError> {
        let connector = self.graph().connector();
        connector.delete_object(self).await
    }

    pub fn to_json(&self) -> JsonValue {
        let mut map: Map<String, JsonValue> = Map::new();
        let keys = self.model().output_keys();
        for key in keys {
            let value = self.get_value(key).unwrap();
            match value {
                Some(v) => {
                    if v != Value::Null {
                        map.insert(key.to_string(), v.to_json_value());
                    }
                }
                None => {}
            }
        }
        return JsonValue::Object(map)
    }

    pub async fn include(&self) -> &Object {
        self
    }

    pub(crate) fn is_new(&self) -> bool {
        self.inner.is_new.load(Ordering::SeqCst)
    }

    async fn set_or_update_json(&self, json_value: &JsonValue, process: bool) -> Result<(), ActionError> {
        let json_object = json_value.as_object().unwrap();
        // check keys first
        let json_keys: Vec<&String> = json_object.keys().map(|k| { k }).collect();
        let allowed_keys = if process {
            self.model().input_keys().iter().map(|k| k).collect::<Vec<&String>>()
        } else {
            self.model().save_keys().iter().map(|k| k).collect::<Vec<&String>>()
        };
        let keys_valid = json_keys.iter().all(|item| allowed_keys.contains(item ));
        if !keys_valid {
            return Err(ActionError::keys_unallowed());
        }
        let all_model_keys = self.model().all_keys().iter().map(|k| k).collect::<Vec<&String>>();
        // assign values
        let initialized = self.inner.is_initialized.load(Ordering::SeqCst);
        let keys_to_iterate = if initialized { &json_keys } else { &all_model_keys };
        for key in keys_to_iterate {
            if self.model().has_field(key) {
                // this is field
                let field = self.model().field(&key).unwrap();
                let json_has_value = if initialized { true } else {
                    json_keys.contains(key)
                };
                if json_has_value {
                    let json_value = &json_object[&key.to_string()];
                    let input_result = decode_field_input(self.graph(), json_value, field, &field.name)?;
                    match input_result {
                        SetValue(value) => {
                            let mut value = value;
                            if process {
                                // pipeline
                                let mut stage = Stage::Value(value);
                                stage = field.on_set_pipeline.process(stage.clone(), &self).await;
                                match stage {
                                    Stage::Invalid(s) => {
                                        return Err(ActionError::invalid_input(&field.name, s));
                                    }
                                    Stage::Value(v) => {
                                        value = v
                                    }
                                    Stage::ConditionTrue(v) => {
                                        value = v
                                    }
                                    Stage::ConditionFalse(v) => {
                                        value = v
                                    }
                                }
                            }
                            if value == Value::Null {
                                if self.inner.is_new.load(Ordering::SeqCst) == false {
                                    self.inner.value_map.borrow_mut().remove(*key);
                                }
                            } else {
                                self.inner.value_map.borrow_mut().insert(key.to_string(), value);
                            }
                            if !self.inner.is_new.load(Ordering::SeqCst) {
                                self.inner.is_modified.store(true, Ordering::SeqCst);
                                self.inner.modified_fields.borrow_mut().insert(key.to_string());
                            }
                        }
                        AtomicUpdate(update_type) => {
                            self.inner.atomic_updator_map.borrow_mut().insert(key.to_string(), update_type);
                        }
                        Input::RelationInput(input) => {

                        }
                    }
                } else {
                    // apply default values
                    if !initialized {
                        if let Some(argument) = &field.default {
                            match argument {
                                Argument::ValueArgument(value) => {
                                    self.inner.value_map.borrow_mut().insert(key.to_string(), value.clone());
                                }
                                Argument::PipelineArgument(pipeline) => {
                                    let stage = pipeline.process(Stage::Value(Value::Null), &self).await;
                                    self.inner.value_map.borrow_mut().insert(key.to_string(), stage.value().unwrap());
                                }
                            }
                        }
                    }
                }
            } else {
                // this is relation
                let relation = self.model().relation(&key).unwrap();
                let relation_object = json_object.get(&key.to_string());
                if relation_object.is_none() {
                    continue;
                }
                let relation_object = relation_object.unwrap();
                let (command, command_input) = one_length_json_obj(relation_object, key)?;
                match command {
                    "create" | "createMany" => {
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let new_object =  graph.new_object(&relation.model);
                            new_object.set_json(entry).await?;
                            new_object.ignore_required_for(&relation.references);
                            self.ignore_required_for(&relation.fields);
                            if self.inner.relation_map.borrow().get(&key.to_string()).is_none() {
                                self.inner.relation_map.borrow_mut().insert(key.to_string(), vec![]);
                            }
                            let mut relation_map = self.inner.relation_map.borrow_mut();
                            let mut objects = relation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Connect(new_object));
                        }
                    }
                    "set" => {

                    }
                    "connect" => {
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let model = graph.model(&relation.model);
                            let unique_query = json!({"where": entry});
                            let new_object = graph.find_unique(model, &unique_query, true).await?;
                            self.ignore_required_for(&relation.fields);
                            if self.inner.relation_map.borrow().get(&key.to_string()).is_none() {
                                self.inner.relation_map.borrow_mut().insert(key.to_string(), vec![]);
                            }
                            let mut relation_map = self.inner.relation_map.borrow_mut();
                            let mut objects = relation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Connect(new_object));
                        }
                    }
                    "connectOrCreate" => {
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let model = graph.model(&relation.model);
                            let r#where = entry.as_object().unwrap().get("where").unwrap();
                            let create = entry.as_object().unwrap().get("create").unwrap();
                            let unique_result = graph.find_unique(model, r#where, true).await;
                            match unique_result {
                                Ok(new_obj) => {
                                    if self.inner.relation_map.borrow().get(&key.to_string()).is_none() {
                                        self.inner.relation_map.borrow_mut().insert(key.to_string(), vec![]);
                                    }
                                    let mut relation_map = self.inner.relation_map.borrow_mut();
                                    let mut objects = relation_map.get_mut(&key.to_string()).unwrap();
                                    objects.push(RelationManipulation::Connect(new_obj));
                                }
                                Err(_err) => {
                                    let new_obj = graph.new_object(&relation.model);
                                    new_obj.set_json(create).await?;
                                    if self.inner.relation_map.borrow().get(&key.to_string()).is_none() {
                                        self.inner.relation_map.borrow_mut().insert(key.to_string(), vec![]);
                                    }
                                    let mut relation_map = self.inner.relation_map.borrow_mut();
                                    let mut objects = relation_map.get_mut(&key.to_string()).unwrap();
                                    objects.push(RelationManipulation::Connect(new_obj));
                                }
                            }

                        }
                    }
                    "disconnect" => {
                        // if new, err
                    }
                    "update" | "updateMany" => {
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let r#where = entry.get("where").unwrap();
                            let update = entry.get("update").unwrap();
                            let model_name = &relation.model;
                            let model = graph.model(model_name);
                            let the_object = graph.find_unique(model, &json!({"where": r#where}), true).await;
                            if the_object.is_err() {
                                return Err(ActionError::object_not_found());
                            }
                            let new_object = the_object.unwrap();
                            new_object.set_json(update).await?;
                            if self.inner.relation_map.borrow().get(&key.to_string()).is_none() {
                                self.inner.relation_map.borrow_mut().insert(key.to_string(), vec![]);
                            }
                            let mut relation_map = self.inner.relation_map.borrow_mut();
                            let mut objects = relation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Keep(new_object));
                        }
                    }
                    "upsert" => {

                    }
                    "delete" | "deleteMany" => {

                    }
                    _ => {
                        return Err(ActionError::wrong_input_type());
                    }
                }
            }
        };
        // set flag
        self.inner.is_initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub(crate) fn identifier(&self) -> Value {
        if let Some(primary_field) = self.model().primary_field() {
            self.get_value(primary_field.name.clone()).unwrap().unwrap()
        } else {
            panic!("Identity model must have primary field defined explicitly.");
        }
    }

    pub(crate) fn ignore_required_for(&self, ignores: &Vec<String>) {
        self.inner.ignore_required_fields.borrow_mut().extend(ignores.iter().map(|v| v.to_string()).collect::<Vec<String>>());
    }

    pub(crate) fn is_instance_of(&self, model_name: &'static str) -> bool {
        self.model().name() == model_name
    }

    pub(crate) fn model(&self) -> &Model {
        unsafe { &*self.inner.model }
    }

    pub(crate) fn graph(&self) -> &Graph {
        unsafe { &*self.inner.graph }
    }
}

pub(crate) struct ObjectInner {
    pub(crate) model: * const Model,
    pub(crate) graph: * const Graph,
    pub(crate) is_initialized: AtomicBool,
    pub(crate) is_new: AtomicBool,
    pub(crate) is_modified: AtomicBool,
    pub(crate) is_partial: AtomicBool,
    pub(crate) is_deleted: AtomicBool,
    pub(crate) selected_fields: RefCell<HashSet<String>>,
    pub(crate) modified_fields: RefCell<HashSet<String>>,
    pub(crate) previous_values: RefCell<HashMap<String, Value>>,
    pub(crate) value_map: RefCell<HashMap<String, Value>>,
    pub(crate) atomic_updator_map: RefCell<HashMap<String, AtomicUpdateType>>,
    pub(crate) relation_map: RefCell<HashMap<String, Vec<RelationManipulation>>>,
    pub(crate) queried_relation_map: RefCell<HashMap<String, Vec<Object>>>,
    pub(crate) ignore_required_fields: RefCell<Vec<String>>,
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct(self.model().name());
        for (key, value) in self.inner.value_map.borrow().iter() {
            result.field(key, value);
        }
        result.finish()
    }
}

unsafe impl Send for ObjectInner {}
unsafe impl Sync for ObjectInner {}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.model() == other.model() && self.identifier() == other.identifier()
    }
}
