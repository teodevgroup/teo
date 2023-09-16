use std::future::Future;
use std::sync::Arc;
use crate::app::cli::command::CLI;
use crate::app::cli::parse_cli::parse_cli;
use crate::app::cli::run_command::run_command;
use crate::app::connect_to_database::connect_to_database;
use crate::app::namespace::Namespace;
use crate::app::parse_schema::{load_schema, parse_schema};
use crate::app::routes::action_ctx::{ActionCtxArgument};
use crate::app::routes::middleware_ctx::Middleware;
use crate::core::callbacks::types::callback::{CallbackArgument, CallbackResult};
use crate::core::callbacks::types::compare::CompareArgument;
use crate::core::callbacks::types::transform::{TransformArgument, TransformResult};
use crate::core::callbacks::types::validate::{ValidateArgument, ValidateResult};
use crate::core::items::function::compare::CompareItem;
use crate::core::items::function::perform::CallbackItem;
use crate::core::items::function::transform::TransformItem;
use crate::core::items::function::validate::ValidateItem;
use crate::prelude::{UserCtx, Value};
use crate::core::error::Error;
use super::app_ctx::AppCtx;
use crate::core::result::Result;
use crate::parser::diagnostics::diagnostics::Diagnostics;

pub struct App(usize);

impl App {

    pub fn new() -> Result<Self> {
        if AppCtx::create() {
            Ok(Self(0))
        } else {
            Err(Error::fatal("A running Teo application cannot have more than 1 app instance."))
        }
    }

    pub fn namespace(&self, name: &'static str) -> &mut Namespace {
        AppCtx::get().unwrap().main_namespace_mut().child_namespace(name)
    }

    pub fn middleware<F>(&self, name: &'static str, f: F) -> Result<()> where
        F: Middleware + 'static,
    {
        AppCtx::get()?.main_namespace_mut().add_middleware(name, f)
    }

    pub fn action<T, F>(&self, group: &'static str, name: &'static str, f: F) -> Result<()> where
        T: 'static,
        F: ActionCtxArgument<T> + 'static,
    {
        AppCtx::get()?.main_namespace_mut().add_action_handler(group, name, f)
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

    pub fn setup<F, T, Fut>(&self, f: F) -> Result<&Self> where
        F: Fn(T) -> Fut + Sync + Send + 'static,
        T: From<UserCtx> + Send,
        Fut: Future<Output = Result<()>> + Send,
    {
        let capture_f = Box::leak(Box::new(f));
        AppCtx::get()?.set_setup(Arc::new(|user_ctx: UserCtx| async {
            capture_f(user_ctx.into()).await
        }));
        Ok(self)
    }

    pub fn prepare(&self) -> Result<CLI> {
        let cli = parse_cli()?;
        let mut diagnostics = Diagnostics::new();
        parse_schema(cli.main(), &mut diagnostics)?;
        load_schema(&mut diagnostics)?;
        Ok(cli)
    }

    pub async fn run_without_prepare(&self, cli: &CLI) -> Result<()> {
        if !cli.command.is_generate() {
            connect_to_database().await?;
        }
        run_command(cli).await?;
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let cli = self.prepare()?;
        self.run_without_prepare(&cli).await?;
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        AppCtx::drop().unwrap()
    }
}