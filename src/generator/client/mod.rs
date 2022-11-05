pub mod kotlin;
pub mod swift;
pub mod typescript;
pub mod flutter;
pub mod csharp;

use crate::core::conf::client::Client;
use crate::generator::lib::generator::Generator;
use crate::core::graph::Graph;

pub(crate) trait ClientGenerator {
    fn generate_main(graph: Graph, client: Client, generator: Generator) -> ();
    fn generate_accessories(graph: Graph, client: Client, generator: Generator) -> ();
}
