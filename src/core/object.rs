use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::{json, Map, Value as JsonValue};
use async_recursion::async_recursion;
use crate::core::pipeline::argument::Argument;
use crate::core::field::Optionality;
use crate::core::input::{AtomicUpdateType, Input};
use crate::core::input::Input::{AtomicUpdate, SetValue};
use crate::core::graph::Graph;
use crate::core::input_decoder::{decode_field_input, input_to_vec, one_length_json_obj};
use crate::core::model::Model;
use crate::core::previous_value::PreviousValueRule;
use crate::core::relation::{Relation, RelationManipulation};
use crate::core::save_session::SaveSession;
use crate::core::pipeline::context::{Context, Purpose};
use crate::core::value::Value;
use crate::core::error::{ActionError, ActionErrorType};
use crate::core::key_path::KeyPathItem;

#[derive(Clone)]
pub struct Object {
    pub(crate) inner: Arc<ObjectInner>
}

impl Object {

    pub(crate) fn new(graph: &Graph, model: &Model) -> Object {
        Object { inner: Arc::new(ObjectInner {
            graph: graph.clone(),
            model: model.clone(),
            is_initialized: AtomicBool::new(false),
            is_new: AtomicBool::new(true),
            is_modified: AtomicBool::new(false),
            is_partial: AtomicBool::new(false),
            is_deleted: AtomicBool::new(false),
            inside_before_save_callback: AtomicBool::new(false),
            inside_write_callback: AtomicBool::new(false),
            selected_fields: Arc::new(Mutex::new(Vec::new())),
            modified_fields: Arc::new(Mutex::new(HashSet::new())),
            previous_value_map: Arc::new(Mutex::new(HashMap::new())),
            value_map: Arc::new(Mutex::new(HashMap::new())),
            atomic_updator_map: Arc::new(Mutex::new(HashMap::new())),
            relation_query_map: Arc::new(Mutex::new(HashMap::new())),
            relation_mutation_map: Arc::new(Mutex::new(HashMap::new())),
            ignore_required_fields: Arc::new(Mutex::new(Vec::new())),
            identity: Arc::new(Mutex::new(None)),
        }) }
    }

    #[async_recursion(?Send)]
    pub async fn set_json(&self, json_value: &JsonValue) -> Result<(), ActionError> {
        self.set_or_update_json(json_value, true).await
    }

    pub async fn update_json(&self, json_value: &JsonValue) -> Result<(), ActionError> {
        self.set_or_update_json(json_value, false).await
    }

    pub fn set(&self, key: impl AsRef<str>, value: impl Into<Value>) -> Result<(), ActionError> {
        self.set_value(key, value.into())
    }

    pub fn set_value(&self, key: impl AsRef<str>, value: Value) -> Result<(), ActionError> {
        let key = key.as_ref().to_string();
        let model_keys = self.model().save_keys();
        if !model_keys.contains(&key) {
            return Err(ActionError::keys_unallowed());
        }
        if value == Value::Null {
            self.inner.value_map.lock().unwrap().remove(&key);
        } else {
            self.inner.value_map.lock().unwrap().insert(key.to_string(), value);
        }
        if !self.inner.is_new.load(Ordering::SeqCst) {
            self.inner.is_modified.store(true, Ordering::SeqCst);
            self.inner.modified_fields.lock().unwrap().insert(key.to_string());
        }
        Ok(())
    }

