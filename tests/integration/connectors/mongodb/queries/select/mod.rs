use actix_http::body::BoxBody;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serial_test::serial;
use actix_web::{test, web, App, error::Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use teo::core::graph::Graph;
use serde_json::json;
use teo::app::app::ServerConfiguration;
use teo::app::serve::make_app;
use crate::helpers::{request, request_get, assert_json_response};

async fn app() -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = Error,
>> {
    let graph = Graph::new(|g| {
        g.data_source().mongodb("mongodb://127.0.0.1:27017/teotest_query_select");
        g.reset_database();
        g.model("Single", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("str", |f| {
                f.required().string();
            });
            m.field("num", |f| {
                f.required().i32();
            });
            m.field("bool", |f| {
                f.required().bool();
            });
        });
    }).await;
    make_app(graph, ServerConfiguration::default())
}

#[test]
#[serial]
async fn select_keeps_selected_scalar_non_primary_fields_on_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "singles", "create", json!({
        "create": {
            "str": "scalar",
            "num": 2,
            "bool": true
        },
        "select": {
            "id": true,
            "str": true,
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "str": {"equals": "scalar"}
        }
    })).await;
}

#[test]
#[serial]
async fn select_removes_scalar_non_primary_fields_on_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "singles", "create", json!({
        "create": {
            "str": "scalar",
            "num": 2,
            "bool": true
        },
        "select": {
            "num": false,
            "bool": false,
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "str": {"equals": "scalar"}
        }
    })).await;
}

#[test]
#[serial]
async fn select_can_remove_primary_fields_in_the_output_on_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "singles", "create", json!({
        "create": {
            "str": "scalar",
            "num": 2,
            "bool": true
        },
        "select": {
            "id": false,
            "str": false,
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "num": {"equals": 2},
            "bool": {"equals": true}
        }
    })).await;
}