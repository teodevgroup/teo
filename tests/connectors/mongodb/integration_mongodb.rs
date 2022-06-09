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
use serde_json::{json, Value as JsonValue};
use teo::server::server::Server;


async fn make_mongodb_graph() -> &'static Graph {
    let graph = Box::leak(Box::new(Graph::new(|g| {
        g.data_source().mongodb("mongodb://localhost:27017/teotestintegration");
        g.reset_database();
        g.model("Simple", |m| {
            m.field("id", |f| {
                f.primary().required().readonly().object_id().column_name("_id").auto();
            });
            m.field("uniqueString", |f| {
                f.unique().required().string();
            });
            m.field("requiredString", |f| {
                f.required().string();
            });
            m.field("optionalString", |f| {
                f.optional().string();
            });
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
async fn create_works() {
    let app = test::init_service(make_app().await).await;
    let req = test::TestRequest::post().uri("/simples/action").set_json(json!({
        "action": "Create",
        "create": {
            "uniqueString": "1",
            "requiredString": "1"
        }
    })).to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body_json: JsonValue = test::read_body_json(resp).await;
    let body_obj = body_json.as_object().unwrap();
    assert_eq!(body_obj.get("meta"), None);
    let body_data = body_obj.get("data").unwrap().as_object().unwrap();
    assert_eq!(body_data.get("uniqueString").unwrap(), &JsonValue::String("1".to_string()));
    assert_eq!(body_data.get("requiredString").unwrap(), &JsonValue::String("1".to_string()));
    let id_str = body_data.get("id").unwrap().as_str().unwrap();
    assert_eq!(id_str.len(), 24);
    let id_regex = Regex::new("[\\da-f]{24}").unwrap();
    assert!(id_regex.is_match(id_str))
}
