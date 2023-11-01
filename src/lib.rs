pub mod cli;
pub mod app;
pub mod server;
pub mod migrate;
pub mod purge;
mod message;

pub mod prelude {
    pub use crate::app::App;
    pub extern crate teo_result;
    pub use teo_result::{Error, Result, ResultExt};
    pub extern crate tokio;
    pub use tokio::main;
    pub extern crate key_path;
    pub use key_path::path;
    pub use teo_runtime::request;
    pub use teo_runtime::response::Response;
    pub use teo_runtime::path;
}

// pub(crate) mod old_seeder;
// pub(crate) mod purge;

// pub mod prelude {
//     pub use crate::old_app::old_app::App;
//     pub use crate::old_app::routes::middleware_ctx::*;
//     pub use crate::old_app::routes::action_ctx::*;
//     pub use crate::old_app::routes::*;
//     pub use crate::old_app::routes::req_local::*;
//     pub use crate::old_app::routes::res::*;
//     pub use crate::old_app::routes::req::*;
//     pub use crate::old_app::routes::readonly_header_map::*;
//     pub use crate::old_server::ReqCtx;
//     pub use crate::old_app::routes::req::Req;
//     pub use crate::old_app::routes::res::Res;
//     pub use crate::old_core::graph::Graph;
//     pub use teo_teon::value::Value;
//     pub use teo_teon::teon;
//     pub use teo_teon::teon_vec;
//     pub use teo_teon::teon_unexpected;
//     pub use teo_teon::teon_expect_expr_comma;
//     pub use crate::old_core::object::Object;
//     pub use crate::old_core::ctx::model::ModelCtx;
//     pub use crate::old_core::ctx::user::UserCtx;
// }
