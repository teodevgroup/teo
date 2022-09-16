use crate::app::app::ClientConfiguration;
use crate::client::csharp::generate_csharp_client;
use crate::client::kotlin::generate_kotlin_client;
use crate::client::swift::generate_swift_client;
use crate::client::typescript::generate_typescript_client;
use crate::core::graph::Graph;

pub(crate) async fn generate_clients(graph: Graph, conf: ClientConfiguration) -> Result<(), std::io::Error> {
    if let Some(ref _type_script_client_conf) = conf.type_script {
        generate_typescript_client(&graph, &conf).await?;
    }
    if let Some(ref _swift_client_conf) = conf.swift {
        generate_swift_client(&graph, &conf).await?;
    }
    if let Some(ref _kotlin_client_conf) = conf.kotlin {
        generate_kotlin_client(&graph, &conf).await?;
    }
    if let Some(ref _csharp_client_conf) = conf.csharp {
        generate_csharp_client(&graph, &conf).await?;
    }
    Ok(())
}