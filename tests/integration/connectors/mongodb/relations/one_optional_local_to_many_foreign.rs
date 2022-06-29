use actix_http::body::BoxBody;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serial_test::serial;
use actix_web::{test, web, App, error::Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use teo::core::graph::Graph;
use teo::server::server::Server;
use serde_json::json;
use crate::helpers::{request, request_get, assert_json_response};

async fn app() -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = Error,
>> {
    let graph = Box::leak(Box::new(Graph::new(|g| {
        g.data_source().mongodb("mongodb://127.0.0.1:27017/teotest_1ol_mf");
        g.reset_database();
        g.model("OneOptionalLocal", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("foreignId", |f| {
                f.optional().object_id();
            });
            m.relation("foreign", |r| {
                r.optional().object("ManyForeign").fields(vec!["foreignId"]).references(vec!["id"]);
            });
        });
        g.model("ManyForeign", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.relation("locals", |r| {
                r.vec("OneOptionalLocal").fields(vec!["id"]).references(vec!["foreignId"]);
            });
        });
        g.host_url("https://www.example.com");
    }).await));
    let server = Box::leak(Box::new(Server::new(graph)));
    server.make_app()
}

#[test]
#[serial]
async fn create_with_nested_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "one-optional-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreignId": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            "foreign": {
                "id": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            }
        }
    })).await;
}

// Output error for user
// #[test]
// #[serial]
// async fn create_with_nested_create_many() {
//     let app = test::init_service(app().await).await;
//     assert!(true);
// }

#[test]
#[serial]
async fn create_with_nested_connect() {
    let app = test::init_service(app().await).await;
    let foreign_id = request_get(&app, "one-optional-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
        "include": {
            "foreign": true
        }
    }), 200, "data.foreign.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-optional-locals", "Create", json!({
        "create": {
            "foreign": {
                "connect": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreignId": {"equals": foreign_id},
            "foreign": {
                "id": {"equals": foreign_id},
            }
        }
    })).await;
}

#[test]
#[serial]
async fn create_with_nested_connect_or_create_actually_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "one-optional-locals", "Create", json!({
        "create": {
            "foreign": {
                "connectOrCreate": {
                    "where": {"id": "123456789012345678901234"},
                    "create": {}
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreignId": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            "foreign": {
                "id": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            }
        }
    })).await;
}

#[test]
#[serial]
async fn create_with_nested_connect_or_create_actually_connect() {
    let app = test::init_service(app().await).await;
    let foreign_id = request_get(&app, "one-optional-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
        "include": {
            "foreign": true
        }
    }), 200, "data.foreign.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-optional-locals", "Create", json!({
        "create": {
            "foreign": {
                "connectOrCreate": {
                    "where": {"id": foreign_id},
                    "create": {}
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreignId": {"equals": foreign_id},
            "foreign": {
                "id": {"equals": foreign_id},
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_create() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "one-optional-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "one-optional-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreign": {
                "create": {}
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreignId": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            "foreign": {
                "id": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            }
        }
    })).await;
}

// #[test]
// #[serial]
// async fn update_with_nested_create_many() {
//     let app = test::init_service(app().await).await;
//     assert!(true);
// }

#[test]
#[serial]
async fn update_with_nested_connect() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "one-optional-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "many-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-optional-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreign": {
                "connect": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreignId": {"equals": foreign_id},
            "foreign": {
                "id": {"equals": foreign_id},
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_connect_or_create_actually_create() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "one-optional-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "one-optional-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreign": {
                "connectOrCreate": {
                    "where": {"id": "123456789012345678901234"},
                    "create": {}
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreignId": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            "foreign": {
                "id": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_connect_or_create_actually_connect() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "one-optional-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "many-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-optional-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreign": {
                "connectOrCreate": {
                    "where": {"id": foreign_id},
                    "create": {}
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreignId": {"equals": foreign_id},
            "foreign": {
                "id": {"equals": foreign_id},
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_set() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "one-optional-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "many-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-optional-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreign": {
                "set": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreignId": {"equals": foreign_id},
            "foreign": {
                "id": {"equals": foreign_id},
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_disconnect() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_update() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_update_many() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_upsert() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_delete() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_delete_many() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn include() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn include_with_where() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn include_with_order_by() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn include_with_skip_and_limit() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn include_with_cursor() {
    let app = test::init_service(app().await).await;
    assert!(true);
}