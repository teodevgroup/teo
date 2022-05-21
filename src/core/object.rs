use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::{Map, Value as JsonValue};
use crate::core::argument::Argument;
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::stage::Stage;
use crate::core::value::Value;
use crate::error::ActionError;


#[derive(Clone)]
pub struct Object {
    pub(crate) inner: Arc<ObjectInner>
}

impl Object {

    pub(crate) fn new(model: &'static Model, graph: &'static Graph) -> Object {
        Object { inner: Arc::new(ObjectInner {
            model,
            graph,
            is_initialized: AtomicBool::new(false),
            is_new: AtomicBool::new(true),
            is_modified: AtomicBool::new(false),
            is_partial: false,
            is_deleted: false,
            selected_fields: RefCell::new(HashSet::new()),
            modified_fields: RefCell::new(HashSet::new()),
            previous_values: RefCell::new(HashMap::new()),
            value_map: RefCell::new(HashMap::new())
        }) }
    }

    pub async fn set_json(&self, json_value: JsonValue) -> Result<(), ActionError> {
        self.set_or_update_json(json_value, true).await
    }

    pub async fn update_json(&self, json_value: JsonValue) -> Result<(), ActionError> {
        self.set_or_update_json(json_value, false).await
    }

    pub fn set_value(&self, key: &'static str, value: Value) -> Result<(), ActionError> {
        let model_keys = self.inner.model.save_keys();
        if !model_keys.contains(&key) {
            return Err(ActionError::keys_unallowed());
        }
        if value == Value::Null {
            self.inner.value_map.borrow_mut().remove(key);
        } else {
            self.inner.value_map.borrow_mut().insert(key.to_string(), value);
        }
        if !self.inner.is_new.load(Ordering::SeqCst) {
            self.inner.is_modified.store(true, Ordering::SeqCst);
            self.inner.modified_fields.borrow_mut().insert(key.to_string());
        }
        Ok(())
    }

    pub fn get_value(&self, key: &'static str) -> Result<Option<Value>, ActionError> {
        let model_keys = &self.inner.model.all_getable_keys(); // TODO: should be all keys
        if !model_keys.contains(&key) {
            return Err(ActionError::keys_unallowed());
        }
        match self.inner.value_map.borrow().get(key) {
            Some(value) => {
                Ok(Some(value.clone()))
            }
            None => {
                Ok(None)
            }
        }
    }

    pub fn select(&self, keys: HashSet<String>) -> Result<(), ActionError> {
        self.inner.selected_fields.borrow_mut().extend(keys);
        Ok(())
    }

    pub fn deselect(&self, keys: HashSet<String>) -> Result<(), ActionError> {
        if self.inner.selected_fields.borrow().len() == 0 {
            self.inner.selected_fields.borrow_mut().extend(self.inner.model.output_keys().iter().map(|s| { s.to_string()}));
        }
        for key in keys {
            self.inner.selected_fields.borrow_mut().remove(&key);
        }
        return Ok(());
    }

    pub async fn save(&self) -> Result<(), ActionError> {
        // apply on save pipeline first
        let model_keys = self.inner.model.save_keys();
        for key in model_keys {
            let field = self.inner.model.field(&key);
            if field.on_save_pipeline._has_any_modifier() {
                let mut stage = match self.inner.value_map.borrow().deref().get(&key.to_string()) {
                    Some(value) => {
                        Stage::Value(value.clone())
                    }
                    None => {
                        Stage::Value(Value::Null)
                    }
                };
                stage = field.on_save_pipeline._process(stage.clone(), self.clone()).await;
                match stage {
                    Stage::Invalid(s) => {
                        return Err(ActionError::invalid_input(key, s));
                    }
                    Stage::Value(v) => {
                        self.inner.value_map.borrow_mut().insert(key.to_string(), v);
                        if !self.inner.is_new.load(Ordering::SeqCst) {
                            self.inner.is_modified.store(true, Ordering::SeqCst);
                            self.inner.modified_fields.borrow_mut().insert(key.to_string());
                        }
                    }
                    Stage::ConditionTrue(_) => {
                        return Err(ActionError::internal_server_error("Pipeline modifiers are invalid.".to_string()))
                    }
                    Stage::ConditionFalse(_) => {
                        return Err(ActionError::internal_server_error("Pipeline modifiers are invalid.".to_string()))
                    }
                }
            }
        }
        // send to database to save
        let connector = self.inner.graph.connector();
        let save_result = connector.save_object(self).await;
        // apply properties
        self.inner.is_new.store(false, Ordering::SeqCst);
        self.inner.is_modified.store(false, Ordering::SeqCst);
        *self.inner.modified_fields.borrow_mut() = HashSet::new();
        // then do nothing haha
        Ok(())
    }

