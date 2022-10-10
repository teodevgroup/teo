use std::collections::HashMap;
use std::fmt::format;
use key_path::KeyPath;
use crate::connectors::shared::user_json_args::{user_json_args, UserJsonArgs};
use crate::connectors::sql::stmts_builder::integration::value_encoder::{IfIMode, ToLike, ToSQLInput, ToWrapped, WrapInArray};
use crate::connectors::sql::stmts::select::r#where::{ToWrappedSQLString, WhereClause};
use crate::connectors::sql::stmts::select::r#where::WhereClause::And;
use crate::connectors::sql::stmts::SQL;
use crate::connectors::sql::to_sql_string::ToSQLString;
use crate::connectors::sql::stmts_builder::integration::value_encoder::ValueToSQLString;
use crate::core::error::ActionError;
use crate::core::field::r#type::FieldType;
use crate::core::input::Input;
use crate::core::model::Model;
use crate::prelude::{Graph, Value};

fn sql_where_item(col_name: &str, op: &str, val: String) -> String {
    format!("{col_name} {op} {val}")
}

fn parse_sql_where_entry_array<'a>(
    column_name: &str,
    r#type: &FieldType,
    optional: bool,
    value: &Value,
    key_path: impl AsRef<KeyPath<'a>>,
    graph: &Graph,
    op: &str
) -> Result<String, ActionError> {
    match value.as_vec() {
        Some(arr_val) => {
            let mut arr: Vec<String> = Vec::new();
            for val in arr_val {
                arr.push(val.to_sql_string(r#type, optional, key_path.as_ref(), graph)?);
            }
            Ok(sql_where_item(column_name, op, arr.join(", ").to_wrapped()))
        }
        None => {
            Err(ActionError::unexpected_input_type("array", key_path))
        }
    }
}

fn parse_sql_where_entry_item<'a>(
    column_name: &str,
    r#type: &FieldType,
    optional: bool,
    value: &Value,
    graph: &Graph,
    dialect: SQLDialect,
    key_path: impl AsRef<KeyPath<'a>>,
    ops: &Vec<&str>,
) -> Result<String, ActionError> {
    if let Some(map) = value.as_hashmap() {
        let mut result: Vec<String> = vec![];
        for (key, value) in map {
            if !ops.contains(&&**key) {
                return Err(ActionError::unexpected_input_key(key, key_path.as_ref()));
            }
            match key.as_str() {
                "equals" => {
                    result.push(sql_where_item(column_name, "=", value.to_sql_string(r#type, optional, key_path.as_ref(), graph)?));
                }
                "not" => {
                    result.push(sql_where_item(column_name, "<>", value.to_sql_string(r#type, optional, key_path.as_ref(), graph)?));
                }
                "gt" => {
                    result.push(sql_where_item(column_name, ">", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?));
                }
                "gte" => {
                    result.push(sql_where_item(column_name, ">=", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?));
                }
                "lt" => {
                    result.push(sql_where_item(column_name, "<", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?));
                }
                "lte" => {
                    result.push(sql_where_item(column_name, "<=", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?));
                }
                "in" => {
                    result.push(parse_sql_where_entry_array(column_name, r#type, optional, value, key_path.as_ref(), graph, "IN")?);
                }
                "notIn" => {
                    result.push(parse_sql_where_entry_array(column_name, r#type, optional, value, key_path.as_ref(), graph, "NOT IN")?);
                }
                "contains" => {
                    let i_mode = Input::has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "LIKE", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?.to_like(true, true).to_i_mode(i_mode)));
                }
                "startsWith" => {
                    let i_mode = Input::has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "LIKE", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?.to_like(false, true).to_i_mode(i_mode)));
                }
                "endsWith" => {
                    let i_mode = Input::has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "LIKE", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?.to_like(true, false).to_i_mode(i_mode)));
                }
                "matches" => {
                    let i_mode = Input::has_i_mode(map);
                    result.push(sql_where_item(&column_name.to_i_mode(i_mode), "REGEXP", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?.to_i_mode(i_mode)));
                }
                "mode" => { }
                "has" => {
                    let element_type = r#type.element_field().unwrap();
                    result.push(sql_where_item(column_name, "@>", value.to_sql_string(element_type.r#type(), element_type.is_optional(), key_path.as_ref(), graph)?.wrap_in_array()));
                }
                "hasEvery" => {
                    result.push(sql_where_item(column_name, "@>", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?));
                }
                "hasSome" => {
                    result.push(sql_where_item(column_name, "&&", value.to_sql_string(r#type, false, key_path.as_ref(), graph)?));
                }
                "isEmpty" => {
                    result.push(sql_where_item(&format!("ARRAY_LENGTH({})", column_name), "=", "0".to_owned()));
                }
                "length" => {
                    result.push(sql_where_item(&format!("ARRAY_LENGTH({})", column_name), "=", value.to_sql_string(&FieldType::U64, false, key_path.as_ref(), graph)?));
                }
                _ => {
                    return Err(ActionError::unexpected_input_key(key, key_path.as_ref()));
                }
            }
        }
        Ok(And(result).to_wrapped_string(dialect))
    } else {
        Ok(sql_where_item(column_name, "=", value.to_sql_string(r#type, optional, key_path.as_ref(), graph)?))
    }
}

fn parse_sql_where_entry<'a>(
    column_name: &str,
    field_type: &FieldType,
    optional: bool,
    value: &Value,
    graph: &Graph,
    dialect: SQLDialect,
    key_path: impl AsRef<KeyPath<'a>>
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
        FieldType::HashMap(_) => {
            panic!()
        }
        FieldType::BTreeMap(_) => {
            panic!()
        }
        FieldType::Object(_) => {
            panic!()
        }
    }
}

pub(crate) fn build_where_from_identifier(model: &Model, graph: &Graph, identifier: &Value, dialect: SQLDialect) -> String {
    let mut retval: Vec<String> = vec![];
    for (key, value) in identifier.as_hashmap().unwrap() {
        let field = model.field(key).unwrap();
        let column_name = field.column_name();
        retval.push(format!("{} = {}", column_name, value.to_string(dialect)));
    }
    And(retval).to_string(dialect)
}

pub(crate) fn build_where_input<'a>(model: &Model, graph: &Graph, r#where: Option<&Value>, dialect: SQLDialect, key_path: impl AsRef<KeyPath<'a>>) -> Result<Option<String>, ActionError> {
    if let None = r#where { return Ok(None); }
    let r#where = r#where.unwrap();
    if !r#where.is_object() {
        return Err(ActionError::unexpected_input_type("object", key_path.as_ref()));
    }
    let r#where = r#where.as_hashmap().unwrap();
    let mut retval: Vec<String> = vec![];
    for (key, value) in r#where.iter() {
        if key == "AND" {
            let path = key_path.as_ref() + "AND";
            let inner = WhereClause::And(value.as_vec().unwrap().iter().map(|w| build_where_input(model, graph, Some(w), dialect, &path).unwrap().unwrap()).collect()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if key == "OR" {
            let path = key_path.as_ref() + "OR";
            let inner = WhereClause::Or(value.as_vec().unwrap().iter().map(|w| build_where_input(model, graph, Some(w), dialect, &path).unwrap().unwrap()).collect()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if key == "NOT" {
            let path = key_path.as_ref() + "NOT";
            let inner = WhereClause::Not(build_where_input(model, graph, Some(value), dialect, &path).unwrap().unwrap()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if !model.query_keys().contains(key) {
            return Err(ActionError::unexpected_input_key(key, &(key_path.as_ref() + key)));
        }
        if let Some(field) = model.field(key) {
            let column_name = field.column_name();
            let optional = field.optionality.is_optional();
            let where_entry = parse_sql_where_entry(column_name, &field.field_type, optional, value, graph, dialect, key_path.as_ref())?;
            retval.push(where_entry);
        } else if let Some(relation) = model.relation(key) {
            panic!("not handle this yet")
            // let relation = model.relation(key).unwrap();
            // let model_name = relation.model();
            // let this_model = graph.model(model_name)?;
            // let (command, inner_where) = Input::key_value(value, "")?;
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

pub(crate) fn build_order_by_input<'a>(
    model: &Model,
    graph: &Graph,
    order_by: Option<&Value>,
    dialect: SQLDialect,
    key_path: impl AsRef<KeyPath<'a>>
) -> Result<Option<String>, ActionError> {
    if let None = order_by { return Ok(None); }
    let order_by = order_by.unwrap();
    if !order_by.is_object() {
        return Err(ActionError::unexpected_input_type("object", key_path));
    }
    let order_by = order_by.as_hashmap().unwrap();
    let mut retval: Vec<String> = vec![];
    for (key, value) in order_by.iter() {
        if let Some(field) = model.field(key) {
            let column_name = field.column_name();
            if let Some(str) = value.as_str() {
                match str {
                    "asc" => retval.push(format!("{} ASC", column_name)),
                    "desc" => retval.push(format!("{} DESC", column_name)),
                    _ => {
                        let path = key_path.as_ref() + key;
                        return Err(ActionError::unexpected_input_value("\"asc\" or \"desc\"", path));
                    }
                }
            } else {
                let path = key_path.as_ref() + key;
                return Err(ActionError::unexpected_input_type("\"asc\" or \"desc\"", path));
            }
        } else {
            let path = key_path.as_ref() + key;
            return Err(ActionError::unexpected_input_key(key, path));
        }
    }
    if retval.is_empty() {
        Ok(None)
    } else {
        Ok(Some(retval.join(",")))
    }
}

pub(crate) fn build_sql_query<'a>(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    args: UserJsonArgs,
    dialect: SQLDialect,
    additional_where: Option<String>,
    additional_left_join: Option<String>,
    key_path: impl AsRef<KeyPath<'a>>,
) -> Result<String, ActionError> {
    let table_name = if additional_left_join.is_some() {
        model.table_name().to_string() + " AS t"
    } else {
        model.table_name().to_string()
    };
    let mut columns: Vec<String> = vec![];
    if additional_left_join.is_some() {
        columns = model.save_keys().iter().map(|k| format!("t.{} AS {}", k, k)).collect::<Vec<String>>();
    }
    let column_refs = columns.iter().map(|c| c.as_str()).collect::<Vec<&str>>();

    let mut stmt = SQL::select(if columns.is_empty() { None } else { Some(&column_refs) }, &table_name);
    if let Some(r#where) = args.r#where {
        let mut path = key_path.as_ref() + "where";
        if let Some(where_result) = build_where_input(model, graph, Some(r#where), dialect, &path)? {
            stmt.r#where(where_result);
        }
    }
    if let Some(additional_where) = additional_where {
        if stmt.r#where.is_some() {
            stmt.r#where(stmt.r#where.as_ref().unwrap().clone() + &additional_where);
        } else {
            stmt.r#where(additional_where.to_string());
        }
    }
    if let Some(additional_left_join) = additional_left_join {
        stmt.left_join(additional_left_join);
    }
    if let Some(order_by) = args.order_by {
        let mut path = key_path.as_ref() + "orderBy";
        if let Some(order_by_result) = build_order_by_input(model, graph, Some(order_by), dialect, &path)? {
            stmt.order_by(order_by_result);
        }
    }
    if args.page_size.is_some() && args.page_number.is_some() {
        let skip: u64 = ((args.page_number.unwrap() - 1) * args.page_size.unwrap()) as u64;
        let limit: u64 = args.page_size.unwrap() as u64;
        stmt.limit(limit, skip);
    } else if args.skip.is_some() || args.take.is_some() {
        let skip: u64 = if args.skip.is_some() { args.skip.unwrap() as u64 } else { 0 };
        let limit: u64 = if args.take.is_some() { args.take.unwrap() as u64 } else { 18446744073709551615 };
        stmt.limit(limit, skip);
    }
    let result = stmt.to_string(dialect);
    if r#type == QueryPipelineType::Count {
        Ok(format!("SELECT COUNT(*) FROM ({}) AS _", result))
    } else {
        Ok(result)
    }
}

pub(crate) fn build_sql_query_from_json<'a>(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    json_value: &Value,
    dialect: SQLDialect,
    additional_where: Option<String>,
    additional_left_join: Option<String>,
    key_path: impl AsRef<KeyPath<'a>>,
) -> Result<String, ActionError> {
    let args = user_json_args(model, graph, r#type, mutation_mode, json_value)?;
    build_sql_query(model, graph, r#type, mutation_mode, args, dialect, additional_where, additional_left_join, key_path)
}
