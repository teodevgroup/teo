pub mod builder;
pub mod environment;
pub mod entrance;
pub(crate) mod conf;
pub(crate) mod serve;
pub(crate) mod command;
pub(crate) mod migrate;

use std::sync::Arc;
use to_mut::ToMut;
use crate::core::app::builder::AsyncCallbackWithoutArgs;
use crate::core::app::command::{CLI, CLICommand, GenerateCommand, SeedCommandAction};
use crate::core::app::conf::{EntityGeneratorConf, ServerConf};
use crate::core::app::entrance::Entrance;
use crate::core::app::environment::EnvironmentVersion;
use crate::core::app::migrate::migrate;
use crate::core::app::serve::serve;
use crate::core::graph::Graph;
use crate::gen::interface::client::conf::Conf as ClientConf;
use crate::gen::interface::client::gen::gen as gen_client;
use crate::gen::interface::server::gen as gen_entity;
use crate::seeder::data_set::DataSet;
use crate::seeder::seed::seed;

pub struct App {
    graph: &'static Graph,
    server_conf: &'static ServerConf,
    entity_generator_confs: Vec<EntityGeneratorConf>,
    client_generator_confs: Vec<ClientConf>,
    environment_version: EnvironmentVersion,
    entrance: Entrance,
    args: Arc<CLI>,
    before_server_start: Option<Arc<dyn AsyncCallbackWithoutArgs>>,
    datasets: Vec<DataSet>,
}

impl App {

    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        match &self.args.command {
            CLICommand::Serve(serve_command) => {
                if !serve_command.no_migration {
                    migrate(self.graph.to_mut(), false).await;
                }
                if !serve_command.no_autoseed {
                    let names = self.datasets.iter().filter_map(|d| if d.autoseed { Some(d.name.clone()) } else { None }).collect();
                    seed(SeedCommandAction::Seed, self.graph, &self.datasets, names).await;
                }
                serve(
                    self.graph,
                    self.server_conf,
                    self.environment_version.clone(),
                    self.entrance.clone(),
                    self.before_server_start.clone(),
                ).await?
            }
            CLICommand::Generate(cmd) => {
                match cmd {
                    GenerateCommand::GenerateEntityCommand(entity_command) => {
                        match self.entity_generator_confs.len() {
                            0 => println!("Cannot find an entity generator declaration."),
                            1 => {
                                let conf = self.entity_generator_confs.get(0).unwrap();
                                gen_entity(&self.graph, conf).await?;
                            },
                            _ => {
                                let mut names = entity_command.names.clone().unwrap_or(vec![]);
                                if entity_command.all {
                                    names = self.entity_generator_confs.iter().map(|c| c.name.clone().unwrap()).collect();
                                }
                                for name in names.iter() {
                                    let conf = self.entity_generator_confs.iter().find(|c| c.name.as_ref().unwrap() == name).unwrap();
                                    gen_entity(&self.graph, conf).await?;
                                }
                            }
                        }
                    }
                    GenerateCommand::GenerateClientCommand(client_command) => {
                        match self.client_generator_confs.len() {
                            0 => println!("Cannot find a client generator declaration."),
                            1 => {
                                let conf = self.client_generator_confs.get(0).unwrap();
                                gen_client(&self.graph, conf).await?;
                            },
                            _ => {
                                let mut names = client_command.names.clone().unwrap_or(vec![]);
                                if client_command.all {
                                    names = self.client_generator_confs.iter().map(|c| c.name.clone().unwrap()).collect();
                                }
                                for name in names.iter() {
                                    let conf = self.client_generator_confs.iter().find(|c| c.name.as_ref().unwrap() == name).unwrap();
                                    gen_client(&self.graph, conf).await?;
                                }
                            }
                        }
                    }
                }
            }
            CLICommand::Migrate(migrate_command) => {
                migrate(self.graph.to_mut(), migrate_command.dry).await;
            }
            CLICommand::Seed(seed_command) => {
                let names = if seed_command.all {
                    self.datasets.iter().map(|d| d.name.clone()).collect()
                } else {
                    seed_command.names.clone().unwrap()
                };
                migrate(self.graph.to_mut(), false).await;
                seed(seed_command.action, self.graph(), &self.datasets, names).await
            }
        }
        Ok(())
    }

}
