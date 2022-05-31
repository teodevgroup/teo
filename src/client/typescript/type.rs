use crate::core::field::{Availability, Type};


pub(crate) trait ToTypeScriptType {
    fn to_typescript_type(&self, optional: bool) -> String;
    fn to_typescript_filter_type(&self, optional: bool) -> String;
    fn to_typescript_input_type(&self, optional: bool) -> String;
}

impl ToTypeScriptType for Type {
    fn to_typescript_type(&self, optional: bool) -> String {
        let mut base: String = match self {
            Type::Undefined => panic!(),
            Type::ObjectId | Type::String | Type::Date | Type::DateTime => "string".to_string(),
            Type::Bool => "boolean".to_string(),
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 | Type::F32 | Type::F64 => "number".to_string(),
            Type::Enum(name) => name.to_string(),
            Type::Vec(internal) => internal.r#type.to_typescript_type(internal.availability == Availability::Optional) + "[]",
            Type::Map(_) => panic!(),
            Type::Object(name) => name.to_string(),
        };
        if optional {
            base + " | undefined"
        } else {
            base
        }
    }

    fn to_typescript_filter_type(&self, optional: bool) -> String {
        let mut base: String = match self {
            Type::Undefined => panic!(),
            Type::ObjectId => "string | ObjectId".to_string(),
            Type::String => "string | String".to_string(),
            Type::Date => "string | Date".to_string(),
            Type::DateTime => "string | Date | Date".to_string(),
            Type::Bool => "boolean | Boolean".to_string(),
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 | Type::F32 | Type::F64 => "number | Number".to_string(),
            Type::Enum(name) => format!(r#"{name} | EnumFilter<{name}>"#),
            Type::Vec(internal) => "undefined | Unimplemented".to_string(),
            Type::Map(_) => panic!(),
            Type::Object(name) => "undefined | Unimplemented".to_string(),
        };
        if optional {
            base + "NullableFilter | null"
        } else {
            base + "Filter"
        }
    }

    fn to_typescript_input_type(&self, optional: bool) -> String {
        let mut base: String = match self {
            Type::Undefined => panic!(),
            Type::ObjectId | Type::String | Type::Date | Type::DateTime => "string".to_string(),
            Type::Bool => "boolean".to_string(),
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 | Type::F32 | Type::F64 => "number".to_string(),
            Type::Enum(name) => name.to_string(),
            Type::Vec(internal) => internal.r#type.to_typescript_type(internal.availability == Availability::Optional) + "[]",
            Type::Map(_) => panic!(),
            Type::Object(name) => name.to_string(),
        };
        if optional {
            base + " | null"
        } else {
            base
        }
    }
}
