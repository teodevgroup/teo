use teo_result::Result;
use crate::app::ctx::Ctx;
use crate::app::database::connect_databases;
use crate::cli::command::{CLI, CLICommand};

pub async fn run(cli: &CLI) -> Result<()> {
    match &cli.command {
        CLICommand::Serve(serve_command) => {
            connect_databases(Ctx::main_namespace_mut())?;
        }
        CLICommand::Generate(generate_command) => {
            todo!()
        }
        CLICommand::Migrate(migrate_command) => {
            connect_databases(Ctx::main_namespace_mut())?;
        }
        CLICommand::Seed(seed_command) => {
            connect_databases(Ctx::main_namespace_mut())?;
        }
        CLICommand::Purge(purge_command) => {
            connect_databases(Ctx::main_namespace_mut())?;
        }
        CLICommand::Lint(lint_command) => (),
        CLICommand::Run(run_command) => {
            connect_databases(Ctx::main_namespace_mut())?;

        },
    }
    Ok(())
}