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
        g.data_source().mongodb("mongodb://127.0.0.1:27017/teotest_1rl_mf");
        g.reset_database();
        g.model("OneRequiredLocal", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("foreignId", |f| {
                f.required().object_id();
            });
            m.relation("foreign", |r| {
                r.required().object("OneOptionalForeign").fields(vec!["foreignId"]).references(vec!["id"]);
            });
        });
        g.model("OneOptionalForeign", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.relation("local", |r| {
                r.optional().object("OneRequiredLocal").fields(vec!["id"]).references(vec!["foreignId"]);
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
    let res = request(&app, "one-required-locals", "Create", json!({
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

#[test]
#[serial]
async fn create_with_nested_create_many_errors() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "createMany": [{}, {}]
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 400, json!({
        "error": {
            "type": {"equals": "InvalidInput"},
            "message": {"equals": "Invalid value found in input values."},
            "errors": {
                "foreign": {"equals": "Single relationship cannot create many."}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn create_with_nested_connect() {
    let app = test::init_service(app().await).await;
    let foreign_id = request_get(&app, "one-required-locals", "Create", json!({
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
    let res = request(&app, "one-required-locals", "Create", json!({
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
    let res = request(&app, "one-required-locals", "Create", json!({
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
    let foreign_id = request_get(&app, "one-required-locals", "Create", json!({
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
    let res = request(&app, "one-required-locals", "Create", json!({
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
    let id = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
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

#[test]
#[serial]
async fn update_with_nested_create_many_errors() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
        "include": {
            "foreign": true
        }
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {
            "id": id
        },
        "update": {
            "foreign": {
                "createMany": [{}, {}]
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 400, json!({
        "error": {
            "type": {"equals": "InvalidInput"},
            "message": {"equals": "Invalid value found in input values."},
            "errors": {
                "foreign": {"equals": "Single relationship cannot create many."}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_connect() {
    let app = test::init_service(app().await).await;
    let id = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "one-optional-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
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
    let id = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
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
    let id = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "one-optional-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
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
    let id = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        },
    }), 200, "data.id").await;
    let id = id.as_str().unwrap();
    let foreign_id = request_get(&app, "one-optional-foreigns", "Create", json!({
        "create": {},
    }), 200, "data.id").await;
    let foreign_id = foreign_id.as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
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
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreign": {
                "disconnect": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 400, json!({
        "error": {
            "type": {"equals": "InvalidInput"},
            "message": {"equals": "Invalid value found in input values."},
            "errors": {
                "foreign": {"equals": "Required relation cannot disconnect."}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_update() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreign": {
                "update": {
                    "where": {"id": foreign_id},
                    "update": {}
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
                "id": {"equals": foreign_id}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_update_many_errors() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreign": {
                "updateMany": {
                    "where": {"id": foreign_id},
                    "update": {}
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 400, json!({
        "error": {
            "type": {"equals": "InvalidInput"},
            "message": {"equals": "Invalid value found in input values."},
            "errors": {
                "foreign": {"equals": "Single relationship cannot update many."}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_upsert_actually_create() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreign": {
                "upsert": {
                    "where": {"id": "123456789009876543211234"},
                    "update": {},
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
            "foreignId": {"is": "$foreignId"},
            "foreign": {
                "id": {"is": "$foreignId"}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_upsert_actually_update() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreign": {
                "upsert": {
                    "where": {"id": foreign_id},
                    "update": {},
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
                "id": {"equals": foreign_id}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_delete() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreign": {
                "delete": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 400, json!({
        "error": {
            "type": {"equals": "InvalidInput"},
            "message": {"equals": "Invalid value found in input values."},
            "errors": {
                "foreign": {"equals": "Required relation cannot delete."}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn update_with_nested_delete_many_errors() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "Update", json!({
        "where": {"id": id},
        "update": {
            "foreign": {
                "deleteMany": {
                    "id": foreign_id
                }
            }
        },
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 400, json!({
        "error": {
            "type": {"equals": "InvalidInput"},
            "message": {"equals": "Invalid value found in input values."},
            "errors": {
                "foreign": {"equals": "Single relationship cannot delete many."}
            }
        }
    })).await;
}

#[test]
#[serial]
async fn include() {
    let app = test::init_service(app().await).await;
    let ids = request_get(&app, "one-required-locals", "Create", json!({
        "create": {
            "foreign": {
                "create": {}
            }
        }
    }), 200, vec!["data.id", "data.foreignId"]).await;
    let id = ids.as_array().unwrap().get(0).unwrap().as_str().unwrap();
    let foreign_id = ids.as_array().unwrap().get(1).unwrap().as_str().unwrap();
    let res = request(&app, "one-required-locals", "FindUnique", json!({
        "where": {"id": id},
        "include": {
            "foreign": true
        }
    })).await;
    assert_json_response(res, 200, json!({
        "data": {
            "id": {"equals": id},
            "foreignId": {"equals": foreign_id},
            "foreign": {
                "id": {"equals": foreign_id}
            }
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