use crate::prelude::{Graph};

pub(crate) async fn migrate(graph: &mut Graph, _dry_run: bool) {
    let result = graph.connector_mut().migrate(graph.models(), false).await;
    if result.is_err() {
        panic!("Migration error");
    }
}
