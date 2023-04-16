#[derive(Copy, Clone, Debug)]
pub enum ProgramLang {
    Rust,
    NodeJS,
    Python,
    Go,
    Java,
    C,
}

#[derive(Clone, Debug)]
pub enum Program {
    Rust(&'static str),
    NodeJS(String),
    Python(String),
    Go(String),
    Java(String),
    C(String),
}

impl Program {
    pub fn to_string(&self) -> String {
        match self {
            Program::Rust(v) => format!("Rust {v}"),
            Program::NodeJS(v) => format!("Node.js {v}"),
            Program::Python(v) => format!("Python {v}"),
            Program::Go(v) => format!("Go {v}"),
            Program::Java(v) => format!("Java {v}"),
            Program::C(v) => format!("C {v}"),
        }
    }
}
