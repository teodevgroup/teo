use std::collections::HashSet;
use key_path::path;
use crate::connectors::shared::query_pipeline_type::QueryPipelineType;
use crate::core::error::ActionError;
use crate::core::model::Model;
use crate::core::result::ActionResult;
use crate::prelude::{Graph, Value};
use crate::tson;

fn unwrap_i32(value: Option<&Value>) -> Option<i32> {
    match value {
        Some(value) => Some(value.as_i64().unwrap() as i32),
        None => None
    }
}

pub(crate) fn validate_where_unique(model: &Model, r#where: &Option<&Value>) -> ActionResult<()> {
    if r#where.is_none() {
        return Err(ActionError::invalid_query_input("Unique query should have a where which represents unique key or keys."));
    }
    let r#where = r#where.unwrap();
    if !r#where.is_object() {
        return Err(ActionError::unexpected_input_type("object", path!["where"]));
    }
    let values = r#where.as_object().unwrap();
    // see if key is valid
    let set_vec: Vec<String> = values.keys().map(|k| k.clone()).collect();
    let set = HashSet::from_iter(set_vec.iter().map(|k| k.clone()));
    if !model.unique_query_keys().contains(&set) {
        return Err(ActionError::field_is_not_unique())
    }
    Ok(())
}

pub struct UserJsonArgs<'a> {
    pub(crate) r#where: Option<&'a Value>,
    pub(crate) order_by: Option<&'a Value>,
    pub(crate) cursor: Option<&'a Value>,
    pub(crate) take: Option<i32>,
    pub(crate) skip: Option<i32>,
    pub(crate) page_size: Option<i32>,
    pub(crate) page_number: Option<i32>,
    pub(crate) include: Option<&'a Value>,
    pub(crate) distinct: Option<&'a Value>,
    pub(crate) select: Option<&'a Value>,
    pub(crate) aggregates: Option<Value>,
    pub(crate) by: Option<&'a Value>,
    pub(crate) having: Option<&'a Value>,
}

pub(crate) fn user_json_args<'a>(
    model: &Model,
    _graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    json_value: &'a Value,
) -> Result<UserJsonArgs<'a>, ActionError> {
    let json_value = json_value.as_object();
    if json_value.is_none() {
        return Err(ActionError::invalid_query_input("Query input should be an object."));
    }
    let json_value = json_value.unwrap();
    let r#where = json_value.get("where");
    if r#type == QueryPipelineType::Unique {
        validate_where_unique(model, &r#where)?;
    }
    let order_by = json_value.get("orderBy");
    let cursor = json_value.get("cursor");
    let take = unwrap_i32(json_value.get("take"));
    let skip = unwrap_i32(json_value.get("skip"));
    let page_number = unwrap_i32(json_value.get("pageNumber"));
    let page_size = unwrap_i32(json_value.get("pageSize"));
    let include = if !mutation_mode { json_value.get("include") } else { None };
    let distinct = if !mutation_mode { json_value.get("distinct") } else { None };
    let select = if !mutation_mode { json_value.get("select") } else { None };
    let mut aggregates: Value = tson!({});
    if let Some(avg) = json_value.get("_avg") {
        aggregates.as_object_mut().unwrap().insert("_avg".to_string(), avg.clone());
    }
    if let Some(sum) = json_value.get("_sum") {
        aggregates.as_object_mut().unwrap().insert("_sum".to_string(), sum.clone());
    }
    if let Some(max) = json_value.get("_max") {
        aggregates.as_object_mut().unwrap().insert("_max".to_string(), max.clone());
    }
    if let Some(min) = json_value.get("_min") {
        aggregates.as_object_mut().unwrap().insert("_min".to_string(), min.clone());
    }
    if let Some(count) = json_value.get("_count") {
        aggregates.as_object_mut().unwrap().insert("_count".to_string(), count.clone());
    }
    let aggregates = if aggregates.as_object().unwrap().is_empty() { None } else { Some(aggregates) };
    let by = if !mutation_mode { json_value.get("by") } else { None };
    let having = if !mutation_mode { json_value.get("having") } else { None };
    Ok(UserJsonArgs {
        r#where,
        order_by,
        cursor,
        take,
        skip,
        page_size,
        page_number,
        include,
        distinct,
        select,
        aggregates,
        by,
        having
    })
}
