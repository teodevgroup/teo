use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::{Date, DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use serde_json::{Map, Value as JsonValue};
use crate::core::argument::Argument;
use crate::core::field::{Field, Optionality, Store};
use crate::core::input::AtomicUpdateType::{Decrement, Divide, Increment, Multiply};
use crate::core::input::{AtomicUpdateType, Input};
use crate::core::input::Input::{AtomicUpdate, SetValue};
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
use crate::core::input_decoder::decode_field_input;
use crate::core::model::Model;
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
        }) }
    }

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
            self.inner.selected_fields.borrow_mut().extend(self.model().output_keys().iter().map(|s| { s.to_string()}));
        }
        for key in keys {
            self.inner.selected_fields.borrow_mut().remove(&key);
        }
        return Ok(());
    }

    pub async fn save(&self) -> Result<(), ActionError> {
        // apply on save pipeline first
        let model_keys = self.model().save_keys();
        for key in model_keys {
            let field = self.model().field(key).unwrap();
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
            let field = self.model().field(key).unwrap();
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

        // send to database to save
        let connector = self.graph().connector();
        let save_result = connector.save_object(self).await;
        match save_result {
            Ok(()) => {
                // apply properties
                self.inner.is_new.store(false, Ordering::SeqCst);
                self.inner.is_modified.store(false, Ordering::SeqCst);
                *self.inner.modified_fields.borrow_mut() = HashSet::new();
                // then do nothing haha
                Ok(())
            }
            Err(error) => {
                Err(error)
            }
        }
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

    pub(crate) fn is_instance_of(&self, model_name: &'static str) -> bool {
        self.model().name() == model_name
    }

    pub(crate) fn model(&self) -> &Model {
        &*self.model()
    }

    pub(crate) fn graph(&self) -> &Graph {
        &*self.graph()
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
