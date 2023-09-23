use std::collections::HashSet;
use std::ops::BitOr;
use maplit::hashset;
use once_cell::sync::Lazy;
use crate::app::app_ctx::AppCtx;
use crate::core::field::field::Field;
use crate::core::r#enum::Enum;

#[derive(Debug, Clone)]
pub enum FieldType {
    #[cfg(feature = "data-source-mongodb")]
    ObjectId,
    Bool,
    I32,
    I64,
    F32,
    F64,
    Decimal,
    String,
    Date,
    DateTime,
    Enum(Vec<String>),
    Vec(Box<Field>),
    HashMap(Box<Field>),
    BTreeMap(Box<Field>),
    Object(String),
}

impl FieldType {

    pub fn is_scalar(&self) -> bool {
        use self::FieldType::*;
        match self {
            Vec(_) | HashMap(_) | BTreeMap(_) | Object(_) | Enum(_) => false,
            _ => true,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            FieldType::String => true,
            _ => false
        }
    }

    pub fn is_enum(&self) -> bool {
        match self {
            FieldType::Enum(_) => true,
            _ => false,
        }
    }

    pub fn enum_name(&self) -> &str {
        match self {
            FieldType::Enum(n) => n.last().unwrap(),
            _ => panic!(),
        }
    }

    pub fn unwrap_enum(&self) -> &Enum {
        let path = match self {
            FieldType::Enum(s) => s.iter().map(|s| s.as_str()).collect(),
            _ => panic!(),
        };
        AppCtx::get().unwrap().r#enum(path).unwrap().unwrap()
    }

    pub fn is_int(&self) -> bool {
        match self {
            FieldType::I32 | FieldType::I64 => true,
            _ => false
        }
    }

    pub fn is_int32(&self) -> bool {
        match self {
            FieldType::I32 => true,
            _ => false
        }
    }

    pub fn is_int64(&self) -> bool {
        match self {
            FieldType::I64 => true,
            _ => false
        }
    }

    pub fn is_float32(&self) -> bool {
        match self {
            FieldType::F32 => true,
            _ => false
        }
    }

    pub fn is_float64(&self) -> bool {
        match self {
            FieldType::F64 => true,
            _ => false
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            FieldType::F32 | FieldType::F64 => true,
            _ => false
        }
    }

    pub fn is_decimal(&self) -> bool {
        match self {
            FieldType::Decimal => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        self.is_int() || self.is_float() || self.is_decimal()
    }

    pub fn is_bool(&self) -> bool {
        match self {
            FieldType::Bool => true,
            _ => false
        }
    }

    #[cfg(feature = "data-source-mongodb")]
    pub fn is_object_id(&self) -> bool {
        match self {
            FieldType::ObjectId => true,
            _ => false,
        }
    }

    pub fn is_date(&self) -> bool {
        match self {
            FieldType::Date => true,
            _ => false
        }
    }

    pub fn is_datetime(&self) -> bool {
        match self {
            FieldType::DateTime => true,
            _ => false
        }
    }

    pub fn is_vec(&self) -> bool {
        match self {
            FieldType::Vec(_) => true,
            _ => false,
        }
    }

    pub fn element_field(&self) -> Option<&Field> {
        match self {
            FieldType::Vec(inner) => Some(inner.as_ref()),
            FieldType::HashMap(inner) => Some(inner.as_ref()),
            FieldType::BTreeMap(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }

    pub(crate) fn default_updators(&self) -> &HashSet<&str> {
        &DEFAULT_UPDATORS
    }

    pub(crate) fn updators(&self) -> &HashSet<&str> {
        if self.is_number() {
            &NUMBER_UPDATORS
        } else if self.is_vec() {
            &VEC_UPDATORS
        } else {
            &DEFAULT_UPDATORS
        }
    }

    pub(crate) fn filters(&self) -> &HashSet<&str> {
        match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => &DEFAULT_FILTERS,
            FieldType::Bool => &BOOL_FILTERS,
            FieldType::I32 | FieldType::I64 |
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

    pub(crate) fn filters_with_aggregates(&self) -> &HashSet<&str> {
        match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => &DEFAULT_FILTERS_WITH_AGGREGATE,
            FieldType::Bool => &BOOL_FILTERS,
            FieldType::I32 | FieldType::I64 |
            FieldType::F32 | FieldType::F64 | FieldType::Decimal => &NUMBER_FILTERS_WITH_AGGREGATE,
            FieldType::Date | FieldType::DateTime => &DEFAULT_FILTERS_WITH_AGGREGATE,
            FieldType::String => &STRING_FILTERS_WITH_AGGREGATE,
            FieldType::Enum(_) => &ENUM_FILTERS_WITH_AGGREGATE,
            FieldType::Vec(_) => &VEC_FILTERS,
            FieldType::HashMap(_) => &MAP_FILTERS,
            FieldType::BTreeMap(_) => &MAP_FILTERS,
            FieldType::Object(_) => panic!("Object filter is not implemented.")
        }
    }
}

pub trait FieldTypeOwner {
    fn field_type(&self) -> &FieldType;
    fn is_optional(&self) -> bool;
    fn set_field_type(&mut self, field_type: FieldType);
}

static DEFAULT_UPDATORS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"set"}
});
static NUMBER_UPDATORS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"set", "increment", "decrement", "multiply", "divide"}
});
static VEC_UPDATORS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"set", "push"}
});
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
static STRING_FILTERS_WITH_AGGREGATE: Lazy<HashSet<&str>> = Lazy::new(|| {
    STRING_FILTERS.bitor(&hashset!{"_min", "_max", "_count"})
});
static NUMBER_FILTERS_WITH_AGGREGATE: Lazy<HashSet<&str>> = Lazy::new(|| {
    DEFAULT_FILTERS.bitor(&hashset!{"_min", "_max", "_count", "_avg", "_sum"})
});
static DEFAULT_FILTERS_WITH_AGGREGATE: Lazy<HashSet<&str>> = Lazy::new(|| {
    DEFAULT_FILTERS.bitor(&hashset!{"_min", "_max", "_count"})
});
static ENUM_FILTERS_WITH_AGGREGATE: Lazy<HashSet<&str>> = Lazy::new(|| {
    ENUM_FILTERS.bitor(&hashset!{"_count"})
});
