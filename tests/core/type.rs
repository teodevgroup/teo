use std::sync::Arc;
use mongodb::options::ClientOptions;
use serde_json::json;
use tokio::test;
use teo::connectors::mongodb::MongoDBConnectorHelpers;
use teo::core::graph::Graph;
use teo::core::value::Value;


async fn make_graph() -> Graph {

    let options = ClientOptions::parse("mongodb://localhost:27017/teotesttype").await.unwrap();

    Graph::new(|g| {

        g.mongodb(options.clone());

        g.model("Simple", |m| {
            m.field("objectId", |f| {
                f.optional().object_id();
            })
        })
    })
}

#[tokio::test]
async fn object_id_input_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"objectId": "1234567890abcd1234567890abcd"})).await;
    let value = simple.get_value("objectId").unwrap().unwrap();
    assert_eq!(value, Value::ObjectId("1234567890abcd1234567890abcd".to_string()));
}

#[tokio::test]
async fn object_id_output_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"objectId": "1234567890abcd1234567890abcd"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("objectId").unwrap().as_str().unwrap(), "1234567890abcd1234567890abcd");
}

#[tokio::test]
async fn object_id_if_no_input_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({})).await;
    let value = simple.get_value("objectId").unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn object_id_if_input_is_null_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"objectId": null})).await;
    let value = simple.get_value("objectId").unwrap();
    assert_eq!(value, None);
}

#[tokio::test]
async fn see_null() {
    let null1 = Value::Null;
    let null2 = Value::Null;
    assert_eq!(null1, null2);
}
