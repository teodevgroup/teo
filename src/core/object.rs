use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use key_path::{KeyPath, path};
use async_recursion::async_recursion;
use crate::core::connector::SaveSession;
use crate::core::env::Env;
use crate::core::env::intent::Intent;
use crate::core::env::source::Source;
use crate::core::pipeline::argument::Argument;
use crate::core::field::{Field, PreviousValueRule};
use crate::core::field::optionality::Optionality;
use crate::core::input::{AtomicUpdateType, Input};
use crate::core::input::Input::{AtomicUpdate, AtomicUpdator, SetValue};
use crate::core::graph::Graph;
use crate::core::input_decoder::{decode_field_input, input_to_vec, one_length_json_obj};
use crate::core::model::Model;
use crate::core::relation::{Relation, RelationManipulation};
use crate::core::save_session::SaveSession;
use crate::core::pipeline::context::{Context};
use crate::core::tson::Value;
use crate::core::error::{ActionError, ActionErrorType};
use crate::core::field::write_rule::WriteRule;
use crate::core::relation::delete_rule::DeleteRule;
use crate::core::relation::delete_rule::DeleteRule::Deny;
use crate::core::result::ActionResult;
use crate::core::tson::decoder::Decoder;
use crate::tson;

#[derive(Clone)]
pub struct Object {
    pub(crate) inner: Arc<ObjectInner>
}

pub(crate) struct ObjectInner {
    pub(crate) model: Model,
    pub(crate) graph: Graph,
    pub(crate) env: Env,
    pub(crate) is_initialized: AtomicBool,
    pub(crate) is_new: AtomicBool,
    pub(crate) is_modified: AtomicBool,
    pub(crate) is_partial: AtomicBool,
    pub(crate) is_deleted: AtomicBool,
    pub(crate) inside_before_save_callback: AtomicBool,
    pub(crate) inside_after_save_callback: AtomicBool,
    pub(crate) selected_fields: Arc<Mutex<Vec<String>>>,
    pub(crate) modified_fields: Arc<Mutex<HashSet<String>>>,
    pub(crate) value_map: Arc<Mutex<HashMap<String, Value>>>,
    pub(crate) previous_value_map: Arc<Mutex<HashMap<String, Value>>>,
    pub(crate) atomic_updator_map: Arc<Mutex<HashMap<String, Value>>>,
    pub(crate) relation_mutation_map: Arc<Mutex<HashMap<String, Value>>>,
    pub(crate) relation_query_map: Arc<Mutex<HashMap<String, Vec<Object>>>>,
    pub(crate) cached_property_map: Arc<Mutex<HashMap<String, Value>>>,
}

fn check_user_json_keys<'a>(map: &HashMap<String, Value>, allowed: &HashSet<&str>, model: &Model) -> ActionResult<()> {
    if let Some(unallowed) = map.keys().find(|k| !allowed.contains(k.as_str())) {
        return Err(ActionError::invalid_key(unallowed, model));
    }
    Ok(())
}

impl Object {

    pub(crate) fn new(graph: &Graph, model: &Model, env: Env) -> Object {
        Object {
            inner: Arc::new(ObjectInner {
                graph: graph.clone(),
                model: model.clone(),
                env,
                is_initialized: AtomicBool::new(false),
                is_new: AtomicBool::new(true),
                is_modified: AtomicBool::new(false),
                is_partial: AtomicBool::new(false),
                is_deleted: AtomicBool::new(false),
                inside_before_save_callback: AtomicBool::new(false),
                inside_after_save_callback: AtomicBool::new(false),
                selected_fields: Arc::new(Mutex::new(Vec::new())),
                modified_fields: Arc::new(Mutex::new(HashSet::new())),
                previous_value_map: Arc::new(Mutex::new(HashMap::new())),
                value_map: Arc::new(Mutex::new(HashMap::new())),
                atomic_updator_map: Arc::new(Mutex::new(HashMap::new())),
                relation_query_map: Arc::new(Mutex::new(HashMap::new())),
                relation_mutation_map: Arc::new(Mutex::new(HashMap::new())),
                cached_property_map: Arc::new(Mutex::new(HashMap::new())),
            })
        }
    }

    #[async_recursion(?Send)]
    pub async fn set_tson(&self, value: &Value) -> ActionResult<()> {
        self.set_tson_with_path_and_user_mode(value, &path![], true).await
    }

    #[async_recursion(?Send)]
    pub(crate) async fn set_tson_with_path(&self, json_value: &Value, path: &KeyPath) -> ActionResult<()> {
        self.set_tson_with_path_and_user_mode(json_value, path, false).await
    }

