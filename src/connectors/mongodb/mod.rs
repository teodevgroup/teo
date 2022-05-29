use std::collections::HashMap;
use std::fmt::{Debug};
use std::sync::atomic::{Ordering};
use serde_json::{Map, Value as JsonValue};
use async_trait::async_trait;
use bson::{Bson, DateTime as BsonDateTime, doc, Document, oid::ObjectId, Regex as BsonRegex};
use chrono::{Date, NaiveDate, Utc, DateTime};
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{options::ClientOptions, Client, Database, Collection, IndexModel};
use mongodb::error::{ErrorKind, WriteFailure, Error as MongoDBError};
use mongodb::options::{CreateIndexOptions, DropDatabaseOptions, IndexOptions};
use regex::Regex;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::object::Object;
use crate::core::builders::graph_builder::GraphBuilder;
use crate::core::field::{Availability, FieldIndex, Type};
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::core::value::Value;
use crate::error::ActionError;


#[derive(Debug)]
pub struct MongoDBConnector {
    client: Client,
    database: Database,
    collections: HashMap<&'static str, Collection<Document>>
}

impl MongoDBConnector {
    pub(crate) async fn new(options: ClientOptions, models: &Vec<Model>, reset_database: bool) -> MongoDBConnector {
        let client = Client::with_options(options.clone()).unwrap();
        let database = client.database(&options.default_database.clone().unwrap());
        if reset_database {
            database.drop(None).await;
        }
        let mut collections: HashMap<&'static str, Collection<Document>> = HashMap::new();
        for model in models {
            let name = model.name();
            let collection: Collection<Document> = database.collection(model.table_name());
            for field in model.index_fields() {
                let index_options = IndexOptions::builder()
                    .name(field.name.to_string())
                    .unique(field.index == FieldIndex::Unique)
                    .sparse(field.availability == Availability::Optional).build();
                let index_model = IndexModel::builder().keys(doc! {field.name.to_string(): 1}).options(index_options).build();
                let create_index_options = CreateIndexOptions::builder().build();
                collection.create_index(index_model, create_index_options).await;
            }
            collections.insert(name, collection);
        }
        MongoDBConnector {
            client,
            database,
            collections
        }
    }

