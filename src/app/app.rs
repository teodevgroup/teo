use std::process::exit;
use teo_result::{Error, Result};
use teo_runtime::namespace::Namespace;
use crate::app::ctx::Ctx;
use crate::app::load::find_main_schema_file;
use crate::cli::parse::{parse as cli_parse};
use teo_parser::{parse as schema_parse};
use teo_parser::diagnostics::printer::print_diagnostics;
use teo_runtime::stdlib::load::{load as load_std};
use teo_runtime::schema::load::load_schema::load_schema;
use crate::cli::run::run;
use dotenvy::dotenv;

#[derive(Debug)]
pub struct App { }

impl App {

    pub fn new() -> Result<Self> {
        // load env first
        let _ = dotenv();
        if !Ctx::create() {
            Err(Error::new("cannot create app while there is an existing instance"))?
        }
        let cli = cli_parse(Ctx::get().runtime_version.clone(), Ctx::get().entrance);
        let main_schema_file = find_main_schema_file(cli.schema.as_ref())?;
        let (schema, diagnostics) = schema_parse(main_schema_file.as_path().to_str().unwrap(), None, None);
        print_diagnostics(&diagnostics, true);
        if diagnostics.has_errors() {
            exit(1);
        }
        load_std(Ctx::main_namespace_mut());
        Ctx::set_schema(schema);
        Ctx::set_cli(cli);
        Ok(Self { })
    }

    pub fn main_namespace(&self) -> &Namespace {
        Ctx::main_namespace()
    }

    pub fn main_namespace_mut(&self) -> &mut Namespace {
        Ctx::main_namespace_mut()
    }

    pub async fn run(&self) -> Result<()> {
        load_schema(Ctx::main_namespace_mut(), Ctx::schema(), Ctx::cli().command.ignores_loading())?;

        run(Ctx::cli()).await
    }
}