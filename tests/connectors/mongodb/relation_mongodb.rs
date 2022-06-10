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
use serde_json::ser::Compound::Map;
use teo::server::server::Server;
use crate::helpers::is_object_id;


async fn make_mongodb_graph() -> &'static Graph {
    let graph = Box::leak(Box::new(Graph::new(|g| {
        g.data_source().mongodb("mongodb://localhost:27017/teotestintegration");
        g.reset_database();
        g.r#enum("Sex", vec!["MALE", "FEMALE"]);
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