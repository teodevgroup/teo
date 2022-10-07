use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_recursion::async_recursion;
use async_trait::async_trait;
use chrono::{Date, DateTime, NaiveDate, Utc};
use key_path::{KeyPath, path};
use sqlx::{AnyPool, Column, Database, Error, Executor, Row, ValueRef};
use sqlx::pool::Pool;
use sqlx::any::{AnyRow, AnyValueRef};
use crate::core::model::Model;
use url::Url;
use crate::connectors::shared::has_negative_take::has_negative_take;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::connectors::sql::migration::migrate::migrate;
use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::integration::select::{build_sql_query_from_json, build_where_from_identifier};
use crate::connectors::sql::query_builder::integration::value_encoder::ToWrapped;
use crate::connectors::sql::query_builder::stmt::SQL;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;
use crate::connectors::sql::save_session::SQLSaveSession;
use crate::core::connector::Connector;
use crate::core::env::Env;
use crate::core::env::intent::Intent;
use crate::core::error::ActionError;
use crate::core::input::AtomicUpdateType;
use crate::core::input_decoder::str_to_target_type;
use crate::core::result::ActionResult;
use crate::core::save_session::SaveSession;
use crate::prelude::{Graph, Object, Value};
use crate::tson;

#[derive(Debug)]
pub(crate) struct SQLConnector {
    dialect: SQLDialect,
    pool: AnyPool,
}

impl SQLConnector {
    pub(crate) async fn new(dialect: SQLDialect, url: String, models: &Vec<Model>, reset_database: bool) -> Self {
        let url_result = Url::parse(&url);
        if url_result.is_err() {
            panic!("Data source URL is invalid.");
        }
        let mut url_without_db = url_result.unwrap();
        let database_name = url_without_db.path()[1..].to_string();
        url_without_db.set_path("/");
        let mut pool: AnyPool = AnyPool::connect(url_without_db.as_str()).await.unwrap();
        Self::create_database_if_needed(dialect, &mut pool, &database_name, reset_database).await;
        let mut pool: AnyPool = AnyPool::connect(url.as_str()).await.unwrap();
        Self::setup_database(dialect, &mut pool, models).await;
        Self {
            dialect,
            pool,
        }
    }

    async fn create_database_if_needed(dialect: SQLDialect, pool: &mut AnyPool, db_name: &str, reset_database: bool) {
        // drop database if needed
        if reset_database {
            let stmt = SQL::drop().database(db_name).
                if_exists().to_string(dialect);
            pool.execute(&*stmt).await.unwrap();
        }
        // create and use database
        let stmt = SQL::create().database(db_name).if_not_exists().to_string(dialect);
        pool.execute(&*stmt).await.unwrap();
        let stmt = SQL::r#use().database(db_name).to_string(dialect);
        pool.execute(&*stmt).await.unwrap();
    }

    async fn setup_database(dialect: SQLDialect, pool: &mut AnyPool, models: &Vec<Model>) {
        migrate(dialect, pool, models).await
    }

