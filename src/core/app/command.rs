#[derive(Debug)]
pub(crate) struct CLI {
    pub(crate) command: Option<CLICommand>,
    pub(crate) schema: Option<String>,
}

#[derive(Debug)]
pub(crate) enum CLICommand {
    Serve(ServeCommand),
    Generate(GenerateCommand),
    Migrate(MigrateCommand),
}

#[derive(Debug)]
pub(crate) struct ServeCommand {
    pub(crate) no_migration: bool,
}

#[derive(Debug)]
pub(crate) enum GenerateCommand {
    GenerateClientCommand(GenerateClientCommand),
    GenerateEntityCommand(GenerateEntityCommand),
}

#[derive(Debug)]
pub(crate) struct GenerateClientCommand {
    pub(crate) all: bool,
    pub(crate) names: Option<Vec<String>>,
}

#[derive(Debug)]
pub(crate) struct GenerateEntityCommand {
    pub(crate) all: bool,
    pub(crate) names: Option<Vec<String>>,
}

#[derive(Debug)]
pub(crate) struct MigrateCommand {
    pub(crate) dry: bool,
}
