use std::path::{Path, PathBuf};
use std::fs::{File};
use std::io::Write;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;
use crate::gen::internal::message::{green_message, red_message, yellow_message};
use pathdiff::diff_paths;
use crate::core::result::{IntoTeoResult, Result};

pub(in crate::gen) struct FileUtil {
    base_dir: PathBuf,
}

impl FileUtil {

    pub(in crate::gen) fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into()
        }
    }

    pub(in crate::gen) async fn ensure_root_directory(&self) -> Result<()> {
        if !self.base_dir.exists() {
            yellow_message("create", diff_paths(&self.base_dir, std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_string());
            create_dir_all(&self.base_dir)?;
        }
        Ok(())
    }

    pub(in crate::gen) async fn ensure_directory<D: Into<String>>(&self, dir_name: D) -> Result<()> {
        let dirname = self.base_dir.join(dir_name.into());
        if !dirname.exists() {
            yellow_message("create", diff_paths(&dirname, std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_string());
            create_dir_all(dirname).io_result_into_teo_result()
        } else {
            Ok(())
        }
    }

    pub(in crate::gen) async fn clear_root_directory(&self) -> Result<()> {
        if !&self.base_dir.exists() {
            yellow_message("create", diff_paths(&self.base_dir, std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_string());
            create_dir_all(&self.base_dir).io_result_into_teo_result()
        } else {
            red_message("clear", diff_paths(&self.base_dir, std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_string());
            remove_dir_all(&self.base_dir).io_result_into_teo_result()?;
            create_dir_all(&self.base_dir).io_result_into_teo_result()
        }
    }

    pub(in crate::gen) async fn clear_directory<D: Into<String>>(&self, dir_name: D) -> Result<()> {
        let dirname = self.base_dir.join(dir_name.into());
        if !&dirname.exists() {
            yellow_message("create", diff_paths(&dirname, std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_string());
            create_dir_all(&dirname).io_result_into_teo_result()
        } else {
            red_message("clear", diff_paths(&dirname, std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_string());
            remove_dir_all(&dirname).io_result_into_teo_result()?;
            create_dir_all(&dirname).io_result_into_teo_result()
        }
    }

    pub(in crate::gen) async fn generate_file<F: Into<String>, S: AsRef<str>>(&self, file_name: F, content: S) -> Result<()> {
        let filename = self.base_dir.join(file_name.into());
        let mut output_file = File::create(&filename)?;
        green_message("create", diff_paths(&filename, std::env::current_dir().unwrap()).unwrap().to_str().unwrap().to_string());
        write!(output_file, "{}", content.as_ref()).io_result_into_teo_result()
    }

    pub(in crate::gen) async fn generate_file_if_not_exist<F: AsRef<str>, S: AsRef<str>>(&self, file_name: F, content: S) -> Result<bool> {
        let filename = self.base_dir.join(PathBuf::from(file_name.as_ref()));
        if !filename.exists() {
            self.generate_file(file_name.as_ref().to_owned(), content.as_ref().to_owned()).await?;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub(in crate::gen) fn find_file_upwards(&self, name: impl AsRef<str>) -> Option<PathBuf> {
        let mut path: PathBuf = self.base_dir.clone();
        let file = Path::new(name.as_ref());
        loop {
            path.push(file);

            if path.is_file() {
                break Some(path);
            }

            if !(path.pop() && path.pop()) { // remove file && remove parent
                break None;
            }
        }
    }

    pub(in crate::gen) fn get_base_dir(&self) -> &Path {
        &self.base_dir
    }

    pub(in crate::gen) fn get_file_path(&self, name: impl AsRef<str>) -> PathBuf {
        self.base_dir.join(name.as_ref())
    }
}
