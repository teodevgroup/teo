use teo_runtime::config::entity::Runtime;
use teo_result::{Result};
use teo_runtime::namespace::Namespace;
use crate::app::ctx::Ctx;

pub struct App { }

impl App {

    pub fn new() -> Self {
        if Ctx::create() {

        } else {

        }
        Self { }
    }

    pub fn main_namespace(&self) -> &Namespace {
        &Ctx::get().main_namespace
    }

    pub fn main_namespace_mut(&self) -> &mut Namespace {
        &mut Ctx::get_mut().main_namespace
    }

    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}