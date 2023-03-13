pub mod save_session;

use std::collections::{HashMap};
use std::fmt::{Debug};
use std::ops::Neg;
use std::sync::Arc;
use std::sync::atomic::{Ordering};
use async_trait::async_trait;
use bson::{Bson, doc, Document};
use futures_util::StreamExt;
use key_path::path;
use mongodb::{options::ClientOptions, Client, Database, Collection, IndexModel};
use mongodb::error::{ErrorKind, WriteFailure, Error as MongoDBError};
use mongodb::options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument};
use regex::Regex;
use crate::connectors::mongodb::aggregation::Aggregation;
use crate::connectors::mongodb::bson::coder::BsonCoder;
use crate::connectors::mongodb::connector::save_session::MongoDBSaveSession;
use crate::core::action::{Action, FIND, MANY, NESTED, SINGLE};
use crate::core::action::source::ActionSource;
use crate::core::connector::Connector;
use crate::core::object::Object;
use crate::core::field::Sort;
use crate::core::graph::Graph;
use crate::core::model::{Model};
use crate::core::model::index::{ModelIndex, ModelIndexType};
use crate::core::connector::SaveSession;
use crate::core::database::r#type::DatabaseType;
use crate::core::teon::Value;
use crate::core::error::Error;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::input::Input;
use crate::core::result::Result;
use crate::teon;

#[derive(Debug)]
pub struct MongoDBConnector {
    loaded: bool,
    client: Client,
    database: Database,
    collections: HashMap<String, Collection<Document>>
}

impl MongoDBConnector {
    pub(crate) async fn new(url: String) -> MongoDBConnector {
        let options = match ClientOptions::parse(url).await {
            Ok(options) => options,
            Err(_) => panic!("MongoDB url is invalid.")
        };
        let database_name = match &options.default_database {
            Some(database_name) => database_name,
            None => panic!("No database name found in MongoDB url.")
        };
        let client = match Client::with_options(options.clone()) {
            Ok(client) => client,
            Err(_) => panic!("MongoDB client creating error.")
        };
        match client.database("xxxxxpingpingpingxxxxx").run_command(doc! {"ping": 1}, None).await {
            Ok(_) => (),
            Err(_) => panic!("Cannot connect to MongoDB database."),
        }
        let database = client.database(&database_name);
        MongoDBConnector {
            loaded: false,
            client,
            database,
            collections: HashMap::new()
        }
    }

