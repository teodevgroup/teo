pub mod builder;
pub mod environment;
pub mod entrance;
pub(crate) mod conf;
pub(crate) mod command;
pub(crate) mod migrate;
pub mod new_app;

use std::sync::Arc;
use to_mut::ToMut;
use crate::app::builder::AsyncCallbackWithoutArgs;
use crate::app::command::{CLI, CLICommand, GenerateCommand, SeedCommandAction};
use crate::app::conf::{DebugConf, EntityGeneratorConf, ServerConf, TestConf};
use crate::app::entrance::Entrance;
use crate::app::environment::EnvironmentVersion;
use crate::app::migrate::migrate;
use crate::serve::serve;
use crate::serve::test_context::{ResetMode, TestContext};
use crate::core::graph::Graph;
use crate::gen::interface::client::conf::Conf as ClientConf;
use crate::gen::interface::client::gen::gen as gen_client;
use crate::gen::interface::server::gen as gen_entity;
use crate::prelude::Value;
use crate::purger::purge;
use crate::seeder::data_set::DataSet;
use crate::seeder::seed::seed;

pub struct App {
    graph: &'static Graph,
    server_conf: &'static ServerConf,
    debug_conf: Option<&'static DebugConf>,
    test_conf: Option<&'static TestConf>,
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
                let env = serve_command.env.as_ref().cloned().unwrap_or(std::env::var("TEO_ENV").unwrap_or("debug".to_string()));
                let test_context: Option<&'static TestContext> = if env.as_str() == "test" {
                    if let Some(test_conf) = self.test_conf {
                        match &test_conf.reset_after_find {
                            Value::Null => None,
                            Value::RawEnumChoice(s, _) => {
                                if s.as_str() == "auto" {
                                    Some(Box::leak(Box::new(TestContext {
                                        reset_mode: ResetMode::AfterQuery,
                                        datasets: self.datasets.iter().filter(|d| d.autoseed == true).map(|d| d.clone()).collect(),
                                    })))
                                } else { None }
                            },
                            Value::String(s) => {
                                Some(Box::leak(Box::new(TestContext {
                                    reset_mode: ResetMode::AfterQuery,
                                    datasets: self.datasets.iter().filter(|d| &d.name == s).map(|d| d.clone()).collect(),
                                })))
                            },
                            Value::Vec(v) => {
                                let sv: Vec<String> = v.iter().map(|v| v.as_str().unwrap().to_owned()).collect();
                                Some(Box::leak(Box::new(TestContext {
                                    reset_mode: ResetMode::AfterQuery,
                                    datasets: self.datasets.iter().filter(|d| sv.contains(&d.name)).map(|d| d.clone()).collect(),
                                })))
                            }
                            _ => unreachable!()
                        }
                    } else { None }
                } else { None };
                if let Some(test_context) = test_context {
                    self.graph.connector().purge(self.graph).await.unwrap();
                    seed(SeedCommandAction::Seed, self.graph, &test_context.datasets, test_context.datasets.iter().map(|d| d.name.clone()).collect()).await;
                } else if !serve_command.no_autoseed && !self.datasets.is_empty() {
                    let names: Vec<String> = self.datasets.iter().filter_map(|d| if d.autoseed { Some(d.name.clone()) } else { None }).collect();
                    if !names.is_empty() {
                        seed(SeedCommandAction::Seed, self.graph, &self.datasets, names).await;
                    }
                }
                serve(
                    self.graph,
                    self.server_conf,
                    self.environment_version.clone(),
                    self.entrance.clone(),
                    self.before_server_start.clone(),
                    test_context,
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
            CLICommand::Purge(_) => {
                purge(self.graph).await.unwrap()
            }
        }
        Ok(())
    }

}
