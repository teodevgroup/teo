use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum RuntimeVersion {
    Rust(&'static str),
    NodeJS(String),
    Python(String),
}

impl Display for RuntimeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RuntimeVersion::Rust(v) => format!("Rust {v}"),
            RuntimeVersion::NodeJS(v) => format!("Node.js {v}"),
            RuntimeVersion::Python(v) => format!("Python {v}"),
        };
        write!(f, "{}", str)
    }
}
