use std::path::PathBuf;
use crate::core::app::environment::Environment;
use crate::parser::ast::client::ClientLanguage;

#[derive(Clone)]
pub struct ServerConf {
    pub(crate) bind: (String, u16),
    pub(crate) jwt_secret: Option<String>,
    pub(crate) path_prefix: Option<String>,
}

#[derive(Clone)]
pub struct EntityGeneratorConf {
    pub(crate) name: Option<String>,
    pub(crate) provider: Environment,
    pub(crate) dest: PathBuf,
}

#[derive(Clone)]
pub struct ClientGeneratorConf {
    pub(crate) name: Option<String>,
    pub(crate) provider: ClientLanguage,
    pub(crate) dest: PathBuf,
    pub(crate) package: bool,
    pub(crate) host: String,
    pub(crate) object_name: Option<String>,
    pub(crate) git_commit: bool,
}
