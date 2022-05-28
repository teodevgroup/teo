use mongodb::options::ClientOptions;
use tokio::test;
use teo::connectors::mongodb::MongoDBConnectorHelpers;
use teo::core::graph::Graph;


async fn make_graph() -> &'static Graph {

    let options = ClientOptions::parse("mongodb://localhost:27017/teotestserver").await.unwrap();

    let graph = Box::leak(Box::new(Graph::new(|g| {
        g.mongodb(options.clone());

        g.model("User", |m| {
            m.field("id", |f| {
                f.required().primary().readonly().object_id().assigned_by_database();
            });
            m.field("name", |f| {
                f.required().string().default("Bson");
            });
            m.field("age", |f| {
                f.required().u8().default(18u8);
            });
        });
    }).await));

    graph
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let graph = make_graph().await;
    graph.start_server(5000).await
}
