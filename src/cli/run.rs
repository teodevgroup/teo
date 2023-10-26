use teo_result::{Error, Result};
use crate::app::ctx::Ctx;
use crate::app::database::connect_databases;
use crate::cli::command::{CLI, CLICommand};
use teo_runtime::connection::Ctx as ConnCtx;

pub async fn run(cli: &CLI) -> Result<()> {
    match &cli.command {
        CLICommand::Serve(serve_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
            let ctx = ConnCtx::from_namespace(Ctx::main_namespace());
        }
        CLICommand::Generate(generate_command) => {
            todo!()
        }
        CLICommand::Migrate(migrate_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
        }
        CLICommand::Seed(seed_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
        }
        CLICommand::Purge(purge_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
        }
        CLICommand::Lint(lint_command) => (),
        CLICommand::Run(run_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
            if let Some(program) = Ctx::get_mut().programs.get(&run_command.name) {
                let ctx = ConnCtx::from_namespace(Ctx::main_namespace());
                program.call(ctx).await?
            } else {
                Err(Error::new(format!("program '{}' is not defined", &run_command.name)))?
            }
        },
    }
    Ok(())
}