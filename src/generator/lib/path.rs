use std::path::{Path, PathBuf};
use path_absolutize::*;

pub(crate) fn relative_to_absolute(rel: impl AsRef<str>) -> PathBuf {
    let p = Path::new(rel.as_ref());
    p.absolutize().unwrap().to_path_buf()
}
