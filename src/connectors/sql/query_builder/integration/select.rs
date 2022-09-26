use std::fmt::format;
use serde_json::{Value as JsonValue};
use crate::connectors::shared::map_has_i_mode::map_has_i_mode;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::connectors::shared::user_json_args::{user_json_args, UserJsonArgs};
use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::integration::value_encoder::{IfIMode, ToLike, ToSQLInput, ToWrapped, WrapInArray};
use crate::connectors::sql::query_builder::stmt::select::r#where::{ToWrappedSQLString, WhereClause};
use crate::connectors::sql::query_builder::stmt::select::r#where::WhereClause::And;
use crate::connectors::sql::query_builder::stmt::SQL;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;
use crate::connectors::sql::query_builder::integration::value_encoder::JsonValueToSQLString;
use crate::core::error::ActionError;
use crate::core::field::r#type::FieldType;
use crate::core::key_path::KeyPathItem;
use crate::core::model::Model;
use crate::prelude::Graph;

fn sql_where_item(col_name: &str, op: &str, val: String) -> String {
    format!("{col_name} {op} {val}")
}

fn parse_sql_where_entry_array(
    column_name: &str,
    r#type: &FieldType,
    optional: bool,
    value: &JsonValue,
    key_path: &Vec<KeyPathItem>,
    graph: &Graph,
    op: &str
) -> Result<String, ActionError> {
    match value.as_array() {
        Some(arr_val) => {
            let mut arr: Vec<String> = Vec::new();
            for val in arr_val {
                arr.push(val.to_sql_string(r#type, optional, key_path, graph)?);
            }
            Ok(sql_where_item(column_name, op, arr.join(", ").to_wrapped()))
        }
        None => {
            Err(ActionError::input_type_error("array", key_path))
        }
    }
}

fn parse_sql_where_entry_item(
    column_name: &str,
    r#type: &FieldType,
    optional: bool,
    value: &JsonValue,
    graph: &Graph,
    dialect: SQLDialect,
    key_path: &Vec<KeyPathItem>,
    ops: &Vec<&str>,
) -> Result<String, ActionError> {
    if let Some(map) = value.as_object() {
        let mut result: Vec<String> = vec![];
        for (key, value) in map {
            if !ops.contains(&&**key) {
                return Err(ActionError::unexpected_input_key(key, key_path));
            }
            match key.as_str() {
                "equals" => {
                    result.push(sql_where_item(column_name, "=", value.to_sql_string(r#type, optional, key_path, graph)?));
                }
                "not" => {
                    result.push(sql_where_item(column_name, "<>", value.to_sql_string(r#type, optional, key_path, graph)?));
                }
                "gt" => {
                    result.push(sql_where_item(column_name, ">", value.to_sql_string(r#type, false, key_path, graph)?));
                }
                "gte" => {
                    result.push(sql_where_item(column_name, ">=", value.to_sql_string(r#type, false, key_path, graph)?));
                }
                "lt" => {
                    result.push(sql_where_item(column_name, "<", value.to_sql_string(r#type, false, key_path, graph)?));
                }
                "lte" => {
                    result.push(sql_where_item(column_name, "<=", value.to_sql_string(r#type, false, key_path, graph)?));
                }
                "in" => {
                    result.push(parse_sql_where_entry_array(column_name, r#type, optional, value, key_path, graph, "IN")?);
                }
                "notIn" => {
                    result.push(parse_sql_where_entry_array(column_name, r#type, optional, value, key_path, graph, "NOT IN")?);
                }
                "contains" => {
                    let i_mode = map_has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "LIKE", value.to_sql_string(r#type, false, key_path, graph)?.to_like(true, true).to_i_mode(i_mode)));
                }
                "startsWith" => {
                    let i_mode = map_has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "LIKE", value.to_sql_string(r#type, false, key_path, graph)?.to_like(false, true).to_i_mode(i_mode)));
                }
                "endsWith" => {
                    let i_mode = map_has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "LIKE", value.to_sql_string(r#type, false, key_path, graph)?.to_like(true, false).to_i_mode(i_mode)));
                }
                "matches" => {
                    let i_mode = map_has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "REGEXP", value.to_sql_string(r#type, false, key_path, graph)?.to_i_mode(i_mode)));
                }
                "mode" => { }
                "has" => {
                    let element_type = r#type.element_field().unwrap();
                    result.push(sql_where_item(column_name, "@>", value.to_sql_string(element_type.r#type(), element_type.is_optional(), key_path, graph)?.wrap_in_array()));
                }
                "hasEvery" => {
                    result.push(sql_where_item(column_name, "@>", value.to_sql_string(r#type, false, key_path, graph)?));
                }
                "hasSome" => {
                    result.push(sql_where_item(column_name, "&&", value.to_sql_string(r#type, false, key_path, graph)?));
                }
                "isEmpty" => {
                    result.push(sql_where_item(&format!("ARRAY_LENGTH({})", column_name), "=", "0".to_owned()));
                }
                "length" => {
                    result.push(sql_where_item(&format!("ARRAY_LENGTH({})", column_name), "=", value.to_sql_string(&FieldType::U64, false, key_path, graph)?));
                }
                _ => {
                    return Err(ActionError::unexpected_input_key(key, key_path));
                }
            }
        }
        Ok(And(result).to_wrapped_string(dialect))
    } else {
        Ok(sql_where_item(column_name, "=", value.to_sql_string(r#type, optional, key_path, graph)?))
    }
}

fn parse_sql_where_entry(
    column_name: &str,
    field_type: &FieldType,
    optional: bool,
    value: &JsonValue,
    graph: &Graph,
    dialect: SQLDialect,
    key_path: &Vec<KeyPathItem>
) -> Result<String, ActionError> {
    return match field_type {
        FieldType::Undefined => {
            panic!("Field is undefined.")
        }
        FieldType::ObjectId => {
            panic!("Object id is not supported on SQL databases.")
        }
        FieldType::Bool => {
            parse_sql_where_entry_item(column_name, field_type, optional, value, graph, dialect, key_path, &vec!["equals", "not"])
        }
        FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 | FieldType::Date | FieldType::DateTime => {
            parse_sql_where_entry_item(column_name, field_type, optional, value, graph, dialect, key_path, &vec!["equals", "not", "gt", "gte", "lt", "lte", "in", "notIn"])
        }
        FieldType::Decimal => {
            todo!()
        }
        FieldType::String => {
            parse_sql_where_entry_item(column_name, field_type, optional, value, graph, dialect, key_path, &vec!["equals", "not", "gt", "gte", "lt", "lte", "in", "notIn", "contains", "startsWith", "endsWith", "matches", "mode"])
        }
        FieldType::Enum(_) => {
            parse_sql_where_entry_item(column_name, field_type, optional, value, graph, dialect, key_path, &vec!["equals", "not", "in", "notIn"])
        }
        FieldType::Vec(inner_field) => {
            parse_sql_where_entry_item(column_name, field_type, optional, value, graph, dialect, key_path, &vec!["equals", "has", "hasEvery", "hasSome", "isEmpty", "length"])
        }
        FieldType::Map(_) => {
            panic!()
        }
        FieldType::Object(_) => {
            panic!()
        }
    }
}

pub(crate) fn build_where_input(model: &Model, graph: &Graph, r#where: Option<&JsonValue>, dialect: SQLDialect, key_path: &Vec<KeyPathItem>) -> Result<Option<String>, ActionError> {
    if let None = r#where { return Ok(None); }
    let r#where = r#where.unwrap();
    if !r#where.is_object() { return Err(ActionError::invalid_query_input("'where' should be an object.")); }
    let r#where = r#where.as_object().unwrap();
    let mut retval: Vec<String> = vec![];
    for (key, value) in r#where.iter() {
        if key == "AND" {
            let mut path = key_path.clone();
            path.push(KeyPathItem::String("AND".to_string()));
            let inner = WhereClause::And(value.as_array().unwrap().iter().map(|w| build_where_input(model, graph, Some(w), dialect, &path).unwrap().unwrap()).collect()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if key == "OR" {
            let mut path = key_path.clone();
            path.push(KeyPathItem::String("OR".to_string()));
            let inner = WhereClause::Or(value.as_array().unwrap().iter().map(|w| build_where_input(model, graph, Some(w), dialect, &path).unwrap().unwrap()).collect()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if key == "NOT" {
            let mut path = key_path.clone();
            path.push(KeyPathItem::String("NOT".to_string()));
            let inner = WhereClause::Not(build_where_input(model, graph, Some(value), dialect, &path).unwrap().unwrap()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if !model.query_keys().contains(key) {
            return Err(ActionError::keys_unallowed());
        }
        if let Some(field) = model.field(key) {
            let column_name = field.column_name();
            let optional = field.optionality.is_optional();
            let where_entry = parse_sql_where_entry(column_name, &field.field_type, optional, value, graph, dialect, key_path)?;
            retval.push(where_entry);
        } else if let Some(relation) = model.relation(key) {
            panic!("not handle this yet")
            // let relation = model.relation(key).unwrap();
            // let model_name = &relation.model;
            // let this_model = graph.model(model_name)?;
            // let (command, inner_where) = one_length_json_obj(value, "")?;
            // let _inner_where = build_where_input(this_model, graph, Some(inner_where))?;
            // match command {
            //     "none" | "isNot" => {
            //         doc.insert(key, doc!{"$size": 0});
            //     }
            //     "some" | "is" => {
            //         doc.insert(key, doc!{"$size": 1});
            //     }
            //     "all" => {
            //         doc.insert(key, doc!{"$size": 0});
            //     }
            //     _ => {
            //
            //     }
            // }
        }
    }
    if retval.is_empty() {
        return Ok(None)
    } else {
        Ok(Some(And(retval).to_string(dialect)))
    }
}

pub(crate) fn build_sql_query(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    args: UserJsonArgs,
    dialect: SQLDialect,
) -> Result<String, ActionError> {
    let mut stmt = SQL::select(None, model.table_name());
    if let Some(r#where) = args.r#where {
        if let Some(where_result) = build_where_input(model, graph, Some(r#where), dialect, &vec![KeyPathItem::String("where".to_string())])? {
            stmt.r#where(where_result);
        }
    }
    Ok(stmt.to_string(dialect))
}

pub(crate) fn build_sql_query_from_json(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    json_value: &JsonValue,
    dialect: SQLDialect,
) -> Result<String, ActionError> {
    let args = user_json_args(model, graph, r#type, mutation_mode, json_value)?;
    build_sql_query(model, graph, r#type, mutation_mode, args, dialect)
}
