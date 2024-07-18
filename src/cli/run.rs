use teo_parser::diagnostics::diagnostics::Diagnostics;
use teo_result::{Error, Result};
use crate::app::database::connect_databases;
use crate::cli::command::{CLI, CLICommand, GenerateCommand, SeedCommandAction};
use crate::server::make::serve;
use teo_runtime::connection::transaction;
use teo_runtime::schema::load::load_data_sets::load_data_sets;
use crate::app::App;
use crate::migrate::migrate;
use crate::purge::purge;
use crate::seeder::seed::seed;

pub async fn run(app: &App) -> Result<()> {
    let cli = &app.cli;
    match &cli.command {
        CLICommand::Serve(serve_command) => {
            connect_databases(app, app.compiled_main_namespace(), cli.silent).await?;
            let conn_ctx = app.conn_ctx();
            // migrate
            if !serve_command.no_migration {
                migrate(app, false, false, cli.silent).await?;
            }
            // seed auto seed data sets
            if !serve_command.no_autoseed {
                if app.compiled_main_namespace().database().is_some() {
                    let mut diagnostics = Diagnostics::new();
                    let data_sets = load_data_sets(app.main_namespace(), None, false, app.schema(), &mut diagnostics)?;
                    let transaction_ctx = transaction::Ctx::new(app.conn_ctx().clone());
                    seed(SeedCommandAction::Seed, data_sets, transaction_ctx, false).await?;
                }
            }
            // setup
            if let Some(setup) = app.setup.lock().unwrap().clone() {
                let transaction_ctx = transaction::Ctx::new(app.conn_ctx().clone());
                setup.call(transaction_ctx).await?;
            }
            // start server
            serve(conn_ctx.namespace(), conn_ctx.namespace().server().as_ref().unwrap(), app.runtime_version(), app.entrance(), cli.silent).await
        }
        CLICommand::Generate(generate_command) => {
            match generate_command {
                GenerateCommand::GenerateClientCommand(command) => {
                    let names = if let Some(names) = command.names.as_ref() {
                        names.clone()
                    } else if command.all {
                        app.compiled_main_namespace().clients().keys().map(|k| k.clone()).collect()
                    } else {
                        match app.compiled_main_namespace().clients().len() {
                            0 => Err(Error::new("no clients found"))?,
                            1 => return teo_generator::client::generate(app.compiled_main_namespace(), app.compiled_main_namespace().clients().first_key_value().unwrap().1).await,
                            _ => Err(Error::new("requires client name"))?,
                        }
                    };
                    for name in names {
                        if let Some(client) = app.compiled_main_namespace().clients().get(&name) {
                            teo_generator::client::generate(app.compiled_main_namespace(), client).await?;
                        } else {
                            Err(Error::new("client not found"))?
                        }
                    }
                    Ok(())
                }
                GenerateCommand::GenerateEntityCommand(command) => {
                    let names = if let Some(names) = command.names.as_ref() {
                        names.clone()
                    } else if command.all {
                        app.compiled_main_namespace().entities().keys().map(|k| k.clone()).collect()
                    } else {
                        match app.compiled_main_namespace().entities().len() {
                            0 => Err(Error::new("no entities found"))?,
                            1 => return teo_generator::entity::generate(app.compiled_main_namespace(), app.compiled_main_namespace().entities().first_key_value().unwrap().1).await,
                            _ => Err(Error::new("requires entity name"))?,
                        }
                    };
                    for name in names {
                        if let Some(entity) = app.compiled_main_namespace().entities().get(&name) {
                            teo_generator::entity::generate(app.compiled_main_namespace(), entity).await?;
                        } else {
                            Err(Error::new("entity not found"))?
                        }
                    }
                    Ok(())
                }
                GenerateCommand::GenerateAdminCommand(_) => {
                    if let Some(admin) = &app.compiled_main_namespace().admin() {
                        teo_generator::admin::generate(app.compiled_main_namespace(), admin, app.compiled_main_namespace().server().as_ref().unwrap()).await?;
                    }
                    Ok(())
                }
            }
        }
        CLICommand::Migrate(migrate_command) => {
            connect_databases(app, app.compiled_main_namespace(), cli.silent).await?;
            migrate(app, migrate_command.dry, false, cli.silent).await?;
            Ok(())
        }
        CLICommand::Seed(seed_command) => {
            connect_databases(app, app.compiled_main_namespace(), cli.silent).await?;
            let mut diagnostics = Diagnostics::new();
            let data_sets = load_data_sets(app.main_namespace(), seed_command.names.as_ref(), seed_command.all, app.schema(), &mut diagnostics)?;
            let transaction_ctx = transaction::Ctx::new(app.conn_ctx().clone());
            seed(seed_command.action, data_sets, transaction_ctx, true).await?;
            Ok(())
        }
        CLICommand::Purge(purge_command) => {
            connect_databases(app, app.compiled_main_namespace(), cli.silent).await?;
            purge(app).await?;
            Ok(())
        }
        CLICommand::Lint(lint_command) => Ok(()),
        CLICommand::Run(run_command) => {
            if run_command.list {
                println!("+-{:<32}-+-{:<64}-+", "--------------------------------", "----------------------------------------------------------------");
                println!("| {:^32} | {:^64} |", "Name", "Description");
                println!("+-{:<32}-+-{:<64}-+", "--------------------------------", "----------------------------------------------------------------");
                if app.programs().is_empty() {
                    println!("| {:^99} |", "No programs.");
                } else {
                    for (name, program) in app.programs().iter() {
                        let desc_str = program.desc.as_ref().map_or_else(
                            || "(No description)".to_string(),
                            |desc| desc.into()
                        );
                        println!("| {:<32} | {:<64} |", name, desc_str);
                    }
                }
                println!("+-{:<32}---{:<64}-+", "--------------------------------", "----------------------------------------------------------------");
            } else {
                if let Some(name) = &run_command.name {
                    connect_databases(app, app.compiled_main_namespace(), cli.silent).await?;
                    let programs = app.programs();
                    let program = programs.get(name).ok_or_else(|| Error::new(format!("Program '{}' is not defined", name)))?;
                    let transaction_ctx = transaction::Ctx::new(app.conn_ctx().clone());
                    program.func.call(transaction_ctx).await?;
                    std::process::exit(0);
                } else {
                    return Err(Error::new("No program name provided"));
                }
            }
            Ok(())
        },
    }
}