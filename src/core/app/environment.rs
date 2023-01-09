#[derive(Copy, Clone)]
pub enum Environment {
    Rust,
    NodeJS,
    Python,
    Go,
    Java,
}

#[derive(Clone)]
pub enum EnvironmentVersion {
    Rust,
    NodeJS(String),
    Python(String),
    Go,
    Java(String),
}

impl EnvironmentVersion {
    pub fn to_string(&self) -> String {
        match self {
            EnvironmentVersion::Rust => "Rust".to_string(),
            EnvironmentVersion::NodeJS(v) => format!("Node.js {v}"),
            EnvironmentVersion::Python(v) => format!("Python {v}"),
            EnvironmentVersion::Go => "Go".to_string(),
            EnvironmentVersion::Java(v) => format!("Java {v}"),
        }
    }
}