    fn document_to_object(&self, document: &Document, object: &Object, select: Option<&Value>, include: Option<&Value>) -> Result<()> {
        for key in document.keys() {
            let object_field = object.model().fields().iter().find(|f| f.column_name() == key);
            if object_field.is_some() {
                // field
                let object_field = object_field.unwrap();
                let object_key = &object_field.name;
                let field_type = object_field.field_type();
                let bson_value = document.get(key).unwrap();
                let value_result = BsonCoder::decode(object.model(), object.graph(), field_type, object_field.is_optional(), bson_value, path![]);
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
                let model_name = relation.model();
                let object_bsons = document.get(key).unwrap().as_array().unwrap();
                let mut related: Vec<Object> = vec![];
                for related_object_bson in object_bsons {
                    let action = Action::from_u32(NESTED | FIND | (if relation.is_vec() { MANY } else { SINGLE }));
                    let related_object = object.graph().new_object(model_name, action, object.action_source().clone())?;
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

    fn _handle_write_error(&self, error_kind: &ErrorKind, object: &Object) -> Error {
        return match error_kind {
            ErrorKind::Write(write) => {
                match write {
                    WriteFailure::WriteError(write_error) => {
                        match write_error.code {
                            11000 => {
                                let regex = Regex::new(r"dup key: \{ (.+?):").unwrap();
                                let field_column_name = regex.captures(write_error.message.as_str()).unwrap().get(1).unwrap().as_str();
                                let field_name = object.model().field_with_column_name(field_column_name).unwrap().name();
                                Error::unique_value_duplicated(field_name)
                            }
                            _ => {
                                Error::unknown_database_write_error()
                            }
                        }
                    }
                    _ => {
                        Error::unknown_database_write_error()
                    }
                }
            }
            _ => {
                Error::unknown_database_write_error()
            }
        }
    }

    async fn aggregate_or_group_by(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Vec<Value>> {
        let aggregate_input = Aggregation::build_for_aggregate(model, graph, finder)?;
        let col = &self.collections[model.name()];
        let cur = col.aggregate(aggregate_input, None).await;
        if cur.is_err() {
            println!("{:?}", cur);
            return Err(Error::unknown_database_find_error());
        }
        let cur = cur.unwrap();
        let results: Vec<std::result::Result<Document, MongoDBError>> = cur.collect().await;
        let mut final_retval: Vec<Value> = vec![];
        for result in results.iter() {
            // there are records
            let data = result.as_ref().unwrap();
            let mut retval = teon!({});
            for (g, o) in data {
                if g.as_str() == "_id" {
                    continue;
                }
                // aggregate
                if g.starts_with("_") {
                    retval.as_hashmap_mut().unwrap().insert(g.clone(), teon!({}));
                    for (dbk, v) in o.as_document().unwrap() {
                        let k = dbk;
                        if let Some(f) = v.as_f64() {
                            retval.as_hashmap_mut().unwrap().get_mut(g.as_str()).unwrap().as_hashmap_mut().unwrap().insert(k.to_string(), teon!(f));
                        } else if let Some(i) = v.as_i64() {
                            retval.as_hashmap_mut().unwrap().get_mut(g.as_str()).unwrap().as_hashmap_mut().unwrap().insert(k.to_string(), teon!(i));
                        } else if let Some(i) = v.as_i32() {
                            retval.as_hashmap_mut().unwrap().get_mut(g.as_str()).unwrap().as_hashmap_mut().unwrap().insert(k.to_string(), teon!(i));
                        } else if v.as_null().is_some() {
                            retval.as_hashmap_mut().unwrap().get_mut(g.as_str()).unwrap().as_hashmap_mut().unwrap().insert(k.to_string(), teon!(null));
                        }
                    }
                } else {
                    // group by field
                    let field = model.field(g).unwrap();
                    let val = if o.as_null().is_some() { Value::Null } else {
                        BsonCoder::decode(model, graph, field.field_type(), true, o, path![])?
                    };
                    let json_val = val;
                    retval.as_hashmap_mut().unwrap().insert(g.to_string(), json_val);
                }
            }
            final_retval.push(retval);
        }
        Ok(final_retval)
    }

    async fn create_object(&self, object: &Object) -> Result<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let col = &self.collections[model.name()];
        let auto_keys = model.auto_keys();
        // create
        let mut doc = doc!{};
        for key in keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                let val: Bson = BsonCoder::encode(field.field_type(), object.get_value(&key).unwrap())?;
                if val != Bson::Null {
                    doc.insert(column_name, val);
                }
            } else if let Some(property) = model.property(key) {
                let val: Bson = BsonCoder::encode(property.field_type(), object.get_property(&key).await.unwrap())?;
                if val != Bson::Null {
                    doc.insert(key, val);
                }
            }
        }
        let result = col.insert_one(doc, None).await;
        match result {
            Ok(insert_one_result) => {
                let id = insert_one_result.inserted_id;
                for key in auto_keys {
                    let field = model.field(key).unwrap();
                    if field.column_name() == "_id" {
                        let new_value = BsonCoder::decode(model, object.graph(), field.field_type(), field.is_optional(), &id, path![]).unwrap();
                        object.set_value(field.name(), new_value)?;
                    }
                }
            }
            Err(error) => {
                return Err(self._handle_write_error(&error.kind, object));
            }
        }
        Ok(())
    }

    async fn update_object(&self, object: &Object) -> Result<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let col = &self.collections[model.name()];
        let identifier: Bson = object.db_identifier().into();
        let identifier = identifier.as_document().unwrap();
        let mut set = doc!{};
        let mut unset = doc!{};
        let mut inc = doc!{};
        let mut mul = doc!{};
        let mut push = doc!{};
        for key in keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                if let Some(updator) = object.get_atomic_updator(key) {
                    let (key, val) = Input::key_value(updator.as_hashmap().unwrap());
                    match key {
                        "increment" => inc.insert(column_name, Bson::from(val)),
                        "decrement" => inc.insert(column_name, Bson::from(&val.neg().unwrap())),
                        "multiply" => mul.insert(column_name, Bson::from(val)),
                        "divide" => mul.insert(column_name, Bson::Double(val.recip())),
                        "push" => push.insert(column_name, Bson::from(val)),
                        _ => panic!("Unhandled key."),
                    };
                } else {
                    let bson_val: Bson = BsonCoder::encode(field.field_type(), object.get_value(&key).unwrap())?;
                    if bson_val == Bson::Null {
                        unset.insert(key, bson_val);
                    } else {
                        set.insert(key, bson_val);
                    }
                }
            } else if let Some(property) = model.property(key) {
                let bson_val: Bson = BsonCoder::encode(property.field_type(), object.get_property(&key).await.unwrap())?;
                if bson_val != Bson::Null {
                    set.insert(key, bson_val);
                } else {
                    unset.insert(key, bson_val);
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
        if update_doc.is_empty() {
            return Ok(());
        }
        if !return_new {
            let result = col.update_one(identifier.clone(), update_doc, None).await;
            return match result {
                Ok(_) => Ok(()),
                Err(error) => {
                    Err(self._handle_write_error(&error.kind, object))
                }
            }
        } else {
            let options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();
            let result = col.find_one_and_update(identifier.clone(), update_doc, options).await;
            match result {
                Ok(updated_document) => {
                    for key in object.inner.atomic_updator_map.lock().unwrap().keys() {
                        let bson_new_val = updated_document.as_ref().unwrap().get(key).unwrap();
                        let field = object.model().field(key).unwrap();
                        let field_value = BsonCoder::decode(model, object.graph(), field.field_type(), field.is_optional(), bson_new_val, path![])?;
                        object.inner.value_map.lock().unwrap().insert(key.to_string(), field_value);
                    }
                }
                Err(error) => {
                    return Err(self._handle_write_error(&error.kind, object));
                }
            }
        }
        Ok(())
    }

}

#[async_trait]
impl Connector for MongoDBConnector {
    fn default_database_type(&self, field_type: &FieldType) -> DatabaseType {
        match field_type {
            FieldType::ObjectId => DatabaseType::ObjectId,
            FieldType::Bool => DatabaseType::Bool,
            FieldType::I32 => DatabaseType::Int32,
            FieldType::I64 => DatabaseType::Int64,
            FieldType::F32 => DatabaseType::Double { m: None, d: None },
            FieldType::F64 => DatabaseType::Double { m: None, d: None },
            FieldType::Decimal => DatabaseType::Decimal { m: None, d: None },
            FieldType::String => DatabaseType::String,
            FieldType::Date => DatabaseType::DateTime(3),
            FieldType::DateTime => DatabaseType::DateTime(3),
            FieldType::Enum(_) => DatabaseType::String,
            FieldType::Vec(_) => panic!(""),
            FieldType::HashMap(_) => panic!(""),
            FieldType::BTreeMap(_) => panic!(""),
            FieldType::Object(_) => panic!(""),
        }
    }

    async fn load(&mut self, models: &Vec<Model>) -> Result<()> {
        let mut collections: HashMap<String, Collection<Document>> = HashMap::new();
        for model in models {
            let collection: Collection<Document> = self.database.collection(model.table_name());
            collections.insert(model.name().to_owned(), collection);
        }
        self.collections = collections;
        Ok(())
    }

    async fn migrate(&mut self, models: &Vec<Model>, reset_database: bool) -> Result<()> {
        if reset_database {
            let _ = self.database.drop(None).await;
        }
        for model in models {
            let name = model.name();
            let collection = self.collections.get(name).unwrap();
            let mut reviewed_names: Vec<String> = Vec::new();
            let cursor_result = collection.list_indexes(None).await;
            if cursor_result.is_ok() {
                let mut cursor = cursor_result.unwrap();
                while let Some(Ok(index)) = cursor.next().await {
                    if index.keys == doc!{"_id": 1} {
                        continue
                    }
                    let name = (&index).options.as_ref().unwrap().name.as_ref().unwrap();
                    let result = model.indices().iter().find(|i| &i.mongodb_name() == name);
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
                                .name(result.mongodb_name())
                                .unique(result.r#type() == ModelIndexType::Unique || result.r#type() == ModelIndexType::Primary)
                                .sparse(true)
                                .build();
                            let mut keys = doc!{};
                            for item in result.items() {
                                let field = model.field(item.field_name()).unwrap();
                                let column_name = field.column_name();
                                keys.insert(column_name, if item.sort() == Sort::Asc { 1 } else { -1 });
                            }
                            let index_model = IndexModel::builder().keys(keys).options(index_options).build();
                            let _result = collection.create_index(index_model, None).await;
                        }
                    }
                    reviewed_names.push(name.clone());
                }
            }
            for index in model.indices() {
                if !reviewed_names.contains(&index.mongodb_name()) {
                    // ignore primary
                    if index.keys().len() == 1 {
                        let field = model.field(index.keys().get(0).unwrap()).unwrap();
                        if field.column_name() == "_id" {
                            continue
                        }
                    }
                    // create this index
                    let index_options = IndexOptions::builder()
                        .name(index.mongodb_name())
                        .unique(index.r#type() == ModelIndexType::Unique || index.r#type() == ModelIndexType::Primary)
                        .sparse(true)
                        .build();
                    let mut keys = doc!{};
                    for item in index.items() {
                        let field = model.field(item.field_name()).unwrap();
                        let column_name = field.column_name();
                        keys.insert(column_name, if item.sort() == Sort::Asc { 1 } else { -1 });
                    }
                    let index_model = IndexModel::builder().keys(keys).options(index_options).build();
                    let result = collection.create_index(index_model, None).await;
                    if result.is_err() {
                        println!("index create error: {:?}", result.err().unwrap());
                    }
                }
            }
        }
        Ok(())
    }

    async fn query_raw(&self, _query: &Value) -> Result<Value> {
        unreachable!()
        // let collection = self.collections.get(table.unwrap()).unwrap();
        // let result = collection.aggregate(BsonCoder::encode_without_default_type(query), None).await;
        // if result.is_err() {
        //
        // }
    }
    
    async fn save_object(&self, object: &Object, _session: Arc<dyn SaveSession>) -> Result<()> {
        if object.inner.is_new.load(Ordering::SeqCst) {
            self.create_object(object).await
        } else {
            self.update_object(object).await
        }
    }

    async fn delete_object(&self, object: &Object, _session: Arc<dyn SaveSession>) -> Result<()> {
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(Error::object_is_not_saved_thus_cant_be_deleted());
        }
        let model = object.model();
        let col = &self.collections[model.name()];
        let bson_identifier: Bson = object.db_identifier().into();
        let document_identifier = bson_identifier.as_document().unwrap();
        let result = col.delete_one(document_identifier.clone(), None).await;
        return match result {
            Ok(_result) => Ok(()),
            Err(_err) => {
                Err(Error::unknown_database_delete_error())
            }
        }
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &Value, _mutation_mode: bool, action: Action, action_source: ActionSource) -> Result<Object> {
        let select = finder.get("select");
        let include = finder.get("include");

        let aggregate_input = Aggregation::build(model, graph, finder)?;
        let col = &self.collections[model.name()];
        let cur = col.aggregate(aggregate_input, None).await;
        if cur.is_err() {
            return Err(Error::unknown_database_find_unique_error());
        }
        let cur = cur.unwrap();
        let results: Vec<std::result::Result<Document, MongoDBError>> = cur.collect().await;
        if results.is_empty() {
            return Err(Error::object_not_found());
        }
        for doc in results {
            let obj = graph.new_object(model.name(), action, action_source.clone())?;
            self.document_to_object(&doc.unwrap(), &obj, select, include)?;
            return Ok(obj);
        }
        Err(Error::object_not_found())
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Value, _mutation_mode: bool, action: Action, action_source: ActionSource) -> Result<Vec<Object>> {
        let select = finder.get("select");
        let include = finder.get("include");
        let aggregate_input = Aggregation::build(model, graph, finder)?;
        let reverse = Input::has_negative_take(finder);
        let col = &self.collections[model.name()];
        // println!("see aggregate input: {:?}", aggregate_input);
        let cur = col.aggregate(aggregate_input, None).await;
        if cur.is_err() {
            println!("{:?}", cur);
            return Err(Error::unknown_database_find_error());
        }
        let cur = cur.unwrap();
        let mut result: Vec<Object> = vec![];
        let results: Vec<std::result::Result<Document, MongoDBError>> = cur.collect().await;
        for doc in results {
            let obj = graph.new_object(model.name(), action, action_source.clone())?;
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

    async fn count(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<usize> {
        let input = Aggregation::build_for_count(model, graph, finder)?;
        let col = &self.collections[model.name()];
        let cur = col.aggregate(input, None).await;
        if cur.is_err() {
            println!("{:?}", cur);
            return Err(Error::unknown_database_find_error());
        }
        let cur = cur.unwrap();
        let results: Vec<std::result::Result<Document, MongoDBError>> = cur.collect().await;
        if results.is_empty() {
            Ok(0)
        } else {
            let v = results.get(0).unwrap().as_ref().unwrap();
            let bson_count = v.get("count").unwrap();
            match bson_count {
                Bson::Int32(i) => Ok(*i as usize),
                Bson::Int64(i) => Ok(*i as usize),
                _ => panic!("Unhandled count number type.")
            }
        }
    }

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value> {
        let results = self.aggregate_or_group_by(graph, model, finder).await?;
        if results.is_empty() {
            // there is no record
            let mut retval = teon!({});
            for (g, o) in finder.as_hashmap().unwrap() {
                retval.as_hashmap_mut().unwrap().insert(g.clone(), teon!({}));
                for (k, _v) in o.as_hashmap().unwrap() {
                    let value = if g == "_count" { teon!(0) } else { teon!(null) };
                    retval.as_hashmap_mut().unwrap().get_mut(g.as_str()).unwrap().as_hashmap_mut().unwrap().insert(k.to_string(), value);
                }
            }
            Ok(retval)
        } else {
            Ok(results.get(0).unwrap().clone())
        }
    }

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value> {
        Ok(Value::Vec(self.aggregate_or_group_by(graph, model, finder).await?))
    }

    fn new_save_session(&self) -> Arc<dyn SaveSession> {
        Arc::new(MongoDBSaveSession {})
    }
}

unsafe impl Sync for MongoDBConnector {}
unsafe impl Send for MongoDBConnector {}
