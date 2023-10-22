use teo_result::{Error, Result};
use std::env::current_dir;
use std::path::PathBuf;

pub(super) fn find_main_schema_file(file: Option<&String>) -> Result<PathBuf> {
    let current_dir = match current_dir() {
        Ok(current_dir) => current_dir,
        Err(e) => Err(Error::new(format!("{}", e)))?,
    };
    if let Some(file) = file {
        let file_path = current_dir.join(&file);
        if file_path.is_file() {
            return Ok(file_path);
        } else {
            return Err(Error::new(format!("cannot find schema file '{}'", file)));
        }
    }
    let default = vec!["schema.teo", "index.teo", "src/schema.teo", "src/index.teo", "schema/index.teo", "src/schema/index.teo"];
    for name in default {
        let file_path = current_dir.join(name);
        if file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(Error::new("cannot find default schema file"))
}

