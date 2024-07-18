pub mod cli;
pub mod server;
pub mod migrate;
pub mod purge;
pub mod seeder;
pub mod result;
mod message;
pub mod database;

pub use cli::app_run::AppExt;

pub mod prelude {
    pub use teo_runtime::app;
    pub use teo_runtime::app::App;
    pub use teo_runtime::app::entrance::Entrance;
    pub use teo_runtime::app::runtime_version::RuntimeVersion;
    pub use crate::server::static_files::serve_static_files;
    pub use teo_runtime::namespace::Namespace;
    pub extern crate teo_result;
    pub use teo_result::{Error, Result, ResultExt};
    pub extern crate tokio;
    pub use tokio::main;
    pub extern crate key_path;
    pub use key_path::path;
    pub use teo_runtime::namespace;
    pub use teo_runtime::r#enum;
    pub use teo_runtime::traits;
    pub use teo_runtime::request;
    pub use teo_runtime::response::Response;
    pub use teo_runtime::model;
    pub use teo_runtime::model::Model;
    pub use teo_runtime::r#struct;
    pub use teo_runtime::r#struct::Struct;
    pub use teo_runtime::value::interface_enum_variant::InterfaceEnumVariant;
    pub use teo_runtime::interface;
    pub use teo_runtime::connection::transaction;
    pub use teo_runtime::value;
    pub use teo_runtime::value::Value;
    pub use teo_runtime::teon;
    pub use teo_runtime::teon_vec;
    pub use teo_runtime::teon_unexpected;
    pub use teo_runtime::teon_expect_expr_comma;
    pub use teo_runtime::value::option_variant::OptionVariant;
    pub use teo_runtime::value::range::Range;
    pub use teo_runtime::value::file::File;
    pub use teo_runtime::request::ctx::extract::ExtractFromRequestCtx;
    pub use teo_runtime::request::Request;
    pub use teo_runtime::response;
    pub use teo_runtime::pipeline;
    pub use teo_runtime::handler;
    pub use teo_runtime::pipeline::Pipeline;
    pub use teo_runtime::pipeline::ctx::extract::ExtractFromPipelineCtx;
    pub use teo_runtime::middleware::middleware::middleware_wrap_fn;
    pub use teo_runtime::connection::transaction::ExtractFromTransactionCtx;
    pub use teo_runtime::arguments::Arguments;
    pub use teo_runtime::middleware::next::Next;
    pub use teo_runtime::middleware::middleware::Middleware;
    pub use teo_runtime::r#enum::Enum;
    pub use teo_runtime::r#enum::member::Member;
    pub mod result {
        pub use teo_result::{Result, Error, ResultExt, ErrorSerializable};
    }
}
