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

    let options = ClientOptions::parse("mongodb://localhost:27017/teotesttype").await.unwrap();

    Box::leak(Box::new(Graph::new(|g| {

        g.mongodb(options.clone());

        g.r#enum("Sex", vec!["MALE", "FEMALE"]);

        g.model("Simple", |m| {
            m.field("objectId", |f| {
                f.optional().object_id();
            });
            m.field("string", |f| {
                f.optional().string();
            });
            m.field("bool", |f| {
                f.optional().bool();
            });
            m.field("i8", |f| {
                f.optional().i8();
            });
            m.field("i16", |f| {
                f.optional().i16();
            });
            m.field("i32", |f| {
                f.optional().i32();
            });
            m.field("i64", |f| {
                f.optional().i64();
            });
            m.field("i128", |f| {
                f.optional().i128();
            });
            m.field("u8", |f| {
                f.optional().u8();
            });
            m.field("u16", |f| {
                f.optional().u16();
            });
            m.field("u32", |f| {
                f.optional().u32();
            });
            m.field("u64", |f| {
                f.optional().u64();
            });
            m.field("u128", |f| {
                f.optional().u128();
            });
            m.field("f32", |f| {
                f.optional().f32();
            });
            m.field("f64", |f| {
                f.optional().f64();
            });
            m.field("date", |f| {
                f.optional().date();
            });
            m.field("datetime", |f| {
                f.optional().datetime();
            });
            m.field("sex", |f| {
                f.optional().r#enum("Sex");
            })
        })
    })))
}

#[test]
async fn object_id_input_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"objectId": "1234567890abcd1234567890abcd"})).await;
    let value = simple.get_value("objectId").unwrap().unwrap();
    assert_eq!(value, Value::ObjectId("1234567890abcd1234567890abcd".to_string()));
}

#[test]
async fn object_id_output_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"objectId": "1234567890abcd1234567890abcd"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("objectId").unwrap().as_str().unwrap(), "1234567890abcd1234567890abcd");
}

#[test]
async fn string_input_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"string": "strval"})).await;
    let value = simple.get_value("string").unwrap().unwrap();
    assert_eq!(value, Value::String("strval".to_string()));
}

#[test]
async fn string_output_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"string": "strval"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("string").unwrap().as_str().unwrap(), "strval");
}

#[test]
async fn bool_input_is_bool() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"bool": false})).await;
    let value = simple.get_value("bool").unwrap().unwrap();
    assert_eq!(value, Value::Bool(false));
}

#[test]
async fn bool_output_is_bool() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"bool": true})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("bool").unwrap().as_bool().unwrap(), true);
}

#[test]
async fn i8_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i8": 2})).await;
    let value = simple.get_value("i8").unwrap().unwrap();
    assert_eq!(value, Value::I8(2));
}

#[test]
async fn i8_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i8": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("i8").unwrap().as_i64().unwrap(), 3);
}

#[test]
async fn i16_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i16": 2})).await;
    let value = simple.get_value("i16").unwrap().unwrap();
    assert_eq!(value, Value::I16(2));
}

#[test]
async fn i16_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i16": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("i16").unwrap().as_i64().unwrap(), 3);
}

#[test]
async fn i32_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i32": 2})).await;
    let value = simple.get_value("i32").unwrap().unwrap();
    assert_eq!(value, Value::I32(2));
}

#[test]
async fn i32_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i32": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("i32").unwrap().as_i64().unwrap(), 3);
}

#[test]
async fn i64_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i64": 2})).await;
    let value = simple.get_value("i64").unwrap().unwrap();
    assert_eq!(value, Value::I64(2));
}

#[test]
async fn i64_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i64": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("i64").unwrap().as_i64().unwrap(), 3);
}

#[test]
async fn i128_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i128": 2})).await;
    let value = simple.get_value("i128").unwrap().unwrap();
    assert_eq!(value, Value::I128(2));
}

#[test]
async fn i128_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"i128": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("i128").unwrap().as_i64().unwrap(), 3);
}

#[test]
async fn u8_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u8": 2})).await;
    let value = simple.get_value("u8").unwrap().unwrap();
    assert_eq!(value, Value::U8(2));
}

