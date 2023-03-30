use inflector::Inflector;

pub fn camelcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_camel_case())
}
pub fn pascalcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_pascal_case())
}
pub fn capitalize_first<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    let mut c = s.chars();
    let result = match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    };
    Ok(result)
}
pub fn decapitalize<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    let mut c = s.chars();
    let result = match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    };
    Ok(result)
}
pub fn escape_swift<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    if vec!["where", "break", "case", "continue", "catch", "default", "defer", "do", "else", "for", "fallthrough", "for", "in", "repeat", "guard", "while", "return", "throw"].contains(&s.as_str()) {
        Ok(format!("`{}`", s))
    } else {
        Ok(s)
    }
}
pub fn escape_csharp<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    if vec!["is", "where"].contains(&s.as_str()) {
        Ok(format!("@{}", s))
    } else {
        Ok(s)
    }
}
