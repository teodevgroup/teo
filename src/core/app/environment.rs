#[derive(Copy, Clone, Debug)]
pub enum Environment {
    Rust,
    NodeJS,
    Python,
    Go,
    Java,
    C,
}

#[derive(Clone, Debug)]
pub enum EnvironmentVersion {
    Rust(String),
    NodeJS(String),
    Python(String),
    Go(String),
    Java(String),
    C(String),
}

impl EnvironmentVersion {
    pub fn to_string(&self) -> String {
        match self {
            EnvironmentVersion::Rust(v) => format!("Rust {v}"),
            EnvironmentVersion::NodeJS(v) => format!("Node.js {v}"),
            EnvironmentVersion::Python(v) => format!("Python {v}"),
            EnvironmentVersion::Go(v) => format!("Go {v}"),
            EnvironmentVersion::Java(v) => format!("Java {v}"),
            EnvironmentVersion::C(v) => format!("C {v}"),
        }
    }
}
