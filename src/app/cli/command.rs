#[derive(Debug)]
pub(crate) struct ServeCommand {
    pub(crate) no_migration: bool,
    pub(crate) no_autoseed: bool,
    pub(crate) env: Option<String>,
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

#[derive(Debug)]
pub(crate) struct SeedCommand {
    pub(crate) action: SeedCommandAction,
    pub(crate) all: bool,
    pub(crate) names: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum SeedCommandAction {
    Seed,
    Unseed,
    Reseed,
}

#[derive(Debug)]
pub(crate) struct PurgeCommand { }

#[derive(Debug)]
pub struct CLI {
    pub(crate) command: CLICommand,
    pub(crate) schema: Option<String>,
}

impl CLI {
    pub(crate) fn main(&self) -> Option<&str> {
        self.schema.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug)]
pub(crate) enum CLICommand {
    Serve(ServeCommand),
    Generate(GenerateCommand),
    Migrate(MigrateCommand),
    Seed(SeedCommand),
    Purge(PurgeCommand),
}

impl CLICommand {
    pub(crate) fn is_generate(&self) -> bool {
        match self {
            CLICommand::Generate(_) => true,
            _ => false,
        }
    }
}