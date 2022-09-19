#[derive(Clone)]
pub enum KeyPathItem {
    String(String),
    Number(i32),
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

    pub fn as_number(&self) -> Option<i32> {
        use KeyPathItem::*;
        match self {
            String(_) => None,
            Number(v) => Some(*v),
        }
    }
}
