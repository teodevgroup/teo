use std::env::current_dir;
use crate::core::conf::Conf;
use crate::core::graph::Graph;
use crate::generator::client::ClientGenerator;
use crate::generator::client::csharp::CSharpClientGenerator;
use crate::generator::client::swift::SwiftClientGenerator;
use crate::generator::client::typescript::TypeScriptClientGenerator;
use crate::generator::lib::generator::Generator;

pub(crate) async fn generate(graph: &Graph, conf: &Conf) -> Result<(), std::io::Error> {
    let generator = Generator::new(current_dir().unwrap());
    for client in conf.clients.iter() {
        if client.is_typescript() {
            let lang = TypeScriptClientGenerator {};
            lang.generate_accessories(graph, client, &generator).await?;
            lang.generate_main(graph, client, &generator).await?;
        } else if client.is_csharp() {
            let lang = CSharpClientGenerator {};
            lang.generate_accessories(graph, client, &generator).await?;
            lang.generate_main(graph, client, &generator).await?;
        } else if client.is_swift() {
            let lang = SwiftClientGenerator {};
            lang.generate_accessories(graph, client, &generator).await?;
            lang.generate_main(graph, client, &generator).await?;
        }
    }
    Ok(())
}
