use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use key_path::{KeyPath, path};
use async_recursion::async_recursion;
use maplit::hashmap;
use indexmap::IndexMap;
use crate::core::env::Env;
use crate::core::env::intent::Intent;
use crate::core::env::source::Source;
use crate::core::pipeline::argument::Argument;
use crate::core::field::{Field, PreviousValueRule};
use crate::core::field::optionality::Optionality;
use crate::core::input::Input;
use crate::core::input::Input::{AtomicUpdator, SetValue};
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::relation::Relation;
use crate::core::connector::SaveSession;
use crate::core::pipeline::context::{Context};
use crate::core::tson::Value;
use crate::core::error::{ActionError, ActionErrorType};
use crate::core::field::write_rule::WriteRule;
use crate::core::relation::delete_rule::DeleteRule;
use crate::core::relation::delete_rule::DeleteRule::Deny;
use crate::core::result::ActionResult;
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
                            let context = Context::initial_state(self.clone())
                                .alter_key_path(path.clone())
                                .alter_value(value);
                            let result_context = field.on_set_pipeline.process(context).await;
                            let value = result_context.value();
                            match result_context.invalid_reason() {
                                Some(reason) => return Err(ActionError::unexpected_input_value_with_reason(reason, &path)),
                                None => {
                                    self.check_write_rule(key, value, &path).await?;
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

    async fn check_model_read_permission(&self) -> ActionResult<()> {
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

    async fn check_field_read_permission(&self, field: &Field) -> bool {
        // test for field permission
        if let Some(permission) = field.permission() {
            if let Some(can_read) = permission.can_read() {
                let ctx = Context::initial_state(self.clone()).alter_value_with_identity();
                let result_ctx = can_read.process(ctx).await;
                if !result_ctx.is_valid() {
                    return false;
                }
            }
        }
        true
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
        if !valid {
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

    fn set_value_to_relation_manipulation_map(&self, key: &str, value: &Value) {
        self.inner.relation_mutation_map.lock().unwrap().insert(key.to_string(), value.clone());
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
        self.set_value_to_value_map(key.as_ref(), value);
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

    pub(crate) fn set_from_database_result_value(&self, value: &Value) {
        let model = self.model();
        for (k, v) in value.as_hashmap().unwrap() {
            if let Some(_) = model.field(k) {
                self.set_value_to_value_map(k, v.clone());
            } else if let Some(relation) = model.relation(k) {
                self.inner.relation_query_map.lock().unwrap().insert(k.to_owned(), vec![]);
                for v in v.as_vec().unwrap() {
                    let object = self.graph().new_object(relation.model(), self.env().alter_intent(Intent::NestedIncluded)).unwrap();
                    object.set_from_database_result_value(v);
                    self.inner.relation_query_map.lock().unwrap().get_mut(k).unwrap().push(object);
                }
            } else if let Some(_property) = model.property(k) {
                self.inner.cached_property_map.lock().unwrap().insert(k.to_owned(), v.clone());
            }
        }
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
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
            if let Some(properties) = self.model().field_property_map().get(key) {
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

    pub fn has_relation_fetched(&self, key: impl AsRef<str>) -> bool {
        self.inner.relation_query_map.lock().unwrap().contains_key(key.as_ref())
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

    fn get_value_map_value(&self, key: &str) -> Value {
        match self.inner.value_map.lock().unwrap().get(key) {
            Some(value) => value.clone(),
            None => Value::Null,
        }
    }

    pub fn get_value(&self, key: impl AsRef<str>) -> Result<Value, ActionError> {
        let model_keys = self.model().all_keys();
        if !model_keys.contains(&key.as_ref().to_string()) {
            return Err(ActionError::invalid_key(key, self.model()));
        }
        Ok(self.get_value_map_value(key.as_ref()))
    }

    pub(crate) fn get_atomic_updator(&self, key: &str) -> Option<Value> {
        self.inner.atomic_updator_map.lock().unwrap().get(key).cloned()
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
    pub(crate) async fn delete_from_database(&self, session: Arc<dyn SaveSession>) -> ActionResult<()> {
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
                    let finder = self.intrinsic_where_unique_for_relation(relation);
                    let count = graph.count(opposite_model.name(), &finder).await.unwrap();
                    if count > 0 {
                        return Err(ActionError::deletion_denied(relation.name()));
                    }
                }
            }
        }
        // real delete
        let connector = self.graph().connector();
        connector.delete_object(self, session.clone()).await?;
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
                        let finder = self.intrinsic_where_unique_for_relation(relation);
                        graph.batch(opposite_model.name(), &finder, Env::new(Source::CustomCode, Intent::Delete), |object| async move {
                            for key in opposite_relation.fields() {
                                object.set_value(key, Value::Null)?;
                            }
                            object.save_with_session_and_path(self.graph().connector().new_save_session(), &path![]).await?;
                            Ok(())
                        }).await?;
                    },
                    DeleteRule::Cascade => {
                        let finder = self.intrinsic_where_unique_for_relation(relation);
                        graph.batch(opposite_model.name(), &finder, Env::new(Source::CustomCode, Intent::Delete), |object| async move {
                            object.delete_from_database(self.graph().connector().new_save_session()).await?;
                            Ok(())
                        }).await?;
                    }
                }
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
        self.perform_relation_manipulations(|r| r.has_foreign_key(), session.clone(), path).await?;
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
        self.perform_relation_manipulations(|r| !r.has_foreign_key(), session.clone(), path).await?;
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
        self.delete_from_database(self.graph().connector().new_save_session()).await
    }

    #[async_recursion]
    pub(crate) async fn to_json(&self) -> ActionResult<Value> {
        // check read permission
        self.check_model_read_permission().await?;
        // output
        let select_list = self.inner.selected_fields.lock().unwrap().clone();
        let select_filter = if select_list.is_empty() { false } else { true };
        let mut map: IndexMap<String, Value> = IndexMap::new();
        let keys = self.model().output_keys();
        for key in keys {
            if (!select_filter) || (select_filter && select_list.contains(key)) {
                if let Some(field) = self.model().field(key) {
                    let mut value = self.get_value(key).unwrap();
                    if !self.check_field_read_permission(field).await {
                        continue
                    }
                    let context = Context::initial_state(self.clone())
                        .alter_value(value)
                        .alter_key_path(path![key.as_str()]);
                    let result_ctx = field.perform_on_output_callback(context).await;
                    value = result_ctx.value;
                    if !value.is_null() {
                        map.insert(key.to_string(), value);
                    }
                } else if let Some(relation) = self.model().relation(key) {
                    if self.has_relation_fetched(relation.name()) {
                        if relation.is_vec() {
                            let o = self.get_relation_object(key).unwrap();
                            match o {
                                Some(o) => {
                                    map.insert(key.to_string(), o.to_json().await.unwrap());
                                },
                                None => ()
                            };
                        } else {
                            let mut result_vec = vec![];
                            let vec = self.get_relation_vec(key).unwrap();
                            for o in vec {
                                result_vec.push(o.to_json().await?);
                            }
                            map.insert(key.to_string(), Value::Vec(result_vec));
                        }
                    }
                } else if let Some(property) = self.model().property(key) {
                    if property.cached && self.inner.cached_property_map.lock().unwrap().contains_key(key) {
                        let value = self.inner.cached_property_map.lock().unwrap().get(key).unwrap().clone();
                        if !value.is_null() {
                            map.insert(key.to_string(), value);
                        }
                    } else {
                        if let Some(getter) = &property.getter {
                            let ctx = Context::initial_state(self.clone());
                            let value = getter.process(ctx).await.value;
                            if !value.is_null() {
                                map.insert(key.to_string(), value);
                            }
                        }
                    }
                }
            }
        }
        return Ok(Value::IndexMap(map))
    }

    pub fn is_new(&self) -> bool {
        self.inner.is_new.load(Ordering::SeqCst)
    }

    pub fn is_modified(&self) -> bool {
        self.inner.is_modified.load(Ordering::SeqCst)
    }

    pub fn model(&self) -> &Model {
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

    async fn perform_relation_manipulations<F: Fn(&Relation) -> bool>(&self, f: F, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        for relation in self.model().relations() {
            if f(relation) {
                let many = relation.is_vec();
                let map = self.inner.relation_mutation_map.lock().unwrap();
                match map.get(relation.name()) {
                    None => {},
                    Some(manipulation) => {
                        if many {
                            self.perform_relation_manipulation_many(relation, manipulation, session.clone(), path).await?;
                        } else {
                            self.perform_relation_manipulation_one(relation, manipulation, session.clone(), path).await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn create_join_object(&self, object: &Object, relation: &Relation, opposite_relation: &Relation, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let join_model = self.graph().model(relation.through().unwrap()).unwrap();
        let env = self.env().alter_intent(Intent::NestedJoinTableRecordCreate);
        let join_object = self.graph().new_object(join_model.name(), env)?;
        join_object.set_tson(&tson!({})).await?; // initialize
        let local = relation.local();
        let foreign = opposite_relation.local();
        let join_local_relation = join_model.relation(local).unwrap();
        self.assign_linked_values_to_related_object(&join_object, join_local_relation);
        let join_foreign_relation = join_model.relation(foreign).unwrap();
        object.assign_linked_values_to_related_object(&join_object, join_foreign_relation);
        match join_object.save_with_session_and_path(session.clone(), path).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ActionError::unexpected_input_value_with_reason("Can't create join record.", path)),
        }
    }

    async fn delete_join_object(&self, object: &Object, relation: &Relation, opposite_relation: &Relation, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let join_model = self.graph().model(relation.through().unwrap()).unwrap();
        let env = self.env().alter_intent(Intent::NestedJoinTableRecordDelete);
        let local = relation.local();
        let foreign = opposite_relation.local();
        let join_local_relation = join_model.relation(local).unwrap();
        let join_foreign_relation = join_model.relation(foreign).unwrap();
        let local_join_relation = self.graph().opposite_relation(join_local_relation).1.unwrap();
        let foreign_join_relation = self.graph().opposite_relation(join_foreign_relation).1.unwrap();
        let where_local = self.intrinsic_where_unique_for_relation(local_join_relation);
        let where_foreign = object.intrinsic_where_unique_for_relation(foreign_join_relation);
        let mut r#where_map: HashMap<String, Value> = hashmap!{};
        r#where_map.extend(where_local.as_hashmap().cloned().unwrap());
        r#where_map.extend(where_foreign.as_hashmap().cloned().unwrap());
        let r#where = Value::HashMap(r#where_map);
        let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
            Ok(object) => object,
            Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Join object is not found.", path)),
        };
        match object.delete_from_database(session.clone()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ActionError::unexpected_input_value_with_reason("Can't delete join record.", path)),
        }
    }

    fn assign_linked_values_to_related_object(&self, object: &Object, opposite_relation: &Relation) {
        for (field, reference) in opposite_relation.iter() {
            object.set_value_to_value_map(field, self.get_value_map_value(reference));
        }
    }

    fn remove_linked_values_from_related_relation(&self, relation: &Relation) {
        for (field, _) in relation.iter() {
            self.set_value_to_value_map(field, Value::Null)
        }
    }

    fn remove_linked_values_from_related_relation_on_related_object(&self, relation: &Relation, object: &Object) {
        for (_, reference) in relation.iter() {
            object.set_value_to_value_map(reference, Value::Null)
        }
    }

    async fn link_and_save_relation_object(&self, relation: &Relation, object: &Object, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let mut linked = false;
        let (_, opposite_relation) = self.graph().opposite_relation(relation);
        if let Some(opposite_relation) = opposite_relation {
            if opposite_relation.has_foreign_key() {
                self.assign_linked_values_to_related_object(object, opposite_relation);
                linked = true;
            }
        }
        object.save_with_session_and_path(session.clone(), path).await?;
        if !linked {
            if relation.has_foreign_key() {
                object.assign_linked_values_to_related_object(object, relation);
            } else if relation.has_join_table() {
                self.create_join_object(object, relation, opposite_relation.unwrap(), session.clone(), path).await?;
            }
        }
        Ok(())
    }

    async fn nested_create_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let env = self.env().alter_intent(Intent::NestedCreate);
        let object = self.graph().new_object(relation.model(), env)?;
        object.set_tson_with_path(value, path).await?;
        self.link_and_save_relation_object(relation, &object, session.clone(), path).await
    }

    async fn nested_connect_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let env = self.env().alter_intent(Intent::NestedConnect);
        let object = match self.graph().find_unique(relation.model(), &tson!({ "where": value }), true, env).await {
            Ok(object) => object,
            Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", path)),
        };
        self.link_and_save_relation_object(relation, &object, session.clone(), path).await
    }

    async fn nested_connect_or_create_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let r#where = value.get("where").unwrap();
        let create = value.get("create").unwrap();
        let env = self.env().alter_intent(Intent::NestedConnect);
        let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
            Ok(object) => object,
            Err(_) => self.graph().new_object_with_tson_and_path(relation.model(), create, &(path + "create"), env).await?,
        };
        self.link_and_save_relation_object(relation, &object, session.clone(), path).await
    }

    fn intrinsic_where_unique_for_relation(&self, relation: &Relation) -> Value {
        Value::HashMap(relation.iter().map(|(f, r)| (r.to_owned(), self.get_value(f).unwrap())).collect())
    }

    async fn nested_disconnect_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let disconnect = value.as_bool().unwrap();
        if !disconnect { return Ok(()) }
        if relation.has_foreign_key() {
            self.remove_linked_values_from_related_relation(relation);
        } else {
            let r#where = self.intrinsic_where_unique_for_relation(relation);
            let env = self.env().alter_intent(Intent::NestedDisconnect);
            let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
                Ok(object) => object,
                Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", path)),
            };
            object.remove_linked_values_from_related_relation_on_related_object(relation, &object);
            object.save_with_session_and_path(session.clone(), path).await?;
        }
        Ok(())
    }

    async fn nested_upsert_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let mut r#where = self.intrinsic_where_unique_for_relation(relation);
        r#where.as_hashmap_mut().unwrap().extend(value.get("where").unwrap().as_hashmap().cloned().unwrap());
        let create = value.get("create").unwrap();
        let update = value.get("update").unwrap();
        let env = self.env().alter_intent(Intent::NestedUpsertActuallyUpdate);
        match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env).await {
            Ok(object) => {
                let path = path + "update";
                object.set_tson_with_path(update, &path).await?;
                object.save_with_session_and_path(session.clone(), &path).await?;
            },
            Err(_) => {
                let env = self.env().alter_intent(Intent::NestedUpsertActuallyCreate);
                let object = self.graph().new_object_with_tson_and_path(relation.model(), create, &(path + "create"), env).await?;
                self.link_and_save_relation_object(relation, &object, session.clone(), path).await?;
            },
        };
        Ok(())
    }

    async fn nested_many_disconnect_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        if relation.has_join_table() {
            let env = self.env().alter_intent(Intent::NestedJoinTableRecordDelete);
            let object = match self.graph().find_unique(relation.model(), &tson!({ "where": value }), true, env.clone()).await {
                Ok(object) => object,
                Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", path)),
            };
            self.delete_join_object(&object, relation, self.graph().opposite_relation(relation).1.unwrap(), session.clone(), path).await?;
        } else {
            let mut r#where = self.intrinsic_where_unique_for_relation(relation);
            r#where.as_hashmap_mut().unwrap().extend(value.as_hashmap().cloned().unwrap().into_iter());
            let env = self.env().alter_intent(Intent::NestedDisconnect);
            let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
                Ok(object) => object,
                Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", path)),
            };
            object.remove_linked_values_from_related_relation_on_related_object(relation, &object);
            object.save_with_session_and_path(session.clone(), path).await?;
        }
        Ok(())
    }

    async fn nested_many_update_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let mut r#where = self.intrinsic_where_unique_for_relation(relation);
        r#where.as_hashmap_mut().unwrap().extend(value.get("where").unwrap().as_hashmap().cloned().unwrap());
        let env = self.env().alter_intent(Intent::NestedUpdate);
        let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
            Ok(object) => object,
            Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", &(path + "where"))),
        };
        object.set_tson_with_path(value.get("update").unwrap(), &(path + "update")).await?;
        object.save_with_session_and_path(session.clone(), path).await?;
        Ok(())
    }

    async fn nested_many_update_many_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let mut r#where = self.intrinsic_where_unique_for_relation(relation);
        r#where.as_hashmap_mut().unwrap().extend(value.get("where").unwrap().as_hashmap().cloned().unwrap());
        let env = self.env().alter_intent(Intent::NestedUpdate);
        let update = value.get("update").unwrap();
        let objects = self.graph().find_many(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await.unwrap();
        for object in objects {
            object.set_tson_with_path(update, path).await?;
            object.save_with_session_and_path(session.clone(), path).await?;
        }
        Ok(())
    }

    async fn nested_update_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let r#where = self.intrinsic_where_unique_for_relation(relation);
        let env = self.env().alter_intent(Intent::NestedUpdate);
        let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
            Ok(object) => object,
            Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", path)),
        };
        object.set_tson_with_path(value, path).await?;
        object.save_with_session_and_path(session.clone(), path).await?;
        Ok(())
    }

    async fn nested_delete_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        let delete = value.as_bool().unwrap();
        if !delete { return Ok(()) }
        if relation.has_foreign_key() {
            self.remove_linked_values_from_related_relation(relation);
        }
        let r#where = self.intrinsic_where_unique_for_relation(relation);
        let env = self.env().alter_intent(Intent::NestedDelete);
        let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
            Ok(object) => object,
            Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", path)),
        };
        object.delete_from_database(session.clone()).await?;
        if relation.has_join_table() {
            let opposite_relation = self.graph().opposite_relation(relation).1.unwrap();
            self.delete_join_object(&object, relation, opposite_relation, session.clone(), path).await?;
        }
        Ok(())
    }

    async fn nested_many_delete_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        if relation.has_foreign_key() {
            self.remove_linked_values_from_related_relation(relation);
        }
        let mut r#where = self.intrinsic_where_unique_for_relation(relation);
        r#where.as_hashmap_mut().unwrap().extend(value.as_hashmap().cloned().unwrap());
        let env = self.env().alter_intent(Intent::NestedUpdate);
        let object = match self.graph().find_unique(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await {
            Ok(object) => object,
            Err(_) => return Err(ActionError::unexpected_input_value_with_reason("Object is not found.", path)),
        };
        object.delete_from_database(session.clone()).await?;
        if relation.has_join_table() {
            let opposite_relation = self.graph().opposite_relation(relation).1.unwrap();
            self.delete_join_object(&object, relation, opposite_relation, session.clone(), path).await?;
        }
        Ok(())
    }

    async fn nested_many_delete_many_relation_object(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        if relation.has_foreign_key() {
            self.remove_linked_values_from_related_relation(relation);
        }
        let mut r#where = self.intrinsic_where_unique_for_relation(relation);
        r#where.as_hashmap_mut().unwrap().extend(value.as_hashmap().cloned().unwrap());
        let env = self.env().alter_intent(Intent::NestedUpdate);
        let objects = self.graph().find_many(relation.model(), &tson!({ "where": r#where }), true, env.clone()).await.unwrap();
        for object in objects {
            object.delete_from_database(session.clone()).await?;
            if relation.has_join_table() {
                let opposite_relation = self.graph().opposite_relation(relation).1.unwrap();
                self.delete_join_object(&object, relation, opposite_relation, session.clone(), path).await?;
            }
        }
        Ok(())
    }

    async fn perform_relation_manipulation_one(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        for (key, value) in value.as_hashmap().unwrap() {
            let key = key.as_str();
            let path = path + key;
            match key {
                "create" => self.nested_create_relation_object(relation, value, session.clone(), &path).await?,
                "connect" | "set" => self.nested_connect_relation_object(relation, value, session.clone(), &path).await?,
                "connectOrCreate" => self.nested_connect_or_create_relation_object(relation, value, session.clone(), &path).await?,
                "disconnect" => self.nested_disconnect_relation_object(relation, value, session.clone(), &path).await?,
                "update" => self.nested_update_relation_object(relation, value, session.clone(), &path).await?,
                "delete" => self.nested_delete_relation_object(relation, value, session.clone(), &path).await?,
                _ => panic!("Unhandled key.")
            }
        }
        Ok(())
    }

    async fn perform_relation_manipulation_many(&self, relation: &Relation, value: &Value, session: Arc<dyn SaveSession>, path: &KeyPath<'_>) -> ActionResult<()> {
        for (key, value) in value.as_hashmap().unwrap() {
            let key = key.as_str();
            let path = path + key;
            match key {
                "create" | "createMany" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_create_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_create_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "connect" | "set" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_connect_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_connect_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "connectOrCreate" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_connect_or_create_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_connect_or_create_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "disconnect" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_many_disconnect_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_many_disconnect_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "upsert" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_upsert_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_upsert_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "update" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_many_update_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_many_update_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "updateMany" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_many_update_many_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_many_update_many_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "delete" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_many_delete_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_many_delete_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                "deleteMany" => {
                    if let Some(_) = value.as_hashmap() {
                        self.nested_many_delete_many_relation_object(relation, value, session.clone(), &path).await?;
                    } else if let Some(vec) = value.as_vec() {
                        for (i, v) in vec.iter().enumerate() {
                            self.nested_many_delete_many_relation_object(relation, v, session.clone(), &(&path + i)).await?;
                        }
                    } else {
                        panic!("Unhandled type.")
                    }
                },
                _ => panic!("Unhandled key."),
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
            "where": self.identifier(),
        });
        if let Some(include) = include {
            finder.as_hashmap_mut().unwrap().insert("include".to_string(), include.clone());
        }
        if let Some(select) = select {
            finder.as_hashmap_mut().unwrap().insert("select".to_string(), select.clone());
        }
        graph.find_unique(self.model().name(), &finder, false, self.env().clone()).await
    }

    pub async fn fetch_relation_object(&self, key: impl AsRef<str>, find_unique_arg: Option<&Value>) -> Result<Option<Object>, ActionError> {
        // get relation
        let model = self.model();
        let relation = model.relation(key.as_ref());
        if relation.is_none() {
            // todo() err here
        }
        let relation = relation.unwrap();
        let mut finder = self.intrinsic_where_unique_for_relation(relation);
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
        let env = self.env().alter_intent(Intent::NestedIncluded);
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
        let env = self.env().alter_intent(Intent::NestedIncluded);
        if let Some(_join_table) = relation.through() {
            let identifier = self.identifier();
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
