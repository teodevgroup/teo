use std::path::PathBuf;
use teo_runtime::path;
use teo_runtime::response::Response;

pub fn serve_static_files(base: impl AsRef<str>, path: impl AsRef<str>) -> path::Result<Response> {
    let base_str = base.as_ref();
    let path_str = path.as_ref();
    let combined_path = PathBuf::from(base_str).join(path_str);
    if combined_path.is_file() {
        Ok(Response::file(combined_path))
    } else {
        Err(path::Error::not_found_message_only())
    }
}
