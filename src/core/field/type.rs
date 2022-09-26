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
    Map(Box<Field>),
    Object(String),
}

impl FieldType {
    pub(crate) fn element_field(&self) -> Option<&Field> {
        match self {
            FieldType::Vec(inner) => Some(inner.as_ref()),
            FieldType::Map(inner) => Some(inner.as_ref()),
            _ => None,
        }
    }
}
