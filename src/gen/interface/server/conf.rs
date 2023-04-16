use std::path::PathBuf;
use crate::app::program::ProgramLang;
use crate::prelude::Value;

#[derive(Clone)]
pub struct EntityGeneratorConf {
    pub(crate) name: Option<String>,
    pub(crate) provider: ProgramLang,
    pub(crate) dest: PathBuf,
}
