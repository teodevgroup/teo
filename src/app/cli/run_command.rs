use crate::app::cli::command::{CLI, CLICommand, SeedCommandAction};
use crate::app::new_app::ctx::AppCtx;
use crate::migrate::migrate;
use crate::app::new_app::new_result::Result;
use crate::core::conf::test::ResetMode;
use crate::server::test_context::TestContext;
use crate::core::teon::Value;
use crate::seeder::seed::seed;
use crate::server::serve;

pub(crate) async fn run_command(cli: CLI) -> Result<()> {
    let app_ctx = AppCtx::get_mut()?;
    let graph = app_ctx.graph()?;
    let datasets = app_ctx.datasets();
    match cli.command {
        CLICommand::Serve(serve_command) => {
            if !serve_command.no_migration {
                migrate(app_ctx.graph()?, false).await;
            }
            let env = serve_command.env.as_ref().cloned().unwrap_or(std::env::var("TEO_ENV").unwrap_or("debug".to_string()));
            let test_context: Option<&'static TestContext> = if env.as_str() == "test" {
                if let Some(test_conf) = app_ctx.test_conf() {
                    match &test_conf.reset_after_find {
                        Value::Null => None,
                        Value::RawEnumChoice(s, _) => {
                            if s.as_str() == "auto" {
                                Some(Box::leak(Box::new(TestContext {
                                    reset_mode: ResetMode::AfterQuery,
                                    datasets: app_ctx.datasets().iter().filter(|d| d.autoseed == true).map(|d| d.clone()).collect(),
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
                app_ctx.connector()?.purge(graph).await.unwrap();
                seed(SeedCommandAction::Seed, graph, &test_context.datasets, test_context.datasets.iter().map(|d| d.name.clone()).collect()).await;
            } else if !serve_command.no_autoseed && !datasets.is_empty() {
                let names: Vec<String> = datasets.iter().filter_map(|d| if d.autoseed { Some(d.name.clone()) } else { None }).collect();
                if !names.is_empty() {
                    seed(SeedCommandAction::Seed, graph, datasets, names).await;
                }
            }
            serve(
                graph,
                app_ctx.server_conf()?,
                app_ctx.program(),
                app_ctx.entrance(),
                app_ctx.setup(),
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
            migrate(self.graph, false).await;
            seed(seed_command.action, self.graph(), &self.datasets, names).await
        }
        CLICommand::Purge(_) => {
            purge(self.graph).await.unwrap()
        }
    }
    Ok(())
}