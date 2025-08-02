pub fn remove_path_prefix<'a>(path: &'a str, prefix: Option<&String>) -> &'a str {
    if let Some(prefix) = prefix {
        let prefix = prefix.as_str();
        let mut prefix = prefix;
        prefix = prefix.trim_end_matches("/");
        let result = path.strip_prefix(prefix).unwrap();
        if result == "" {
            "/"
        } else {
            result
        }
    } else {
        path
    }
}