    pub fn get_relation_object(&self, key: impl AsRef<str>) -> Result<Option<Object>, ActionError> {
        let key = key.as_ref();
        let model_keys = self.model().get_value_keys();
        if !model_keys.contains(&key.to_string()) {
            return Err(ActionError::keys_unallowed());
        }
        match self.inner.relation_query_map.lock().unwrap().get(key) {
            Some(list) => {
                if list.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(list.get(0).unwrap().clone()))
                }
            }
            None => {
                Ok(None)
            }
        }
    }

    pub fn get_relation_objects(&self, key: impl AsRef<str>) -> Result<Option<Vec<Object>>, ActionError> {
        let key = key.as_ref();
        let model_keys = self.model().get_value_keys();
        if !model_keys.contains(&key.to_string()) {
            return Err(ActionError::keys_unallowed());
        }
        match self.inner.relation_query_map.lock().unwrap().get(key) {
            Some(list) => {
                Ok(Some(list.clone()))
            }
            None => {
                Ok(None)
            }
        }
    }

    pub fn get<T>(&self, key: impl AsRef<str>) -> Result<T, ActionError> where T: From<Value> {
        match self.get_value(key) {
            Ok(optional_value) => {
                Ok(optional_value.into())
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn get_value(&self, key: impl AsRef<str>) -> Result<Value, ActionError> {
        let key = key.as_ref();
        let model_keys = self.model().get_value_keys(); // TODO: should be all keys
        if !model_keys.contains(&key.to_string()) {
            let model = self.model();
            return Err(ActionError::get_value_error(model.name(), key));
        }
        match self.inner.value_map.lock().unwrap().get(&key.to_string()) {
            Some(value) => {
                Ok(value.clone())
            }
            None => {
                match self.inner.relation_query_map.lock().unwrap().get(&key.to_string()) {
                    Some(list) => {
                        let relation = self.model().relation(&key).unwrap();
                        if relation.is_vec {
                            let vec = list.iter().map(|o| Value::Object(o.clone())).collect();
                            Ok(Value::Vec(vec))
                        } else {
                            if list.is_empty() {
                                Ok(Value::Null)
                            } else {
                                Ok(Value::Object(list.get(0).unwrap().clone()))
                            }
                        }
                    }
                    None => {
                        Ok(Value::Null)
                    }
                }
            }
        }
    }

    pub fn set_select(&self, select: Option<&JsonValue>) -> Result<(), ActionError> {
        if select.is_none() {
            return Ok(());
        }
        let mut true_list: Vec<&str> = vec![];
        let mut false_list: Vec<&str> = vec![];
        let map = select.unwrap().as_object().unwrap();
        for (key, value) in map {
            let bool_value = value.as_bool().unwrap();
            if bool_value {
                true_list.push(key.as_str());
            } else {
                false_list.push(key.as_str());
            }
        }
        let true_empty = true_list.is_empty();
        let false_empty = false_list.is_empty();
        if true_empty && false_empty {
            // just do nothing
            return Ok(());
        } else if !false_empty {
            // all - false
            let mut result: Vec<String> = vec![];
            self.model().all_keys().iter().for_each(|k| {
                let field = self.model().field(k);
                if let Some(field) = field {
                    if !false_list.contains(&&***&k) {
                        result.push(field.name.clone());
                    }
                }
            });
            *self.inner.selected_fields.lock().unwrap() = result;
            return Ok(());
        } else {
            // true
            let mut result: Vec<String> = vec![];
            self.model().all_keys().iter().for_each(|k| {
                let field = self.model().field(k);
                if let Some(field) = field {
                    if true_list.contains(&&***&k) {
                        result.push(field.name.clone());
                    }
                }
            });
            *self.inner.selected_fields.lock().unwrap() = result;
            return Ok(());
        }
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
                let initial_value = match self.inner.value_map.lock().unwrap().deref().get(&key.to_string()) {
                    Some(value) => {
                        value.clone()
                    }
                    None => {
                        Value::Null
                    }
                };
                let purpose = if self.is_new() {
                    Purpose::Create
                } else {
                    Purpose::Update
                };
                let context = Context::initial_state(self.clone(), purpose)
                    .alter_value(initial_value)
                    .alter_key_path(vec![KeyPathItem::String((&field).name.to_string())]);
                let result_ctx = field.perform_on_save_callback(context).await;
                match result_ctx.invalid_reason() {
                    Some(reason) => {
                        return Err(ActionError::invalid_input(key, reason));
                    }
                    None => {
                        self.inner.value_map.lock().unwrap().insert(key.to_string(), result_ctx.value);
                        if !self.inner.is_new.load(Ordering::SeqCst) {
                            self.inner.is_modified.store(true, Ordering::SeqCst);
                            self.inner.modified_fields.lock().unwrap().insert(key.to_string());
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
            if self.inner.ignore_required_fields.lock().unwrap().contains(&field.name) {
                continue;
            }
            if field.auto || field.auto_increment {
                continue
            }
            if field.r#virtual {
                continue
            }
            if field.optionality == Optionality::Required {
                let value = self.get_value(key).unwrap();
                if value.is_null() {
                    return Err(ActionError::value_required(key))
                }
            }
        }
        for relation in self.model().relations() {
            let name = &relation.name;
            let map = self.inner.relation_mutation_map.lock().unwrap();
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

    pub(crate) fn clear_new_state(&self) {
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
    }

    pub(crate) fn clear_state(&self) {
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
        *self.inner.modified_fields.lock().unwrap() = HashSet::new();
    }

    #[async_recursion(?Send)]
    pub(crate) async fn delete_from_database(&self, _session: Arc<dyn SaveSession>, _no_recursive: bool) -> Result<(), ActionError> {
        let connector = self.graph().connector();
        connector.delete_object(self).await?;
        Ok(())
    }

    #[async_recursion(?Send)]
    pub(crate) async fn save_to_database(&self, session: Arc<dyn SaveSession>, no_recursive: bool) -> Result<(), ActionError> {
        // handle relations and manipulations
        if !no_recursive {
            for relation in self.model().relations() {
                let name = &relation.name;
                let map = self.inner.relation_mutation_map.lock().unwrap();
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
        self.clear_new_state();
        // links
        if !no_recursive {
            for relation in self.model().relations() {
                let name = &relation.name;
                let map = self.inner.relation_mutation_map.lock().unwrap();
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
                                RelationManipulation::Delete(obj) => {
                                    self.link_disconnect(obj, relation, session.clone()).await?;
                                    obj.delete().await?;
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
                let relation_model = self.graph().model(through)?;
                let relation_object = self.graph().new_object(through)?;
                relation_object.set_json(&json!({})).await?;
                let local_relation_name = relation.fields.get(0).unwrap();
                let foreign_relation_name = relation.references.get(0).unwrap();
                let local_relation = relation_model.relation(local_relation_name).unwrap();
                let foreign_relation = relation_model.relation(foreign_relation_name).unwrap();
                for (index, field_name) in local_relation.fields.iter().enumerate() {
                    let local_field_name = local_relation.references.get(index).unwrap();
                    let val = self.get_value(local_field_name).unwrap();
                    relation_object.set_value(field_name, val.clone()).unwrap();
                }
                for (index, field_name) in foreign_relation.fields.iter().enumerate() {
                    let foreign_field_name = foreign_relation.references.get(index).unwrap();
                    let val = obj.get_value(foreign_field_name).unwrap();
                    relation_object.set_value(field_name, val.clone()).unwrap();
                }
                relation_object.save().await?;
            }
            None => { // no join table
                for (index, reference) in relation.references.iter().enumerate() {
                    let field_name = relation.fields.get(index).unwrap();
                    if relation.is_vec {
                        // if relation is vec, othersize must have saved the value
                        let local_value = self.get_value(field_name)?;
                        if !local_value.is_null() {
                            obj.set_value(reference, local_value.clone())?;
                            obj.save_to_database(session.clone(), true).await?;
                        }
                    } else {
                        // get foreign relation
                        if let Some(foreign_relation) = obj.model().relations().iter().find(|r| {
                            r.fields == relation.references && r.references == relation.fields
                        }) {
                            if foreign_relation.is_vec {
                                let foreign_value = obj.get_value(reference)?;
                                if !foreign_value.is_null() {
                                    self.set_value(field_name, foreign_value.clone())?;
                                    self.save_to_database(session.clone(), true).await?;
                                }
                            } else {
                                // both sides are singular
                                for item in &self.model().primary_index().items {
                                    if &item.field_name == field_name {
                                        let local_value = self.get_value(field_name)?;
                                        if !local_value.is_null() {
                                            obj.set_value(reference, local_value.clone())?;
                                            obj.save_to_database(session.clone(), true).await?;
                                        }
                                        break;
                                    }
                                }
                                // write on our side since it's not primary
                                let foreign_value = obj.get_value(reference)?;
                                if !foreign_value.is_null() {
                                    self.set_value(field_name, foreign_value.clone())?;
                                    self.save_to_database(session.clone(), true).await?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn link_disconnect(&self, obj: &Object, relation: &Relation, session: Arc<dyn SaveSession>) -> Result<(), ActionError> {
        match &relation.through {
            Some(through) => { // with join table
                let relation_model = self.graph().model(through)?;
                let mut finder: Map<String, JsonValue> = Map::new();
                let local_relation_name = relation.fields.get(0).unwrap();
                let foreign_relation_name = relation.references.get(0).unwrap();
                let local_relation = relation_model.relation(local_relation_name).unwrap();
                let foreign_relation = relation_model.relation(foreign_relation_name).unwrap();
                for (index, field_name) in local_relation.fields.iter().enumerate() {
                    let local_field_name = local_relation.references.get(index).unwrap();
                    let val = self.get_value(local_field_name).unwrap();
                    finder.insert(field_name.to_string(), val.to_json_value());
                }
                for (index, field_name) in foreign_relation.fields.iter().enumerate() {
                    let foreign_field_name = foreign_relation.references.get(index).unwrap();
                    let val = obj.get_value(foreign_field_name).unwrap();
                    finder.insert(field_name.to_string(), val.to_json_value());
                }
                let relation_object = self.graph().find_unique(through, &json!({"where": finder}), true).await?;
                relation_object.delete_from_database(session.clone(), false).await?;
            }
            None => { // no join table
                for (index, reference) in relation.references.iter().enumerate() {
                    let field_name = relation.fields.get(index).unwrap();
                    let local_value = self.get_value(field_name)?;
                    let foreign_value = obj.get_value(reference)?;
                    let local_field = self.model().field(field_name).unwrap();
                    let foreign_field = obj.model().field(reference).unwrap();
                    if !local_value.is_null() && !foreign_value.is_null() {
                        if local_field.optionality == Optionality::Optional {
                            self.set_value(field_name, Value::Null)?;
                            self.save_to_database(session.clone(), true).await?;
                        } else if foreign_field.optionality == Optionality::Optional {
                            obj.set_value(reference, Value::Null)?;
                            obj.save_to_database(session.clone(), true).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn save(&self) -> Result<(), ActionError> {
        let inside_before_callback = self.inner.inside_before_save_callback.load(Ordering::SeqCst);
        if inside_before_callback {
            return Err(ActionError::save_calling_error(self.model().name()));
        }
        let is_new = self.is_new();
        self.apply_on_save_pipeline_and_validate_required_fields().await?;
        self.trigger_before_write_callbacks(is_new).await?;
        let connector = self.graph().connector();
        let session = connector.new_save_session();
        self.save_to_database(session, false).await?;
        self.trigger_write_callbacks(is_new).await?;
        Ok(())
    }

    async fn trigger_before_write_callbacks(&self, newly_created: bool) -> Result<(), ActionError> {
        // let model = self.model();
        // if newly_created {
        //     for cb in model.before_create_fns() {
        //         cb(self.clone()).await?;
        //     }
        // } else {
        //     for cb in model.before_update_fns() {
        //         cb(self.clone()).await?;
        //     }
        // }
        // for cb in model.before_save_fns() {
        //     cb(self.clone()).await?;
        // }
        Ok(())
    }

    async fn trigger_write_callbacks(&self, newly_created: bool) -> Result<(), ActionError> {
        let inside_write_callback = self.inner.inside_write_callback.load(Ordering::SeqCst);
        if inside_write_callback {
            return Ok(());
        }
        self.inner.inside_write_callback.store(true, Ordering::SeqCst);
        // let model = self.model();
        // for cb in model.after_save_fns() {
        //     cb(self.clone()).await?;
        // }
        // if newly_created {
        //     for cb in model.after_create_fns() {
        //         cb(self.clone()).await?;
        //     }
        // } else {
        //     for field in model.fields() {
        //         if field.previous_value_rule == PreviousValueRule::KeepAfterSaved {
        //             for cb in &field.compare_after_update {
        //                 if let Some(prev) = self.inner.previous_value_map.lock().unwrap().get(&field.name) {
        //                     if let current = self.get_value(&field.name).unwrap() {
        //                         if !current.is_null() {
        //                             cb(prev.clone(), current.clone(), self.clone()).await?
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     for cb in model.after_update_fns() {
        //         cb(self.clone()).await?;
        //     }
        // }
        self.inner.inside_write_callback.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub async fn delete(&self) -> Result<(), ActionError> {
        let connector = self.graph().connector();
        connector.delete_object(self).await
    }

    pub fn to_json(&self) -> JsonValue {
        let select_list = self.inner.selected_fields.lock().unwrap();
        let select_filter = if select_list.is_empty() { false } else { true };
        let mut map: Map<String, JsonValue> = Map::new();
        let keys = self.model().output_keys();
        for key in keys {
            let value = self.get_value(key).unwrap();
            if !value.is_null() {
                if (!select_filter) || (select_filter && select_list.contains(key)) {
                    map.insert(key.to_string(), value.to_json_value());
                }
            }
        }
        return JsonValue::Object(map)
    }

    pub fn is_new(&self) -> bool {
        self.inner.is_new.load(Ordering::SeqCst)
    }

    pub fn is_modified(&self) -> bool {
        self.inner.is_modified.load(Ordering::SeqCst)
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
                                let purpose = if self.is_new() {
                                    Purpose::Create
                                } else {
                                    Purpose::Update
                                };
                                let context = Context::initial_state(self.clone(), purpose)
                                    .alter_key_path(vec![KeyPathItem::String(key.to_string())])
                                    .alter_value(value);
                                let result_context = field.on_set_pipeline.process(context).await;
                                match result_context.invalid_reason() {
                                    Some(reason) => {
                                        return Err(ActionError::invalid_input(&field.name, reason));
                                    }
                                    None => {
                                        value = result_context.value
                                    }
                                }
                            }
                            let save_previous = field.previous_value_rule != PreviousValueRule::DontKeep;

                            if value == Value::Null {
                                if self.inner.is_new.load(Ordering::SeqCst) == false {
                                    if save_previous {
                                        if let Some(current) = self.inner.value_map.lock().unwrap().get(key.as_str()) {
                                            self.inner.previous_value_map.lock().unwrap().insert(key.to_string(), current.clone());
                                        }
                                    }
                                    self.inner.value_map.lock().unwrap().remove(*key);
                                }
                            } else {
                                if save_previous {
                                    if let Some(current) = self.inner.value_map.lock().unwrap().get(key.as_str()) {
                                        self.inner.previous_value_map.lock().unwrap().insert(key.to_string(), current.clone());
                                    }
                                }
                                self.inner.value_map.lock().unwrap().insert(key.to_string(), value);
                            }
                            if !self.inner.is_new.load(Ordering::SeqCst) {
                                self.inner.is_modified.store(true, Ordering::SeqCst);
                                self.inner.modified_fields.lock().unwrap().insert(key.to_string());
                            }
                        }
                        AtomicUpdate(update_type) => {
                            self.inner.atomic_updator_map.lock().unwrap().insert(key.to_string(), update_type);
                        }
                        Input::RelationInput(_input) => {

                        }
                    }
                } else {
                    // apply default values
                    if !initialized {
                        if let Some(argument) = &field.default {
                            match argument {
                                Argument::ValueArgument(value) => {
                                    self.inner.value_map.lock().unwrap().insert(key.to_string(), value.clone());
                                }
                                Argument::PipelineArgument(pipeline) => {
                                    let ctx = Context::initial_state(self.clone(), Purpose::Create);
                                    let value = pipeline.process(ctx).await.value;
                                    self.inner.value_map.lock().unwrap().insert(key.to_string(), value);
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
                        if !relation.is_vec && command == "createMany" {
                            return Err(ActionError::invalid_input(key.as_str(), "Single relationship cannot create many."));
                        }
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let new_object =  graph.new_object(&relation.model)?;
                            new_object.set_json(entry).await?;
                            new_object.ignore_required_for(&relation.references);
                            self.ignore_required_for(&relation.fields);
                            if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                            }
                            let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                            let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Connect(new_object));
                        }
                    }
                    "set" | "connect" => {
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let unique_query = json!({"where": entry});
                            let new_object = graph.find_unique(&relation.model, &unique_query, true).await?;
                            new_object.ignore_required_for(&relation.references);
                            self.ignore_required_for(&relation.fields);
                            if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                            }
                            let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                            let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Connect(new_object));
                        }
                    }
                    "connectOrCreate" => {
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let r#where = entry.as_object().unwrap().get("where").unwrap();
                            let create = entry.as_object().unwrap().get("create").unwrap();
                            let unique_result = graph.find_unique(&relation.model, &json!({"where": r#where}), true).await;
                            match unique_result {
                                Ok(new_obj) => {
                                    if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                        self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                                    }
                                    new_obj.ignore_required_for(&relation.references);
                                    self.ignore_required_for(&relation.fields);
                                    let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                                    let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                                    objects.push(RelationManipulation::Connect(new_obj));
                                }
                                Err(_err) => {
                                    let new_obj = graph.new_object(&relation.model)?;
                                    new_obj.set_json(create).await?;
                                    new_obj.ignore_required_for(&relation.references);
                                    self.ignore_required_for(&relation.fields);
                                    if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                        self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                                    }
                                    let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                                    let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                                    objects.push(RelationManipulation::Connect(new_obj));
                                }
                            }

                        }
                    }
                    "disconnect" => {
                        if self.is_new() {
                            return Err(ActionError::new_object_cannot_disconnect());
                        }
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let model = graph.model(&relation.model)?;
                            if !relation.is_vec && (relation.optionality == Optionality::Required) {
                                return Err(ActionError::invalid_input(key.as_str(), "Required relation cannot disconnect."));
                            }
                            let opposite_relation = model.relations().iter().find(|r| {
                                r.fields == relation.references && r.references == relation.fields
                            });
                            if opposite_relation.is_some() {
                                let opposite_relation = opposite_relation.unwrap();
                                if !opposite_relation.is_vec && (opposite_relation.optionality == Optionality::Required) {
                                    return Err(ActionError::invalid_input(key.as_str(), "Required relation cannot disconnect."));
                                }
                            }
                            let unique_query = json!({"where": entry});
                            let object_to_disconnect = graph.find_unique(&relation.model, &unique_query, true).await?;
                            if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                            }
                            let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                            let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Disconnect(object_to_disconnect));
                        }
                    }
                    "update" | "updateMany" => {
                        if !relation.is_vec && command == "updateMany" {
                            return Err(ActionError::invalid_input(key.as_str(), "Single relationship cannot update many."));
                        }
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let r#where = entry.get("where").unwrap();
                            let update = entry.get("update").unwrap();
                            let model_name = &relation.model;
                            let the_object = graph.find_unique(model_name, &json!({"where": r#where}), true).await;
                            if the_object.is_err() {
                                return Err(ActionError::object_not_found());
                            }
                            let new_object = the_object.unwrap();
                            new_object.set_json(update).await?;
                            if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                            }
                            let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                            let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Keep(new_object));
                        }
                    }
                    "upsert" => {
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let r#where = entry.as_object().unwrap().get("where").unwrap();
                            let create = entry.as_object().unwrap().get("create").unwrap();
                            let update = entry.as_object().unwrap().get("update").unwrap();
                            let the_object = graph.find_unique(&relation.model, &json!({"where": r#where}), true).await;
                            match the_object {
                                Ok(obj) => {
                                    obj.set_json(update).await?;
                                    if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                        self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                                    }
                                    let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                                    let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                                    objects.push(RelationManipulation::Keep(obj));
                                }
                                Err(_) => {
                                    let new_obj = graph.new_object(&relation.model)?;
                                    new_obj.ignore_required_for(&relation.references);
                                    self.ignore_required_for(&relation.fields);
                                    new_obj.set_json(create).await?;
                                    if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                        self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                                    }
                                    let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                                    let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                                    objects.push(RelationManipulation::Connect(new_obj));
                                }
                            }
                        }
                    }
                    "delete" | "deleteMany" => {
                        if !relation.is_vec && command == "deleteMany" {
                            return Err(ActionError::invalid_input(key.as_str(), "Single relationship cannot delete many."));
                        }
                        let entries = input_to_vec(command_input)?;
                        let graph = self.graph();
                        for entry in entries {
                            let r#where = entry;
                            let model_name = &relation.model;
                            let model = graph.model(model_name)?;
                            if !relation.is_vec && (relation.optionality == Optionality::Required) {
                                return Err(ActionError::invalid_input(key.as_str(), "Required relation cannot delete."));
                            }
                            let opposite_relation = model.relations().iter().find(|r| {
                                r.fields == relation.references && r.references == relation.fields
                            });
                            if opposite_relation.is_some() {
                                let opposite_relation = opposite_relation.unwrap();
                                if !opposite_relation.is_vec && (opposite_relation.optionality == Optionality::Required) {
                                    return Err(ActionError::invalid_input(key.as_str(), "Required relation cannot delete."));
                                }
                            }
                            let the_object = graph.find_unique(model_name, &json!({"where": r#where}), true).await;
                            if the_object.is_err() {
                                return Err(ActionError::object_not_found());
                            }
                            let new_object = the_object.unwrap();
                            if self.inner.relation_mutation_map.lock().unwrap().get(&key.to_string()).is_none() {
                                self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
                            }
                            let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
                            let objects = relation_mutation_map.get_mut(&key.to_string()).unwrap();
                            objects.push(RelationManipulation::Delete(new_object));
                        }
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
            self.get_value(primary_field.name.clone()).unwrap()
        } else {
            panic!("Identity model must have primary field defined explicitly.");
        }
    }

    pub(crate) fn ignore_required_for(&self, ignores: &Vec<String>) {
        self.inner.ignore_required_fields.lock().unwrap().extend(ignores.iter().map(|v| v.to_string()).collect::<Vec<String>>());
    }

    pub(crate) fn is_instance_of(&self, model_name: &'static str) -> bool {
        self.model().name() == model_name
    }

    pub(crate) fn model(&self) -> &Model {
        &self.inner.model
    }

    pub fn graph(&self) -> &Graph {
        &self.inner.graph
    }

    pub(crate) fn json_identifier(&self) -> JsonValue {
        let model = self.model();
        let mut identifier = json!({});
        for item in &model.primary_index().items {
            let val = self.get_value(&item.field_name).unwrap();
            identifier.as_object_mut().unwrap().insert(item.field_name.clone(), val.to_json_value());
        }
        identifier
    }

    pub async fn refreshed(&self, include: Option<&JsonValue>, select: Option<&JsonValue>) -> Result<Object, ActionError> {
        let graph = self.graph();
        let mut finder = json!({
            "where": self.json_identifier(),
        });
        if include.is_some() {
            finder.as_object_mut().unwrap().insert("include".to_string(), include.unwrap().clone());
        }
        if select.is_some() {
            finder.as_object_mut().unwrap().insert("select".to_string(), select.unwrap().clone());
        }
        graph.find_unique(self.model().name(), &finder, false).await
    }

    pub fn set_identity(&self, identity: Option<Object>) {
        *self.inner.identity.lock().unwrap() = identity.clone();
    }

    pub fn get_identity(&self) -> Option<Object> {
        self.inner.identity.lock().unwrap().clone()
    }

    pub async fn fetch_relation_object(&self, key: impl AsRef<str>, find_unique_arg: Option<&JsonValue>) -> Result<Option<Object>, ActionError> {
        // get relation
        let model = self.model();
        let relation = model.relation(key.as_ref());
        if relation.is_none() {
            // todo() err here
        }
        let relation = relation.unwrap();
        // find object
        let mut finder_where = json!({});
        for (index, local_field_name) in relation.fields.iter().enumerate() {
            let foreign_field_name = relation.references.get(index).unwrap();
            let value = self.get_value(local_field_name).unwrap();
            if value == Value::Null {
                return Ok(None);
            }
            let json_value = value.to_json_value();
            finder_where.as_object_mut().unwrap().insert(foreign_field_name.to_owned(), json_value);
        }
        let mut finder = json!({"where": finder_where});
        if let Some(find_unique_arg) = find_unique_arg {
            if let Some(include) = find_unique_arg.get("include") {
                finder.as_object_mut().unwrap().insert("include".to_owned(), include.clone());
            }
            if let Some(select) = find_unique_arg.get("select") {
                finder.as_object_mut().unwrap().insert("select".to_owned(), select.clone());
            }
        }
        let relation_model_name = &relation.model;
        let graph = self.graph();
        match graph.find_unique(relation_model_name, &finder, false).await {
            Ok(result) => {
                self.inner.relation_query_map.lock().unwrap().insert(key.as_ref().to_string(), vec![result]);
                let obj = self.inner.relation_query_map.lock().unwrap().get(key.as_ref()).unwrap().get(0).unwrap().clone();
                Ok(Some(obj.clone()))
            }
            Err(err) => {
                if err.r#type == ActionErrorType::ObjectNotFound {
                    self.inner.relation_query_map.lock().unwrap().insert(key.as_ref().to_string(), vec![]);
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    pub async fn fetch_relation_objects(&self, key: impl AsRef<str>, find_many_arg: Option<&JsonValue>) -> Result<Vec<Object>, ActionError> {
        // get relation
        let model = self.model();
        let relation = model.relation(key.as_ref());
        if relation.is_none() {
            // todo() err here
        }
        let relation = relation.unwrap();
        let empty = json!({});
        let include_inside = if find_many_arg.is_some() {
            find_many_arg.unwrap()
        } else {
            &empty
        };
        if let Some(_join_table) = &relation.through {
            let identifier = self.json_identifier();
            let new_self = self.graph().find_unique(model.name(), &json!({
                "where": identifier,
                "include": {
                    key.as_ref(): include_inside
                }
            }), false).await?;
            let vec = new_self.inner.relation_query_map.lock().unwrap().get(key.as_ref()).unwrap().clone();
            self.inner.relation_query_map.lock().unwrap().insert(key.as_ref().to_string(), vec);
            Ok(self.inner.relation_query_map.lock().unwrap().get(key.as_ref()).unwrap().clone())
        } else {
            let mut finder = json!({});
            if let Some(find_many_arg) = find_many_arg {
                for (k, v) in find_many_arg.as_object().unwrap().iter() {
                    finder.as_object_mut().unwrap().insert(k.clone(), v.clone());
                }
            }
            if finder.as_object().unwrap().get("where").is_none() {
                finder.as_object_mut().unwrap().insert("where".to_string(), json!({}));
            }
            for (index, local_field_name) in relation.fields.iter().enumerate() {
                let foreign_field_name = relation.references.get(index).unwrap();
                let value = self.get_value(local_field_name).unwrap();
                if value == Value::Null {
                    return Ok(vec![]);
                }
                let json_value = value.to_json_value();
                finder.as_object_mut().unwrap().get_mut("where").unwrap().as_object_mut().unwrap().insert(foreign_field_name.to_owned(), json_value);
            }
            let relation_model_name = &relation.model;
            let graph = self.graph();
            let results = graph.find_many(relation_model_name, &finder, false).await?;
            self.inner.relation_query_map.lock().unwrap().insert(key.as_ref().to_string(), results.clone());
            Ok(results)
        }
    }
}

pub(crate) struct ObjectInner {
    pub(crate) model: Model,
    pub(crate) graph: Graph,
    pub(crate) is_initialized: AtomicBool,
    pub(crate) is_new: AtomicBool,
    pub(crate) is_modified: AtomicBool,
    pub(crate) is_partial: AtomicBool,
    pub(crate) is_deleted: AtomicBool,
    pub(crate) inside_before_save_callback: AtomicBool,
    pub(crate) inside_write_callback: AtomicBool,
    pub(crate) selected_fields: Arc<Mutex<Vec<String>>>,
    pub(crate) modified_fields: Arc<Mutex<HashSet<String>>>,
    pub(crate) previous_value_map: Arc<Mutex<HashMap<String, Value>>>,
    pub(crate) value_map: Arc<Mutex<HashMap<String, Value>>>,
    pub(crate) atomic_updator_map: Arc<Mutex<HashMap<String, AtomicUpdateType>>>,
    pub(crate) relation_mutation_map: Arc<Mutex<HashMap<String, Vec<RelationManipulation>>>>,
    pub(crate) relation_query_map: Arc<Mutex<HashMap<String, Vec<Object>>>>,
    pub(crate) ignore_required_fields: Arc<Mutex<Vec<String>>>,
    pub(crate) identity: Arc<Mutex<Option<Object>>>,
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct(self.model().name());
        for (key, value) in self.inner.value_map.lock().unwrap().iter() {
            result.field(key, value);
        }
        result.finish()
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.model() == other.model() && self.identifier() == other.identifier()
    }
}

unsafe impl Send for Object { }
unsafe impl Sync for Object { }
