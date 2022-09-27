use std::collections::{HashMap};
use std::fmt::{Debug};
use std::sync::Arc;
use rust_decimal::prelude::FromStr;
use std::sync::atomic::{Ordering};
use serde_json::{json, Value as JsonValue};
use async_trait::async_trait;
use bson::{Bson, doc, Document};
use futures_util::StreamExt;
use mongodb::{options::ClientOptions, Client, Database, Collection, IndexModel};
use mongodb::error::{ErrorKind, WriteFailure, Error as MongoDBError};
use mongodb::options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument};
use regex::Regex;
use rust_decimal::Decimal;
use crate::connectors::mongodb::aggregation_builder::{build_query_pipeline_from_json, build_where_input, has_negative_take, ToBsonValue};
use crate::connectors::mongodb::save_session::MongoDBSaveSession;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::core::connector::Connector;
use crate::core::object::Object;
use crate::core::field::Sort;
use crate::core::field::r#type::FieldType;
use crate::core::graph::Graph;
use crate::core::input::AtomicUpdateType;
use crate::core::model::{Model, ModelIndex, ModelIndexType};
use crate::core::save_session::SaveSession;
use crate::core::value::Value;
use crate::core::error::ActionError;

#[derive(Debug)]
pub struct MongoDBConnector {
    client: Client,
    database: Database,
    collections: HashMap<String, Collection<Document>>
}

impl MongoDBConnector {
    pub(crate) async fn new(url: String, models: &Vec<Model>, reset_database: bool) -> MongoDBConnector {
        let options = ClientOptions::parse(url).await;
        if options.is_err() {
            println!("mongodb option is error");
            panic!("Wrong mongodb options");
        }
        let options = options.unwrap();
        let client = Client::with_options(options.clone());
        if client.is_err() {
            println!("mongodb connection error");
            panic!("Error occurred when establishing mongodb connection.");
        }
        let client = client.unwrap();
        match client
            .database("xxxxxpingpingpingxxxxx")
            .run_command(doc! {"ping": 1}, None)
            .await {
            Ok(_) => {
            },
            Err(_) => {
                println!("cannot connect mongodb");
                panic!("cannot connect mongodb");
            }
        }
        let database = client.database(&options.default_database.clone().unwrap());
        if reset_database {
            let _ = database.drop(None).await;
        }
        let mut collections: HashMap<String, Collection<Document>> = HashMap::new();
        for model in models {
            let name = model.name();
            let collection: Collection<Document> = database.collection(model.table_name());
            let mut reviewed_names: Vec<String> = Vec::new();
            let cursor_result = collection.list_indexes(None).await;
            if cursor_result.is_ok() {
                let mut cursor = cursor_result.unwrap();
                while let Some(Ok(index)) = cursor.next().await {
                    if index.keys == doc!{"_id": 1} {
                        continue
                    }
                    let name = (&index).options.as_ref().unwrap().name.as_ref().unwrap();
                    let result = model.indices().iter().find(|i| &i.name == name);
                    if result.is_none() {
                        // not in our model definition, but in the database
                        // drop this index
                        let _ = collection.drop_index(name, None).await.unwrap();
                    } else {
                        let result = result.unwrap();
                        let our_format_index: ModelIndex = (&index).into();
                        if result != &our_format_index {
                            // alter this index
                            // drop first
                            let _ = collection.drop_index(name, None).await.unwrap();
                            // create index
                            let index_options = IndexOptions::builder()
                                .name(result.name.clone())
                                .unique(result.index_type == ModelIndexType::Unique || result.index_type == ModelIndexType::Primary)
                                .sparse(true)
                                .build();
                            let mut keys = doc!{};
                            for item in &result.items {
                                let field = model.field(&item.field_name).unwrap();
                                let column_name = field.column_name();
                                keys.insert(column_name, if item.sort == Sort::Asc { 1 } else { -1 });
                            }
                            let index_model = IndexModel::builder().keys(keys).options(index_options).build();
                            let _result = collection.create_index(index_model, None).await;
                        }
                    }
                    reviewed_names.push(name.clone());
                }
            }
            for index in model.indices() {
                if !reviewed_names.contains(&index.name) {
                    // create this index
                    let index_options = IndexOptions::builder()
                        .name(index.name.clone())
                        .unique(index.index_type == ModelIndexType::Unique || index.index_type == ModelIndexType::Primary)
                        .sparse(true)
                        .build();
                    let mut keys = doc!{};
                    for item in &index.items {
                        let field = model.field(&item.field_name).unwrap();
                        let column_name = field.column_name();
                        keys.insert(column_name, if item.sort == Sort::Asc { 1 } else { -1 });
                    }
                    let index_model = IndexModel::builder().keys(keys).options(index_options).build();
                    let _result = collection.create_index(index_model, None).await;
                }
            }
            collections.insert(name.to_owned(), collection);
        }
        MongoDBConnector {
            client,
            database,
            collections
        }
    }

