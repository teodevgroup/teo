use teo_result::{Error, Result};
use crate::app::ctx::Ctx;
use crate::app::database::connect_databases;
use crate::cli::command::{CLI, CLICommand};
use crate::server::make::serve;
use teo_runtime::connection;

pub async fn run(cli: &CLI) -> Result<()> {
    match &cli.command {
        CLICommand::Serve(serve_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
            let conn_ctx = Ctx::conn_ctx();
            serve(conn_ctx.namespace(), conn_ctx.namespace().server.as_ref().unwrap(), &Ctx::get().runtime_version, &Ctx::get().entrance).await
        }
        CLICommand::Generate(generate_command) => {
            todo!()
        }
        CLICommand::Migrate(migrate_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
            Ok(())
        }
        CLICommand::Seed(seed_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
            Ok(())
        }
        CLICommand::Purge(purge_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
            Ok(())
        }
        CLICommand::Lint(lint_command) => Ok(()),
        CLICommand::Run(run_command) => {
            connect_databases(Ctx::main_namespace_mut()).await?;
            if let Some(program) = Ctx::get_mut().programs.get(&run_command.name) {
                let ctx = connection::Ctx::from_namespace(Ctx::main_namespace());
                program.call(ctx).await
            } else {
                Err(Error::new(format!("program '{}' is not defined", &run_command.name)))
            }
        },
    }
}