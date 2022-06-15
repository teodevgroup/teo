use serde_json::{Value as JsonValue};
use bson::{doc, Document};
use crate::core::graph::Graph;
use crate::core::model::Model;
use crate::error::ActionError;


#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum QueryPipelineType {
    Unique,
    First,
    Many
}

fn build_where_input(
    model: &Model,
    graph: &Graph,
    r#where: Option<&JsonValue>,
) -> Result<Document, ActionError> {
    Ok(doc!{})
}

fn build_query_pipeline(
    model: &Model,
    graph: &Graph,
    r#type: QueryPipelineType,
    mutation_mode: bool,
    r#where: Option<&JsonValue>,
    order_by: Option<&JsonValue>,
    take: Option<usize>,
    skip: Option<usize>,
    page_size: Option<usize>,
    page_number: Option<usize>,
    include: Option<&JsonValue>,
    select: Option<&JsonValue>,
) -> Result<Vec<Document>, ActionError> {
    let mut retval: Vec<Document> = vec![];
    // $match
    let r#match = build_where_input(model, graph, r#where)?;
    if !r#match.is_empty() {
        retval.push(r#match);
    }
    // $sort

    // $skip and $limit
    if page_size.is_some() && page_number.is_some() {
        retval.push(doc!{"$skip": (page_number.unwrap() - 1) * page_size.unwrap()});
        retval.push(doc!{"limit": page_size});
    } else {
        if skip.is_some() {
            retval.push(doc!{"$skip": skip.unwrap()});
        }
        if take.is_some() {
            retval.push(doc!{"$limit": skip.unwrap()});
        }
    }
    // $project
    // $lookup

    Ok(retval)
}