    pub fn delete(&self) -> &Object {
        self
    }

    pub fn to_json(&self) -> JsonValue {
        let mut map: Map<String, JsonValue> = Map::new();
        let keys = self.inner.model.output_keys();
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

    async fn set_or_update_json(&self, json_value: JsonValue, process: bool) -> Result<(), ActionError> {
        let json_object = json_value.as_object().unwrap().clone();
        // check keys first
        let json_keys: Vec<&str> = json_object.keys().map(|k| { k.as_str() }).collect();
        let model_keys = if process {
            self.inner.model.input_keys()
        } else {
            self.inner.model.save_keys()
        };
        let keys_valid = json_keys.iter().all(|&item| model_keys.contains(&item));
        if !keys_valid {
            return Err(ActionError::keys_unallowed());
        }
        // assign values
        let initialized = self.inner.is_initialized.load(Ordering::SeqCst);
        let keys_to_iterate = if initialized { &json_keys } else { model_keys };
        for key in keys_to_iterate {
            let field = self.inner.model.field(&key);
            let json_has_value = if initialized { true } else {
                json_keys.contains(key)
            };
            if json_has_value {
                let json_value = &json_object[&key.to_string()];
                let mut value_result = field.r#type.decode_value(json_value, self.inner.graph);
                let mut value;
                match value_result {
                    Ok(v) => { value = v }
                    Err(e) => { return Err(e) }
                }
                if process {
                    // pipeline
                    let mut stage = Stage::Value(value);
                    stage = field.on_set_pipeline._process(stage.clone(), self.clone()).await;
                    match stage {
                        Stage::Invalid(s) => {
                            return Err(ActionError::invalid_input(field.name, s));
                        }
                        Stage::Value(v) => {
                            value = v
                        }
                        Stage::ConditionTrue(_) => {
                            return Err(ActionError::internal_server_error("Pipeline modifiers are invalid.".to_string()))
                        }
                        Stage::ConditionFalse(_) => {
                            return Err(ActionError::internal_server_error("Pipeline modifiers are invalid.".to_string()))
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
            } else {
                // apply default values
                if !initialized {
                    if let Some(argument) = &field.default {
                        match argument {
                            Argument::ValueArgument(value) => {
                                self.inner.value_map.borrow_mut().insert(key.to_string(), value.clone());
                            }
                            Argument::PipelineArgument(pipeline) => {
                                let stage = pipeline._process(Stage::Value(Value::Null), self.clone()).await;
                                self.inner.value_map.borrow_mut().insert(key.to_string(), stage.value().unwrap());
                            }
                            Argument::FunctionArgument(farg) => {
                                let stage = farg.call(Value::Null, self.clone()).await;
                                self.inner.value_map.borrow_mut().insert(key.to_string(), stage.value().unwrap());
                            }
                        }
                    }
                }
            }
        };
        // set flag
        self.inner.is_initialized.store(true, Ordering::SeqCst);
        Ok(())
    }
}

pub(crate) struct ObjectInner {
    pub(crate) model: &'static Model,
    pub(crate) graph: &'static Graph,
    pub(crate) is_initialized: AtomicBool,
    pub(crate) is_new: AtomicBool,
    pub(crate) is_modified: AtomicBool,
    pub(crate) is_partial: bool,
    pub(crate) is_deleted: bool,
    pub(crate) selected_fields: RefCell<HashSet<String>>,
    pub(crate) modified_fields: RefCell<HashSet<String>>,
    pub(crate) previous_values: RefCell<HashMap<String, Value>>,
    pub(crate) value_map: RefCell<HashMap<String, Value>>,
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct(self.inner.model.name());
        for (key, value) in self.inner.value_map.borrow().iter() {
            result.field(key, value);
        }
        result.finish()
    }
}

unsafe impl Send for ObjectInner {}
unsafe impl Sync for ObjectInner {}
