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
use crate::core::field_input::AtomicUpdateType::{Decrement, Divide, Increment, Multiply};
use crate::core::field_input::FieldInput;
use crate::core::field_input::FieldInput::{AtomicUpdate, SetValue};
use crate::core::field_type::FieldType;
use crate::core::graph::Graph;
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
            let field = self.model().field(&key).unwrap();
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

    fn number_value_from_target_type( // float and u is decimal
        &self, field_name: &str, target: &FieldType, value: &JsonValue, is_float: bool, is_u: bool
    ) -> Result<Value, ActionError> {
        let is_decimal = is_float && is_u;
        if is_decimal {
            let s = value.as_str();
            match s {
                None => Err(ActionError::expected("decimal number", field_name)),
                Some(s) => {
                    match Decimal::from_str(s) {
                        Some(d) => Ok(Value::Decimal(d)),
                        None => Err(ActionError::expected("decimal number", field_name)),
                    }
                }
            }
        } else if is_u {
            let u = value.as_u64();
            match u {
                None => Err(ActionError::expected("unsigned integer number", field_name)),
                Some(n) => match target {
                    FieldType::U8 => Value::U8(n.into()),
                    FieldType::U16 => Value::U16(n.into()),
                    FieldType::U32 => Value::U32(n.into()),
                    FieldType::U64 => Value::U64(n.into()),
                    FieldType::U128 => Value::U128(n.into()),
                    _ => panic!()
                }
            }
        } else if is_float {
            let f = value.as_f64();
            match f {
                None => Err(ActionError::expected("float number", field_name)),
                Some(n) => match target {
                    FieldType::F32 => Value::F32(n.into()),
                    FieldType::F64 => Value::F64(n.into()),
                    _ => panic!()
                }
            }
        } else {
            panic!()
            // let i = value.as_i64();
            // match f {
            //     None => Err(ActionError::expected("integer number", field_name)),
            //     Some(n) => match target {
            //         FieldType::I8 => Value::I8(n.into()),
            //         FieldType::I16 => Value::I16(n.into()),
            //         FieldType::I32 => Value::I32(n.into()),
            //         FieldType::I64 => Value::I64(n.into()),
            //         FieldType::I128 => Value::I128(n.into()),
            //         _ => panic!()
            //     }
            // }
        }
    }

    fn decode_user_number_field_input( // float and u is decimal
        &self, field_name: &str, json_value: &JsonValue, is_new: bool, target: &FieldType, is_float: bool, is_u: bool
    ) -> Result<FieldInput, ActionError> {
        let is_decimal = is_float && is_u;
        if json_value.is_string() {
            if is_decimal {
                match self.number_value_from_target_type(field_name, target, json_value, is_float, is_u) {
                    Err(err) => Err(err),
                    Ok(val) => Ok(SetValue(val))
                }
            } else {
                Err(ActionError::wrong_input_type())
            }
        } else if json_value.is_number() {
            match self.number_value_from_target_type(field_name, target, json_value, is_float, is_u) {
                Err(err) => Err(err),
                Ok(val) => Ok(SetValue(val))
            }
        } else if json_value.is_object() && !is_new {
            let json_obj = json_value.as_object().unwrap();
            if json_obj.keys().len() != 1 {
                Err(ActionError::wrong_input_updator(field_name))
            } else {
                for (key, value) in json_obj {
                    return match key.as_str() {
                        "set" => {
                            match value {
                                JsonValue::Null => {
                                    Err(ActionError::unexpected_null(&field_name))
                                    // if field.optionality == Optionality::Optional {
                                    //     Ok(SetValue(Value::Null))
                                    // } else {
                                    //     Err(ActionError::unexpected_null(&field.name))
                                    // }
                                }
                                JsonValue::Number(num) => {
                                    match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                        Err(err) => Err(err),
                                        Ok(val) => Ok(SetValue((val)))
                                    }
                                }
                                JsonValue::String(str) => {
                                    if is_decimal {
                                        match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                            Err(err) => Err(err),
                                            Ok(val) => Ok(SetValue((val)))
                                        }
                                    } else {
                                        Err(ActionError::wrong_input_type())
                                    }
                                }
                                _ => {
                                    Err(ActionError::wrong_input_type())
                                }
                            }
                        }
                        "increment" => {
                            match value {
                                JsonValue::Number(num) => {
                                    match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                        Err(err) => Err(err),
                                        Ok(val) => Ok(AtomicUpdate(Increment(val)))
                                    }
                                }
                                JsonValue::String(str) => {
                                    if is_decimal {
                                        match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                            Err(err) => Err(err),
                                            Ok(val) => Ok(AtomicUpdate(Increment(val)))
                                        }
                                    } else {
                                        Err(ActionError::wrong_input_type())
                                    }
                                }
                                _ => {
                                    Err(ActionError::wrong_input_updator(field_name))
                                }
                            }
                        }
                        "decrement" => {
                            match value {
                                JsonValue::Number(num) => {
                                    match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                        Err(err) => Err(err),
                                        Ok(val) => Ok(AtomicUpdate(Decrement(val)))
                                    }
                                }
                                JsonValue::String(str) => {
                                    if is_decimal {
                                        match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                            Err(err) => Err(err),
                                            Ok(val) => Ok(AtomicUpdate(Decrement(val)))
                                        }
                                    } else {
                                        Err(ActionError::wrong_input_type())
                                    }
                                }
                                _ => {
                                    Err(ActionError::wrong_input_updator(field_name))
                                }
                            }
                        }
                        "multiply" => {
                            match value {
                                JsonValue::Number(num) => {
                                    match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                        Err(err) => Err(err),
                                        Ok(val) => Ok(AtomicUpdate(Multiply(val)))
                                    }
                                }
                                JsonValue::String(str) => {
                                    if is_decimal {
                                        match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                            Err(err) => Err(err),
                                            Ok(val) => Ok(AtomicUpdate(Multiply(val)))
                                        }
                                    } else {
                                        Err(ActionError::wrong_input_type())
                                    }
                                }
                                _ => {
                                    Err(ActionError::wrong_input_updator(field_name))
                                }
                            }
                        }
                        "divide" => {
                            match value {
                                JsonValue::Number(num) => {
                                    match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                        Err(err) => Err(err),
                                        Ok(val) => Ok(AtomicUpdate(Divide(val)))
                                    }
                                }
                                JsonValue::String(str) => {
                                    if is_decimal {
                                        match self.number_value_from_target_type(field_name, target, value, is_float, is_u) {
                                            Err(err) => Err(err),
                                            Ok(val) => Ok(AtomicUpdate(Divide(val)))
                                        }
                                    } else {
                                        Err(ActionError::wrong_input_type())
                                    }
                                }
                                _ => {
                                    Err(ActionError::wrong_input_updator(field_name))
                                }
                            }
                        }
                        _ => {
                            Err(ActionError::wrong_input_updator(field_name))
                        }
                    }
                }
            }
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    fn decode_user_string_inout_into_type(&self, field_name: &str, target: &FieldType, json_value: &JsonValue, graph: &Graph) -> Result<Value, ActionError> {
        match target {
            FieldType::ObjectId => Ok(Value::ObjectId(json_value.as_str().unwrap().to_string())),
            FieldType::String => Ok(Value::String(json_value.as_str().unwrap().to_string())),
            FieldType::Date => match NaiveDate::parse_from_str(&json_value.as_str().unwrap(), "%Y-%m-%d") {
                Ok(naive_date) => {
                    let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                    Ok(Value::Date(date))
                }
                Err(_) => {
                    Err(ActionError::wrong_date_format())
                }
            }
            FieldType::DateTime => match DateTime::parse_from_rfc3339(&json_value.as_str().unwrap()) {
                Ok(fixed_offset_datetime) => {
                    let datetime: DateTime<Utc> = fixed_offset_datetime.with_timezone(&Utc);
                    Ok(Value::DateTime(datetime))
                }
                Err(_) => {
                    Err(ActionError::wrong_datetime_format())
                }
            }
            FieldType::Enum(enum_name) => {
                let enum_choice = json_value.as_str().unwrap();
                let enums = graph.enums();
                let vals = enums.get(&enum_name.to_string()).unwrap();
                if vals.contains(&enum_choice.to_string()) {
                    Ok(Value::String(enum_choice.into()))
                } else {
                    Err(ActionError::wrong_enum_choice())
                }
            }
            _ => panic!()
        }
    }

    fn decode_user_field_input_string_set_only(
        &self, field_name: &str, target: &FieldType, is_new: bool, json_value: &JsonValue, graph: &Graph
    ) -> Result<FieldInput, ActionError> {
        if json_value.is_string() {
            match self.decode_user_string_inout_into_type(field_name, target, json_value, graph) {
                Ok(val) => Ok(SetValue(val)),
                Err(err) => Err(err),
            }
        } else if json_value.is_object() && !is_new {
            let json_obj = json_value.as_object().unwrap();
            if json_obj.keys().len() != 1 {
                Err(ActionError::wrong_input_updator(field_name))
            } else {
                for (key, value) in json_obj {
                    return match key.as_str() {
                        "set" => {
                            match value {
                                JsonValue::Null => {
                                    Err(ActionError::unexpected_null(field_name))
                                    // if field.optionality == Optionality::Optional {
                                    //     Ok(SetValue(Value::Null))
                                    // } else {
                                    //     Err(ActionError::unexpected_null(field_name))
                                    // }
                                }
                                JsonValue::String(string_value) => {
                                    match self.decode_user_string_inout_into_type(field_name, target, json_value) {
                                        Ok(val) => Ok(SetValue(val)),
                                        Err(err) => Err(err)
                                    }
                                }
                                _ => {
                                    Err(ActionError::wrong_input_type())
                                }
                            }
                        }
                        _ => {
                            Err(ActionError::wrong_input_updator(field_name))
                        }
                    }
                }
            }
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    fn decode_user_field_input(&self, json_value: &JsonValue, field: &Field, path_name: &str) -> Result<FieldInput, ActionError> {
        if json_value == &JsonValue::Null {
            return if field.optionality == Optionality::Optional {
                Ok(SetValue(Value::Null))
            } else {
                Err(ActionError::unexpected_null(path_name))
            }
        }
        let is_new = self.inner.is_new.load(Ordering::SeqCst);
        let graph = self.graph();
        return match &field.field_type {
            FieldType::Undefined => { panic!("Field type should not be undefined!") }
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => {
                self.decode_user_field_input_string_set_only(path_name, &field.field_type, is_new, json_value, graph)
            }
            FieldType::String | FieldType::Date | FieldType::DateTime | FieldType::Enum(_) => {
                self.decode_user_field_input_string_set_only(path_name, &field.field_type, is_new, json_value, graph)
            }
            FieldType::Bool => {
                if json_value.is_boolean() {
                    Ok(SetValue(Value::Bool(json_value.as_bool().unwrap())))
                } else if json_value.is_object() && !is_new {
                    let json_obj = json_value.as_object().unwrap();
                    if json_obj.keys().len() != 1 {
                        Err(ActionError::wrong_input_updator(path_name))
                    } else {
                        for (key, value) in json_obj {
                            return match key.as_str() {
                                "set" => {
                                    match value {
                                        JsonValue::Null => {
                                            if field.optionality == Optionality::Optional {
                                                Ok(SetValue(Value::Null))
                                            } else {
                                                Err(ActionError::unexpected_null(path_name))
                                            }
                                        }
                                        JsonValue::Bool(bool_value) => {
                                            Ok(SetValue(Value::Bool(*bool_value)))
                                        }
                                        _ => {
                                            Err(ActionError::wrong_input_type())
                                        }
                                    }
                                }
                                _ => {
                                    Err(ActionError::wrong_input_updator(path_name))
                                }
                            }
                        }
                    }
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 => {
                self.decode_user_number_field_input(
                    path_name, json_value, is_new, &field.field_type, false, false)
            }
            FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 => {
                self.decode_user_number_field_input(
                    path_name, json_value, is_new, &field.field_type, false, true)
            }
            FieldType::F32 | FieldType::F64 => {
                self.decode_user_number_field_input(
                    path_name, json_value, is_new, &field.field_type, true, false)
            }
            FieldType::Decimal => {
                self.decode_user_number_field_input(
                    path_name, json_value, is_new, &field.field_type, true, true)
            }
            FieldType::Vec(field) => {
                if json_value.is_array() {
                    let arr = json_value.as_array().unwrap();
                    Ok(SetValue(Value::Vec(arr.iter().enumerate().map(|(i, v)| {
                        let new_path_name = path_name.to_string() + "." + &String::from(&i);
                        match self.decode_user_field_input(v, field, &new_path_name, "") {
                            SetValue(v) => v,
                            _ => panic!()
                        }
                    }).collect())))
                } else if json_value.is_object() && !is_new {
                    let json_obj = json_value.as_object().unwrap();
                    if json_obj.keys().len() != 1 {
                        Err(ActionError::wrong_input_updator(path_name))
                    } else {
                        for (key, value) in json_obj {
                            return match key.as_str() {
                                "set" => {
                                    match value {
                                        JsonValue::Null => {
                                            if field.optionality == Optionality::Optional {
                                                Ok(SetValue(Value::Null))
                                            } else {
                                                Err(ActionError::unexpected_null(path_name))
                                            }
                                        }
                                        JsonValue::Array(arr) => {
                                            Ok(SetValue(Value::Vec(arr.iter().enumerate().map(|(i, v)| {
                                                let new_path_name = path_name.to_string() + "." + &String::from(&i);
                                                match self.decode_user_field_input(v, field, &new_path_name) {
                                                    SetValue(v) => v,
                                                    _ => panic!()
                                                }
                                            }).collect())))
                                        }
                                        _ => {
                                            Err(ActionError::wrong_input_type())
                                        }
                                    }
                                }
                                "push" => {

                                }
                                _ => {
                                    Err(ActionError::wrong_input_updator(path_name))
                                }
                            }
                        }
                    }
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            _ => {
                panic!()
            }
        }
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
                let input_result = self.decode_user_field_input(json_value, field, "");
                let value_result = field.field_type.decode_value(json_value, self.graph());
                let mut value;
                match value_result {
                    Ok(v) => { value = v }
                    Err(e) => {
                        match e.r#type {
                            ActionErrorType::WrongEnumChoice => {
                                return Err(ActionError::unexpected_enum_value(*key))
                            }
                            _ => return Err(e)
                        }
                    }
                }
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
    pub(crate) atomic_updator_map: RefCell<HashMap<String, JsonValue>>,
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
