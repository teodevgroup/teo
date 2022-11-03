pub mod builder;
pub(crate) mod serve;
pub(crate) mod command;

use crate::core::app::command::Command;
use crate::core::conf::Conf;
use crate::core::graph::Graph;

pub struct App {
    graph: Graph,
    conf: Conf,
}

impl App {
    pub async fn run() -> Result<(), std::io::Error> {
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

}
