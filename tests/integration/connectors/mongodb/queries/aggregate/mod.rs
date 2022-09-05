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
        g.data_source().mongodb("mongodb://127.0.0.1:27017/teotest_query_aggregate");
        g.reset_database();
        g.model("Person", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("name", |f| {
                f.required().string();
            });
            m.field("age", |f| {
                f.required().i32();
            });
            m.field("salary", |f| {
                f.required().f64();
            });
        });
    }).await;
    make_app(graph, ServerConfiguration::default())
}

#[test]
#[serial]
async fn aggregate_returns_null_for_fields_if_no_record() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "people", "aggregate", json!({
        "_sum": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "_sum": {
                "age": {"is": "null"},
                "salary": {"is": "null"}
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_can_sum_for_each_number_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "jobs",
            "age": 22,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", json!({
        "_sum": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "_sum": {
                "age": {"equals": 42},
                "salary": {"equals": 5001.0}
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_can_avg_for_each_number_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", json!({
        "_avg": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "_avg": {
                "age": {"equals": 20.5},
                "salary": {"equals": 2500.5}
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_can_min_for_each_number_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", json!({
        "_min": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "_min": {
                "age": {"equals": 20},
                "salary": {"equals": 1.0}
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_can_max_for_each_number_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", json!({
        "_max": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "_max": {
                "age": {"equals": 21},
                "salary": {"equals": 5000.0}
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_can_do_together_for_each_number_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", json!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", json!({
        "_min": {
            "age": true
        },
        "_max": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "_min": {
                "age": {"equals": 20}
            },
            "_max": {
                "age": {"equals": 21},
                "salary": {"equals": 5000.0}
            },
        }
    })).await;
}
