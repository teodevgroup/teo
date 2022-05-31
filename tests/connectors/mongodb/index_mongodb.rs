use bson::{doc, Document};
use futures_util::StreamExt;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use tokio::test;
use teo::connectors::mongodb::MongoDBConnectorHelpers;
use teo::core::graph::Graph;
use teo::core::value::Value;
use teo::error::ActionError;


async fn make_client_options() -> ClientOptions {
    ClientOptions::parse("mongodb://localhost:27017/teotestindex").await.unwrap()
}

async fn make_mongodb_graph() -> &'static Graph {

    let options = make_client_options().await;

    let graph = Box::leak(Box::new(Graph::new(|g| {

        g.mongodb(options.clone());

        g.reset_database();

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

    graph
}

#[test]
async fn unique_value_cannot_have_duplications_on_create() {
    let graph = make_mongodb_graph().await;
    let object1 = graph.new_object("UniqueIndex");
    let _ = object1.set_value("unique", Value::String("123".to_string()));
    let _ = object1.save().await;
    let object2 = graph.new_object("UniqueIndex");
    let _ = object2.set_value("unique", Value::String("123".to_string()));
    let result = object2.save().await;
    assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("unique"));
}

#[test]
async fn unique_sparse_value_cannot_have_duplications_on_create() {
    let graph = make_mongodb_graph().await;
    let object1 = graph.new_object("UniqueSparseIndex");
    let _ = object1.set_value("uniqueSparse", Value::String("123".to_string()));
    let _ = object1.save().await;
    let object2 = graph.new_object("UniqueSparseIndex");
    let _ = object2.set_value("uniqueSparse", Value::String("123".to_string()));
    let result = object2.save().await;
    assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("uniqueSparse"));
}

#[test]
async fn unique_value_cannot_have_duplications_on_update() {
    let graph = make_mongodb_graph().await;
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
async fn unique_sparse_value_cannot_have_duplications_on_update() {
    let graph = make_mongodb_graph().await;
    let object1 = graph.new_object("UniqueSparseIndex");
    let _ = object1.set_value("uniqueSparse", Value::String("123".to_string()));
    let _ = object1.save().await;
    let object2 = graph.new_object("UniqueSparseIndex");
    let _ = object2.set_value("uniqueSparse", Value::String("222".to_string()));
    let _ = object2.save().await;
    let _ = object2.set_value("uniqueSparse", Value::String("123".to_string()));
    let result = object2.save().await;
    assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("uniqueSparse"));
}

#[test]
async fn unique_sparse_value_can_have_duplicated_nulls() {
    let graph = make_mongodb_graph().await;
    let object1 = graph.new_object("UniqueSparseIndex");
    let _ = object1.save().await;
    let object2 = graph.new_object("UniqueSparseIndex");
    let result = object2.save().await;
    assert_eq!(result.ok(), None);
}

#[test]
async fn index_field_is_indexed() {
    let _graph = make_mongodb_graph().await;
    let options = make_client_options().await;
    let client = Client::with_options(options).unwrap();
    let database = client.default_database().unwrap();
    let collection: Collection<Document> = database.collection("indices");
    let mut cursor = collection.list_indexes(None).await.unwrap();
    while let Some(Ok(index)) = cursor.next().await {
        if index.keys == doc!{"_id": 1} {
            continue
        } else {
            assert_eq!(index.keys, doc!{"index": 1});
            assert_eq!(index.clone().options.unwrap().unique, None);
            assert_eq!(index.clone().options.unwrap().sparse.unwrap(), false);
        }
    }
}

#[test]
async fn sparse_index_field_is_sparse_indexed() {
    let _graph = make_mongodb_graph().await;
    let options = make_client_options().await;
    let client = Client::with_options(options).unwrap();
    let database = client.default_database().unwrap();
    let collection: Collection<Document> = database.collection("sparseindices");
    let mut cursor = collection.list_indexes(None).await.unwrap();
    while let Some(Ok(index)) = cursor.next().await {
        if index.keys == doc!{"_id": 1} {
            continue
        } else {
            assert_eq!(index.keys, doc!{"indexSparse": 1});
            assert_eq!(index.clone().options.unwrap().unique, None);
            assert_eq!(index.clone().options.unwrap().sparse.unwrap(), true);
        }
    }
}
