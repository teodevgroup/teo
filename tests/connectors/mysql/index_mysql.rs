use tokio::test;
use teo::connectors::mysql::MySQLConnectorHelpers;
use teo::core::graph::Graph;
use teo::core::value::Value;
use teo::error::ActionError;


// async fn make_mysql_pool() -> Pool<MySql> {
//     let pool = MySqlPoolOptions::new().max_connections(5).connect("mysql://root:@localhost").await.unwrap();
//     sqlx::query("create database teotestindex").execute(&pool).await;
//     pool
// }

async fn make_mysql_graph() -> &'static Graph {

    let graph = Box::leak(Box::new(Graph::new(|g| {

        g.mysql("mysql://root:@localhost/teotestindex");

        g.model("UniqueIndex", |m| {
            m.field("id", |f| {
                f.primary().readonly().required().i64().auto_increment();
            });
            m.field("unique", |f| {
                f.unique().optional().string();
            });
        });

        g.model("Index", |m| {
            m.field("id", |f| {
                f.primary().readonly().required().i64().auto_increment();
            });
            m.field("index", |f| {
                f.index().optional().string();
            })
        });

    }).await));

    graph
}

#[test]
async fn unique_value_cannot_have_duplications_on_create() {
    let graph = make_mysql_graph().await;
    let object1 = graph.new_object("UniqueIndex");
    let _ = object1.set_value("unique", Value::String("123".to_string()));
    let _ = object1.save().await;
    let object2 = graph.new_object("UniqueIndex");
    let _ = object2.set_value("unique", Value::String("123".to_string()));
    let result = object2.save().await;
    assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("unique"));
}

#[test]
async fn unique_value_cannot_have_duplications_on_update() {
    let graph = make_mysql_graph().await;
    let object1 = graph.new_object("UniqueIndex");
    let _ = object1.set_value("unique", Value::String("123".to_string()));
    let _ = object1.save().await;
    let object2 = graph.new_object("UniqueIndex");
    let _ = object2.set_value("unique", Value::String("222".to_string()));
    let _ = object2.save().await;
    let _ = object2.set_value("unique", Value::String("123".to_string()));
    let result = object2.save().await;
    assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("unique"));
}

#[test]
async fn unique_value_can_have_duplicated_nulls() {
    let graph = make_mysql_graph().await;
    let object1 = graph.new_object("UniqueIndex");
    let _ = object1.save().await;
    let object2 = graph.new_object("UniqueIndex");
    let result = object2.save().await;
    assert_eq!(result.ok(), None);
}

#[test]
async fn index_field_is_indexed() {
    //let graph = make_mysql_graph().await;
}
