use crate::app::app::ClientConfiguration;
use crate::core::graph::Graph;

pub(crate) async fn generate_clients(_graph: Graph, _conf: ClientConfiguration) -> Result<(), std::io::Error> {
    Ok(())
}