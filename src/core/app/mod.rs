pub mod builder;
pub(crate) mod serve;
pub(crate) mod command;
pub mod environment;
pub mod entrance;
pub(crate) mod conf;

use std::sync::Arc;
use crate::core::app::command::{CLI, CLICommand, GenerateCommand};
use crate::core::app::conf::{ClientGeneratorConf, EntityGeneratorConf, ServerConf};
use crate::core::app::entrance::Entrance;
use crate::core::app::environment::EnvironmentVersion;
use crate::core::app::serve::serve;
use crate::core::graph::Graph;
use crate::generator::client::generate_client;
use crate::generator::server::generate_entity;


pub struct App {
    graph: Graph,
    server_conf: ServerConf,
    entity_generator_confs: Vec<EntityGeneratorConf>,
    client_generator_confs: Vec<ClientGeneratorConf>,
    environment_version: EnvironmentVersion,
    entrance: Entrance,
    args: Arc<CLI>,
}

impl App {
    pub async fn run(&self) -> Result<(), std::io::Error> {
        match &self.args.command {
            CLICommand::Serve(_) => {
                serve(self.graph.clone(), self.server_conf.clone(), self.environment_version.clone(), self.entrance.clone()).await?
            }
            CLICommand::Generate(cmd) => {
                match cmd {
                    GenerateCommand::GenerateEntityCommand(entity_command) => {
                        match self.entity_generator_confs.len() {
                            0 => println!("Cannot find an entity generator declaration."),
                            1 => {
                                let conf = self.entity_generator_confs.get(0).unwrap();
                                generate_entity(conf.provider, &conf.dest).await?;
                            },
                            _ => {
                                let mut names = entity_command.names.clone().unwrap_or(vec![]);
                                if entity_command.all {
                                    names = self.entity_generator_confs.iter().map(|c| c.name.clone().unwrap()).collect();
                                }
                                for name in names.iter() {
                                    let conf = self.entity_generator_confs.iter().find(|c| c.name.as_ref().unwrap() == name).unwrap();
                                    generate_entity(conf.provider, &conf.dest).await?;
                                }
                            }
                        }
                    }
                    GenerateCommand::GenerateClientCommand(client_command) => {
                        match self.client_generator_confs.len() {
                            0 => println!("Cannot find a client generator declaration."),
                            1 => {
                                let conf = self.client_generator_confs.get(0).unwrap();
                                generate_client(&self.graph, conf).await?;
                            },
                            _ => {
                                let mut names = client_command.names.clone().unwrap_or(vec![]);
                                if client_command.all {
                                    names = self.client_generator_confs.iter().map(|c| c.name.clone().unwrap()).collect();
                                }
                                for name in names.iter() {
                                    let conf = self.client_generator_confs.iter().find(|c| c.name.as_ref().unwrap() == name).unwrap();
                                    generate_client(&self.graph, conf).await?;
                                }
                            }
                        }
                    }
                }
            }
            _ => {

            }
        }
        Ok(())
    }

}
