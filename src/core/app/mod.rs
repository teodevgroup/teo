pub mod builder;
pub(crate) mod serve;
pub(crate) mod generate;
pub(crate) mod command;
pub mod environment;
pub mod entrance;

use std::sync::Arc;
use crate::core::app::command::{CLI, CLICommand};
use crate::core::app::entrance::Entrance;
use crate::core::app::environment::EnvironmentVersion;
use crate::core::app::generate::generate;
use crate::core::app::serve::serve;
use crate::core::conf::Conf;
use crate::core::graph::Graph;

pub struct App {
    graph: Graph,
    conf: Conf,
    environment_version: EnvironmentVersion,
    entrance: Entrance,
    args: Arc<CLI>,
}

impl App {
    pub async fn run(&self) -> Result<(), std::io::Error> {
        match &self.args.command {
            CLICommand::Serve(_) => {
                serve(self.graph.clone(), self.conf.clone(), self.environment_version.clone(), self.entrance.clone()).await?
            }
            CLICommand::Generate(_) => {
                generate(&self.graph, &self.conf).await?
            }
            _ => {

            }
        }
        Ok(())
    }

}