    fn row_to_object(&self, row: &AnyRow, object: &Object, select: Option<&Value>, include: Option<&Value>, left_join: bool) -> ActionResult<()> {
        for column in row.columns() {
            let column_name = column.name();
            let result_name = if left_join {
                // "t.".to_owned() + column_name
                column_name.to_owned()
            } else {
                column_name.to_owned()
            };
            if let Some(field) = object.model().field_with_column_name(column_name) {
                if field.r#type().is_bool() {
                    let any_value: AnyValueRef = row.try_get_raw(result_name.as_str()).unwrap();
                    if !any_value.is_null() {
                        let bool_value: bool = row.get(result_name.as_str());
                        object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::Bool(bool_value));
                    }
                } else if field.r#type().is_int() {
                    let any_value: AnyValueRef = row.try_get_raw(result_name.as_str()).unwrap();
                    if !any_value.is_null() {
                        let i64_value: i64 = row.get(result_name.as_str());
                        object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::number_from_i64(i64_value, field.r#type()));
                    }
                } else if field.r#type().is_float() {
                    let any_value: AnyValueRef = row.try_get_raw(result_name.as_str()).unwrap();
                    if !any_value.is_null() {
                        let f64_value: f64 = row.get(result_name.as_str());
                        object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::number_from_f64(f64_value, field.r#type()));
                    }
                } else if field.r#type().is_string() {
                    let any_value: AnyValueRef = row.try_get_raw(result_name.as_str()).unwrap();
                    if !any_value.is_null() {
                        let string_value: String = row.get(result_name.as_str());
                        object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::String(string_value));
                    }
                } else if field.r#type().is_date() {
                    let any_value: AnyValueRef = row.try_get_raw(result_name.as_str()).unwrap();
                    #[cfg(not(feature = "data-source-mssql"))]
                    if !any_value.is_null() {
                        let naive_date: NaiveDate = row.get(result_name.as_str());
                        let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                        object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::Date(date));
                    }
                } else if field.r#type().is_datetime() {
                    let any_value: AnyValueRef = row.try_get_raw(result_name.as_str()).unwrap();
                    #[cfg(not(feature = "data-source-mssql"))]
                    if !any_value.is_null() {
                        let datetime_value: DateTime<Utc> = row.get(result_name.as_str());
                        object.inner.value_map.lock().unwrap().insert(field.name().to_owned(), Value::DateTime(datetime_value));
                    }
                }
            } else if let Some(relation) = object.model().relation(column_name) {}
        }
        object.inner.is_initialized.store(true, Ordering::SeqCst);
        object.inner.is_new.store(false, Ordering::SeqCst);
        if let Some(select) = select {
            object.set_select(Some(select)).unwrap();
        }
        Ok(())

        // // relation
                // let relation = object.model().relation(key);
                // if relation.is_none() {
                //     continue;
                // }
                // let inner_finder = if let Some(include) = include {
                //     include.get(key)
                // } else {
                //     None
                // };
                // let inner_select = if let Some(inner_finder) = inner_finder {
                //     inner_finder.get("select")
                // } else {
                //     None
                // };
                // let inner_include = if let Some(inner_finder) = inner_finder {
                //     inner_finder.get("include")
                // } else {
                //     None
                // };
                // let relation = relation.unwrap();
                // let model_name = relation.model();
                // let object_bsons = document.get(key).unwrap().as_vec().unwrap();
                // let mut related: Vec<Object> = vec![];
                // for related_object_bson in object_bsons {
                //     let related_object = object.graph().new_object(model_name)?;
                //     self.document_to_object(related_object_bson.as_document().unwrap(), &related_object, inner_select, inner_include)?;
                //     related.push(related_object);
                // }
                // object.inner.relation_query_map.lock().unwrap().insert(key.to_string(), related);
    }

    async fn create_object(&self, object: &Object) -> ActionResult<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let auto_keys = model.auto_keys();
        let mut values: Vec<(&str, String)> = vec![];
        for key in keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                let val = object.get_value(key).unwrap();
                values.push((column_name, val.to_string(self.dialect)));
            } else if let Some(property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, val.to_string(self.dialect)));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let stmt = SQL::insert_into(model.table_name()).values(value_refs).to_string(self.dialect);
        let result = self.pool.execute(&*stmt).await.unwrap();
        for key in auto_keys {
            object.set_value(key, Value::I64(result.last_insert_id().unwrap()))?;
        }
        Ok(())
    }

    async fn update_object(&self, object: &Object) -> ActionResult<()> {
        let model = object.model();
        let keys = object.keys_for_save();
        let mut values: Vec<(&str, String)> = vec![];
        for key in &keys {
            if let Some(field) = model.field(key) {
                let column_name = field.column_name();
                let updator_map = object.inner.atomic_updator_map.lock().unwrap();
                if updator_map.contains_key(*key) {
                    let updator = updator_map.get(*key).unwrap();
                    match updator {
                        AtomicUpdateType::Increment(val) => {
                            values.push((column_name, format!("{} + {}", column_name, val.to_string(self.dialect))));
                        }
                        AtomicUpdateType::Decrement(val) => {
                            values.push((column_name, format!("{} - {}", column_name, val.to_string(self.dialect))));
                        }
                        AtomicUpdateType::Multiply(val) => {
                            values.push((column_name, format!("{} * {}", column_name, val.to_string(self.dialect))));
                        }
                        AtomicUpdateType::Divide(val) => {
                            values.push((column_name, format!("{} / {}", column_name, val.to_string(self.dialect))));
                        }
                        AtomicUpdateType::Push(val) => {
                            values.push((column_name, format!("ARRAY_APPEND({}, {})", column_name, val.to_string(self.dialect))));
                        }
                    }
                } else {
                    let val = object.get_value(key).unwrap();
                    values.push((column_name, val.to_string(self.dialect)));
                }
            } else if let Some(property) = model.property(key) {
                let val: Value = object.get_property(key).await.unwrap();
                values.push((key, val.to_string(self.dialect)));
            }
        }
        let value_refs: Vec<(&str, &str)> = values.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let r#where = build_where_from_identifier(model, object.graph(), &object.identifier(), self.dialect);
        let stmt = SQL::update(model.table_name()).values(value_refs).r#where(&r#where).to_string(self.dialect);
        let result = self.pool.execute(stmt.as_str()).await;
        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err(ActionError::unknown_database_write_error());
        }
        let select_stmt = SQL::select(Some(&keys), model.table_name()).r#where(r#where).to_string(self.dialect);
        let results = self.pool.fetch_optional(&*select_stmt).await;
        match results {
            Ok(row) => {
                match row {
                    Some(row) => {
                        self.row_to_object(&row, &object, None, None, false)?;
                        Ok(())
                    }
                    None => {
                        Err(ActionError::object_not_found())
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
                Err(ActionError::unknown_database_find_unique_error())
            }
        }
    }

    #[async_recursion]
    async fn perform_query<'a>(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, key_path: &KeyPath, additional_where: Option<String>, additional_left_join: Option<String>, env: Env) -> Result<Vec<Object>, ActionError> {
        let select = finder.get("select");
        let include = finder.get("include");
        let sql_query = build_sql_query_from_json(model, graph, QueryPipelineType::Many, mutation_mode, finder, self.dialect, additional_where, additional_left_join.clone(), key_path)?;
        println!("see sql query: {}", sql_query);
        let reverse = has_negative_take(finder);
        let results = self.pool.fetch_all(&*sql_query).await;
        let mut retval: Vec<Object> = vec![];
        match results {
            Ok(rows) => {
                for row in &rows {
                    let obj = graph.new_object(model.name(), env.clone())?;
                    self.row_to_object(&row, &obj, select, include, additional_left_join.is_some())?;
                    if reverse {
                        retval.insert(0, obj);
                    } else {
                        retval.push(obj);
                    }
                }
                if let Some(include) = include {
                    if let Some(include_map) = include.as_hashmap() {
                        for (relation_name, include_value) in include_map {
                            let relation = model.relation(relation_name);
                            if relation.is_none() {
                                let path = key_path.as_ref() + relation_name;
                                return Err(ActionError::unexpected_input_key(relation_name, path));
                            }
                            let relation = relation.unwrap();
                            let empty = tson!({});
                            let mut nested_include = if include_value.is_bool() {
                                if include_value.as_bool().unwrap() == true {
                                    Some(&empty)
                                } else {
                                    None
                                }
                            } else if include_value.is_object() {
                                Some(include_value)
                            } else {
                                let path = key_path.as_ref() + relation_name;
                                return Err(ActionError::unexpected_input_value("bool or object", &path));
                            };
                            if let Some(nested_include) = nested_include {
                                if relation.through().is_none() { // no join tables
                                    let relation_model = graph.model(relation.model()).unwrap();
                                    let left_fields = relation.fields();
                                    let right_fields = relation.references();
                                    let path = key_path.as_ref() + relation_name;
                                    // todo: transform to column name
                                    let before_in: String = if right_fields.len() == 1 {
                                        right_fields.get(0).unwrap().to_string()
                                    } else {
                                        right_fields.join(",").to_wrapped()
                                    };
                                    let after_in: String = if right_fields.len() == 1 {
                                        // (?,?,?,?,?)
                                        let field_name = left_fields.get(0).unwrap();
                                        // let column_name = relation_model.field(field_name).unwrap().column_name();
                                        let values: Vec<String> = retval.iter().map(|o| {
                                            o.get_value(field_name).unwrap().to_string(self.dialect)
                                        }).collect();
                                        values.join(",").to_wrapped()
                                    } else {
                                        // (VALUES (?,?),(?,?))
                                        let pairs = retval.iter().map(|o| {
                                            left_fields.iter().map(|f| o.get_value(f).unwrap().to_string(self.dialect)).collect::<Vec<String>>().join(",").to_wrapped()
                                        }).collect::<Vec<String>>().join(",");
                                        format!("(VALUES {})", pairs)
                                    };
                                    let relation_where = format!("{} IN {}", before_in, after_in);
                                    let included = self.perform_query(graph, relation_model, nested_include, mutation_mode, &path, Some(relation_where), None, env.nested(Intent::NestedIncluded)).await?;
                                    println!("see included: {:?}", included);
                                    for o in included {
                                        let owners = retval.iter().filter(|r| {
                                            for (index, left_field) in left_fields.iter().enumerate() {
                                                let right_field = &right_fields[index];
                                                if o.get_value(right_field) != r.get_value(left_field) {
                                                    return false;
                                                }
                                            }
                                            true
                                        });
                                        for owner in owners {
                                            if owner.inner.relation_query_map.lock().unwrap().get_mut(relation_name).is_none() {
                                                owner.inner.relation_query_map.lock().unwrap().insert(relation_name.to_string(), vec![]);
                                            }
                                            owner.inner.relation_query_map.lock().unwrap().get_mut(relation_name).unwrap().push(o.clone());
                                        }
                                    }
                                } else { // with join tables
                                    let relation_model = graph.model(relation.model()).unwrap();
                                    let relation_model_table_name = relation_model.table_name();
                                    let through_model = graph.model(&relation.through().as_ref().unwrap()).unwrap();
                                    let through_table_name = through_model.table_name();
                                    let through_relation = through_model.relation(relation.references().get(0).unwrap()).unwrap();
                                    let mut join_parts: Vec<String> = vec![];
                                    for (index, field_name) in through_relation.fields().iter().enumerate() {
                                        let reference_name = through_relation.references().get(index).unwrap();
                                        join_parts.push(format!("t.{} = j.{}", reference_name, field_name));
                                    }
                                    let joins = join_parts.join(" AND ");
                                    let left_join = format!("{} AS j ON {}", through_table_name, joins);
                                    let this_relation_on_join_table = through_model.relation(relation.fields().get(0).unwrap()).unwrap();
                                    let left_fields = this_relation_on_join_table.references();
                                    let right_fields = this_relation_on_join_table.fields();
                                    let before_in: String = if right_fields.len() == 1 {
                                        "j.".to_owned() + right_fields.get(0).unwrap()
                                    } else {
                                        right_fields.iter().map(|f| format!("j.{}", f)).collect::<Vec<String>>().join(",").to_wrapped()
                                    };
                                    let after_in: String = if right_fields.len() == 1 {
                                        // (?,?,?,?,?)
                                        let field_name = left_fields.get(0).unwrap();
                                        // let column_name = relation_model.field(field_name).unwrap().column_name();
                                        let values: Vec<String> = retval.iter().map(|o| {
                                            let result = o.get_value(field_name).unwrap().to_string(self.dialect);
                                            println!("see retval: {:?}", &retval);
                                            result
                                        }).collect();
                                        values.join(",").to_wrapped()
                                    } else {
                                        // (VALUES (?,?),(?,?))
                                        let pairs = retval.iter().map(|o| {
                                            left_fields.iter().map(|f| o.get_value(f).unwrap().to_string(self.dialect)).collect::<Vec<String>>().join(",").to_wrapped()
                                        }).collect::<Vec<String>>().join(",");
                                        format!("(VALUES {})", pairs)
                                    };
                                    let relation_where = format!("{} IN {}", before_in, after_in);
                                    let path = key_path.as_ref() + relation_name;
                                    let included = self.perform_query(graph, relation_model, nested_include, mutation_mode, &path, Some(relation_where), Some(left_join), env.nested(Intent::NestedIncluded)).await?;
                                    println!("see included {:?}", included);
                                    for o in included {
                                        let owners = retval.iter().filter(|r| {
                                            for (index, left_field) in left_fields.iter().enumerate() {
                                                let right_field = &right_fields[index];
                                                if o.get_value(right_field) != r.get_value(left_field) {
                                                    return false;
                                                }
                                            }
                                            true
                                        });
                                        for owner in owners {
                                            if owner.inner.relation_query_map.lock().unwrap().get_mut(relation_name).is_none() {
                                                owner.inner.relation_query_map.lock().unwrap().insert(relation_name.to_string(), vec![]);
                                            }
                                            owner.inner.relation_query_map.lock().unwrap().get_mut(relation_name).unwrap().push(o.clone());
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        let path = key_path.as_ref() + "include";
                        return Err(ActionError::unexpected_input_type("object", &path));
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
                return Err(ActionError::unknown_database_find_error());
            }
        }
        Ok(retval)
    }
}

#[async_trait]
impl Connector for SQLConnector {
    async fn save_object(&self, object: &Object) -> ActionResult<()> {
        let is_new = object.inner.is_new.load(Ordering::SeqCst);
        if is_new {
            self.create_object(object).await
        } else {
            self.update_object(object).await
        }
    }

    async fn delete_object(&self, object: &Object) -> ActionResult<()> {
        if object.inner.is_new.load(Ordering::SeqCst) {
            return Err(ActionError::object_is_not_saved());
        }
        let model = object.model();
        let r#where = build_where_from_identifier(model, object.graph(), &object.identifier(), self.dialect);
        let stmt = SQL::delete_from(model.table_name()).r#where(r#where).to_string(self.dialect);
        let result = self.pool.execute(stmt.as_str()).await;
        if result.is_err() {
            println!("{:?}", result.err().unwrap());
            return Err(ActionError::unknown_database_write_error());
        } else {
            Ok(())
        }
    }

    async fn find_unique(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, env: Env) -> Result<Object, ActionError> {
        let objects = self.perform_query(graph, model, finder, mutation_mode, &path![], None, None, env).await?;
        if objects.is_empty() {
            Err(ActionError::object_not_found())
        } else {
            Ok(objects.get(0).unwrap().clone())
        }
    }

    async fn find_many(&self, graph: &Graph, model: &Model, finder: &Value, mutation_mode: bool, env: Env) -> Result<Vec<Object>, ActionError> {
        self.perform_query(graph, model, finder, mutation_mode, &path![], None, None, env).await
    }

    async fn count(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<usize, ActionError> {
        let sql_query = build_sql_query_from_json(model, graph, QueryPipelineType::Count, false, finder, self.dialect, None, None, &path![])?;
        let result = self.pool.fetch_one(&*sql_query).await;
        match result {
            Ok(row) => {
                let result: i64 = row.get(0);
                Ok(result as usize)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(ActionError::unknown_database_find_error())
            }
        }
    }

    async fn aggregate(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value, ActionError> {
        todo!()
    }

    async fn group_by(&self, graph: &Graph, model: &Model, finder: &Value) -> Result<Value, ActionError> {
        todo!()
    }

    fn new_save_session(&self) -> Arc<dyn SaveSession> {
        Arc::new(SQLSaveSession { })
    }
}
