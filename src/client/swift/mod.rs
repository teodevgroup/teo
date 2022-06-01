use async_trait::async_trait;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;
use crate::client::shared::{clear_directory, ensure_directory, generate_file};
use crate::client::swift::pkg::gitignore::generate_gitignore;
use crate::client::swift::pkg::package_swift::generate_package_swift;
use crate::client::swift::pkg::readme_md::generate_readme_md;
use crate::core::graph::Graph;
use crate::core::client::Client;

pub(crate) mod pkg;


#[derive(Debug)]
pub struct SwiftClient {
    pub(crate) at: Arc<Mutex<String>>,
    pub(crate) combine_observable_objects: AtomicBool,
}

impl SwiftClient {
    pub(crate) fn new() -> Self {
        SwiftClient {
            at: Arc::new(Mutex::new("".to_string())),
            combine_observable_objects: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl Client for SwiftClient {
    async fn generate(&self, graph: &'static Graph) -> std::io::Result<()> {
        ensure_directory("client").await?;
        clear_directory("client/swift").await?;
        generate_file("client/swift/README.md", generate_readme_md(graph).await).await?;
        generate_file("client/swift/.gitignore", generate_gitignore(graph).await).await?;
        generate_file("client/swift/Package.swift", generate_package_swift(graph).await).await
    }
}
