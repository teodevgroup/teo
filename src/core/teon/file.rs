#[derive(Debug, Clone)]
pub struct TeonFile {
    pub filepath: String,
    pub content_type: Option<String>,
    pub filename: String,
    pub filename_ext: Option<String>,
}

impl TeonFile {
    pub(crate) fn from_json_value(json_value: &serde_json::Value) -> Self {
        let obj = json_value.as_object().unwrap();
        Self {
            filepath: obj.get("filepath").unwrap().as_str().unwrap().to_owned(),
            content_type: obj.get("contentType").map(|c| if c.is_string() { Some(c.as_str().unwrap().to_owned()) } else { None }).flatten(),
            filename: obj.get("filename").unwrap().as_str().unwrap().to_owned(),
            filename_ext: obj.get("filenameExt").map(|c| if c.is_string() { Some(c.as_str().unwrap().to_owned()) } else { None }).flatten(),
        }
    }
}