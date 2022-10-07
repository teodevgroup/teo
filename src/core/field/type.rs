use std::collections::HashSet;
use maplit::hashset;
use once_cell::sync::Lazy;
use crate::core::field::Field;

#[derive(Debug, Clone)]
pub(crate) enum FieldType {
    Undefined,
    #[cfg(feature = "data-source-mongodb")]
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

    pub(crate) fn is_decimal(&self) -> bool {
        match self {
            FieldType::Decimal => true,
            _ => false,
        }
    }

    pub(crate) fn is_number(&self) -> bool {
        self.is_int() || self.is_float() || self.is_decimal()
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
            _ => None,
        }
    }

    pub(crate) fn filters(&self) -> &HashSet<&str> {
        match self {
            FieldType::Undefined => panic!("Field type cannot be undefined."),
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => &DEFAULT_FILTERS,
            FieldType::Bool => &BOOL_FILTERS,
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 |
            FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 |
            FieldType::F32 | FieldType::F64 | FieldType::Date | FieldType::DateTime |
            FieldType::Decimal => &DEFAULT_FILTERS,
            FieldType::String => &STRING_FILTERS,
            FieldType::Enum(_) => &ENUM_FILTERS,
            FieldType::Vec(_) => &VEC_FILTERS,
            FieldType::HashMap(_) => &MAP_FILTERS,
            FieldType::BTreeMap(_) => &MAP_FILTERS,
            FieldType::Object(_) => panic!("Object filter is not implemented.")
        }
    }
}

static BOOL_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"equals", "not"}
});
static STRING_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"equals", "not", "gt", "gte", "lt", "lte", "in", "notIn", "contains", "startsWith", "endsWith", "matches", "mode"}
});
static DEFAULT_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"equals", "not", "gt", "gte", "lt", "lte", "in", "notIn"}
});
static ENUM_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"equals", "not", "in", "notIn"}
});
static VEC_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"equals", "has", "hasEvery", "hasSome", "isEmpty", "length"}
});
static MAP_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"equals", "has", "hasEvery", "hasSome", "isEmpty", "length", "hasKey"}
});
