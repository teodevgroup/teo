use rustc_version::version;

fn main() {
    // export rustc version to the executable
    let version_string = version().unwrap().to_string();
    println!("cargo:rustc-env=TEO_RUSTC_VERSION={}", version_string);
}