    fn document_to_object(&self, document: &Document, object: &Object, select: Option<&JsonValue>, include: Option<&JsonValue>) -> Result<(), ActionError> {
        for key in document.keys() {
            let object_field = object.model().fields().iter().find(|f| f.column_name() == key);
            if object_field.is_some() {
                // field
                let object_field = object_field.unwrap();
                let object_key = &object_field.name;
                let field_type = &object_field.field_type;
                let bson_value = document.get(key).unwrap();
                let value_result = self.bson_value_to_field_value(object_key, bson_value, field_type);
                match value_result {
                    Ok(value) => {
                        object.inner.value_map.lock().unwrap().insert(object_key.to_string(), value);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else {
                // relation
                let relation = object.model().relation(key);
                if relation.is_none() {
                    continue;
                }
                let inner_finder = if let Some(include) = include {
                    include.get(key)
                } else {
                    None
                };
                let inner_select = if let Some(inner_finder) = inner_finder {
                    inner_finder.get("select")
                } else {
                    None
                };
                let inner_include = if let Some(inner_finder) = inner_finder {
                    inner_finder.get("include")
                } else {
                    None
                };
                let relation = relation.unwrap();
                let model_name = &relation.model;
                let object_bsons = document.get(key).unwrap().as_array().unwrap();
                let mut related: Vec<Object> = vec![];
                for related_object_bson in object_bsons {
                    let related_object = object.graph().new_object(model_name)?;
                    self.document_to_object(related_object_bson.as_document().unwrap(), &related_object, inner_select, inner_include)?;
                    related.push(related_object);
                }
                object.inner.relation_query_map.lock().unwrap().insert(key.to_string(), related);
            }
        }
        object.inner.is_initialized.store(true, Ordering::SeqCst);
        object.inner.is_new.store(false, Ordering::SeqCst);
        object.set_select(select).unwrap();
        Ok(())
    }

    fn bson_value_to_field_value(&self, field_name: &str, bson_value: &Bson, field_type: &FieldType) -> Result<Value, ActionError> {
        return match field_type {
            FieldType::Undefined => {
                panic!()
            }
            FieldType::ObjectId => {
                match bson_value.as_object_id() {
                    Some(object_id) => {
                        Ok(Value::ObjectId(object_id.to_hex()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }

            }
            FieldType::Bool => {
                match bson_value.as_bool() {
                    Some(bool) => {
                        Ok(Value::Bool(bool))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::I8 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::I8(val as i8))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::I16 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::I16(val as i16))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::I32 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::I32(val as i32))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::I64 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::I64(val))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::I128 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::I128(val as i128))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::U8 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::U8(val as u8))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::U16 => {
                match bson_value.as_i32() {
                    Some(val) => {
                        Ok(Value::U16(val as u16))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::U32 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::U32(val as u32))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::U64 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::U64(val as u64))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::U128 => {
                match bson_value.as_i64() {
                    Some(val) => {
                        Ok(Value::U128(val as u128))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::F32 => {
                match bson_value.as_f64() {
                    Some(val) => {
                        Ok(Value::F32(val as f32))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::F64 => {
                match bson_value.as_f64() {
                    Some(val) => {
                        Ok(Value::F64(val))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::Decimal => {
                match bson_value {
                    Bson::Decimal128(d128) => {
                        Ok(Value::Decimal(Decimal::from_str(&d128.to_string()).unwrap()))
                    }
                    _ => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::String => {
                match bson_value.as_str() {
                    Some(val) => {
                        Ok(Value::String(val.to_string()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::Date => {
                match bson_value.as_datetime() {
                    Some(val) => {
                        Ok(Value::Date(val.to_chrono().date()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::DateTime => {
                match bson_value.as_datetime() {
                    Some(val) => {
                        Ok(Value::DateTime(val.to_chrono()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::Enum(_) => {
                match bson_value.as_str() {
                    Some(val) => {
                        Ok(Value::String(val.to_string()))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::Vec(inner) => {
                match bson_value.as_array() {
                    Some(arr) => {
                        let mut vec: Vec<Value> = vec![];
                        for val in arr {
                            vec.push(self.bson_value_to_field_value("", val, &inner.field_type)?);
                        }
                        Ok(Value::Vec(vec))
                    }
                    None => {
                        Err(ActionError::unmatched_data_type_in_database(field_name))
                    }
                }
            }
            FieldType::Map(_) => {
                panic!()
            }
            FieldType::Object(_) => {
                panic!()
            }
        };
    }

    fn _handle_write_error(&self, error_kind: &ErrorKind) -> ActionError {
        return match error_kind {
            ErrorKind::Write(write) => {
                match write {
                    WriteFailure::WriteError(write_error) => {
                        match write_error.code {
                            11000 => {
                                let regex = Regex::new(r"dup key: \{ (.+?):").unwrap();
                                let match_result = regex.captures(write_error.message.as_str()).unwrap().get(1);
                                ActionError::unique_value_duplicated(match_result.unwrap().as_str())
                            }
                            _ => {
                                ActionError::unknown_database_write_error()
                            }
                        }
                    }
                    _ => {
                        ActionError::unknown_database_write_error()
                    }
                }
            }
            _ => {
                ActionError::unknown_database_write_error()
            }
        }
    }

    async fn aggregate_or_group_by(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<Vec<JsonValue>, ActionError> {
        let aggregate_input = build_query_pipeline_from_json(model, graph, QueryPipelineType::Many, false, finder)?;
        let col = &self.collections[model.name()];
        let cur = col.aggregate(aggregate_input, None).await;
        if cur.is_err() {
            println!("{:?}", cur);
            return Err(ActionError::unknown_database_find_error());
        }
        let cur = cur.unwrap();
        let results: Vec<Result<Document, MongoDBError>> = cur.collect().await;
        let mut final_retval: Vec<JsonValue> = vec![];
        for result in results.iter() {
            // there are records
            let data = result.as_ref().unwrap();
            let mut retval = json!({});
            for (g, o) in data {
                if g.as_str() == "_id" {
                    continue;
                }
                // aggregate
                if g.starts_with("_") {
                    retval.as_object_mut().unwrap().insert(g.clone(), json!({}));
                    for (dbk, v) in o.as_document().unwrap() {
                        let k = if dbk == "_all" { "_all" } else { model.column_name_for_field_name(dbk).unwrap() };
                        if let Some(f) = v.as_f64() {
                            retval.as_object_mut().unwrap().get_mut(g.as_str()).unwrap().as_object_mut().unwrap().insert(k.to_string(), json!(f));
                        } else if let Some(i) = v.as_i64() {
                            retval.as_object_mut().unwrap().get_mut(g.as_str()).unwrap().as_object_mut().unwrap().insert(k.to_string(), json!(i));
                        } else if let Some(i) = v.as_i32() {
                            retval.as_object_mut().unwrap().get_mut(g.as_str()).unwrap().as_object_mut().unwrap().insert(k.to_string(), json!(i));
                        } else if v.as_null().is_some() {
                            retval.as_object_mut().unwrap().get_mut(g.as_str()).unwrap().as_object_mut().unwrap().insert(k.to_string(), json!(null));
                        }
                    }
                } else {
                    // group by field
                    let field = model.field(g).unwrap();
                    let val = if o.as_null().is_some() { Value::Null } else {
                        self.bson_value_to_field_value(g, o, &field.field_type).unwrap()
                    };
                    let json_val = val.to_json_value();
                    retval.as_object_mut().unwrap().insert(g.to_string(), json_val);
                }
            }
            final_retval.push(retval);
        }
        Ok(final_retval)
    }
}

#[async_trait]
impl Connector for MongoDBConnector {

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        let primary_name = object.model().primary_field_name();
        let keys = object.keys_for_save();
        let col = &self.collections[object.model().name()];
        if is_new {
            let mut doc = doc!{};
            for key in keys {
                let val = object.get_value(&key).unwrap();
                if Some(key) == primary_name {
                    if val.is_null() {
                        continue;
                    }
                }
                let json_val = match val {
                    Value::Null => Bson::Null,
                    _ => val.to_bson_value()
                };
                if json_val != Bson::Null {
                    doc.insert(key, json_val);
                }
            }
            let result = col.insert_one(doc, None).await;
            match result {
                Ok(insert_one_result) => {
                    let id = insert_one_result.inserted_id.as_object_id().unwrap().to_hex();
                    if let Some(primary_field) = object.model().primary_field() {
                        object.set_value(&primary_field.name, Value::ObjectId(id));
                    } else {
                        object.inner.value_map.lock().unwrap().insert("__id".to_string(), Value::ObjectId(id));
                    }
                }
                Err(error) => {
                    println!("see error reason, {:?}", error);
                    return Err(self._handle_write_error(&error.kind));
                }
            }
        } else {
            let object_id = if let Some(primary_field) = object.model().primary_field() {
                object.get_value(&primary_field.name).unwrap().to_bson_value()
            } else {
                object.inner.value_map.lock().unwrap().get("__id").unwrap().to_bson_value()
            };
            let mut set = doc!{};
            let mut unset = doc!{};
            let mut inc = doc!{};
            let mut mul = doc!{};
            let mut push = doc!{};
            for key in keys {
                let column_name = object.model().field(key).unwrap().column_name();
                let aumap = object.inner.atomic_updator_map.lock().unwrap();
                if aumap.contains_key(key) {
                    let updator = aumap.get(key).unwrap();
                    match updator {
                        AtomicUpdateType::Increment(val) => {
                            inc.insert(column_name, val.to_bson_value());
                        }
                        AtomicUpdateType::Decrement(val) => {
                            inc.insert(column_name, (val.neg()).to_bson_value());
                        }
                        AtomicUpdateType::Multiply(val) => {
                            mul.insert(column_name, val.to_bson_value());
                        }
                        AtomicUpdateType::Divide(val) => {
                            mul.insert(column_name, Bson::Double(val.recip()));
                        }
                        AtomicUpdateType::Push(val) => {
                            push.insert(column_name, val.to_bson_value());
                        }
                    };
                } else {
                    let val = object.get_value(key).unwrap();
                    let json_val = val.to_bson_value();
                    match primary_name {
                        Some(name) => {
                            if key == name {
                                if json_val != Bson::Null {
                                    set.insert("_id", json_val);
                                }
                            } else {
                                if json_val == Bson::Null {
                                    unset.insert(key, json_val);
                                } else {
                                    set.insert(key, json_val);
                                }
                            }
                        }
                        None => {
                            if json_val == Bson::Null {
                                unset.insert(key, json_val);
                            } else {
                                set.insert(key, json_val);
                            }
                        }
                    }
                }
            }
            let mut update_doc = doc!{};
            let mut return_new = false;
            if !set.is_empty() {
                update_doc.insert("$set", set);
            }
            if !unset.is_empty() {
                update_doc.insert("$unset", unset);
            }
            if !inc.is_empty() {
                update_doc.insert("$inc", inc);
                return_new = true;
            }
            if !mul.is_empty() {
                update_doc.insert("$mul", mul);
                return_new = true;
            }
            if !push.is_empty() {
                update_doc.insert("$push", push);
                return_new = true;
            }
            if !return_new {
                if !update_doc.is_empty() {
                    let result = col.update_one(doc!{"_id": object_id}, update_doc, None).await;
                    // sync result back
                    return match result {
                        Ok(_update_result) => {
                            Ok(())
                        }
                        Err(error) => {
                            println!("{:?}", error);
                            Err(self._handle_write_error(&error.kind))
                        }
                    }
                }
            } else {
                if !update_doc.is_empty() {
                    let options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();
                    let result = col.find_one_and_update(doc!{"_id": object_id}, update_doc, options).await;
                    match &result {
                        Ok(updated_document) => {
                            for key in object.inner.atomic_updator_map.lock().unwrap().keys() {
                                let bson_new_val = updated_document.as_ref().unwrap().get(key).unwrap();
                                let field = object.model().field(key).unwrap();
                                let field_value = self.bson_value_to_field_value(key, bson_new_val, &field.field_type);
                                match field_value {
                                    Ok(field_value) => {
                                        object.inner.value_map.lock().unwrap().insert(key.to_string(), field_value);
                                    }
                                    Err(err) => {
                                        println!("{:?}", err);
                                        panic!("here cannot error");
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            println!("{:?}", error);
                            return Err(self._handle_write_error(&error.kind));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn delete_object(&self, object: &Object) -> Result<(), ActionError> {
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(ActionError::object_is_not_saved());
        }
        let model = object.model();
        let mut query = doc!{};
        for item in &model.primary().unwrap().items {
            let field_name = &item.field_name;
            let column_name = model.field(field_name).unwrap().column_name();
            let value = object.get_value(field_name).unwrap().to_bson_value();
            query.insert(column_name, value);
        }
        let col = &self.collections[model.name()];
        let result = col.delete_one(query, None).await;
        return match result {
            Ok(_result) => {
                Ok(())
            }
            Err(_err) => {
                Err(ActionError::unknown_database_delete_error())
            }
        }
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        let select = finder.get("select");
        let include = finder.get("include");
        let aggregate_input = build_query_pipeline_from_json(model, graph, QueryPipelineType::Unique, mutation_mode, finder)?;
        let col = &self.collections[model.name()];
        let cur = col.aggregate(aggregate_input, None).await;
        if cur.is_err() {
            return Err(ActionError::unknown_database_find_unique_error());
        }
        let cur = cur.unwrap();
        let results: Vec<Result<Document, MongoDBError>> = cur.collect().await;
        if results.is_empty() {
            return Err(ActionError::object_not_found());
        }
        for doc in results {
            let obj = graph.new_object(model.name())?;
            self.document_to_object(&doc.unwrap(), &obj, select, include)?;
            return Ok(obj);
        }
        Err(ActionError::object_not_found())
    }

    async fn find_first(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Object, ActionError> {
        let mut finder = finder.as_object().clone().unwrap().clone();
        finder.insert("take".to_string(), JsonValue::Number(1.into()));
        let result = self.find_many(graph, model, &JsonValue::Object(finder), mutation_mode).await;
        match result {
            Err(err) => Err(err),
            Ok(retval) => {
                if retval.is_empty() {
                    Err(ActionError::object_not_found())
                } else {
                    Ok(retval.get(0).unwrap().clone())
                }
            }
        }
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &JsonValue, mutation_mode: bool) -> Result<Vec<Object>, ActionError> {
        let select = finder.get("select");
        let include = finder.get("include");
        let aggregate_input = build_query_pipeline_from_json(model, graph, QueryPipelineType::Many, mutation_mode, finder)?;
        let reverse = has_negative_take(finder);
        let col = &self.collections[model.name()];
        let cur = col.aggregate(aggregate_input, None).await;
        if cur.is_err() {
            println!("{:?}", cur);
            return Err(ActionError::unknown_database_find_error());
        }
        let cur = cur.unwrap();
        let mut result: Vec<Object> = vec![];
        let results: Vec<Result<Document, MongoDBError>> = cur.collect().await;
        for doc in results {
            let obj = graph.new_object(model.name())?;
            match self.document_to_object(&doc.unwrap(), &obj, select, include) {
                Ok(_) => {
                    if reverse {
                        result.insert(0, obj);
                    } else {
                        result.push(obj);
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(result)
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<usize, ActionError> {
        let finder = finder.as_object().unwrap();
        let r#where = finder.get("where");
        let col = &self.collections[model.name()];
        let where_input = build_where_input(model, graph, r#where);
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

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<JsonValue, ActionError> {
        let results = self.aggregate_or_group_by(graph, model, finder).await?;
        if results.is_empty() {
            // there is no record
            let mut retval = json!({});
            for (g, o) in finder.as_object().unwrap() {
                retval.as_object_mut().unwrap().insert(g.clone(), json!({}));
                for (k, _v) in o.as_object().unwrap() {
                    let value = if g == "_count" { json!(0) } else { json!(null) };
                    retval.as_object_mut().unwrap().get_mut(g.as_str()).unwrap().as_object_mut().unwrap().insert(k.to_string(), value);
                }
            }
            Ok(retval)
        } else {
            Ok(results.get(0).unwrap().clone())
        }
    }

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &JsonValue) -> Result<JsonValue, ActionError> {
        Ok(JsonValue::Array(self.aggregate_or_group_by(graph, model, finder).await?))
    }

    fn new_save_session(&self) -> Arc<dyn SaveSession> {
        Arc::new(MongoDBSaveSession {})
    }
}

unsafe impl Sync for MongoDBConnector {}
unsafe impl Send for MongoDBConnector {}
