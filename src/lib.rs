pub mod parser;
pub mod core;
pub mod connectors;
pub mod generator;
pub mod interface;

pub mod prelude {
    pub use crate::core::app::App;
    pub use crate::core::app::builder::AppBuilder;
    pub use crate::core::graph::Graph;
    pub use crate::core::graph::builder::GraphBuilder;
    pub use crate::core::pipeline::builder::PipelineBuilder;
    pub use crate::core::tson::Value;
    pub use crate::tson;
    pub use crate::core::object::Object;
    pub extern crate tokio;
    pub use tokio::main;
    pub extern crate key_path;
    pub use key_path::path;
}
