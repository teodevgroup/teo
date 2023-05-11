use crate::app::cli::command::CLI;
use crate::app::cli::run_command::run_command;
use crate::app::connect_to_database::connect_to_database;
use crate::core::result::Result;

pub async fn run_without_prepare(cli: &CLI) -> Result<()> {
    if !cli.command.is_generate() {
        connect_to_database().await?;
    }
    run_command(cli).await?;
    Ok(())
}