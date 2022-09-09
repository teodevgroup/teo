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
    Enum(&'static str),
    Vec(Box<Field>),
    Map(Box<Field>),
    Object(&'static str)
}
