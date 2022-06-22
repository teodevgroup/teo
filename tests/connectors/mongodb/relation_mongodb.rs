use actix_http::body::{BoxBody, MessageBody};
use actix_http::Request;
use bson::{doc, Document};
use futures_util::StreamExt;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use serial_test::serial;
use teo::core::graph::Graph;
use teo::core::value::Value;
use teo::error::ActionError;
use actix_web::{test, web, App, error::Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use regex::Regex;
use serde_json::{json, Number, Value as JsonValue};
use teo::server::server::Server;
use crate::helpers::is_object_id;


async fn make_mongodb_graph() -> &'static Graph {
    let graph = Box::leak(Box::new(Graph::new(|g| {
        g.data_source().mongodb("mongodb://localhost:27017/teotestintegration");
        g.reset_database();
        g.r#enum("Sex", |e| {
            e.localized_name("性别");
            e.description("性别，多用于用户和管理员。");
            e.choice("MALE", |c| {
                c.localized_name("男");
            });
            e.choice("FEMALE", |c| {
                c.localized_name("女");
            });
        });

        g.model("Author", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("name", |f| {
                f.required().string();
            });
            m.relation("articles", |r| {
                r.vec("Article").fields(vec!["id"]).references(vec!["authorId"]);
            });
        });
        g.model("Article", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("title", |f| {
                f.required().string();
            });
            m.field("authorId", |f| {
                f.required().object_id();
            });
            m.relation("author", |r| {
                r.object("Author").fields(vec!["authorId"]).references(vec!["id"]);
            });
            m.relation("categoriesOnArticles", |r| {
                r.vec("CategoriesOnArticles").fields(vec!["id"]).references(vec!["articleId"]);
            });
            m.relation("categories", |r| {
                r.vec("Category").through("CategoriesOnArticles").local("category").foreign("article");
            });
        });
        g.model("Category", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("name", |f| {
                f.unique().required().string();
            });
            m.relation("categoriesOnArticles", |r| {
                r.vec("CategoriesOnArticles").fields(vec!["id"]).references(vec!["categoryId"]);
            });
            m.relation("articles", |r| {
                r.vec("Article").through("CategoriesOnArticles").local("category").foreign("article");
            });
        });
        g.model("CategoriesOnArticles", |m| {
            m.field("articleId", |f| {
                f.required().write_on_create().object_id();
            });
            m.relation("article", |r| {
                r.object("Article").fields(vec!["articleId"]).references(vec!["id"]);
            });
            m.field("categoryId", |f| {
                f.required().write_on_create().object_id();
            });
            m.relation("category", |r| {
                r.object("Category").fields(vec!["categoryId"]).references(vec!["id"]);
            });
            m.primary(vec!["articleId", "categoryId"]);
        });
        g.host_url("http://www.example.com");
    }).await));
    graph
}

async fn make_app() -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = Error,
>> {
    let graph = make_mongodb_graph().await;
    let server = Box::leak(Box::new(Server::new(graph)));
    server.make_app()
}

#[test]
#[serial]
async fn create_with_relation_create() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "create": {
                    "title": "A Great Developer"
                }
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_create_many_implicitly() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "create": [{
                    "title": "A Great Developer"
                }, {
                    "title": "I Found It"
                }]
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_create_many() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "createMany": [{
                    "title": "A Great Developer"
                }, {
                    "title": "I Found It"
                }]
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_set() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "set": {
                    "id": "xxx"
                }
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_set_many_implicitly() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "set": [{
                    "id": "xxx"
                }, {
                    "id": "xxx"
                }]
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_connect() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "connect": {
                    "id": "xxx"
                }
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_connect_many_implicitly() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "connect": [{
                    "id": "xxx"
                }, {
                    "id": "xxx"
                }]
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_connect_or_create() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "connect": {
                    "id": "xxx"
                }
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}

#[test]
#[serial]
async fn create_with_relation_connect_or_create_many_implicitly() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/authors/action").set_json(json!({
        "action": "Create",
        "create": {
            "name": "John Peterson",
            "articles": {
                "connect": [{
                    "id": "xxx"
                }, {
                    "id": "xxx"
                }]
            }
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    assert_eq!(body_obj.get("errors"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("name").unwrap(), &JsonValue::String("John Peterson".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert!(is_object_id(id_str))
}
