use crate::app::cli::command::{CLI, CLICommand, GenerateCommand, SeedCommandAction};
use crate::app::ctx::AppCtx;
use crate::migrate::migrate;
use crate::app::new_app::new_result::Result;
use crate::core::conf::test::{ResetDatasets, ResetMode};
use crate::server::test_context::{TestContext};
use crate::core::teon::Value;
use crate::purger::purge;
use crate::seeder::seed::seed;
use crate::server::serve;
use crate::gen::interface::server::gen as gen_entity;
use crate::gen::interface::client::gen::gen as gen_client;

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
                    if let Some(reset) = &test_conf.reset {
                        match &reset.datasets {
                            ResetDatasets::Auto => Some(Box::leak(Box::new(TestContext {
                                reset_mode: ResetMode::AfterQuery,
                                datasets: app_ctx.datasets().iter().filter(|d| d.autoseed == true).map(|d| d.clone()).collect(),
                            }))),
                            ResetDatasets::Names(names) => {
                                let sv: Vec<String> = names.iter().map(|v| v.to_owned()).collect();
                                Some(Box::leak(Box::new(TestContext {
                                    reset_mode: ResetMode::AfterQuery,
                                    datasets: app_ctx.datasets().iter().filter(|d| sv.contains(&d.name)).map(|d| d.clone()).collect(),
                                })))
                            }
                        }
                    } else {
                        None
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
                    match app_ctx.entities().len() {
                        0 => println!("Cannot find an entity generator declaration."),
                        1 => {
                            let conf = app_ctx.entities().get(0).unwrap();
                            gen_entity(graph, conf).await?;
                        },
                        _ => {
                            let mut names = entity_command.names.clone().unwrap_or(vec![]);
                            if entity_command.all {
                                names = app_ctx.entities().iter().map(|c| c.name.clone().unwrap()).collect();
                            }
                            for name in names.iter() {
                                let conf = app_ctx.entities().iter().find(|c| c.name.as_ref().unwrap() == name).unwrap();
                                gen_entity(graph, conf).await?;
                            }
                        }
                    }
                }
                GenerateCommand::GenerateClientCommand(client_command) => {
                    match app_ctx.clients().len() {
                        0 => println!("Cannot find a client generator declaration."),
                        1 => {
                            let conf = app_ctx.clients().get(0).unwrap();
                            gen_client(graph, conf).await?;
                        },
                        _ => {
                            let mut names = client_command.names.clone().unwrap_or(vec![]);
                            if client_command.all {
                                names = app_ctx.clients().iter().map(|c| c.name.clone().unwrap()).collect();
                            }
                            for name in names.iter() {
                                let conf = app_ctx.clients().iter().find(|c| c.name.as_ref().unwrap() == name).unwrap();
                                gen_client(graph, conf).await?;
                            }
                        }
                    }
                }
            }
        }
        CLICommand::Migrate(migrate_command) => {
            migrate(graph, migrate_command.dry).await;
        }
        CLICommand::Seed(seed_command) => {
            let names = if seed_command.all {
                datasets.iter().map(|d| d.name.clone()).collect()
            } else {
                seed_command.names.clone().unwrap()
            };
            migrate(graph, false).await;
            seed(seed_command.action, graph(), datasets, names).await
        }
        CLICommand::Purge(_) => {
            purge(graph).await.unwrap()
        }
    }
    Ok(())
}