use std::panic::AssertUnwindSafe;
use tokio::test;
use futures::FutureExt;
use teo::core::graph::Graph;


#[test]
async fn graph_without_connector_would_panic() {
    let result = AssertUnwindSafe(async {
        Graph::new(|g| {
            g.model("Model", |m| {
                m.field("id", |f| { f.string(); })
            })
        }).await
    }).catch_unwind().await;
    assert!(result.is_err())
}
