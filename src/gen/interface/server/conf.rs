use std::path::PathBuf;
use crate::app::program::ProgramLang;

#[derive(Clone, Debug)]
pub struct EntityGeneratorConf {
    pub(crate) name: Option<String>,
    pub(crate) provider: ProgramLang,
    pub(crate) dest: PathBuf,
}
