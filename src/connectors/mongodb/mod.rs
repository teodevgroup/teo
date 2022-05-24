use std::collections::HashMap;
use std::fmt::{Debug};
use std::sync::atomic::{Ordering};
use serde_json::{Value as JsonValue};
use async_trait::async_trait;
use bson::{Bson, DateTime, doc, Document, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, Database, Collection, IndexModel};
use mongodb::error::{ErrorKind, WriteFailure};
use mongodb::options::{CreateIndexOptions, DropDatabaseOptions, IndexOptions};
use regex::Regex;
use crate::core::connector::{Connector, ConnectorBuilder};
use crate::core::object::Object;
use crate::core::builders::GraphBuilder;
use crate::core::field::{Availability, FieldIndex};
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
            client: client,
            database: database,
            collections: collections
        }
    }
}

impl MongoDBConnector {
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

}

#[async_trait]
impl Connector for MongoDBConnector {

    async fn save_object(&self, object: &Object) -> Result<(), ActionError> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
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
                set.insert(key, json_val);
            }
            let result = col.update_one(doc!{"_id": object_id}, doc!{"$set": set}, None).await;
            match result {
                Ok(update_result) => {
                    return Ok(());
                }
                Err(error) => {
                    return Err(self._handle_write_error(*error.kind));
                }
            }
        }
        Ok(())
    }

    async fn delete_object(&self, object: &Object) {
        todo!()
    }

    async fn find_unique(&self, model: &Model, finder: JsonValue) -> Option<Object> {
        todo!()
    }

    async fn find_one(&self, model: &Model, finder: JsonValue) -> Option<Object> {
        todo!()
    }

    async fn find_many(&self, model: &Model, finder: JsonValue) -> Vec<Object> {
        todo!()
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
                Bson::DateTime(DateTime::parse_rfc3339_str(val.format("%Y-%m-%d").to_string()).unwrap())
            }
            Value::DateTime(val) => {
                Bson::DateTime(DateTime::from(*val))
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
        }
    }
}
