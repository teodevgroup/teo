#[derive(Clone, Debug)]
pub enum RuntimeVersion {
    Rust(&'static str),
    NodeJS(String),
    Python(String),
}

impl ToString for RuntimeVersion {

    fn to_string(&self) -> String {
        match self {
            RuntimeVersion::Rust(v) => format!("Rust {v}"),
            RuntimeVersion::NodeJS(v) => format!("Node.js {v}"),
            RuntimeVersion::Python(v) => format!("Python {v}"),
        }
    }
}
