use inflector::Inflector;

pub fn camelcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_camel_case())
}

pub fn pascalcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_pascal_case())
}

pub fn snakecase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_snake_case())
}

pub fn wordcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_word_case())
}

pub fn pluralize<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.to_plural())
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

pub fn escape_ts<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    if vec!["for", "break", "case", "continue", "catch", "do", "else", "for", "while", "return", "throw"].contains(&s.as_str()) {
        Ok(format!("'{}'", s))
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

pub fn rust_unwrap_vec<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let mut s = s.to_string();
    s = s.strip_prefix("Vec<").unwrap().to_owned();
    s = s.strip_suffix(">").unwrap().to_owned();
    Ok(s)
}

pub fn rust_as<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let string = s.to_string();
    let s = string.as_str();
    Ok(match s {
        "String" => "str",
        "NaiveDate" => "date",
        "DateTime<Utc>" => "datetime",
        "BigDecimal" => "decimal",
        "ObjectId" => "object_id",
        _ => s,
    }.to_string())
}

pub fn rust_ref_if_needed<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let string = s.to_string();
    Ok(if string.chars().nth(0).unwrap().is_uppercase() {
        if string.as_str() == "String" {
            "&str".to_owned()
        } else if string.as_str() == "BigDecimal" {
            "BigDecimal".to_owned()
        } else {
            "&".to_owned() + string.as_str()
        }
    } else {
        string
    })
}

pub fn append_slash<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let string = s.to_string();
    if string.ends_with("/") {
        Ok(string)
    } else {
        Ok(string + "/")
    }
}