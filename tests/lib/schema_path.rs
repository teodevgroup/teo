use std::path::Path;

pub(crate) fn schema_path_args(file: &str, schema_file_name: &str) -> Vec<String> {
    let path = Path::new(file);
    let source = path.parent().unwrap().join(schema_file_name);
    vec!["teo".to_owned(), "serve".to_owned(), "--schema".to_owned(), source.to_str().unwrap().to_owned()]
}
