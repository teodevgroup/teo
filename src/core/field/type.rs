use crate::core::field::Field;

#[derive(Debug, Clone)]
pub(crate) enum FieldType {
    Undefined,
    ObjectId,
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Decimal,
    String,
    Date,
    DateTime,
    Enum(String),
    Vec(Box<Field>),
    HashMap(Box<Field>),
    BTreeMap(Box<Field>),
    HashSet(Box<Field>),
    BTreeSet(Box<Field>),
    Object(String),
}

impl FieldType {

    pub(crate) fn is_string(&self) -> bool {
        match self {
            FieldType::String => true,
            _ => false
        }
    }

    pub(crate) fn is_int(&self) -> bool {
        match self {
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 |
            FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 => true,
            _ => false
        }
    }

    pub(crate) fn is_float(&self) -> bool {
        match self {
            FieldType::F32 | FieldType::F64 => true,
            _ => false
        }
    }

    pub(crate) fn is_bool(&self) -> bool {
        match self {
            FieldType::Bool => true,
            _ => false
        }
    }

    pub(crate) fn is_date(&self) -> bool {
        match self {
            FieldType::Date => true,
            _ => false
        }
    }

    pub(crate) fn is_datetime(&self) -> bool {
        match self {
            FieldType::DateTime => true,
            _ => false
        }
    }

    pub(crate) fn element_field(&self) -> Option<&Field> {
        match self {
            FieldType::Vec(inner) => Some(inner.as_ref()),
            FieldType::HashMap(inner) => Some(inner.as_ref()),
            FieldType::BTreeMap(inner) => Some(inner.as_ref()),
            FieldType::HashSet(inner) => Some(inner.as_ref()),
            FieldType::BTreeSet(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }
}
