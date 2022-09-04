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
        g.data_source().mongodb("mongodb://127.0.0.1:27017/teotest_query_distinct");
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
        g.model("Nested", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("str", |f| {
                f.required().string();
            });
            m.relation("items", |r| {
                r.vec("Item").fields(vec!["id"]).references(vec!["nestedId"]);
            });
        });
        g.model("Item", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("str", |f| {
                f.required().string();
            });
            m.field("nestedId", |f| {
                f.required().object_id();
            });
            m.relation("nested", |r| {
                r.object("Nested").fields(vec!["nestedId"]).references(vec!["id"]);
            });
        });
        g.model("Apple", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("str", |f| {
                f.required().string();
            });
            m.relation("pears", |r| {
                r.vec("Pear").through("Fruit").local("apple").foreign("pear");
            });
        });
        g.model("Pear", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("str", |f| {
                f.required().string();
            });
            m.relation("apples", |r| {
                r.vec("Apple").through("Fruit").local("pear").foreign("apple");
            });
        });
        g.model("Fruit", |m| {
            m.field("appleId", |f| {
                f.required().object_id();
            });
            m.field("pearId", |f| {
                f.required().object_id();
            });
            m.relation("apple", |r| {
                r.object("Apple").fields(vec!["appleId"]).references(vec!["id"]);
            });
            m.relation("pear", |r| {
                r.object("Pear").fields(vec!["pearId"]).references(vec!["id"]);
            });
            m.primary(vec!["appleId", "pearId"]);
        });

    }).await;
    make_app(graph, ServerConfiguration::default())
}

#[test]
#[serial]
async fn distinct_removes_duplicated_records_for_one_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "singles", "create", json!({
        "create": {
            "str": "scalar",
            "num": 2,
            "bool": true
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "singles", "create", json!({
        "create": {
            "str": "scalar",
            "num": 2,
            "bool": true
        },
    }), 200, "data.id").await;
    let res = request(&app, "singles", "findMany", json!({
        "select": {
            "id": true,
            "str": true,
        },
        "distinct": ["num"]
    })).await;
    assert_json_response(res, 200, json!({
        "meta": {
            "count": {"equals": 2}
        },
        "data": [
            {
                "id": {"is": "objectId"},
                "str": {"equals": "scalar"}
            }
        ]
    })).await;
}

#[test]
#[serial]
async fn distinct_removes_duplicated_records_for_multiple_fields() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "singles", "create", json!({
        "create": {
            "str": "scalar",
            "num": 2,
            "bool": true
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "singles", "create", json!({
        "create": {
            "str": "scalar",
            "num": 1,
            "bool": true
        },
    }), 200, "data.id").await;
    let _id3 = request_get(&app, "singles", "create", json!({
        "create": {
            "str": "fixed",
            "num": 2,
            "bool": true
        },
    }), 200, "data.id").await;
    let _id4 = request_get(&app, "singles", "create", json!({
        "create": {
            "str": "fixed",
            "num": 2,
            "bool": true
        },
    }), 200, "data.id").await;
    let res = request(&app, "singles", "findMany", json!({
        "orderBy": [
            {
                "num": "asc"
            },
            {
                "str": "asc"
            }
        ],
        "select": {
            "id": true,
            "str": true,
            "num": true,
        },
        "distinct": ["num", "str"]
    })).await;
    assert_json_response(res, 200, json!({
        "meta": {
            "count": {"equals": 4}
        },
        "data": [
            {
                "id": {"is": "objectId"},
                "str": {"equals": "scalar"},
                "num": {"equals": 1}
            },
            {
                "id": {"is": "objectId"},
                "str": {"equals": "fixed"},
                "num": {"equals": 2}
            },
            {
                "id": {"is": "objectId"},
                "str": {"equals": "scalar"},
                "num": {"equals": 2}
            },
        ]
    })).await;
}

#[test]
#[serial]
async fn distinct_can_remove_duplicates_in_the_output_of_nested_many_in_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "nesteds", "create", json!({
        "create": {
            "str": "scalar",
            "items": {
                "createMany": [
                    {
                        "str": "scalar"
                    },
                    {
                        "str": "scalar"
                    }
                ]
            }
        },
        "include": {
            "items": {
                "select": {
                    "id": false,
                    "nestedId": false
                },
                "distinct": ["str"]
            }
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "str": {"equals": "scalar"},
            "items": [
                {
                    "str": {"equals": "scalar"}
                },
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn distinct_can_remove_duplicates_in_the_output_of_nested_joined_many_in_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "apples", "create", json!({
        "create": {
            "str": "scalar",
            "pears": {
                "createMany": [
                    {
                        "str": "scalar"
                    },
                    {
                        "str": "scalar"
                    }
                ]
            }
        },
        "include": {
            "pears": {
                "select": {
                    "id": false
                },
                "distinct": ["str"]
            }
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "str": {"equals": "scalar"},
            "pears": [
                {
                    "str": {"equals": "scalar"}
                },
            ]
        }
    })).await;
}
