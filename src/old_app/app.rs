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
use crate::parser::diagnostics::printer;

pub struct App;

impl App {

    pub fn middleware<F>(&self, name: &'static str, f: F) -> Result<()> where
        F: Middleware + 'static,
    {
        AppCtx::get()?.main_namespace_mut().add_middleware(name, f)
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

    pub fn program<F, T, Fut>(&self, name: impl Into<String>, f: F) -> Result<&Self> where
        F: Fn(T) -> Fut + Sync + Send + 'static,
        T: From<UserCtx> + Send,
        Fut: Future<Output = Result<()>> + Send,
    {
        let capture_f = Box::leak(Box::new(f));
        AppCtx::get()?.insert_program(name.into(), Arc::new(|user_ctx: UserCtx| async {
            capture_f(user_ctx.into()).await
        }));
        Ok(self)
    }
}

impl Drop for App {
    fn drop(&mut self) {
        AppCtx::drop().unwrap()
    }
}