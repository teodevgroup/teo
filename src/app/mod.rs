use crate::app::app::App;
use crate::app::command::Command;
use crate::app::generate::generate_clients;
use crate::app::serve::serve;
use crate::core::graph::Graph;

pub mod app;
pub mod command;
pub mod serve;
pub mod generate;

pub async fn run(graph: Graph, app: App) -> Result<(), std::io::Error> {
    let command = match std::env::args().nth(1) {
        Some(result) => result.into(),
        None => Command::Serve
    };
    match command {
        Command::Serve => {
            serve(graph.clone(), app.server.clone()).await?
        }
        Command::Client => {
            generate_clients(graph.clone(), app.client.clone()).await?
        }
    }
    Ok(())
}

