use std::borrow::Cow;
use async_recursion::async_recursion;
use sqlx::{AnyPool, Column, Executor, Row};
use sqlx::any::AnyRow;
use crate::connectors::sql::query::Query;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::decode::RowDecoder;
use crate::connectors::sql::schema::value::encode::{ToSQLString, ToWrapped};
use crate::core::env::Env;
use crate::core::error::ActionError;
use crate::core::input::Input;
use crate::core::model::Model;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Object, Value};
use crate::tson;

pub(crate) struct Execution { }

impl Execution {

    fn row_to_value(model: &Model, graph: &Graph, row: &AnyRow) -> Value {
        Value::HashMap(row.columns().iter().filter_map(|column| {
            let column_name = column.name();
            if let Some(field) = model.field_with_column_name(column_name) {
                Some((field.name().to_owned(), RowDecoder::decode(field.r#type(), field.is_optional(), row, column_name)))
            } else if let Some(property) = model.property(column_name) {
                Some((property.name().to_owned(), RowDecoder::decode(property.r#type(), property.is_optional(), row, column_name)))
            } else if column_name.contains(".") {
                let names: Vec<&str> = column_name.split(".").collect();
                let relation_name = names[0];
                let field_name = names[1];
                if relation_name == "c" { // cursor fetch, should remove
                    None
                } else {
                    let opposite_model = graph.model(model.relation(relation_name).unwrap().model()).unwrap();
                    let field = opposite_model.field(field_name).unwrap();
                    Some((column_name.to_owned(), RowDecoder::decode(field.r#type(), field.is_optional(), row, column_name)))
                }
            } else {
                panic!("Unhandled key {}.", column_name);
            }
        }).collect())
    }

    pub(crate) async fn query_objects(pool: &AnyPool, model: &Model, graph: &Graph, finder: &Value, dialect: SQLDialect, env: Env) -> ActionResult<Vec<Object>> {
        let values = Self::query(pool, model, graph, finder, dialect).await?;
        let select = finder.as_hashmap().unwrap().get("select");
        let include = finder.as_hashmap().unwrap().get("include");
        let mut results = vec![];
        for value in values {
            let object = graph.new_object(model.name(), env.clone())?;
            object.set_from_database_result_value(&value, select, include);
            results.push(object);
        }
        Ok(results)
    }

    #[async_recursion]
    async fn query_internal(pool: &AnyPool, model: &Model, graph: &Graph, value: &Value, dialect: SQLDialect, additional_where: Option<String>, additional_left_join: Option<String>, join_table_results: Option<Vec<String>>, force_negative_take: bool) -> ActionResult<Vec<Value>> {
        let select = value.get("select");
        let include = value.get("include");
        let stmt = Query::build(model, graph, value, dialect, additional_where, additional_left_join, join_table_results, force_negative_take);
        println!("sql stmt: {}", &stmt);
        let reverse = Input::has_negative_take(value);
        let rows = match pool.fetch_all(&*stmt).await {
            Ok(rows) => rows,
            Err(err) => {
                println!("{:?}", err);
                return Err(ActionError::unknown_database_find_error());
            }
        };
        if rows.is_empty() {
            return Ok(vec![])
        }
        let mut results = rows.iter().map(|row| Self::row_to_value(model, graph, row)).collect::<Vec<Value>>();
        if reverse {
            results.reverse();
        }
        if let Some(include) = include.map(|i| i.as_hashmap().unwrap()) {
            for (key, value) in include {
                let skip = value.as_hashmap().map(|m| m.get("skip")).flatten().map(|v| v.as_u64().unwrap());
                let take = value.as_hashmap().map(|m| m.get("take")).flatten().map(|v| v.as_i64().unwrap());
                let take_abs = take.map(|t| t.abs() as u64);
                let negative_take = take.map(|v| v.is_negative()).unwrap_or(false);
                let relation = model.relation(key).unwrap();
                let (opposite_model, _) = graph.opposite_relation(relation);
                if !relation.has_join_table() {
                    let fields = relation.fields();
                    let opposite_fields = relation.references();
                    let names = if opposite_fields.len() == 1 { // todo: column name
                        Cow::Borrowed(opposite_model.field(opposite_fields.get(0).unwrap()).unwrap().column_name())
                    } else {
                        Cow::Owned(opposite_fields.iter().map(|f| opposite_model.field(f).unwrap().column_name()).collect::<Vec<&str>>().join(",").to_wrapped())
                    };
                    let values = if opposite_fields.len() == 1 {
                        // in a (?,?,?,?,?) format
                        let field_name = fields.get(0).unwrap();
                        results.iter().map(|v| {
                            v.as_hashmap().unwrap().get(field_name).unwrap().to_string(dialect)
                        }).collect::<Vec<String>>().join(",").to_wrapped()
                    } else {
                        // in a (VALUES (?,?),(?,?)) format
                        format!("(VALUES {})", results.iter().map(|o| {
                            fields.iter().map(|f| o.as_hashmap().unwrap().get(f).unwrap().to_string(dialect)).collect::<Vec<String>>().join(",").to_wrapped()
                        }).collect::<Vec<String>>().join(","))
                    };
                    let where_addition = Query::where_item(names.as_ref(), "IN", &values);
                    let nested_query = if value.is_hashmap() {
                        Self::without_paging_and_skip_take(value)
                    } else {
                        Cow::Owned(tson!({}))
                    };
                    let mut included_values = Self::query_internal(pool, opposite_model, graph, &nested_query, dialect, Some(where_addition), None, None, negative_take).await?;
                    if negative_take {
                        included_values.reverse()
                    }
                    // println!("see included: {:?}", included_values);
                    for result in results.iter_mut() {
                        let mut skipped = 0;
                        let mut taken = 0;
                        for included_value in included_values.iter() {
                            let mut matched = true;
                            for (field, reference) in relation.iter() {
                                if included_value.get(reference).is_none() && result.get(field).is_none() {
                                    matched = false;
                                    break;
                                }
                                if included_value.get(reference) != result.get(field) {
                                    matched = false;
                                    break;
                                }
                            }
                            if matched {
                                if (skip.is_none() || skip.unwrap() <= skipped) && (take.is_none() || taken < take_abs.unwrap()) {
                                    if result.get(relation.name()).is_none() {
                                        result.as_hashmap_mut().unwrap().insert(relation.name().to_owned(), Value::Vec(vec![]));
                                    }
                                    if negative_take {
                                        result.as_hashmap_mut().unwrap().get_mut(relation.name()).unwrap().as_vec_mut().unwrap().insert(0, included_value.clone());
                                    } else {
                                        result.as_hashmap_mut().unwrap().get_mut(relation.name()).unwrap().as_vec_mut().unwrap().push(included_value.clone());
                                    }
                                    taken += 1;
                                    if take.is_some() && (taken >= take_abs.unwrap()) {
                                        break;
                                    }
                                } else {
                                    skipped += 1;
                                }
                            }
                        }
                    }
                } else {
                    let (opposite_model, opposite_relation) = graph.opposite_relation(relation);
                    let (through_model, through_opposite_relation) = graph.through_opposite_relation(relation);
                    let mut join_parts: Vec<String> = vec![];
                    for (field, reference) in through_opposite_relation.iter() {
                        let field_column_name = through_model.field(field).unwrap().column_name();
                        let reference_column_name = opposite_model.field(reference).unwrap().column_name();
                        join_parts.push(format!("t.{} = j.{}", reference_column_name, field_column_name));
                    }
                    let joins = join_parts.join(" AND ");
                    let left_join = format!("{} AS j ON {}", through_model.table_name(), joins);
                    let (_, through_relation) = graph.through_relation(relation);
                    let names = if through_relation.len() == 1 { // todo: column name
                        format!("j.{}", through_relation.fields().get(0).unwrap())
                    } else {
                        through_relation.fields().iter().map(|f| format!("j.{}", f)).collect::<Vec<String>>().join(",").to_wrapped()
                    };
                    let values = if through_relation.len() == 1 { // (?,?,?,?,?) format
                        let field_name = through_relation.references().get(0).unwrap();
                        results.iter().map(|v| {
                            v.as_hashmap().unwrap().get(field_name).unwrap().to_string(dialect)
                        }).collect::<Vec<String>>().join(",").to_wrapped()
                    } else { // (VALUES (?,?),(?,?)) format
                        let pairs = results.iter().map(|o| {
                            through_relation.references().iter().map(|f| o.as_hashmap().unwrap().get(f).unwrap().to_string(dialect)).collect::<Vec<String>>().join(",").to_wrapped()
                        }).collect::<Vec<String>>().join(",");
                        format!("(VALUES {})", pairs)
                    };
                    let where_addition = Query::where_item(names.as_ref(), "IN", &values);
                    let nested_query = if value.is_hashmap() {
                        Self::without_paging_and_skip_take(value)
                    } else {
                        Cow::Owned(tson!({}))
                    };
                    let join_table_results = through_relation.iter().map(|(f, r)| {
                        let through_column_name = through_model.field(f).unwrap().column_name().to_string();
                        format!("j.{} AS `{}.{}`", through_column_name, opposite_relation.unwrap().name(), r)
                    }).collect();
                    let mut included_values = Self::query_internal(pool, opposite_model, graph, &nested_query, dialect, Some(where_addition), Some(left_join), Some(join_table_results), negative_take).await?;
                    if negative_take {
                        included_values.reverse()
                    }
                    // println!("see included {:?}", included_values);
                    for result in results.iter_mut() {
                        let mut skipped = 0;
                        let mut taken = 0;
                        for included_value in included_values.iter() {
                            let mut matched = true;
                            for (_field, reference) in through_relation.iter() {
                                let key = format!("{}.{}", opposite_relation.unwrap().name(), reference);
                                if result.get(reference).is_none() && included_value.get(&key).is_none() {
                                    matched = false;
                                    break;
                                }
                                if result.get(reference) != included_value.get(&key) {
                                    matched = false;
                                    break;
                                }
                            }
                            if matched {
                                if (skip.is_none() || skip.unwrap() <= skipped) && (take.is_none() || taken < take_abs.unwrap()) {
                                    if result.get(relation.name()).is_none() {
                                        result.as_hashmap_mut().unwrap().insert(relation.name().to_owned(), Value::Vec(vec![]));
                                    }
                                    if negative_take {
                                        result.as_hashmap_mut().unwrap().get_mut(relation.name()).unwrap().as_vec_mut().unwrap().insert(0, included_value.clone());
                                    } else {
                                        result.as_hashmap_mut().unwrap().get_mut(relation.name()).unwrap().as_vec_mut().unwrap().push(included_value.clone());
                                    }
                                    taken += 1;
                                    if take.is_some() && (taken >= take_abs.unwrap()) {
                                        break;
                                    }
                                } else {
                                    skipped += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(results)
    }

    pub(crate) async fn query(pool: &AnyPool, model: &Model, graph: &Graph, value: &Value, dialect: SQLDialect) -> ActionResult<Vec<Value>> {
       Self::query_internal(pool, model, graph, value, dialect, None, None, None, false).await
    }

    pub(crate) async fn query_count(pool: &AnyPool, model: &Model, graph: &Graph, finder: &Value, dialect: SQLDialect) -> ActionResult<u64> {
        let stmt = Query::build_for_count(model, graph, finder, dialect, None, None, None, false);
        match pool.fetch_one(&*stmt).await {
            Ok(result) => {
                let count: i64 = result.get(0);
                Ok(count as u64)
            },
            Err(err) => {
                println!("{:?}", err);
                return Err(ActionError::unknown_database_find_error());
            }
        }
    }

    fn without_paging_and_skip_take(value: &Value) -> Cow<Value> {
        let map = value.as_hashmap().unwrap();
        if map.contains_key("take") || map.contains_key("skip") || map.contains_key("pageSize") || map.contains_key("pageNumber") {
            let mut map = map.clone();
            map.remove("take");
            map.remove("skip");
            map.remove("pageSize");
            map.remove("pageNumber");
            Cow::Owned(Value::HashMap(map))
        } else {
            Cow::Borrowed(value)
        }
    }
}
