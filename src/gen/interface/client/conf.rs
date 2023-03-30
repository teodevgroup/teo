use std::path::PathBuf;
use crate::gen::interface::client::kind::Kind;

#[derive(Clone)]
pub struct Conf {
    pub(crate) kind: Kind,
    pub(crate) name: Option<String>,
    pub(crate) dest: PathBuf,
    pub(crate) package: bool,
    pub(crate) package_name: Option<String>,
    pub(crate) host: String,
    pub(crate) object_name: String,
    pub(crate) git_commit: bool,
}
