use std::env::current_dir;
use std::fs::{File};
use std::io::Write;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;

pub mod code;


pub async fn ensure_directory<D: Into<String>>(dir_name: D) -> std::io::Result<()> {
    let dirname = current_dir().unwrap().join(dir_name.into());
    if !dirname.exists() {
        create_dir_all(dirname)
    } else {
        Ok(())
    }
}

pub async fn clear_directory<D: Into<String>>(dir_name: D) -> std::io::Result<()> {
    let dirname = current_dir().unwrap().join(dir_name.into());
    if !&dirname.exists() {
        create_dir_all(&dirname)
    } else {
        remove_dir_all(&dirname)?;
        create_dir_all(&dirname)
    }
}

pub async fn generate_file<F: Into<String>, S: Into<String>>(file_name: F, content: S) -> std::io::Result<()> {
    let filename = current_dir().unwrap().join(file_name.into());
    println!("{:?}", filename);
    let mut output_file = File::create(filename)?;
    write!(output_file, "{}", content.into())
}
