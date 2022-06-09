use regex::Regex;

pub fn is_object_id(value: &str) -> bool {
    let regex = Regex::new("[\\da-f]{24}").unwrap();
    regex.is_match(value)
}
