use std::path::PathBuf;
use std::env::current_dir;
use std::fs::{File};
use std::io::Write;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;

pub(crate) struct Generator {
    base_dir: PathBuf,
}

impl Generator {

    pub(crate) fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into()
        }
    }

    pub(crate) async fn ensure_directory<D: Into<String>>(&self, dir_name: D) -> std::io::Result<()> {
        let dirname = self.base_dir.join(dir_name.into());
        if !dirname.exists() {
            create_dir_all(dirname)
        } else {
            Ok(())
        }
    }

    pub(crate) async fn clear_directory<D: Into<String>>(&self, dir_name: D) -> std::io::Result<()> {
        let dirname = self.base_dir.join(dir_name.into());
        if !&dirname.exists() {
            create_dir_all(&dirname)
        } else {
            remove_dir_all(&dirname)?;
            create_dir_all(&dirname)
        }
    }

    pub(crate) async fn generate_file<F: Into<String>, S: Into<String>>(&self, file_name: F, content: S) -> std::io::Result<()> {
        let filename = self.base_dir.join(file_name.into());
        println!("{:?}", filename);
        let mut output_file = File::create(filename)?;
        write!(output_file, "{}", content.into())
    }
}