    fn document_to_object(&self, document: &Document, object: &Object) -> Result<(), ActionError> {
        let primary_name = if let Some(primary_field) = object.inner.model.primary_field() {
            primary_field.name
        } else {
            "__id"
        };
        for key in document.keys() {
            let object_key = if key == "_id" { primary_name } else { key };
            let field_type = if key == "_id" { &Type::ObjectId } else { &object.inner.model.field(key).r#type };
            let bson_value = document.get(key).unwrap();
            let value_result = self.bson_value_to_type(object_key, bson_value, field_type);
            match value_result {
                Ok(value) => {
                    object.inner.value_map.borrow_mut().insert(object_key.to_string(), value);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        object.inner.is_initialized.store(true, Ordering::SeqCst);
        object.inner.is_new.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn bson_value_to_type(&self, field_name: &str, bson_value: &Bson, field_type: &Type) -> Result<Value, ActionError> {
        return match field_type {
            Type::Undefined => {
                panic!()
            }
            Type::ObjectId => {
                match bson_value.as_object_id() {
                    Some(object_id) => {
                        Ok(Value::ObjectId(object_id.to_hex()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }

            }
            Type::Bool => {
                match bson_value.as_bool() {
                    Some(bool) => {
                        Ok(Value::Bool(bool))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::I8 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::I8(val as i8))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::I16 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::I16(val as i16))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::I32 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::I32(val as i32))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::I64 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::I64(val))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::I128 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::I128(val as i128))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::U8 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::U8(val as u8))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::U16 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::U16(val as u16))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::U32 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::U32(val as u32))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::U64 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::U64(val as u64))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::U128 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::U128(val as u128))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::F32 => {
                match bson_value.as_f64() {
                    Some(val) => {
                        Ok(Value::F32(val as f32))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::F64 => {
                match bson_value.as_f64() {
                    Some(val) => {
                        Ok(Value::F64(val))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::String => {
                match bson_value.as_str() {
                    Some(val) => {
                        Ok(Value::String(val.to_string()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::Date => {
                match bson_value.as_datetime() {
                    Some(val) => {
                        Ok(Value::Date(val.to_chrono().date()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::DateTime => {
                match bson_value.as_datetime() {
                    Some(val) => {
                        Ok(Value::DateTime(val.to_chrono()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::Enum(_) => {
                match bson_value.as_str() {
                    Some(val) => {
                        Ok(Value::String(val.to_string()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            Type::Vec(_) => {
                panic!()
            }
            Type::Map(_) => {
                panic!()
            }
            Type::Object(_) => {
                panic!()
            }
        };
    }

    fn _handle_write_error(&self, error_kind: ErrorKind) -> ActionError {
        match error_kind {
            ErrorKind::Write(write) => {
                match write {
                    WriteFailure::WriteError(write_error) => {
                        match write_error.code {
                            11000 => {
                                let regex = Regex::new(r"dup key: \{ (.+):").unwrap();
                                let match_result = regex.captures(write_error.message.as_str()).unwrap().get(1);
                                return ActionError::unique_value_duplicated(match_result.unwrap().as_str())
                            }
                            _ => {
                                return ActionError::unknown_database_write_error()
                            }
                        }
                    }
                    _ => {
                        return ActionError::unknown_database_write_error()
                    }
                }
            }
            _ => {
                return ActionError::unknown_database_write_error()
            }
        }
    }

    fn build_where_input(&self, model: &Model, r#where: Option<&JsonValue>, graph: &Graph) -> Result<Document, ActionError> {
        if let None = r#where { return Ok(doc!{}); }
        let r#where = r#where.unwrap();
        if !r#where.is_object() { return Err(ActionError::wrong_json_format()); }
        let r#where = r#where.as_object().unwrap();
        let mut doc = doc!{};
        for (key, value) in r#where.iter() {
            if !model.query_keys().contains(&key.as_str()) {
                return Err(ActionError::keys_unallowed());
            }
            let field = model.field(key);
            let db_key = if field.primary { "_id" } else { field.name };
            let bson_result = self.parse_bson_where_entry(&field.r#type, value, graph);
            match bson_result {
                Ok(bson) => {
                    doc.insert(db_key, bson);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(doc)
    }

    fn parse_bson_where_entry(&self, field_type: &Type, value: &JsonValue, graph: &Graph) -> Result<Bson, ActionError> {
        return match field_type {
            Type::Undefined => {
                panic!()
            }
            Type::ObjectId => {
                if value.is_string() {
                    self.parse_object_id(value)
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let oid = self.parse_object_id(value)?;
                                result.insert("$eq", oid);
                            }
                            "not" => {
                                let oid = self.parse_object_id(value)?;
                                result.insert("$eq", oid);
                            }
                            "gt" => {
                                let oid = self.parse_object_id(value)?;
                                result.insert("$gt", oid);
                            }
                            "gte" => {
                                let oid = self.parse_object_id(value)?;
                                result.insert("$gt", oid);
                            }
                            "lt" => {
                                let oid = self.parse_object_id(value)?;
                                result.insert("$gt", oid);
                            }
                            "lte" => {
                                let oid = self.parse_object_id(value)?;
                                result.insert("$gt", oid);
                            }
                            "in" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_object_id(val)?);
                                        }
                                        result.insert("$in", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "notIn" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_object_id(val)?);
                                        }
                                        result.insert("$nin", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::Bool => {
                if value.is_boolean() {
                    Ok(Bson::Boolean(value.as_bool().unwrap()))
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let b = self.parse_bool(value)?;
                                result.insert("$eq", b);
                            }
                            "not" => {
                                let b = self.parse_bool(value)?;
                                result.insert("$eq", b);
                            }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 => {
                if value.is_i64() {
                    Ok(Bson::Int64(value.as_i64().unwrap()))
                } else if value.is_u64() {
                    Ok(Bson::Int64(value.as_u64().unwrap() as i64))
                } else if value.is_f64() {
                    Ok(Bson::Int64(value.as_f64().unwrap() as i64))
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let b = self.parse_i64(value)?;
                                result.insert("$eq", b);
                            }
                            "not" => {
                                let b = self.parse_i64(value)?;
                                result.insert("$eq", b);
                            }
                            "gt" => {
                                let oid = self.parse_i64(value)?;
                                result.insert("$gt", oid);
                            }
                            "gte" => {
                                let oid = self.parse_i64(value)?;
                                result.insert("$gt", oid);
                            }
                            "lt" => {
                                let oid = self.parse_i64(value)?;
                                result.insert("$gt", oid);
                            }
                            "lte" => {
                                let oid = self.parse_i64(value)?;
                                result.insert("$gt", oid);
                            }
                            "in" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_i64(val)?);
                                        }
                                        result.insert("$in", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "notIn" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_i64(val)?);
                                        }
                                        result.insert("$nin", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::F32 | Type::F64 => {
                if value.is_i64() {
                    Ok(Bson::Double(value.as_i64().unwrap() as f64))
                } else if value.is_u64() {
                    Ok(Bson::Double(value.as_u64().unwrap() as f64))
                } else if value.is_f64() {
                    Ok(Bson::Double(value.as_f64().unwrap()))
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let b = self.parse_f64(value)?;
                                result.insert("$eq", b);
                            }
                            "not" => {
                                let b = self.parse_f64(value)?;
                                result.insert("$eq", b);
                            }
                            "gt" => {
                                let oid = self.parse_f64(value)?;
                                result.insert("$gt", oid);
                            }
                            "gte" => {
                                let oid = self.parse_f64(value)?;
                                result.insert("$gt", oid);
                            }
                            "lt" => {
                                let oid = self.parse_f64(value)?;
                                result.insert("$gt", oid);
                            }
                            "lte" => {
                                let oid = self.parse_f64(value)?;
                                result.insert("$gt", oid);
                            }
                            "in" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_f64(val)?);
                                        }
                                        result.insert("$in", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "notIn" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_f64(val)?);
                                        }
                                        result.insert("$nin", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::String => {
                if value.is_string() {
                    Ok(Bson::String(value.as_str().unwrap().to_string()))
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let b = self.parse_string(value)?;
                                result.insert("$eq", b);
                            }
                            "not" => {
                                let b = self.parse_string(value)?;
                                result.insert("$eq", b);
                            }
                            "gt" => {
                                let oid = self.parse_string(value)?;
                                result.insert("$gt", oid);
                            }
                            "gte" => {
                                let oid = self.parse_string(value)?;
                                result.insert("$gt", oid);
                            }
                            "lt" => {
                                let oid = self.parse_string(value)?;
                                result.insert("$gt", oid);
                            }
                            "lte" => {
                                let oid = self.parse_string(value)?;
                                result.insert("$gt", oid);
                            }
                            "in" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_string(val)?);
                                        }
                                        result.insert("$in", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "notIn" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_string(val)?);
                                        }
                                        result.insert("$nin", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "contains" => {
                                let bson_regex = BsonRegex {
                                    pattern: regex::escape(self.parse_string(value)?.as_str().unwrap()),
                                    options: if self.has_i_mode(map) { "i".to_string() } else { "".to_string() }
                                };
                                let regex = Bson::RegularExpression(bson_regex);
                                result.insert("$regex", regex);
                            }
                            "startsWith" => {
                                let bson_regex = BsonRegex {
                                    pattern: "^".to_string() + &*regex::escape(self.parse_string(value)?.as_str().unwrap()),
                                    options: if self.has_i_mode(map) { "i".to_string() } else { "".to_string() }
                                };
                                let regex = Bson::RegularExpression(bson_regex);
                                result.insert("$regex", regex);
                            }
                            "endsWith" => {
                                let bson_regex = BsonRegex {
                                    pattern: regex::escape(self.parse_string(value)?.as_str().unwrap()) + "$",
                                    options: if self.has_i_mode(map) { "i".to_string() } else { "".to_string() }
                                };
                                let regex = Bson::RegularExpression(bson_regex);
                                result.insert("$regex", regex);
                            }
                            "matches" => {
                                let bson_regex = BsonRegex {
                                    pattern: self.parse_string(value)?.as_str().unwrap().to_string(),
                                    options: if self.has_i_mode(map) { "i".to_string() } else { "".to_string() }
                                };
                                let regex = Bson::RegularExpression(bson_regex);
                                result.insert("$regex", regex);
                            }
                            "mode" => { }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::Date => {
                if value.is_string() {
                    self.parse_date(value)
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let b = self.parse_date(value)?;
                                result.insert("$eq", b);
                            }
                            "not" => {
                                let b = self.parse_date(value)?;
                                result.insert("$eq", b);
                            }
                            "gt" => {
                                let oid = self.parse_date(value)?;
                                result.insert("$gt", oid);
                            }
                            "gte" => {
                                let oid = self.parse_date(value)?;
                                result.insert("$gt", oid);
                            }
                            "lt" => {
                                let oid = self.parse_date(value)?;
                                result.insert("$gt", oid);
                            }
                            "lte" => {
                                let oid = self.parse_date(value)?;
                                result.insert("$gt", oid);
                            }
                            "in" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_date(val)?);
                                        }
                                        result.insert("$in", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "notIn" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_date(val)?);
                                        }
                                        result.insert("$nin", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::DateTime => {
                if value.is_string() {
                    self.parse_datetime(value)
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let b = self.parse_datetime(value)?;
                                result.insert("$eq", b);
                            }
                            "not" => {
                                let b = self.parse_datetime(value)?;
                                result.insert("$eq", b);
                            }
                            "gt" => {
                                let oid = self.parse_datetime(value)?;
                                result.insert("$gt", oid);
                            }
                            "gte" => {
                                let oid = self.parse_datetime(value)?;
                                result.insert("$gt", oid);
                            }
                            "lt" => {
                                let oid = self.parse_datetime(value)?;
                                result.insert("$gt", oid);
                            }
                            "lte" => {
                                let oid = self.parse_datetime(value)?;
                                result.insert("$gt", oid);
                            }
                            "in" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_datetime(val)?);
                                        }
                                        result.insert("$in", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "notIn" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_datetime(val)?);
                                        }
                                        result.insert("$nin", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::Enum(enum_name) => {
                if value.is_string() {
                    self.parse_enum(value, enum_name, graph)
                } else if value.is_object() {
                    let map = value.as_object().unwrap();
                    let mut result = doc!{};
                    for (key, value) in map {
                        match key.as_str() {
                            "equals" => {
                                let b = self.parse_enum(value, enum_name, graph)?;
                                result.insert("$eq", b);
                            }
                            "not" => {
                                let b = self.parse_enum(value, enum_name, graph)?;
                                result.insert("$eq", b);
                            }
                            "in" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_enum(value, enum_name, graph)?);
                                        }
                                        result.insert("$in", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            "notIn" => {
                                match value.as_array() {
                                    Some(arr_val) => {
                                        let mut arr: Vec<Bson> = Vec::new();
                                        for val in arr_val {
                                            arr.push(self.parse_enum(value, enum_name, graph)?);
                                        }
                                        result.insert("$nin", arr);
                                    }
                                    None => {
                                        return Err(ActionError::wrong_input_type());
                                    }
                                }
                            }
                            &_ => {
                                return Err(ActionError::wrong_input_type());
                            }
                        }
                    }
                    Ok(Bson::Document(result))
                } else {
                    Err(ActionError::wrong_input_type())
                }
            }
            Type::Vec(_) => {
                panic!()
            }
            Type::Map(_) => {
                panic!()
            }
            Type::Object(_) => {
                panic!()
            }
        }
    }

    fn has_i_mode(&self, map: &Map<String, JsonValue>) -> bool {
        match map.get("mode") {
            Some(val) => {
                if (val.is_string()) {
                    return val.as_str().unwrap() == "caseInsensitive"
                } else {
                    false
                }
            }
            None => {
                false
            }
        }
    }

    fn parse_object_id(&self, value: &JsonValue) -> Result<Bson, ActionError> {
        match value.as_str() {
            Some(val) => {
                match ObjectId::parse_str(val) {
                    Ok(oid) => {
                        Ok(Bson::ObjectId(oid))
                    }
                    Err(_) => {
                        Err(ActionError::wrong_input_type())
                    }
                }
            }
            None => {
                Err(ActionError::wrong_input_type())
            }
        }
    }

    fn parse_string(&self, value: &JsonValue) -> Result<Bson, ActionError> {
        match value.as_str() {
            Some(val) => {
                Ok(Bson::String(val.to_string()))
            }
            None => {
                Err(ActionError::wrong_input_type())
            }
        }
    }

    fn parse_bool(&self, value: &JsonValue) -> Result<Bson, ActionError> {
        match value.as_bool() {
            Some(val) => {
                Ok(Bson::Boolean(val))
            }
            None => {
                Err(ActionError::wrong_input_type())
            }
        }
    }

    fn parse_i64(&self, value: &JsonValue) -> Result<Bson, ActionError> {
        if value.is_i64() {
            Ok(Bson::Int64(value.as_i64().unwrap()))
        } else if value.is_u64() {
            Ok(Bson::Int64(value.as_u64().unwrap() as i64))
        } else if value.is_f64() {
            Ok(Bson::Int64(value.as_f64().unwrap() as i64))
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    fn parse_f64(&self, value: &JsonValue) -> Result<Bson, ActionError> {
        if value.is_i64() {
            Ok(Bson::Double(value.as_i64().unwrap() as f64))
        } else if value.is_u64() {
            Ok(Bson::Double(value.as_u64().unwrap() as f64))
        } else if value.is_f64() {
            Ok(Bson::Double(value.as_f64().unwrap()))
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    fn parse_date(&self, value: &JsonValue) -> Result<Bson, ActionError> {
        if value.is_string() {
            match NaiveDate::parse_from_str(&value.as_str().unwrap(), "%Y-%m-%d") {
                Ok(naive_date) => {
                    let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                    let val = Value::Date(date);
                    Ok(val.to_bson_value())
                }
                Err(_) => {
                    Err(ActionError::wrong_date_format())
                }
            }
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    fn parse_datetime(&self, value: &JsonValue) -> Result<Bson, ActionError> {
        if value.is_string() {
            match DateTime::parse_from_rfc3339(&value.as_str().unwrap()) {
                Ok(fixed_offset_datetime) => {
                    let datetime: DateTime<Utc> = fixed_offset_datetime.with_timezone(&Utc);
                    let value = Value::DateTime(datetime);
                    Ok(value.to_bson_value())
                }
                Err(_) => {
                    Err(ActionError::wrong_datetime_format())
                }
            }
        } else {
            Err(ActionError::wrong_input_type())
        }
    }

    fn parse_enum(&self, value: &JsonValue, enum_name: &str, graph: &Graph) -> Result<Bson, ActionError> {
        if value.is_string() {
            let str = value.as_str().unwrap();
            let r#enum = graph.r#enum(enum_name);
            if r#enum.contains(&str) {
                Ok(Bson::String(str.to_string()))
            } else {
                Err(ActionError::undefined_enum_value())
            }
        } else {
            Err(ActionError::wrong_input_type())
        }
        //Bson::DateTime()  from_chrono
    }

}

#[async_trait]
impl Connector for MongoDBConnector {

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        let primary_name = object.inner.model.primary_field_name();
        let keys = if is_new {
            object.inner.model.save_keys().clone()
        } else {
            object.inner.model.save_keys().iter().filter(|k| {
                object.inner.modified_fields.borrow().contains(&k.to_string())
            }).map(|k| { *k }).collect()
        };
        let col = &self.collections[object.inner.model.name()];
        if is_new {
            let mut doc = doc!{};
            for key in keys {
                let val = object.get_value(key).unwrap();
                if Some(key) == primary_name {
                    if val == None {
                        continue;
                    }
                }
                let json_val = match val {
                    None => Bson::Null,
                    Some(v) => v.to_bson_value()
                };
                doc.insert(key, json_val);
            }
            let result = col.insert_one(doc, None).await;
            match result {
                Ok(insert_one_result) => {
                    let id = insert_one_result.inserted_id.as_object_id().unwrap().to_hex();
                    if let Some(primary_field) = object.inner.model.primary_field() {
                        object.set_value(primary_field.name, Value::ObjectId(id));
                    } else {
                        object.inner.value_map.borrow_mut().insert("__id".to_string(), Value::ObjectId(id));
                    }
                }
                Err(error) => {
                    return Err(self._handle_write_error(*error.kind));
                }
            }
        } else {
            let object_id = if let Some(primary_field) = object.inner.model.primary_field() {
                object.get_value(primary_field.name).unwrap().unwrap().to_bson_value()
            } else {
                object.inner.value_map.borrow().get("__id").unwrap().to_bson_value()
            };
            let mut set = doc!{};
            for key in keys {
                let val = object.get_value(key).unwrap();
                let json_val = match val {
                    None => Bson::Null,
                    Some(v) => v.to_bson_value()
                };
                match primary_name {
                    Some(name) => {
                        if key == name {
                            if json_val != Bson::Null {
                                set.insert("_id", json_val);
                            }
                        } else {
                            set.insert(key, json_val);
                        }
                    }
                    None => {
                        set.insert(key, json_val);
                    }
                }
            }
            let result = col.update_one(doc!{"_id": object_id}, doc!{"$set": set}, None).await;
            return match result {
                Ok(update_result) => {
                    Ok(())
                }
                Err(error) => {
                    Err(self._handle_write_error(*error.kind))
                }
            }
        }
        Ok(())
    }

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError> {
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(ActionError::object_is_not_saved());
        }
        let object_id = if let Some(primary_field) = object.inner.model.primary_field() {
            object.get_value(primary_field.name).unwrap().unwrap().to_bson_value()
        } else {
            object.inner.value_map.borrow().get("__id").unwrap().to_bson_value()
        };
        let col = &self.collections[object.inner.model.name()];
        let result = col.delete_one(doc!{"_id": object_id}, None).await;
        return match result {
            Ok(_result) => {
                Ok(())
            }
            Err(_err) => {
                Err(ActionError::unknown_database_delete_error())
            }
        }
    }

    async fn find_unique(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError> {
        let r#where = finder.get("where");
        if r#where == None {
            return Err(ActionError::missing_input_section());
        }
        let r#where = r#where.unwrap();
        if !r#where.is_object() {
            return Err(ActionError::wrong_json_format());
        }
        let values = r#where.as_object().unwrap();
        // only allow single key for now
        if values.len() != 1 {
            return Err(ActionError::wrong_json_format());
        }
        // see if key is valid
        let key = values.keys().next().unwrap().as_str();
        if !model.unique_query_keys().contains(&key) {
            return Err(ActionError::field_is_not_unique(key))
        }
        // cast value
        let value = values.values().next().unwrap();
        let field = model.field(key);
        let query_key = if field.primary { "_id" } else { key };
        let decode_result = field.r#type.decode_value(value, graph);
        match decode_result {
            Ok(value) => {
                let col = &self.collections[model.name()];
                let find_result = col.find_one(doc!{query_key: value.to_bson_value()}, None).await;
                match find_result {
                    Ok(document_option) => {
                        match document_option {
                            Some(document) => {
                                let mut object = graph.new_object(model.name());
                                self.document_to_object(&document, &mut object);
                                return Ok(object);
                            }
                            None => {
                                return Err(ActionError::object_not_found())
                            }
                        }
                    }
                    Err(err) => {
                        return Err(ActionError::unknown_database_find_unique_error());
                    }
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    async fn find_first(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Object, ActionError> {
        todo!()
    }

    async fn find_many(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<Vec<Object>, ActionError> {
        let r#where = finder.get("where");
        let order_by = finder.get("orderBy");
        let take = finder.get("take");
        let skip = finder.get("skip");
        let col = &self.collections[model.name()];
        let where_input = self.build_where_input(model, r#where, graph);
        if let Err(err) = where_input {
            return Err(err);
        }
        let where_input = where_input.unwrap();
        println!("where input see: {}", where_input);
        let mut cur = col.find(where_input, None).await;
        match cur {
            Ok(cur) => {
                let mut result: Vec<Object> = vec![];
                let results: Vec<Result<Document, MongoDBError>> = cur.collect().await;
                for doc in results {
                    let obj = graph.new_object(model.name());
                    match self.document_to_object(&doc.unwrap(), &obj) {
                        Ok(_) => {
                            result.push(obj);
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                return Ok(result);
            }
            Err(err) => {
                return Err(ActionError::unknown_database_find_error());
            }
        }
    }

    async fn count(&self, graph: &'static Graph, model: &'static Model, finder: &Map<String, JsonValue>) -> Result<usize, ActionError> {
        let r#where = finder.get("where");
        let col = &self.collections[model.name()];
        let where_input = self.build_where_input(model, r#where, graph);
        if let Err(err) = where_input {
            return Err(err);
        }
        let where_input = where_input.unwrap();
        let result = col.count_documents(where_input, None).await;
        match result {
            Ok(val) => {
                Ok(val as usize)
            }
            Err(_) => {
                Err(ActionError::unknown_database_count_error())
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct MongoDBConnectorBuilder {
    options: ClientOptions
}

impl MongoDBConnectorBuilder {
    pub(crate) fn new(options: ClientOptions) -> MongoDBConnectorBuilder {
        MongoDBConnectorBuilder { options }
    }
}

#[async_trait]
impl ConnectorBuilder for MongoDBConnectorBuilder {
    async fn build_connector(&self, models: &Vec<Model>, reset_database: bool) -> Box<dyn Connector> {
        // for model in graph.models() {
        //     let col: Collection<Document> = self.database.collection(model.table_name());
        //     self.collections.insert(model.name(), col);
        // }
        Box::new(MongoDBConnector::new(self.options.clone(), models, reset_database).await)
    }
}

pub trait MongoDBConnectorHelpers {
    fn mongodb(&mut self, options: ClientOptions);
}

impl MongoDBConnectorHelpers for GraphBuilder {

    fn mongodb(&mut self, options: ClientOptions) {
        self.connector_builder = Some(Box::new(MongoDBConnectorBuilder::new(options)))
    }
}

unsafe impl Sync for MongoDBConnector {}
unsafe impl Send for MongoDBConnector {}

pub trait ToBsonValue {
    fn to_bson_value(&self) -> Bson;
}

impl ToBsonValue for Value {
    fn to_bson_value(&self) -> Bson {
        match self {
            Value::Null => {
                Bson::Null
            }
            Value::ObjectId(val) => {
                Bson::ObjectId(ObjectId::parse_str(val.as_str()).unwrap())
            }
            Value::Bool(val) => {
                Bson::Boolean(*val)
            }
            Value::I8(val) => {
                Bson::Int32(*val as i32)
            }
            Value::I16(val) => {
                Bson::Int32(*val as i32)
            }
            Value::I32(val) => {
                Bson::Int32(*val)
            }
            Value::I64(val) => {
                Bson::Int64(*val)
            }
            Value::I128(val) => {
                Bson::Int64(*val as i64)
            }
            Value::U8(val) => {
                Bson::Int32(*val as i32)
            }
            Value::U16(val) => {
                Bson::Int32(*val as i32)
            }
            Value::U32(val) => {
                Bson::Int64(*val as i64)
            }
            Value::U64(val) => {
                Bson::Int64(*val as i64)
            }
            Value::U128(val) => {
                Bson::Int64(*val as i64)
            }
            Value::F32(val) => {
                Bson::from(val)
            }
            Value::F64(val) => {
                Bson::from(val)
            }
            Value::String(val) => {
                Bson::String(val.clone())
            }
            Value::Date(val) => {
                Bson::DateTime(BsonDateTime::parse_rfc3339_str(val.format("%Y-%m-%d").to_string()).unwrap())
            }
            Value::DateTime(val) => {
                Bson::DateTime(BsonDateTime::from(*val))
            }
            Value::Vec(val) => {
                Bson::Array(val.iter().map(|i| { i.to_bson_value() }).collect())
            }
            Value::Map(val) => {
                let mut doc = doc!{};
                for (k, v) in val {
                    doc.insert(k.to_string(), v.to_bson_value());
                }
                Bson::Document(doc)
            }
            Value::Object(obj) => {
                panic!()
            }
        }
    }
}
