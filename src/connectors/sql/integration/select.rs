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
