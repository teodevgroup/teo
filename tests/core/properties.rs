use std::sync::Arc;
use chrono::{NaiveDate, NaiveTime, Utc};
use chrono::format::Fixed::TimezoneOffset;
use mongodb::options::ClientOptions;
use serde_json::{json, from_str};
use chrono::prelude::{Date, DateTime};
use tokio::test;
use teo::connectors::mongodb::MongoDBConnectorHelpers;
use teo::core::graph::Graph;
use teo::core::value::Value;
use teo::error::ActionError;


async fn make_graph() -> &'static Graph {

    let options = ClientOptions::parse("mongodb://localhost:27017/teotestproperties").await.unwrap();

    let graph = Graph::new(|g| {

        g.mongodb(options.clone());

        g.model("Required", |m| {
            m.field("string", |f| {
                f.required().string();
            });
        });

        g.model("Optional", |m| {
            m.field("string", |f| {
                f.optional().string();
            });
        });

        g.model("Readonly", |m| {
            m.field("readonly", |f| {
                f.readonly().optional().string();
            })
        });

        g.model("Writeonly", |m| {
            m.field("writeonly", |f| {
                f.writeonly().optional().string();
            })
        });

        g.model("Internal", |m| {
            m.field("internal", |f| {
                f.internal().optional().string();
            })
        });
    });

    graph.drop_database();

    graph
}

#[test]
async fn optional_field_if_no_input_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.new_object("Optional");
    simple.set_json(json!({})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn optional_field_if_input_is_null_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.new_object("Optional");
    simple.set_json(json!({"string": null})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn required_field_if_no_input_value_is_none() {
    let graph = make_graph().await;
    let simple = graph.new_object("Required");
    simple.set_json(json!({})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn required_field_if_input_is_null_returns_none() {
    let graph = make_graph().await;
    let simple = graph.new_object("Required");
    simple.set_json(json!({"string": null})).await;
    let value = simple.get_value("string").unwrap();
    assert_eq!(value, None);
}

#[test]
async fn readonly_field_cannot_accept_value_through_set_json() {
    let graph = make_graph().await;
    let simple = graph.new_object("Readonly");
    let result = simple.set_json(json!({"readonly": "my_value"})).await;
    assert_eq!(result.err().unwrap(), ActionError::keys_unallowed());
}

#[test]
async fn readonly_field_can_accept_value_through_update_json() {
    let graph = make_graph().await;
    let simple = graph.new_object("Readonly");
    simple.update_json(json!({"readonly": "my_value"})).await;
    let value = simple.get_value("readonly");
    assert_eq!(value.unwrap().unwrap(), Value::String("my_value".to_string()));
}

#[test]
async fn readonly_field_can_accept_value_through_set_value() {
    let graph = make_graph().await;
    let simple = graph.new_object("Readonly");
    simple.set_value("readonly", Value::String("ok".to_string()));
    let value = simple.get_value("readonly");
    assert_eq!(value.unwrap().unwrap(), Value::String("ok".to_string()));
}

#[test]
async fn writeonly_field_cannot_output_into_to_json() {
    let graph = make_graph().await;
    let simple = graph.new_object("Writeonly");
    let result = simple.set_json(json!({"writeonly": "123"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("writeonly"), None);
}

#[test]
async fn writeonly_field_value_can_be_get_through_get_value() {
    let graph = make_graph().await;
    let simple = graph.new_object("Writeonly");
    let result = simple.set_json(json!({"writeonly": "123"})).await;
    let value = simple.get_value("writeonly").unwrap().unwrap();
    assert_eq!(value, Value::String("123".to_string()));
}

#[test]
async fn internal_field_cannot_accept_value_through_set_json() {
    let graph = make_graph().await;
    let simple = graph.new_object("Internal");
    let result = simple.set_json(json!({"internal": "my_value"})).await;
    assert_eq!(result.err().unwrap(), ActionError::keys_unallowed());
}

#[test]
async fn internal_field_can_accept_value_through_update_json() {
    let graph = make_graph().await;
    let simple = graph.new_object("Internal");
    simple.update_json(json!({"internal": "my_value"})).await;
    let value = simple.get_value("internal");
    assert_eq!(value.unwrap().unwrap(), Value::String("my_value".to_string()));
}

#[test]
async fn internal_field_can_accept_value_through_set_value() {
    let graph = make_graph().await;
    let simple = graph.new_object("Internal");
    simple.set_value("internal", Value::String("ok".to_string()));
    let value = simple.get_value("internal");
    assert_eq!(value.unwrap().unwrap(), Value::String("ok".to_string()));
}

#[test]
async fn internal_field_cannot_output_into_to_json() {
    let graph = make_graph().await;
    let simple = graph.new_object("Internal");
    let result = simple.set_json(json!({"internal": "123"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("internal"), None);
}

#[test]
async fn internal_field_value_can_be_get_through_get_value() {
    let graph = make_graph().await;
    let simple = graph.new_object("Internal");
    simple.set_value("internal", Value::String("123".to_string()));
    let value = simple.get_value("internal").unwrap().unwrap();
    assert_eq!(value, Value::String("123".to_string()));
}
