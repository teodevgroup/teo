use mongodb::options::ClientOptions;
use serde_json::{json};
use tokio::test;
use teo::connectors::mongodb::MongoDBConnectorHelpers;
use teo::core::graph::Graph;
use teo::core::value::Value;
use teo::error::ActionError;


async fn make_mongodb_graph() -> &'static Graph {

    let options = ClientOptions::parse("mongodb://localhost:27017/teotestindex").await.unwrap();

    let graph = Box::leak(Box::new(Graph::new(|g| {
        g.mongodb(options.clone());

        g.model("UniqueIndex", |m| {
            m.field("unique", |f| {
                f.unique().required().string();
            });
        });

        g.model("UniqueSparseIndex", |m| {
            m.field("uniqueSparse", |f| {
                f.unique().optional().string();
            });
        });

        g.model("Index", |m| {
            m.field("index", |f| {
                f.index().required().string();
            })
        });

        g.model("SparseIndex", |m| {
            m.field("indexSparse", |f| {
                f.index().optional().string();
            })
        });
    }).await));

    graph.drop_database().await;

    graph
}

#[test]
async fn unique_value_cannot_have_duplications() {
    let graph = make_mongodb_graph().await;
    let object1 = graph.new_object("UniqueIndex");
    object1.set_value("unique", Value::String("123".to_string()));
    object1.save().await;
    let object2 = graph.new_object("UniqueIndex");
    object2.set_value("unique", Value::String("123".to_string()));
    let result = object2.save().await;
    assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("unique"));
}

#[test]
async fn unique_sparse_value_cannot_have_duplications() {
    let graph = make_mongodb_graph().await;
    let object1 = graph.new_object("UniqueSparseIndex");
    object1.set_value("uniqueSparse", Value::String("123".to_string()));
    object1.save().await;
    let object2 = graph.new_object("UniqueSparseIndex");
    object2.set_value("uniqueSparse", Value::String("123".to_string()));
    let result = object2.save().await;
    assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("uniqueSparse"));
}
