use std::sync::Arc;
use crate::app::cli::command::CLI;
use crate::app::cli::parse_cli::parse_cli;
use crate::app::cli::run_command::run_command;
use crate::app::connect_to_database::connect_to_database;
use crate::app::parse_schema::{load_schema, parse_schema};
use crate::core::callbacks::types::callback::{CallbackArgument, CallbackResult};
use crate::core::callbacks::types::callback_without_args::AsyncCallbackWithoutArgs;
use crate::core::callbacks::types::compare::CompareArgument;
use crate::core::callbacks::types::transform::{TransformArgument, TransformResult};
use crate::core::callbacks::types::validate::{ValidateArgument, ValidateResult};
use crate::core::items::function::compare::CompareItem;
use crate::core::items::function::perform::CallbackItem;
use crate::core::items::function::transform::TransformItem;
use crate::core::items::function::validate::ValidateItem;
use crate::prelude::Value;
use crate::core::error::Error;
use super::ctx::AppCtx;
use crate::core::result::Result;

pub struct App(usize);

impl App {

    pub fn new() -> Result<Self> {
        if AppCtx::create() {
            Ok(Self(0))
        } else {
            Err(Error::fatal("A running Teo application cannot have more than 1 app instance."))
        }
    }

    pub fn transform<A, O, F, R>(&self, name: &'static str, f: F) -> Result<&Self> where
        A: Send + Sync + 'static,
        O: Into<Value> + Send + Sync + 'static,
        R: Into<TransformResult<O>> + Send + Sync + 'static,
        F: TransformArgument<A, O, R> + 'static {
        AppCtx::get()?.callbacks_mut().add_transform(name, Arc::new(TransformItem::new(f)));
        Ok(self)
    }

    pub fn callback<T, F, O>(&self, name: &'static str, f: F) -> Result<&Self> where
        T: Send + Sync + 'static,
        F: CallbackArgument<T, O> + 'static,
        O: Into<CallbackResult> + Send + Sync + 'static {
        AppCtx::get()?.callbacks_mut().add_callback(name, Arc::new(CallbackItem::new(f)));
        Ok(self)
    }

    pub fn validate<T, O, F>(&self, name: &'static str, f: F) -> Result<&Self> where
        T: Send + Sync + 'static,
        O: Into<ValidateResult> + Send + Sync + 'static,
        F: ValidateArgument<T, O> + 'static {
        AppCtx::get()?.callbacks_mut().add_validator(name, Arc::new(ValidateItem::new(f)));
        Ok(self)
    }

    pub fn compare<T, O, F>(&self, name: &'static str, f: F) -> Result<&Self> where
        T: Send + Sync + 'static,
        O: Into<ValidateResult> + Send + Sync + 'static,
        F: CompareArgument<T, O> + 'static {
        AppCtx::get()?.callbacks_mut().add_compare(name, Arc::new(CompareItem::new(f)));
        Ok(self)
    }

    pub fn setup<F>(&self, f: F) -> Result<&Self> where F: AsyncCallbackWithoutArgs + 'static {
        AppCtx::get()?.set_setup(Arc::new(f));
        Ok(self)
    }

    pub fn prepare(&self) -> Result<CLI> {
        let cli = parse_cli()?;
        parse_schema(cli.main())?;
        load_schema()?;
        Ok(cli)
    }

    pub async fn run_without_prepare(&self, cli: CLI) -> Result<()> {
        if !cli.command.is_generate() {
            connect_to_database().await?;
        }
        run_command(cli).await?;
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let cli = self.prepare()?;
        self.run_without_prepare(cli).await?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        AppCtx::drop()
    }
}