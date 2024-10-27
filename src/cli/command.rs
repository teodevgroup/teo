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
    GenerateAdminCommand(GenerateAdminCommand),
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
pub(crate) struct GenerateAdminCommand { }

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
pub enum SeedCommandAction {
    Seed,
    Unseed,
    Reseed,
}

#[derive(Debug)]
pub(crate) struct PurgeCommand { }

#[derive(Debug)]
pub(crate) struct LintCommand { }

#[derive(Debug)]
pub(crate) struct RunCommand {
    pub(crate) list: bool,
    pub(crate) name: Option<String>,
}

#[derive(Debug)]
pub struct CLI {
    pub command: CLICommand,
    pub(crate) schema: Option<String>,
    pub(crate) silent: bool,
}

impl CLI {
    pub(crate) fn main(&self) -> Option<&str> {
        self.schema.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug)]
pub(crate) enum CLICommand {
    Serve(ServeCommand),
    ServeHyper(ServeCommand),
    Generate(GenerateCommand),
    Migrate(MigrateCommand),
    Seed(SeedCommand),
    Purge(PurgeCommand),
    Lint(LintCommand),
    Run(RunCommand),
}

impl CLICommand {

    pub fn ignores_loading(&self) -> bool {
        match self {
            CLICommand::Generate(_) => true,
            CLICommand::Lint(_) => true,
            _ => false,
        }
    }
}
