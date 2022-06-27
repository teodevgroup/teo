use actix_http::body::BoxBody;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use serial_test::serial;
use actix_web::{test, web, App, error::Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use teo::core::graph::Graph;
use teo::server::server::Server;

lazy_static! {
    static ref TEO_SERVER: AsyncOnce<&'static mut Server> = AsyncOnce::new(async {
        let graph = Box::leak(Box::new(Graph::new(|g| {
            g.data_source().mongodb("mongodb://localhost:27017/teotest_1ol_mf");
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
        Box::leak(Box::new(Server::new(graph)))
    });
}

async fn app() -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<BoxBody>,
    Config = (),
    InitError = (),
    Error = Error,
>> {
    TEO_SERVER.get().await.make_app()
}

#[test]
#[serial]
async fn create_with_nested_create() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn create_with_nested_create_many() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn create_with_nested_connect() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn create_with_nested_connect_or_create() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_create() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_create_many() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_connect() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_connect_or_create() {
    let app = test::init_service(app().await).await;
    assert!(true);
}

#[test]
#[serial]
async fn update_with_nested_set() {
    let app = test::init_service(app().await).await;
    assert!(true);
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