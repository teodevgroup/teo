use sqlx::types::JsonValue;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::connectors::shared::user_json_args::{user_json_args, UserJsonArgs};
use crate::connectors::sql::query_builder::dialect::SQLDialect;
use crate::connectors::sql::query_builder::stmt::select::r#where::WhereClause;
use crate::connectors::sql::query_builder::stmt::SQL;
use crate::connectors::sql::query_builder::traits::to_sql_string::ToSQLString;
use crate::core::error::ActionError;
use crate::core::model::Model;
use crate::prelude::Graph;

pub(crate) fn build_where_input(model: &Model, graph: &Graph, r#where: Option<&JsonValue>, dialect: SQLDialect) -> Result<Option<String>, ActionError> {
    if let None = r#where { return Ok(None); }
    let r#where = r#where.unwrap();
    if !r#where.is_object() { return Err(ActionError::invalid_query_input("'where' should be an object.")); }
    let r#where = r#where.as_object().unwrap();
    let mut retval: Vec<String> = vec![];
    for (key, value) in r#where.iter() {
        if key == "AND" {
            let inner = WhereClause::And(value.as_array().unwrap().iter().map(|w| build_where_input(model, graph, Some(w), dialect).unwrap().unwrap()).collect()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if key == "OR" {
            let inner = WhereClause::Or(value.as_array().unwrap().iter().map(|w| build_where_input(model, graph, Some(w), dialect).unwrap().unwrap()).collect()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if key == "NOT" {
            let inner = WhereClause::Not(build_where_input(model, graph, Some(value), dialect).unwrap().unwrap()).to_string(dialect);
            let val = "(".to_owned() + &inner + ")";
            retval.push(val);
            continue;
        } else if !model.query_keys().contains(key) {
            return Err(ActionError::keys_unallowed());
        }
        if let Some(field) = model.field(key) {
            let db_key = field.column_name();
            let bson_result = parse_bson_where_entry(&field.field_type, value, graph);
            match bson_result {
                Ok(bson) => {
                    doc.insert(db_key, bson);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        } else if let Some(relation) = model.relation(key) {
            let relation = model.relation(key).unwrap();
            let model_name = &relation.model;
            let this_model = graph.model(model_name)?;
            let (command, inner_where) = one_length_json_obj(value, "")?;
            let _inner_where = build_where_input(this_model, graph, Some(inner_where))?;
            match command {
                "none" | "isNot" => {
                    doc.insert(key, doc!{"$size": 0});
                }
                "some" | "is" => {
                    doc.insert(key, doc!{"$size": 1});
                }
                "all" => {
                    doc.insert(key, doc!{"$size": 0});
                }
                _ => {

                }
            }
        }
    }
    Ok(Some(WhereClause::And(retval).to_string(dialect)))
}

pub(crate) fn build_sql_query(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    args: UserJsonArgs
) -> Result<String, ActionError> {
    let mut stmt = SQL::select(None, model.table_name());
    if let Some(r#where) = args.r#where {

        stmt.r#where
    }
}

pub(crate) fn build_sql_query_from_json(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    json_value: &JsonValue
) -> Result<String, ActionError> {
    let args = user_json_args(model, graph, r#type, mutation_mode, json_value)?;
    build_sql_query(model, graph, r#type, mutation_mode, args)
}
