use std::path::PathBuf;
use crate::core::app::environment::Environment;

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
