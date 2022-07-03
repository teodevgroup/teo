use actix_http::body::BoxBody;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serial_test::serial;
use actix_web::{test, web, App, error::Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use teo::core::graph::Graph;
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
        g.data_source().mongodb("mongodb://127.0.0.1:27017/teotest_ml_mf");
        g.reset_database();
        g.model("ManyLocal", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.relation("joins", |r| {
                r.vec("ManyJoin").fields(vec!["id"]).references(vec!["localId"]);
            });
            m.relation("foreigns", |r| {
                r.vec("ManyForeign").through("ManyJoin").local("local").foreign("foreign");
            });
        });
        g.model("ManyForeign", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.relation("joins", |r| {
                r.vec("ManyJoin").fields(vec!["id"]).references(vec!["foreignId"]);
            });
            m.relation("locals", |r| {
                r.vec("ManyLocal").through("ManyJoin").local("foreign").foreign("local");
            });
        });
        g.model("ManyJoin", |m| {
            m.field("localId", |f| {
                f.required().object_id();
            });
            m.field("foreignId", |f| {
                f.required().object_id();
            });
            m.relation("local", |r| {
                r.required().object("ManyLocal").fields(vec!["localId"]).references(vec!["id"]);
            });
            m.relation("foreign", |r| {
                r.required().object("ManyForeign").fields(vec!["foreignId"]).references(vec!["id"]);
            });
            m.primary(vec!["localId", "foreignId"]);
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
    let res = request(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": [
                {
                    "id": {"and": [{"is": "objectId"}, {"is": "$foreignId"}]},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn create_with_nested_create_many() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "createMany": [{}, {}]
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": [
                {
                    "id": {"is": "objectId"},
                },
                {
                    "id": {"is": "objectId"},
                },
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn create_with_nested_connect() {
    let app = test::init_service(app().await).await;
    let foreign_id = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, "data.foreigns.0.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "connect": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": [
                {
                    "id": {"equals": foreign_id},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn create_with_nested_connect_or_create_actually_create() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "connectOrCreate": {
                    "where": {"id": "123456789012345678901234"},
                    "create": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": [
                {
                    "id": {"is": "objectId"}
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn create_with_nested_connect_or_create_actually_connect() {
    let app = test::init_service(app().await).await;
    let foreign_id = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, "data.foreigns.0.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "connectOrCreate": {
                    "where": {"id": foreign_id},
                    "create": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": [
                {
                    "id": {"equals": foreign_id},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_create() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "many-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": [
                {
                    "id": {"is": "objectId"},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_create_many() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "many-locals", "Create", json!({
        "create": {},
        "include": {
            "foreigns": true
        }
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreigns": {
                "createMany": [{}, {}]
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": [
                {
                    "id": {"is": "objectId"},
                },
                {
                    "id": {"is": "objectId"},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_connect() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "many-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "many-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreigns": {
                "connect": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"equals": foreign_id},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_connect_or_create_actually_create() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "many-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreigns": {
                "connectOrCreate": {
                    "where": {"id": "123456789012345678901234"},
                    "create": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"is": "objectId"}
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_connect_or_create_actually_connect() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "many-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "many-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreigns": {
                "connectOrCreate": {
                    "where": {"id": foreign_id},
                    "create": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"equals": foreign_id},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_set() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "many-locals", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "many-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreigns": {
                "set": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"equals": foreign_id},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_disconnect() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, vec!["data.id", "data.foreigns.0.id"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreigns": {
                "disconnect": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"is": "objectId"},
            "foreigns": []
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_update() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, vec!["data.id", "data.foreigns.0.id"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreigns": {
                "update": {
                    "where": {"id": foreign_id},
                    "update": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"equals": foreign_id}
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_update_many() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, vec!["data.id", "data.foreigns.0.id"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreigns": {
                "updateMany": {
                    "where": {"id": foreign_id},
                    "update": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"equals": foreign_id}
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_upsert_actually_create() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        }
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreigns": {
                "upsert": {
                    "where": {"id": "123456789009876543211234"},
                    "update": {},
                    "create": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"is": "objectId"},
                },
                {
                    "id": {"is": "objectId"},
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_upsert_actually_update() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, vec!["data.id", "data.foreigns.0.id"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreigns": {
                "upsert": {
                    "where": {"id": foreign_id},
                    "update": {},
                    "create": {}
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"equals": foreign_id}
                }
            ]
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_delete() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, vec!["data.id", "data.foreigns.0.id"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreigns": {
                "delete": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": []
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_delete_many() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, vec!["data.id", "data.foreigns.0.id"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "many-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreigns": {
                "deleteMany": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": []
        }
    })).await;
}

#[test]
#[serial]
async fn include() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "many-locals", "Create", json!({
        "create": {
            "foreigns": {
                "create": {}
            }
        },
        "include": {
            "foreigns": true
        }
    }), 200, vec!["data.id", "data.foreigns.0.id"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "many-locals", "FindUnique", json!({
        "where": {"id": id},
        "include": {
            "foreigns": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreigns": [
                {
                    "id": {"equals": foreign_id}
                }
            ]
        }
    })).await;
}

// include with select

// include with include

// #[test]
// #[serial]
// async fn include_with_where() {
//     let app = test::init_service(app().await).await;
//     assert!(true);
// }

// #[test]
// #[serial]
// async fn include_with_order_by() {
//     let app = test::init_service(app().await).await;
//     assert!(true);
// }

// #[test]
// #[serial]
// async fn include_with_skip_and_limit() {
//     let app = test::init_service(app().await).await;
//     assert!(true);
// }

// #[test]
// #[serial]
// async fn include_with_cursor() {
//     let app = test::init_service(app().await).await;
//     assert!(true);
// }
