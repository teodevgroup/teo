use actix_http::body::BoxBody;


use serial_test::serial;
use actix_web::{test, App, error::Error};
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
                f.optional().f64();
            });
        });
    }).await;
    make_app(graph, ServerConfiguration::default())
}

#[test]
#[serial]
async fn aggregate_returns_null_for_fields_if_no_record() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "people", "aggregate", tson!({
        "_sum": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
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
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 22,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_sum": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
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
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_avg": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
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
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_min": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
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
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_max": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
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
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": 1
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_min": {
            "age": true
        },
        "_max": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
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

#[test]
#[serial]
async fn aggregate_can_count_for_each_number_field() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": null
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_count": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_count": {
                "age": {"equals": 2},
                "salary": {"equals": 1}
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_can_count_all() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": 5000
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": null
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_count": {
            "age": true,
            "salary": true,
            "_all": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_count": {
                "age": {"equals": 2},
                "salary": {"equals": 1},
                "_all": {"equals": 2}
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_records_avg_is_null() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "people", "aggregate", tson!({
        "_avg": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_avg": {
                "age": {"is": "null"},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_records_sum_is_null() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "people", "aggregate", tson!({
        "_sum": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_sum": {
                "age": {"is": "null"},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_records_min_is_null() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "people", "aggregate", tson!({
        "_min": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_min": {
                "age": {"is": "null"},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_records_max_is_null() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "people", "aggregate", tson!({
        "_max": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_max": {
                "age": {"is": "null"},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_records_count_is_zero() {
    let app = test::init_service(app().await).await;
    let res = request(&app, "people", "aggregate", tson!({
        "_count": {
            "age": true,
            "salary": true,
            "_all": true
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_count": {
                "age": {"equals": 0},
                "salary": {"equals": 0},
                "_all": {"equals": 0},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_value_avg_is_null() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": null
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": null
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_avg": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_avg": {
                "age": {"equals": 20.5},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_value_sum_is_null() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": null
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": null
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_sum": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_sum": {
                "age": {"equals": 41},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_value_min_is_null() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": null
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": null
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_min": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_min": {
                "age": {"equals": 20},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_value_max_is_null() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": null
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": null
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_max": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_max": {
                "age": {"equals": 21},
                "salary": {"is": "null"},
            },
        }
    })).await;
}

#[test]
#[serial]
async fn aggregate_when_no_value_count_is_zero() {
    let app = test::init_service(app().await).await;
    let _id1 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "scalar",
            "age": 20,
            "salary": null
        },
    }), 200, "data.id").await;
    let _id2 = request_get(&app, "people", "create", tson!({
        "create": {
            "name": "jobs",
            "age": 21,
            "salary": null
        },
    }), 200, "data.id").await;
    let res = request(&app, "people", "aggregate", tson!({
        "_count": {
            "age": true,
            "salary": true,
        },
    })).await;
    assert_json_response(res, 200, tson!({
        "data": {
            "_count": {
                "age": {"equals": 2},
                "salary": {"equals": 0},
            },
        }
    })).await;
}
