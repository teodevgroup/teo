use teo_result::Result;
use crate::cli::command::{CLI, CLICommand};

pub async fn run(cli: &CLI) -> Result<()> {
    match &cli.command {
        CLICommand::Serve(serve_command) => {}
        CLICommand::Generate(generate_command) => {}
        CLICommand::Migrate(migrate_command) => {}
        CLICommand::Seed(seed_command) => {}
        CLICommand::Purge(purge_command) => {}
        CLICommand::Lint(lint_command) => (),
        CLICommand::Run(run_command) => (),
    }
    Ok(())
}