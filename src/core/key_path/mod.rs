#[derive(Clone, Debug)]
pub enum KeyPathItem {
    String(String),
    Number(usize),
}

impl KeyPathItem {

    pub fn is_string(&self) -> bool {
        use KeyPathItem::*;
        match self {
            String(_) => true,
            Number(_) => false,
        }
    }

    pub fn is_number(&self) -> bool {
        use KeyPathItem::*;
        match self {
            String(_) => false,
            Number(_) => true,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        use KeyPathItem::*;
        match self {
            String(v) => Some(&v),
            Number(_) => None,
        }
    }

    pub fn as_number(&self) -> Option<usize> {
        use KeyPathItem::*;
        match self {
            String(_) => None,
            Number(v) => Some(*v),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            KeyPathItem::String(s) => s.to_owned(),
            KeyPathItem::Number(n) => n.to_string(),
        }
    }
}

pub trait ToDottedString {
    fn to_dotted_string(&self) -> String;
}

impl ToDottedString for Vec<KeyPathItem> {
    fn to_dotted_string(&self) -> String {
        self.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(".")
    }
}
