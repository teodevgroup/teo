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
pub enum LanguagePlatform {
    Rust(&'static str),
    NodeJS(String),
    Python(String),
    Go(String),
    Java(String),
    C(String),
}

impl LanguagePlatform {
    pub fn to_string(&self) -> String {
        match self {
            LanguagePlatform::Rust(v) => format!("Rust {v}"),
            LanguagePlatform::NodeJS(v) => format!("Node.js {v}"),
            LanguagePlatform::Python(v) => format!("Python {v}"),
            LanguagePlatform::Go(v) => format!("Go {v}"),
            LanguagePlatform::Java(v) => format!("Java {v}"),
            LanguagePlatform::C(v) => format!("C {v}"),
        }
    }
}
