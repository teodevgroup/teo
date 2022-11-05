pub mod builder;
pub(crate) mod serve;
pub(crate) mod generate;
pub(crate) mod command;

use crate::core::app::command::Command;
use crate::core::app::generate::generate;
use crate::core::app::serve::serve;
use crate::core::conf::Conf;
use crate::core::graph::Graph;

pub struct App {
    graph: Graph,
    conf: Conf,
}

impl App {
    pub async fn run(&self) -> Result<(), std::io::Error> {
        let command = match std::env::args().nth(1) {
            Some(result) => result.into(),
            None => Command::Serve
        };
        match command {
            Command::Serve => {
                serve(self.graph.clone(), self.conf.clone()).await?
            }
            Command::Client => {
                generate(&self.graph, &self.conf).await?
            }
        }
        Ok(())
    }

}
