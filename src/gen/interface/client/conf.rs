use std::path::PathBuf;
use inflector::Inflector;
use crate::gen::interface::client::kind::Kind;

/// # Client conf
///
/// Client configuration.
#[derive(Clone, Debug)]
pub struct ClientConf {
    pub(crate) kind: Kind,
    pub(crate) name: Option<String>,
    pub(crate) dest: PathBuf,
    pub(crate) package: bool,
    pub(crate) host: String,
    pub(crate) object_name: String,
    pub(crate) git_commit: bool,
}

impl ClientConf {

    pub(crate) fn class_name(&self) -> String {
        let first_char = self.object_name.chars().nth(0).unwrap();
        if first_char.is_uppercase() {
            format!("{}Class", self.object_name)
        } else {
            format!("{}{}", self.object_name.chars().nth(0).unwrap().to_uppercase(), &self.object_name[1..])
        }
    }

    /// # Inferred package name
    ///
    /// Infer the package name from last path component
    pub(crate) fn inferred_package_name(&self) -> &str {
        self.dest.file_name().map(|s| s.to_str().unwrap()).unwrap_or("untitled")
    }

    pub(crate) fn inferred_package_name_snake_case(&self) -> String {
        self.inferred_package_name().to_snake_case()
    }

    pub(crate) fn inferred_package_name_camel_case(&self) -> String {
        self.inferred_package_name().to_camel_case()
    }
}
