use std::borrow::Cow;
use sqlx::{AnyPool, Column, Executor, Row};
use sqlx::any::AnyRow;
use crate::connectors::sql::query::Query;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::schema::value::decode::RowDecoder;
use crate::core::error::ActionError;
use crate::core::input::Input;
use crate::core::model::Model;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Object, Value};

pub(crate) struct Execution { }

impl Execution {

    fn row_to_value(model: &Model, graph: &Graph, row: &AnyRow) -> Value {
        Value::HashMap(row.columns().iter().map(|column| {
            let column_name = column.name();
            if let Some(field) = model.field_with_column_name(column_name) {
                (field.name().to_owned(), RowDecoder::decode(field.r#type(), field.is_optional(), row, column_name))
            } else if let Some(property) = model.property(column_name) {
                (property.name().to_owned(), RowDecoder::decode(property.r#type(), property.is_optional(), row, column_name))
            }
        }).collect())
    }

    pub(crate) async fn query(pool: &AnyPool, model: &Model, graph: &Graph, value: &Value, dialect: SQLDialect) -> ActionResult<Vec<Value>> {
        let select = value.get("select");
        let include = value.get("include");
        let stmt = Query::build(model, graph, value, dialect, None, None);
        let reverse = Input::has_negative_take(finder);
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
        let results = rows.iter().map(|row| Self::row_to_value(model, graph, row)).collect::<Vec<Value>>();
        if let Some(include) = include.map(|i| i.as_hashmap().unwrap()) {
            for (key, value) in include {
                let relation = model.relation(key).unwrap();
                let (opposite_model, _) = graph.opposite_relation(relation);
                if !relation.has_join_table() {
                    let fields = relation.fields();
                    let opposite_fields = relation.references();
                    let names = if opposite_fields.len() == 1 {
                        Cow::Borrowed(opposite_model.field(opposite_fields.get(0).unwrap()).unwrap().column_name())
                    } else {
                        Cow::Owned(opposite_fields.iter().map(|f| opposite_model.field(f).unwrap().column_name()).collect::<Vec<&String>>().join(",").to_wrapped())
                    };
                    let values = if opposite_fields.len() == 1 {
                        // (?,?,?,?,?)
                        let field_name = left_fields.get(0).unwrap();
                        // let column_name = relation_model.field(field_name).unwrap().column_name();
                        let values: Vec<String> = results.iter().map(|o| {
                            o.get_value(field_name).unwrap().to_string(self.dialect)
                        }).collect::<Vec<String>>();
                        values.join(",").to_wrapped()
                    } else {
                        // (VALUES (?,?),(?,?))
                        let pairs = retval.iter().map(|o| {
                            left_fields.iter().map(|f| o.get_value(f).unwrap().to_string(self.dialect)).collect::<Vec<String>>().join(",").to_wrapped()
                        }).collect::<Vec<String>>().join(",");
                        format!("(VALUES {})", pairs)

                    };
                    let where_addition = Query::where_item(names.as_ref(), "IN", &values);

                }
            }
            for (relation_name, include_value) in include_map {
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
                        // todo: transform to column name


                        let relation_where = format!("{} IN {}", before_in, after_in);
                        let included = self.perform_query(graph, relation_model, nested_include, mutation_mode, &path, Some(relation_where), None, env.alter_intent(Intent::NestedIncluded)).await?;
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
                        let included = self.perform_query(graph, relation_model, nested_include, mutation_mode, &path, Some(relation_where), Some(left_join), env.alter_intent(Intent::NestedIncluded)).await?;
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
        }
        Ok(vec![])
    }

    pub(crate) fn query_count(model: &Model, graph: &Graph, finder: &Value) -> ActionResult<u64> {

    }
}
