use std::env;
use std::ffi::OsString;
use clap::{Arg, ArgAction, Command as ClapCommand};
use crate::cli::entrance::Entrance;
use crate::cli::runtime_version::RuntimeVersion;
use crate::cli::command::{CLI, CLICommand, GenerateClientCommand, GenerateCommand, GenerateEntityCommand, LintCommand, MigrateCommand, PurgeCommand, RunCommand, SeedCommand, SeedCommandAction, ServeCommand};

pub(crate) fn parse(runtime_version: RuntimeVersion, entrance: Entrance, argv: Option<Vec<String>>) -> CLI {
    let argv = argv.unwrap_or(env::args_os().map(|s| s.to_str().unwrap().to_owned()).collect());
    let version = Box::leak(Box::new(format!("Teo {} ({}) [{}]", env!("CARGO_PKG_VERSION"), runtime_version.to_string(), entrance.to_str())));
    let about = Box::leak(Box::new(match entrance {
        Entrance::CLI => format!("{version}\n\nRun Teo application with CLI."),
        Entrance::APP => format!("{version}\n\nRun Teo application with user app loaded."),
    }));
    let matches = ClapCommand::new("teo")
        .version(version.as_str())
        .disable_version_flag(true)
        .disable_help_subcommand(true)
        .arg_required_else_help(true)
        .about(about.as_str())
        .subcommand_required(true)
        .arg(Arg::new("SCHEMA_FILE")
            .short('s')
            .long("schema")
            .help("The schema file to load").action(ArgAction::Set)
            .required(false)
            .num_args(1)
            .global(true))
        .arg(Arg::new("ENV")
            .short('e')
            .long("env")
            .help("The environment to use")
            .action(ArgAction::Set)
            .required(false)
            .num_args(1)
            .global(true))
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .help("Print version information")
            .action(ArgAction::Version))
        .arg(Arg::new("silent")
            .short('S')
            .long("silent")
            .help("Silent outputs")
            .action(ArgAction::SetTrue))
        .subcommand(ClapCommand::new("serve")
            .about("Run migration and start the server")
            .arg_required_else_help(false)
            .arg(Arg::new("no-migration")
                .short('M')
                .long("no-migration")
                .help("Start server without running migration")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("no-autoseed")
                .short('S')
                .long("no-autoseed")
                .help("Start server without auto seeding autoseed dataset")
                .action(ArgAction::SetTrue)))
        .subcommand(ClapCommand::new("generate")
            .about("Generate code")
            .arg_required_else_help(true)
            .subcommand(ClapCommand::new("client")
                .about("Generate client")
                .arg(Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Generate all clients")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("NAME"))
                .arg(Arg::new("NAME")
                    .action(ArgAction::Append)
                    .conflicts_with("all")
                    .help("Client names to generate")
                    .num_args(1..)))
            .subcommand(ClapCommand::new("entity")
                .about("Generate model entities")
                .arg_required_else_help(false)
                .arg(Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("Generate all entities")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("NAME"))
                .arg(Arg::new("NAME")
                    .action(ArgAction::Append)
                    .conflicts_with("all")
                    .help("Entity names to generate")
                    .num_args(1..))))
        .subcommand(ClapCommand::new("migrate")
            .about("Run migration")
            .arg(Arg::new("dry")
                .short('d')
                .long("dry")
                .help("Dry run")
                .action(ArgAction::SetTrue)))
        .subcommand(ClapCommand::new("seed")
            .about("Seed data")
            .arg(Arg::new("unseed")
                .short('u')
                .long("unseed")
                .help("Unseed records")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("reseed")
                .short('r')
                .long("reseed")
                .help("Reseed records")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("all")
                .short('a')
                .long("all")
                .help("Do for all data sets")
                .action(ArgAction::SetTrue)
                .conflicts_with("NAME"))
            .arg(Arg::new("NAME")
                .action(ArgAction::Append)
                .conflicts_with("all")
                .help("Data set names to process")
                .num_args(1..)))
        .subcommand(ClapCommand::new("purge")
            .about("Purge and clear the database without dropping tables."))
        .subcommand(ClapCommand::new("lint")
            .about("Lint the schema files"))
        .subcommand(ClapCommand::new("run")
            .about("Run a defined program")
            .arg(Arg::new("NAME")
                .required(true)
                .action(ArgAction::Append)
                .help("Program name to run")
                .num_args(1)))
        .get_matches_from(match runtime_version {
            RuntimeVersion::Python(_) | RuntimeVersion::NodeJS(_) => {
                let result = argv.iter().enumerate().filter(|(i, x)| (*i != 1) && !x.as_str().ends_with(".ts")).map(|(_i, x)| x.clone()).collect::<Vec<String>>();
                result
            },
            RuntimeVersion::Rust(_) => argv.iter().enumerate().filter(|(i, x)| {
                !((*i == 1) && x.as_str() == "teo")
            }).map(|(_i, x)| x.clone()).collect::<Vec<String>>(),
        });
    let silent: bool = matches.get_flag("silent");
    let schema: Option<&String> = matches.get_one("SCHEMA_FILE");
    let command = match matches.subcommand() {
        Some(("serve", submatches)) => {
            let env: Option<&String> = submatches.get_one("ENV");
            CLICommand::Serve(ServeCommand { no_migration: submatches.get_flag("no-migration"), no_autoseed: submatches.get_flag("no-autoseed"), env: env.cloned() })
        }
        Some(("generate", submatches)) => {
            match submatches.subcommand() {
                Some(("client", submatches)) => {
                    let names: Option<Vec<String>> = submatches.get_many::<String>("NAME").map(|s| s.map(|v| v.to_string()).collect::<Vec<String>>());
                    CLICommand::Generate(GenerateCommand::GenerateClientCommand(GenerateClientCommand { all: submatches.get_flag("all"), names }))
                }
                Some(("entity", submatches)) => {
                    let names: Option<Vec<String>> = submatches.get_many::<String>("NAME").map(|s| s.map(|v| v.to_string()).collect::<Vec<String>>());
                    CLICommand::Generate(GenerateCommand::GenerateEntityCommand(GenerateEntityCommand { all: submatches.get_flag("all"), names }))
                }
                _ => unreachable!()
            }
        }
        Some(("migrate", submatches)) => {
            CLICommand::Migrate(MigrateCommand { dry: submatches.get_flag("dry") })
        }
        Some(("seed", submatches)) => {
            let action = if submatches.get_flag("reseed") {
                SeedCommandAction::Reseed
            } else if submatches.get_flag("unseed") {
                SeedCommandAction::Unseed
            } else {
                SeedCommandAction::Seed
            };
            let names: Option<Vec<String>> = submatches.get_many::<String>("NAME").map(|s| s.map(|v| v.to_string()).collect::<Vec<String>>());
            CLICommand::Seed(SeedCommand {
                action,
                all: submatches.get_flag("all"),
                names,
            })
        }
        Some(("purge", _submatches)) => {
            CLICommand::Purge(PurgeCommand { })
        }
        Some(("lint", _submatches)) => {
            CLICommand::Lint(LintCommand { })
        }
        Some(("run", submatches)) => {
            let name: Option<String> = submatches.get_one::<String>("NAME").map(|s| s.clone());
            CLICommand::Run(RunCommand { name: name.unwrap() })
        }
        _ => unreachable!()
    };
    CLI { command, schema: schema.map(|s| s.to_string()), silent }
}