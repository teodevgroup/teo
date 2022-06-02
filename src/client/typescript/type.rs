use crate::core::field::{Optionality};
use crate::core::field_type::FieldType;


pub(crate) trait ToTypeScriptType {
    fn to_typescript_type(&self, optional: bool) -> String;
    fn to_typescript_filter_type(&self, optional: bool) -> String;
    fn to_typescript_input_type(&self, optional: bool) -> String;
}

impl ToTypeScriptType for FieldType {
    fn to_typescript_type(&self, optional: bool) -> String {
        let mut base: String = match self {
            FieldType::Undefined => panic!(),
            FieldType::ObjectId | FieldType::String | FieldType::Date | FieldType::DateTime => "string".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Enum(name) => name.to_string(),
            FieldType::Vec(internal) => internal.field_type.to_typescript_type(internal.optionality == Optionality::Optional) + "[]",
            FieldType::Map(_) => panic!(),
            FieldType::Object(name) => name.to_string(),
        };
        if optional {
            base + " | undefined"
        } else {
            base
        }
    }

    fn to_typescript_filter_type(&self, optional: bool) -> String {
        let mut base: String = match self {
            FieldType::Undefined => panic!(),
            FieldType::ObjectId => "string | ObjectId".to_string(),
            FieldType::String => "string | String".to_string(),
            FieldType::Date => "string | Date".to_string(),
            FieldType::DateTime => "string | Date | Date".to_string(),
            FieldType::Bool => "boolean | Boolean".to_string(),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 => "number | Number".to_string(),
            FieldType::Enum(name) => format!(r#"{name} | EnumFilter<{name}>"#),
            FieldType::Vec(internal) => "undefined | Unimplemented".to_string(),
            FieldType::Map(_) => panic!(),
            FieldType::Object(name) => "undefined | Unimplemented".to_string(),
        };
        if optional {
            base + "NullableFilter | null"
        } else {
            base + "Filter"
        }
    }

    fn to_typescript_input_type(&self, optional: bool) -> String {
        let mut base: String = match self {
            FieldType::Undefined => panic!(),
            FieldType::ObjectId | FieldType::String | FieldType::Date | FieldType::DateTime => "string".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Enum(name) => name.to_string(),
            FieldType::Vec(internal) => internal.field_type.to_typescript_type(internal.optionality == Optionality::Optional) + "[]",
            FieldType::Map(_) => panic!(),
            FieldType::Object(name) => name.to_string(),
        };
        if optional {
            base + " | null"
        } else {
            base
        }
    }
}
