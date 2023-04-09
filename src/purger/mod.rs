use crate::prelude::Graph;
use crate::core::result::Result;

pub(crate) async fn purge(graph: &Graph) -> Result<()> {
    graph.connector().purge(graph).await
}