    pub(crate) async fn set_tson_with_path_and_user_mode(&self, value: &Value, path: &KeyPath<'_>, user_mode: bool) -> ActionResult<()> {
        let model = self.model();
        let is_new = self.is_new();
        // permission
        if !user_mode {
            self.check_model_write_permission().await?;
        }
        // get value map
        let value_map = value.as_hashmap().unwrap();
        let value_map_keys: Vec<&String> = value_map.keys().collect();
        // check keys
        if user_mode {
            check_user_json_keys(value_map, &model.input_keys().iter().map(|k| k.as_str()).collect(), model)?;
        }
        // find keys to iterate
        let initialized = self.inner.is_initialized.load(Ordering::SeqCst);
        let keys = if initialized {
            self.model().all_keys().iter().filter(|k| value_map_keys.contains(k)).collect::<Vec<&String>>()
        } else {
            self.model().all_keys().iter().collect::<Vec<&String>>()
        };
        // assign values
        for key in keys {
            let path = path + key;
            if let Some(field) = self.model().field(key) {
                let need_to_trigger_default_value = if initialized { false } else {
                    !value_map_keys.contains(&key)
                };
                if need_to_trigger_default_value {
                    // apply default values
                    if let Some(argument) = &field.default {
                        match argument {
                            Argument::ValueArgument(value) => {
                                self.set_value_to_value_map(key, value.clone());
                            }
                            Argument::PipelineArgument(pipeline) => {
                                let ctx = Context::initial_state(self.clone());
                                let value = pipeline.process(ctx).await.value;
                                // todo: default value calculation error here
                                self.set_value_to_value_map(key, value);
                            }
                        }
                    }
                } else {
                    if !user_mode {
                        self.check_field_write_permission(field).await?;
                    }
                    // set_value_to_value_map
                    let value = value_map.get(key).unwrap();
                    match Input::decode_field(value) {
                        AtomicUpdator(updator) => self.set_value_to_atomic_updator_map(key, updator),
                        SetValue(value) => {
                            // record previous value if needed
                            self.record_previous_value_for_field_if_needed(field);
                            // on set pipeline
                            let mut value = value;
                            let context = Context::initial_state(self.clone())
                                .alter_key_path(path.clone())
                                .alter_value(value);
                            let result_context = field.on_set_pipeline.process(context).await;
                            match result_context.invalid_reason() {
                                Some(reason) => return Err(ActionError::unexpected_input_value_with_reason(reason, &path)),
                                None => {
                                    self.check_write_rule(key, &value, &path).await?;
                                    self.set_value_to_value_map(key, value.clone());
                                }
                            }
                        }
                    }
                }
            } else if let Some(_) = self.model().relation(key) {
                let manipulation = match value_map.get(&key.to_string()) {
                    Some(value) => value,
                    None => continue,
                };
                self.set_value_to_relation_manipulation_map(key, manipulation);
            } else if let Some(property) = self.model().property(key) {
                if value_map_keys.contains(&key) {
                    if let Some(setter) = property.setter.as_ref() {
                        let value = value_map.get(&key.to_string()).unwrap();
                        let input_result = Input::decode_field(value);
                        let value = match input_result {
                            Input::SetValue(v) => v,
                            _ => return Err(ActionError::unexpected_input_type("value", &(path + key))),
                        };
                        let ctx = Context::initial_state(self.clone())
                            .alter_value(value);
                        if let Some(reason) = setter.process(ctx).await.invalid_reason() {
                            return Err(ActionError::unexpected_input_value_with_reason(reason, &(path + key)));
                        }
                    }
                }
            }
        };
        // set flag
        self.inner.is_initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn check_model_write_permission(&self) -> ActionResult<()> {
        let is_new = self.is_new();
        if let Some(permission) = self.model().permission() {
            if let Some(can) = if is_new { permission.can_create() } else { permission.can_update() } {
                let ctx = Context::initial_state(self.clone()).alter_value_with_identity();
                let result_ctx = can.process(ctx).await;
                if !result_ctx.is_valid() {
                    return Err(ActionError::permission_denied(if is_new { "create" } else { "update" }));
                }
            }
        }
        Ok(())
    }

    async fn check_field_write_permission(&self, field: &Field) -> ActionResult<()> {
        let is_new = self.is_new();
        if let Some(permission) = field.permission() {
            if let Some(can) = if is_new { permission.can_create() } else { permission.can_update() } {
                let ctx = Context::initial_state(self.clone()).alter_value_with_identity();
                let result_ctx = can.process(ctx).await;
                if !result_ctx.is_valid() {
                    return Err(ActionError::permission_denied(if is_new { "create" } else { "update" }));
                }
            }
        }
        Ok(())
    }

    fn record_previous_value_for_field_if_needed(&self, field: &Field) {
        if !self.is_new() && field.previous_value_rule == PreviousValueRule::Keep {
            if self.inner.previous_value_map.lock().unwrap().get(field.name()).is_none() {
                self.inner.previous_value_map.lock().unwrap().insert(field.name().to_string(), self.get_value(field.name()).unwrap());
            }
        }
    }

    async fn check_write_rule(&self, key: impl AsRef<str>, value: &Value, path: &KeyPath<'_>) -> ActionResult<()> {
        let field = self.model().field(key.as_ref()).unwrap();
        let is_new = self.is_new();
        let valid = match &field.write_rule {
            WriteRule::NoWrite => false,
            WriteRule::Write => true,
            WriteRule::WriteOnCreate => is_new,
            WriteRule::WriteOnce => if is_new { true } else { self.get_value(key.as_ref()).unwrap().is_null() },
            WriteRule::WriteNonNull => if is_new { true } else { !value.is_null() },
            WriteRule::WriteIf(pipeline) => {
                let context = Context::initial_state(self.clone())
                    .alter_key_path(path![key.as_ref()])
                    .alter_value(value.clone());
                let result_context = pipeline.process(context).await;
                result_context.is_valid()
            }
        };
        if valid {
            Err(ActionError::unexpected_input_key(key.as_ref(), path))
        } else {
            Ok(())
        }
    }

    fn set_value_to_atomic_updator_map(&self, key: &str, value: Value) {
        self.inner.atomic_updator_map.lock().unwrap().insert(key.to_string(), value);
        if !self.is_new() {
            self.inner.is_modified.store(true, Ordering::SeqCst);
            self.inner.modified_fields.lock().unwrap().insert(key.to_string());
        }
    }

    pub fn set(&self, key: impl AsRef<str>, value: impl Into<Value>) -> ActionResult<()> {
        self.set_value(key, value.into())
    }

    pub fn set_value(&self, key: impl AsRef<str>, value: Value) -> ActionResult<()> {
        let model_keys = self.model().save_keys();
        if !model_keys.contains(&key.as_ref().to_string()) {
            return Err(ActionError::invalid_key(key, self.model()));
        }
        self.set_value_to_value_map(key, value);
        Ok(())
    }

    pub async fn set_property(&self, key: impl AsRef<str>, value: impl Into<Value>) -> ActionResult<()> {
        let property = self.model().property(key.as_ref()).unwrap();
        let setter = property.setter.as_ref().unwrap();
        let ctx = Context::initial_state(self.clone())
            .alter_value(value.into());
        let _ = setter.process(ctx).await;
        Ok(())
    }

    fn set_value_to_value_map(&self, key: &str, value: Value) {
        if value.is_null() {
            self.inner.value_map.lock().unwrap().remove(key);
        } else {
            self.inner.value_map.lock().unwrap().insert(key.to_string(), value);
        }
        if !self.is_new() {
            self.inner.is_modified.store(true, Ordering::SeqCst);
            self.inner.modified_fields.lock().unwrap().insert(key.to_string());
            if let Some(properties) = self.model().field_property_map().get(&key) {
                for property in properties {
                    self.inner.modified_fields.lock().unwrap().insert(property.clone());
                    self.inner.cached_property_map.lock().unwrap().remove(property);
                }
            }
        }
    }

    pub fn get_relation_object(&self, key: impl AsRef<str>) -> ActionResult<Option<Object>> {
        let key = key.as_ref();
        let model_keys = self.model().all_keys();
        if !model_keys.contains(&key.to_string()) {
            return Err(ActionError::invalid_key(key, self.model()));
        }
        match self.inner.relation_query_map.lock().unwrap().get(key) {
            Some(list) => Ok(list.get(0).cloned()),
            None => Ok(None)
        }
    }

    pub fn get_relation_vec(&self, key: impl AsRef<str>) -> ActionResult<Vec<Object>> {
        let key = key.as_ref();
        let model_keys = self.model().all_keys();
        if !model_keys.contains(&key.to_string()) {
            return Err(ActionError::invalid_key(key, self.model()));
        }
        match self.inner.relation_query_map.lock().unwrap().get(key) {
            Some(list) => Ok(list.clone()),
            None => Ok(vec![]),
        }
    }

    pub async fn get_property<T>(&self, key: impl AsRef<str>) -> Result<T, ActionError> where T: From<Value> {
        let property = self.model().property(key.as_ref()).unwrap();
        if property.cached {
            if let Some(value) = self.inner.cached_property_map.lock().unwrap().get(key.as_ref()) {
                return Ok(value.clone().into());
            }
        }
        let getter = property.getter.as_ref().unwrap();
        let ctx = Context::initial_state(self.clone());
        let value = getter.process(ctx).await.value;
        if property.cached {
            self.inner.cached_property_map.lock().unwrap().insert(key.as_ref().to_string(), value.clone());
        }
        Ok(value.into())
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

    pub(crate) fn get_previous_value(&self, key: impl AsRef<str>) -> Result<Value, ActionError> {
        let key = key.as_ref();
        let model_keys = self.model().all_keys();
        if !model_keys.contains(&key.to_string()) {
            let model = self.model();
            return Err(ActionError::invalid_key(key, model));
        }
        let map = self.inner.previous_value_map.lock().unwrap();
        match map.get(key) {
            Some(value) => Ok(value.clone()),
            None => Ok(Value::Null),
        }
    }

    fn get_value_map_value(&self, key: impl AsRef<str>) -> Value {
        match self.inner.value_map.lock().unwrap().get(&key.as_ref().to_string()) {
            Some(value) => value.clone(),
            None => Value::Null,
        }
    }

    pub fn get_value(&self, key: impl AsRef<str>) -> Result<Value, ActionError> {
        let model_keys = self.model().all_keys();
        if !model_keys.contains(&key.to_string()) {
            return Err(ActionError::invalid_key(key, self.model()));
        }
        Ok(self.get_value_map_value(key))
    }

    pub fn set_select(&self, select: Option<&Value>) -> ActionResult<()> {
        if select.is_none() {
            return Ok(());
        }
        let mut true_list: Vec<&str> = vec![];
        let mut false_list: Vec<&str> = vec![];
        let map = select.unwrap().as_hashmap().unwrap();
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
                if let Some(field) = self.model().field(k) {
                    if !false_list.contains(&&***&k) {
                        result.push(field.name.clone());
                    }
                } else if let Some(property) = self.model().property(k) {
                    if !false_list.contains(&&***&k) {
                        result.push(property.name.clone());
                    }
                }
            });
            *self.inner.selected_fields.lock().unwrap() = result;
            return Ok(());
        } else {
            // true
            let mut result: Vec<String> = vec![];
            self.model().all_keys().iter().for_each(|k| {
                if let Some(field) = self.model().field(k) {
                    if true_list.contains(&&***&k) {
                        result.push(field.name.clone());
                    }
                } else if let Some(property) = self.model().property(k) {
                    if true_list.contains(&&***&k) {
                        result.push(property.name.clone());
                    }
                }
            });
            *self.inner.selected_fields.lock().unwrap() = result;
            return Ok(());
        }
    }

    #[async_recursion(?Send)]
    pub(crate) async fn apply_on_save_pipeline_and_validate_required_fields(&self, path: &KeyPath) -> ActionResult<()> {
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
                let context = Context::initial_state(self.clone())
                    .alter_value(initial_value)
                    .alter_key_path(path![field.name.as_str()]);
                let result_ctx = field.perform_on_save_callback(context).await;
                match result_ctx.invalid_reason() {
                    Some(reason) => {
                        return Err(ActionError::unexpected_input_value_with_reason(reason, &(path + key)));
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
            if let Some(field) = self.model().field(key) {
                if field.auto || field.auto_increment {
                    continue
                }
                match &field.optionality {
                    Optionality::Optional => (),
                    Optionality::Required => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            return Err(ActionError::missing_required_input(key, path));
                        }
                    }
                    Optionality::PresentWith(keys) => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            for key in keys {
                                let value_at_key = self.get_value(key).unwrap();
                                if !value_at_key.is_null() {
                                    return Err(ActionError::missing_required_input(key, path))
                                }
                            }
                        }
                    }
                    Optionality::PresentWithout(keys) => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            for key in keys {
                                let value_at_key = self.get_value(key).unwrap();
                                if !value_at_key.is_null() {
                                    continue;
                                }
                                return Err(ActionError::missing_required_input(key, path))
                            }
                        }
                    }
                    Optionality::PresentIf(pipeline) => {
                        let value = self.get_value(key).unwrap();
                        if value.is_null() {
                            let ctx = Context::initial_state(self.clone());
                            let invalid = pipeline.process(ctx).await.invalid_reason().is_some();
                            if invalid {
                                return Err(ActionError::missing_required_input(key, path))
                            }
                        }
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
    pub(crate) async fn delete_from_database(&self, session: Arc<dyn SaveSession>, no_recursive: bool) -> ActionResult<()> {
        let model = self.model();
        let graph = self.graph();
        // check deny first
        for relation in model.relations() {
            if relation.through().is_some() {
                continue
            }
            let (opposite_model, opposite_relation) = graph.opposite_relation(relation);
            if let Some(opposite_relation) = opposite_relation {
                if opposite_relation.delete_rule() == Deny {
                    let finder = self.finder_for_relation(relation);
                    let count = graph.count(opposite_model.name(), &finder).await.unwrap();
                    if count > 0 {
                        return Err(ActionError::deletion_denied(relation.name()));
                    }
                }
            }
        }
        // real delete
        let connector = self.graph().connector();
        connector.delete_object(self).await?;
        // nullify and cascade
        for relation in model.relations() {
            if relation.through().is_some() {
                continue
            }
            let (opposite_model, opposite_relation) = graph.opposite_relation(relation);
            if let Some(opposite_relation) = opposite_relation {
                match opposite_relation.delete_rule() {
                    DeleteRule::Default => {}, // do nothing
                    DeleteRule::Deny => {}, // done before
                    DeleteRule::Nullify => {
                        if !opposite_relation.has_foreign_key() {
                            continue
                        }
                        let finder = self.finder_for_relation(relation);
                        graph.batch(opposite_model.name(), &finder, Env::new(Source::CustomCode, Intent::Delete), |object| async move {
                            for key in opposite_relation.fields() {
                                object.set_value(key, Value::Null)?;
                            }
                            object.save_with_session_and_path(self.graph().connector().new_save_session(), no_recursive, &path![]).await?;
                            Ok(())
                        }).await?;
                    },
                    DeleteRule::Cascade => {
                        let finder = self.finder_for_relation(relation);
                        graph.batch(opposite_model.name(), &finder, Env::new(Source::CustomCode, Intent::Delete), |object| async move {
                            object.delete_from_database(self.graph().connector().new_save_session(), no_recursive).await?;
                            Ok(())
                        }).await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_manipulation(&self, relation: &Relation, manipulation: &RelationManipulation, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let _many = relation.is_vec();
        use RelationManipulation::*;
        let graph = self.graph();
        match manipulation {
            Create(entry) => {
                let env = self.env().nested(Intent::NestedCreate);
                let new_object = graph.new_object(relation.model(), env)?;
                new_object.set_tson(entry).await?;
                if relation.through().is_none() {
                    self.link_connect(&new_object, relation, session.clone()).await?;
                }
                new_object.save_with_session_and_path(session.clone(), false, &(path + relation.name())).await?;
                self.link_connect(&new_object, relation, session.clone()).await?;
            }
            ConnectOrCreate(entry) => {
                let r#where = entry.as_hashmap().unwrap().get("where").unwrap();
                let create = entry.as_hashmap().unwrap().get("create").unwrap();
                let env = self.env().nested(Intent::NestedConnectOrCreateActuallyConnect);
                let unique_result = graph.find_unique(relation.model(), &tson!({"where": r#where}), true, env).await;
                match unique_result {
                    Ok(exist_object) => {
                        if relation.through().is_none() {
                            self.link_connect(&exist_object, relation, session.clone()).await?;
                        }
                        exist_object.save_with_session_and_path(session.clone(), false, &(path + relation.name())).await?;
                        self.link_connect(&exist_object, relation, session.clone()).await?;
                    }
                    Err(_err) => {
                        let env = self.env().nested(Intent::NestedConnectOrCreateActuallyCreate);
                        let new_obj = graph.new_object(relation.model(), env)?;
                        new_obj.set_tson(create).await?;
                        if relation.through().is_none() {
                            self.link_connect(&new_obj, relation, session.clone()).await?;
                        }
                        new_obj.save_with_session_and_path(session.clone(), false, &(path + relation.name())).await?;
                        self.link_connect(&new_obj, relation, session.clone()).await?;
                    }
                }
            }
            Connect(entry) | Set(entry) => {
                let unique_query = tson!({"where": entry});
                let env = self.env().nested(Intent::NestedConnect);
                let exist_object = graph.find_unique(relation.model(), &unique_query, true, env).await?;
                if relation.through().is_none() {
                    self.link_connect(&exist_object, relation, session.clone()).await?;
                }
                exist_object.save_with_session_and_path(session.clone(), false, &(path + relation.name())).await?;
                self.link_connect(&exist_object, relation, session.clone()).await?;
            }
            Update(entry) => {
                let env = self.env().nested(Intent::NestedUpdate);
                let r#where = entry.get("where").unwrap();
                let update = entry.get("update").unwrap();
                let model_name = relation.model();
                let the_object = graph.find_unique(model_name, &tson!({"where": r#where}), true, env).await;
                if the_object.is_err() {
                    return Err(ActionError::object_not_found());
                }
                let the_object = the_object.unwrap();
                the_object.set_tson(update).await?;
                the_object.save_with_session_and_path(session, false, &(path + relation.name())).await?;
            }
            Upsert(entry) => {
                let env = self.env().nested(Intent::NestedUpsertActuallyUpdate);
                let r#where = entry.as_hashmap().unwrap().get("where").unwrap();
                let create = entry.as_hashmap().unwrap().get("create").unwrap();
                let update = entry.as_hashmap().unwrap().get("update").unwrap();
                let the_object = graph.find_unique(relation.model(), &tson!({"where": r#where}), true, env).await;
                match the_object {
                    Ok(obj) => {
                        obj.set_tson(update).await?;
                        obj.save_with_session_and_path(session, false, &(path + relation.name())).await?;
                    }
                    Err(_) => {
                        let env = self.env().nested(Intent::NestedUpsertActuallyCreate);
                        let new_obj = graph.new_object(relation.model(), env)?;
                        new_obj.set_tson(create).await?;
                        if relation.through().is_none() {
                            self.link_connect(&new_obj, relation, session.clone()).await?;
                        }
                        new_obj.save_with_session_and_path(session.clone(), false, &(path + relation.name())).await?;
                        self.link_connect(&new_obj, relation, session.clone()).await?;
                    }
                }
            }
            Disconnect(entry) => {
                let env = self.env().nested(Intent::NestedDisconnect);
                let unique_query = tson!({"where": entry});
                let object_to_disconnect = graph.find_unique(relation.model(), &unique_query, true, env).await?;
                self.link_disconnect(&object_to_disconnect, relation, session.clone()).await?;
            }
            Delete(entry) => {
                let env = self.env().nested(Intent::NestedDelete);
                let r#where = entry;
                let the_object = graph.find_unique(relation.model(), &tson!({"where": r#where}), true, env).await;
                if the_object.is_err() {
                    return Err(ActionError::object_not_found());
                }
                let the_object = the_object.unwrap();
                self.link_disconnect(&the_object, relation, session.clone()).await?;
                the_object.delete().await?;
            }
        }
        Ok(())
    }

    #[async_recursion(?Send)]
    async fn save_to_database(&self, session: Arc<dyn SaveSession>) -> ActionResult<()> {
        let connector = self.graph().connector();
        connector.save_object(self, session).await?;
        self.clear_new_state();
        Ok(())
    }

    pub(crate) async fn link_connect(&self, obj: &Object, relation: &Relation, session: Arc<dyn SaveSession>) -> ActionResult<()> {
        match &relation.through() {
            Some(through) => { // with join table
                let relation_model = self.graph().model(through).unwrap();
                let env = self.env().nested(Intent::NestedJoinTableRecordCreate);
                let relation_object = self.graph().new_object(through, env)?;
                relation_object.set_tson(&tson!({})).await?;
                let local_relation_name = relation.fields().get(0).unwrap();
                let foreign_relation_name = relation.references().get(0).unwrap();
                let local_relation = relation_model.relation(local_relation_name).unwrap();
                let foreign_relation = relation_model.relation(foreign_relation_name).unwrap();
                for (index, field_name) in local_relation.fields().iter().enumerate() {
                    let local_field_name = local_relation.references().get(index).unwrap();
                    let val = self.get_value(local_field_name).unwrap();
                    relation_object.set_value(field_name, val.clone()).unwrap();
                }
                for (index, field_name) in foreign_relation.fields().iter().enumerate() {
                    let foreign_field_name = foreign_relation.references().get(index).unwrap();
                    let val = obj.get_value(foreign_field_name).unwrap();
                    relation_object.set_value(field_name, val.clone()).unwrap();
                }
                relation_object.save_with_session_and_path(session.clone(), &path![]).await?;
            }
            None => { // no join table
                for (index, reference) in relation.references().iter().enumerate() {
                    let field_name = relation.fields().get(index).unwrap();
                    if relation.is_vec() {
                        // if relation is vec, otherwise must have saved the value
                        let local_value = self.get_value(field_name)?;
                        if !local_value.is_null() {
                            obj.set_value(reference, local_value.clone())?;
                        }
                    } else {
                        // array are removed here, see if bug happens
                        // write on their side since it's primary
                        for item in self.model().primary_index().items() {
                            if item.field_name() == field_name {
                                let local_value = self.get_value(field_name)?;
                                if !local_value.is_null() {
                                    obj.set_value(reference, local_value.clone())?;
                                }
                                break;
                            }
                        }
                        // write on our side since it's not primary
                        let foreign_value = obj.get_value(reference)?;
                        if !foreign_value.is_null() {
                            self.set_value(field_name, foreign_value.clone())?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn link_disconnect(&self, obj: &Object, relation: &Relation, session: Arc<dyn SaveSession>) -> ActionResult<()> {
        match relation.through() {
            Some(through) => { // with join table
                let env = self.env().nested(Intent::NestedJoinTableRecordDelete);
                let relation_model = self.graph().model(through).unwrap();
                let mut finder: HashMap<String, Value> = HashMap::new();
                let local_relation_name = relation.fields().get(0).unwrap();
                let foreign_relation_name = relation.references().get(0).unwrap();
                let local_relation = relation_model.relation(local_relation_name).unwrap();
                let foreign_relation = relation_model.relation(foreign_relation_name).unwrap();
                for (index, field_name) in local_relation.fields().iter().enumerate() {
                    let local_field_name = local_relation.references().get(index).unwrap();
                    let val = self.get_value(local_field_name).unwrap();
                    finder.insert(field_name.to_string(), val);
                }
                for (index, field_name) in foreign_relation.fields().iter().enumerate() {
                    let foreign_field_name = foreign_relation.references().get(index).unwrap();
                    let val = obj.get_value(foreign_field_name).unwrap();
                    finder.insert(field_name.to_string(), val);
                }
                let relation_object = self.graph().find_unique(through, &tson!({"where": finder}), true, env).await?;
                relation_object.delete_from_database(session.clone(), false).await?;
            }
            None => { // no join table
                for (index, reference) in relation.references().iter().enumerate() {
                    let field_name = relation.fields().get(index).unwrap();
                    let local_value = self.get_value(field_name)?;
                    let foreign_value = obj.get_value(reference)?;
                    let local_field = self.model().field(field_name).unwrap();
                    let foreign_field = obj.model().field(reference).unwrap();
                    if !local_value.is_null() && !foreign_value.is_null() {
                        if local_field.optionality.is_optional() {
                            self.set_value(field_name, Value::Null)?;
                        } else if foreign_field.optionality.is_optional() {
                            obj.set_value(reference, Value::Null)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn before_save_callback_check(&self) -> ActionResult<()> {
        let inside_before_callback = self.inner.inside_before_save_callback.load(Ordering::SeqCst);
        if inside_before_callback {
            return Err(ActionError::invalid_operation("Save called inside before callback."));
        }
        Ok(())
    }

    #[async_recursion(?Send)]
    pub(crate) async fn save_with_session_and_path(&self, session: Arc<dyn SaveSession>, path: &KeyPath) -> ActionResult<()> {
        // check if it's inside before callback
        self.before_save_callback_check()?;
        let is_new = self.is_new();
        // perform relation manipulations (has foreign key)
        self.perform_relation_manipulations(|r| r.has_foreign_key())?;
        // validate and save
        let is_modified = self.is_modified();
        if is_modified || is_new {
            // apply pipeline
            self.apply_on_save_pipeline_and_validate_required_fields(path).await?;
            self.trigger_before_save_callbacks().await?;
            if !self.model().r#virtual() {
                self.save_to_database(session.clone()).await?;
            }
        }
        // perform relation manipulations (doesn't have foreign key)
        self.perform_relation_manipulations(|r| !r.has_foreign_key())?;
        // clear properties
        self.clear_state();
        if is_modified || is_new {
            self.trigger_after_save_callbacks().await?;
        }
        Ok(())
    }

    pub async fn save(&self) -> ActionResult<()> {
        let session = self.graph().connector().new_save_session();
        self.save_with_session_and_path(session, &path![]).await
    }

    async fn trigger_before_save_callbacks(&self) -> ActionResult<()> {
        let model = self.model();
        let pipeline = model.before_save_pipeline();
        let context = Context::initial_state(self.clone());
        let _result = pipeline.process(context).await;
        Ok(())
    }

    async fn trigger_after_save_callbacks(&self) -> ActionResult<()> {
        let inside_after_save_callback = self.inner.inside_after_save_callback.load(Ordering::SeqCst);
        if inside_after_save_callback {
            return Ok(());
        }
        self.inner.inside_after_save_callback.store(true, Ordering::SeqCst);
        let model = self.model();
        let pipeline = model.after_save_pipeline();
        let context = Context::initial_state(self.clone());
        let _result = pipeline.process(context).await;
        self.inner.inside_after_save_callback.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub async fn delete(&self) -> ActionResult<()> {
        self.delete_from_database(self.graph().connector().new_save_session(), false).await
    }

    pub(crate) async fn to_json(&self) -> ActionResult<Value> {
        // test for model permission
        if let Some(permission) = self.model().permission() {
            if let Some(can_read) = permission.can_read() {
                let ctx = Context::initial_state(self.clone()).alter_value_with_identity();
                let result_ctx = can_read.process(ctx).await;
                if !result_ctx.is_valid() {
                    return Err(ActionError::permission_denied("read"));
                }
            }
        }
        // output
        let select_list = self.inner.selected_fields.lock().unwrap().clone();
        let select_filter = if select_list.is_empty() { false } else { true };
        let mut map: HashMap<String, Value> = HashMap::new();
        let keys = self.model().output_keys();
        for key in keys {
            if (!select_filter) || (select_filter && select_list.contains(key)) {
                let mut value = self.get_value(key).unwrap();
                if let Some(field) = self.model().field(key) {
                    // test for field permission
                    if let Some(permission) = field.permission() {
                        if let Some(can_read) = permission.can_read() {
                            let ctx = Context::initial_state(self.clone()).alter_value_with_identity();
                            let result_ctx = can_read.process(ctx).await;
                            if !result_ctx.is_valid() {
                                continue;
                            }
                        }
                    }
                    let context = Context::initial_state(self.clone())
                        .alter_value(value)
                        .alter_key_path(path![key.as_str()]);
                    let result_ctx = field.perform_on_output_callback(context).await;
                    value = result_ctx.value
                } else if let Some(property) = self.model().property(key) {
                    if let Some(getter) = &property.getter {
                        let ctx = Context::initial_state(self.clone());
                        value = getter.process(ctx).await.value
                    }
                }
                if !value.is_null() {
                    if value.is_hashmap() {
                        map.insert(key.to_string(), value.to_object_json_value().await.unwrap());
                    } else if value.is_object_vec() {
                        map.insert(key.to_string(), value.to_object_vec_json_value().await.unwrap());
                    } else {
                        map.insert(key.to_string(), value);
                    }
                }
            }
        }
        return Ok(Value::HashMap(map))
    }

    pub fn is_new(&self) -> bool {
        self.inner.is_new.load(Ordering::SeqCst)
    }

    pub fn is_modified(&self) -> bool {
        self.inner.is_modified.load(Ordering::SeqCst)
    }

    fn insert_relation_manipulation(&self, key: &str, manipulation: RelationManipulation) {
        if self.inner.relation_mutation_map.lock().unwrap().get(key).is_none() {
            self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), vec![]);
        }
        let mut relation_mutation_map = self.inner.relation_mutation_map.lock().unwrap();
        let objects = relation_mutation_map.get_mut(key).unwrap();
        objects.push(manipulation);
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

    pub(crate) fn identifier(&self) -> Value {
        let model = self.model();
        let mut identifier: HashMap<String, Value> = HashMap::new();
        for item in model.primary_index().items() {
            let val = self.get_value(item.field_name()).unwrap();
            identifier.insert(item.field_name().to_owned(), val.clone());
        }
        Value::HashMap(identifier)
    }

    fn perform_relation_manipulations<F: Fn(&Relation) -> bool>(&self, f: F) -> ActionResult<()> {
        for relation in self.model().relations() {
            if f(relation) {
                let many = relation.is_vec();
                let name = relation.name();
                let map = self.inner.relation_mutation_map.lock().unwrap();
                let vec_option = map.get(name);
                match vec_option {
                    None => {},
                    Some(vec) => {
                        for manipulation in vec {
                            self.handle_manipulation(relation, manipulation, session.clone(), path).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn refreshed(&self, include: Option<&Value>, select: Option<&Value>) -> Result<Object, ActionError> {
        if self.model().r#virtual() {
            self.set_select(select).unwrap();
            return Ok(self.clone())
        }
        let graph = self.graph();
        let mut finder = tson!({
            "where": self.json_identifier(),
        });
        if include.is_some() {
            finder.as_hashmap_mut().unwrap().insert("include".to_string(), include.unwrap().clone());
        }
        if select.is_some() {
            finder.as_hashmap_mut().unwrap().insert("select".to_string(), select.unwrap().clone());
        }
        graph.find_unique(self.model().name(), &finder, false, self.env().clone()).await
    }

    pub(crate) fn finder_for_relation(&self, relation: &Relation) -> Value {
        let mut finder_where = tson!({});
        for (index, local_field_name) in relation.fields().iter().enumerate() {
            let foreign_field_name = relation.references().get(index).unwrap();
            let value = self.get_value(local_field_name).unwrap();
            let json_value = value;
            finder_where.as_hashmap_mut().unwrap().insert(foreign_field_name.to_owned(), json_value);
        }
        tson!({ "where": finder_where })
    }

    pub async fn fetch_relation_object(&self, key: impl AsRef<str>, find_unique_arg: Option<&Value>) -> Result<Option<Object>, ActionError> {
        // get relation
        let model = self.model();
        let relation = model.relation(key.as_ref());
        if relation.is_none() {
            // todo() err here
        }
        let relation = relation.unwrap();
        let mut finder = self.finder_for_relation(relation);
        if let Some(find_unique_arg) = find_unique_arg {
            if let Some(include) = find_unique_arg.get("include") {
                finder.as_hashmap_mut().unwrap().insert("include".to_owned(), include.clone());
            }
            if let Some(select) = find_unique_arg.get("select") {
                finder.as_hashmap_mut().unwrap().insert("select".to_owned(), select.clone());
            }
        }
        let relation_model_name = relation.model();
        let graph = self.graph();
        let env = self.env().nested(Intent::NestedIncluded);
        match graph.find_unique(relation_model_name, &finder, false, env).await {
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

    pub async fn fetch_relation_objects(&self, key: impl AsRef<str>, find_many_arg: Option<&Value>) -> Result<Vec<Object>, ActionError> {
        // get relation
        let model = self.model();
        let relation = model.relation(key.as_ref());
        if relation.is_none() {
            // todo() err here
        }
        let relation = relation.unwrap();
        let empty = tson!({});
        let include_inside = if find_many_arg.is_some() {
            find_many_arg.unwrap()
        } else {
            &empty
        };
        let env = self.env().nested(Intent::NestedIncluded);
        if let Some(_join_table) = relation.through() {
            let identifier = self.json_identifier();
            let new_self = self.graph().find_unique(model.name(), &tson!({
                "where": identifier,
                "include": {
                    key.as_ref(): include_inside
                }
            }), false, env).await?;
            let vec = new_self.inner.relation_query_map.lock().unwrap().get(key.as_ref()).unwrap().clone();
            self.inner.relation_query_map.lock().unwrap().insert(key.as_ref().to_string(), vec);
            Ok(self.inner.relation_query_map.lock().unwrap().get(key.as_ref()).unwrap().clone())
        } else {
            let mut finder = tson!({});
            if let Some(find_many_arg) = find_many_arg {
                for (k, v) in find_many_arg.as_hashmap().unwrap().iter() {
                    finder.as_hashmap_mut().unwrap().insert(k.clone(), v.clone());
                }
            }
            if finder.as_hashmap().unwrap().get("where").is_none() {
                finder.as_hashmap_mut().unwrap().insert("where".to_string(), tson!({}));
            }
            for (index, local_field_name) in relation.fields().iter().enumerate() {
                let foreign_field_name = relation.references().get(index).unwrap();
                let value = self.get_value(local_field_name).unwrap();
                if value == Value::Null {
                    return Ok(vec![]);
                }
                let json_value = value;
                finder.as_hashmap_mut().unwrap().get_mut("where").unwrap().as_hashmap_mut().unwrap().insert(foreign_field_name.to_owned(), json_value);
            }
            let relation_model_name = relation.model();
            let graph = self.graph();
            let results = graph.find_many(relation_model_name, &finder, false, env).await?;
            self.inner.relation_query_map.lock().unwrap().insert(key.as_ref().to_string(), results.clone());
            Ok(results)
        }
    }

    pub(crate) fn keys_for_save(&self) -> Vec<&str> {
        if self.is_new() {
            self.model().save_keys().iter().map(|k| k.as_str()).collect()
        } else {
            self.model().save_keys().iter().filter(|k| {
                self.inner.modified_fields.lock().unwrap().contains(&k.to_string()) ||
                    self.inner.atomic_updator_map.lock().unwrap().contains_key(&k.to_string())
            }).map(|k| k.as_str()).collect()
        }
    }

    pub(crate) fn env(&self) -> &Env {
        &self.inner.env
    }

    // pub(crate) fn obsolete_code() {
    //     match key {
    //         "create" | "createMany" => {
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 self.insert_relation_manipulation(key, RelationManipulation::Create(entry.clone()));
    //             }
    //         }
    //         "connect" => {
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 self.insert_relation_manipulation(key, RelationManipulation::Connect(entry.clone()));
    //             }
    //         }
    //         "set" => {
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 self.insert_relation_manipulation(key, RelationManipulation::Set(entry.clone()));
    //             }
    //         }
    //         "connectOrCreate" => {
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 self.insert_relation_manipulation(key, RelationManipulation::ConnectOrCreate(entry.clone()));
    //             }
    //         }
    //         "disconnect" => {
    //             if self.is_new() {
    //                 return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //             }
    //             let graph = self.graph();
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 let model = graph.model(relation.model()).unwrap();
    //                 if !relation.is_vec() && (relation.is_required()) {
    //                     return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //                 }
    //                 let opposite_relation = model.relations().iter().find(|r| {
    //                     r.fields() == relation.references() && r.references() == relation.fields()
    //                 });
    //                 if opposite_relation.is_some() {
    //                     let opposite_relation = opposite_relation.unwrap();
    //                     if !opposite_relation.is_vec() && (opposite_relation.is_required()) {
    //                         return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //                     }
    //                 }
    //                 self.insert_relation_manipulation(key, RelationManipulation::Disconnect(entry.clone()));
    //             }
    //         }
    //         "update" | "updateMany" => {
    //             if !relation.is_vec() && command == "updateMany" {
    //                 return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //             }
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 self.insert_relation_manipulation(key, RelationManipulation::Update(entry.clone()));
    //             }
    //         }
    //         "upsert" => {
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 self.insert_relation_manipulation(key, RelationManipulation::Upsert(entry.clone()));
    //             }
    //         }
    //         "delete" | "deleteMany" => {
    //             if self.is_new() {
    //                 return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //             }
    //             if !relation.is_vec() && command == "deleteMany" {
    //                 return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //             }
    //             let graph = self.graph();
    //             let model_name = relation.model();
    //             let model = graph.model(model_name).unwrap();
    //             if !relation.is_vec() && (relation.is_required()) {
    //                 return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //             }
    //             let opposite_relation = model.relations().iter().find(|r| {
    //                 r.fields() == relation.references() && r.references() == relation.fields()
    //             });
    //             if opposite_relation.is_some() {
    //                 let opposite_relation = opposite_relation.unwrap();
    //                 if !opposite_relation.is_vec() && (opposite_relation.is_required()) {
    //                     return Err(ActionError::unexpected_input_value(key.as_str(), &(path + key.as_str())));
    //                 }
    //             }
    //             let entries = input_to_vec(command_input, path)?;
    //             for entry in entries {
    //                 self.insert_relation_manipulation(key, RelationManipulation::Delete(entry.clone()));
    //             }
    //         }
    //         _ => {
    //             return Err(ActionError::unexpected_input_key(command, &(path + key + command)));
    //         }
    //     }
    // }
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
