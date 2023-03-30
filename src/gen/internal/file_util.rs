use std::path::{Path, PathBuf};
use std::fs::{File};
use std::io::Write;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;

pub(in crate::gen) struct FileUtil {
    base_dir: PathBuf,
}

impl FileUtil {

    pub(in crate::gen) fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into()
        }
    }

    pub(in crate::gen) async fn ensure_root_directory(&self) -> std::io::Result<()> {
        if !self.base_dir.exists() {
            create_dir_all(&self.base_dir)?;
        }
        Ok(())
    }

    pub(in crate::gen) async fn ensure_directory<D: Into<String>>(&self, dir_name: D) -> std::io::Result<()> {
        let dirname = self.base_dir.join(dir_name.into());
        if !dirname.exists() {
            create_dir_all(dirname)
        } else {
            Ok(())
        }
    }

    pub(in crate::gen) async fn clear_root_directory(&self) -> std::io::Result<()> {
        if !&self.base_dir.exists() {
            create_dir_all(&self.base_dir)
        } else {
            remove_dir_all(&self.base_dir)?;
            create_dir_all(&self.base_dir)
        }
    }

    pub(in crate::gen) async fn clear_directory<D: Into<String>>(&self, dir_name: D) -> std::io::Result<()> {
        let dirname = self.base_dir.join(dir_name.into());
        if !&dirname.exists() {
            create_dir_all(&dirname)
        } else {
            remove_dir_all(&dirname)?;
            create_dir_all(&dirname)
        }
    }

    pub(in crate::gen) async fn generate_file<F: Into<String>, S: AsRef<str>>(&self, file_name: F, content: S) -> std::io::Result<()> {
        let filename = self.base_dir.join(file_name.into());
        println!("{}", filename.as_os_str().to_str().unwrap());
        let mut output_file = File::create(filename)?;
        write!(output_file, "{}", content.as_ref())
    }

    pub(in crate::gen) async fn generate_file_if_not_exist<F: AsRef<str>, S: AsRef<str>>(&self, file_name: F, content: S) -> std::io::Result<bool> {
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
