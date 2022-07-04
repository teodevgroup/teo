use crate::core::field::{Optionality};
use crate::core::field_type::FieldType;


pub(crate) trait ToTypeScriptType {
    fn to_typescript_type(&self, optional: bool) -> String;
    fn to_typescript_filter_type(&self, optional: bool) -> String;
    fn to_typescript_create_input_type(&self, optional: bool) -> String;
    fn to_typescript_update_input_type(&self, optional: bool) -> String;
    fn to_typescript_update_operation_input(&self, optional: bool) -> String;
}

impl ToTypeScriptType for FieldType {
    fn to_typescript_type(&self, optional: bool) -> String {
        let base: String = match self {
            FieldType::Undefined => panic!(),
            FieldType::ObjectId | FieldType::String | FieldType::Date | FieldType::DateTime => "string".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Decimal => "string".to_string(),
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
        let mut with_generic = false;
        let base: String = match self {
            FieldType::Undefined => panic!(),
            FieldType::ObjectId => "string | ObjectId".to_string(),
            FieldType::String => "string | String".to_string(),
            FieldType::Date => "string | Date | Date".to_string(),
            FieldType::DateTime => "string | Date | DateTime".to_string(),
            FieldType::Bool => "boolean | Bool".to_string(),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 => "number | Number".to_string(),
            FieldType::Decimal => "string | DecimalFilter".to_string(),
            FieldType::Enum(name) => {
                with_generic = true;
                if optional {
                    format!(r#"{name} | EnumNullableFilter<{name}> | null"#)
                } else {
                    format!(r#"{name} | EnumFilter<{name}>"#)
                }
            },
            FieldType::Vec(internal) => {
                with_generic = true;
                let create_type = internal.field_type.to_typescript_create_input_type(false);
                if optional {
                    format!("{create_type}[] | ArrayNullableFilter<{create_type}> | null")
                } else {
                    format!("{create_type}[] | ArrayFilter<{create_type}>")
                }
            },
            FieldType::Map(_) => panic!(),
            FieldType::Object(_name) => "undefined | Unimplemented".to_string(),
        };
        if !with_generic {
            if optional {
                base + "NullableFilter | null"
            } else {
                base + "Filter"
            }
        } else {
            base
        }
    }

    fn to_typescript_create_input_type(&self, optional: bool) -> String {
        let base: String = match self {
            FieldType::Undefined => panic!(),
            FieldType::ObjectId | FieldType::String | FieldType::Decimal => "string".to_string(),
            FieldType::Date | FieldType::DateTime => "Date | string".to_string(),
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

    fn to_typescript_update_operation_input(&self, optional: bool) -> String {
        let mut generic = "".to_owned();
        let base: &str = match self {
            FieldType::Undefined => panic!(),
            FieldType::ObjectId => "ObjectId",
            FieldType::String => "String",
            FieldType::Date => "Date",
            FieldType::DateTime => "DateTime",
            FieldType::Decimal => "Decimal",
            FieldType::Bool => "Bool",
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 => "Number",
            FieldType::Enum(name) => {
                generic = format!("<{name}>");
                "Enum"
            },
            FieldType::Vec(inner) => {
                let create_type = inner.field_type.to_typescript_create_input_type(false);
                generic = format!("<{create_type}>");
                "Array"
            },
            _ => panic!(),
        };
        let suffix = "FieldUpdateOperationsInput";
        let prefix = if optional { "Nullable" } else { "" };
        format!("{prefix}{base}{suffix}{generic}")
    }

    fn to_typescript_update_input_type(&self, optional: bool) -> String {
        let update_operation = self.to_typescript_update_operation_input(optional);
        let create_input = self.to_typescript_create_input_type(optional);
        return format!("{update_operation} | {create_input}");
    }
}
