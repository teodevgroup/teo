pub mod core;
pub mod connectors;
pub mod client;
pub mod app;

pub mod prelude {
    pub use crate::app::app::App;
    pub use crate::app::run;
    pub use crate::core::graph::Graph;
    pub use crate::core::graph::builder::GraphBuilder;
    pub use crate::core::pipeline::builder::PipelineBuilder;
    pub use crate::core::value::Value;
    pub use crate::core::object::Object;
    pub extern crate tokio;
    pub use tokio::main;
    pub extern crate serde_json;
    pub use serde_json::json;
}