#[test]
async fn u8_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u8": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("u8").unwrap().as_u64().unwrap(), 3);
}

#[test]
async fn u16_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u16": 2})).await;
    let value = simple.get_value("u16").unwrap().unwrap();
    assert_eq!(value, Value::U16(2));
}

#[test]
async fn u16_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u16": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("u16").unwrap().as_u64().unwrap(), 3);
}

#[test]
async fn u32_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u32": 2})).await;
    let value = simple.get_value("u32").unwrap().unwrap();
    assert_eq!(value, Value::U32(2));
}

#[test]
async fn u32_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u32": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("u32").unwrap().as_u64().unwrap(), 3);
}

#[test]
async fn u64_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u64": 2})).await;
    let value = simple.get_value("u64").unwrap().unwrap();
    assert_eq!(value, Value::U64(2));
}

#[test]
async fn u64_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u64": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("u64").unwrap().as_u64().unwrap(), 3);
}

#[test]
async fn u128_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u128": 2})).await;
    let value = simple.get_value("u128").unwrap().unwrap();
    assert_eq!(value, Value::U128(2));
}

#[test]
async fn u128_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"u128": 3})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("u128").unwrap().as_u64().unwrap(), 3);
}

#[test]
async fn f32_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"f32": 2.0})).await;
    let value = simple.get_value("f32").unwrap().unwrap();
    assert_eq!(value, Value::F32(2.0));
}

#[test]
async fn f32_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"f32": 2.0})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("f32").unwrap().as_f64().unwrap(), 2.0);
}

#[test]
async fn f64_input_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"f64": 2.0})).await;
    let value = simple.get_value("f64").unwrap().unwrap();
    assert_eq!(value, Value::F64(2.0));
}

#[test]
async fn f64_output_is_number() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"f64": 2.0})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("f64").unwrap().as_f64().unwrap(), 2.0);
}

#[test]
async fn date_input_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"date": "2022-05-20"})).await;
    let value = simple.get_value("date").unwrap().unwrap();
    let date = Date::from_utc(NaiveDate::parse_from_str("2022-05-20", "%Y-%m-%d").unwrap(), Utc);
    assert_eq!(value, Value::Date(date));
}

#[test]
async fn date_output_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"date": "2022-05-20"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("date").unwrap().as_str().unwrap(), "2022-05-20");
}

#[test]
async fn returns_err_if_date_format_is_unexpected() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    let result = simple.set_json(json!({"date": "2022-0520"})).await;
    assert_eq!(result.err().unwrap(), ActionError::wrong_date_format());
}

#[test]
async fn datetime_input_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"datetime": "2022-05-20T04:27:16.428Z"})).await;
    let value = simple.get_value("datetime").unwrap().unwrap();
    let datetime = DateTime::parse_from_rfc3339("2022-05-20T04:27:16.428Z").unwrap().with_timezone(&Utc);
    assert_eq!(value, Value::DateTime(datetime));
}

#[test]
async fn datetime_output_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"datetime": "2022-05-20T04:27:16.428Z"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("datetime").unwrap().as_str().unwrap(), "2022-05-20T04:27:16.428Z");
}

#[test]
async fn returns_err_if_datetime_format_is_unexpected() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    let result = simple.set_json(json!({"datetime": "2022-05-20::04:27:16.428"})).await;
    assert_eq!(result.err().unwrap(), ActionError::wrong_datetime_format());
}

#[test]
async fn enum_input_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"sex": "MALE"})).await;
    let value = simple.get_value("sex").unwrap().unwrap();
    assert_eq!(value, Value::String("MALE".to_string()));
}

#[test]
async fn enum_output_is_string() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    simple.set_json(json!({"sex": "FEMALE"})).await;
    let json_output = simple.to_json();
    assert_eq!(json_output.as_object().unwrap().get("sex").unwrap().as_str().unwrap(), "FEMALE");
}

#[test]
async fn returns_err_if_enum_value_is_unexpected() {
    let graph = make_graph().await;
    let simple = graph.new_object("Simple");
    let result = simple.set_json(json!({"sex": "NAM"})).await;
    assert_eq!(result.err().unwrap(), ActionError::wrong_enum_choice());
}
