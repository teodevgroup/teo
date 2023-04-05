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

pub fn constantize<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_snake_case().to_uppercase())
}

pub fn escape_swift<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    if vec!["is", "where", "break", "case", "continue", "catch", "default", "defer", "do", "else", "for", "fallthrough", "for", "in", "repeat", "guard", "while", "return", "throw"].contains(&s.as_str()) {
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

pub fn escape_dart<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    if s.starts_with("_") {
        Ok(s.replace("_", "$"))
    } else {
        match s.as_str() {
            "is" => Ok("matches".to_owned()),
            "AND" => Ok("$and".to_owned()),
            "OR" => Ok("$or".to_owned()),
            "NOT" => Ok("$not".to_owned()),
            _ => Ok(s),
        }
    }
}

pub fn escape_kotlin<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    if ["is"].contains(&s.as_str()) {
        Ok(format!("`{}`", s.as_str()))
    } else {
        Ok(s)
    }
}

pub fn type_annotation_kotlin<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    match s.as_str() {
        "Any" => Ok("@Serializable(AnySerializer::class) Any".to_string()),
        "LocalDate" => Ok("@Serializable(DateSerializer::class) LocalDate".to_string()),
        "OffsetDateTime" => Ok("@Serializable(DateTimeSerializer::class) OffsetDateTime".to_string()),
        _ => Ok(s),
    }
}
