
use crate::core::field::r#type::FieldType;

pub(crate) trait ToCSharpType {
    fn to_csharp_type(&self, optional: bool) -> String;
    fn to_csharp_filter_type(&self, optional: bool) -> String;
    fn to_csharp_create_input_type(&self, optional: bool, no_question_mark: bool) -> String;
    fn to_csharp_update_input_type(&self, optional: bool, no_question_mark: bool) -> String;
    fn to_csharp_update_operation_input(&self, optional: bool) -> String;
}

fn to_optional(t: &str, optional: bool) -> String {
    if !optional {
        t.to_owned()
    } else {
        format!("Optional<{t}>")
    }
}

fn nullable_if_optional(optional: bool) -> &'static str {
    if optional {
        "Nullable"
    } else {
        ""
    }
}

fn one_of(t0: String, t1: String) -> String {
    format!("OneOf<{t0}, {t1}>")
}

fn array_prefix(t: &str) -> &str {
    if t == "string" {
        "Ref"
    } else if t == "DateOnly" {
        "Ref"
    } else if t == "DateTime" {
        "Ref"
    } else {
        "Value"
    }
}

impl ToCSharpType for FieldType {
    fn to_csharp_type(&self, optional: bool) -> String {
        let base: String = match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "string".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::I8 => "sbyte".to_string(),
            FieldType::U8 => "byte".to_string(),
            FieldType::I16 => "short".to_string(),
            FieldType::U16 => "ushort".to_string(),
            FieldType::I32 => "int".to_string(),
            FieldType::U32 => "uint".to_string(),
            FieldType::I64 => "long".to_string(),
            FieldType::U64 => "ulong".to_string(),
            FieldType::I128 => "long".to_string(),
            FieldType::U128 => "ulong".to_string(),
            FieldType::F32 => "float".to_string(),
            FieldType::F64 => "double".to_string(),
            FieldType::Decimal => "decimal".to_string(),
            FieldType::Date => "DateOnly".to_string(),
            FieldType::DateTime => "DateTime".to_string(),
            FieldType::Enum(name) => name.to_string(),
            FieldType::Vec(internal) => internal.field_type().to_csharp_type(internal.optionality.is_optional()) + "[]",
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => name.to_string(),
        };
        if optional {
            base + "?"
        } else {
            base
        }
    }

    fn to_csharp_filter_type(&self, optional: bool) -> String {
        let nullable = nullable_if_optional(optional);
        let base_type = to_optional(&self.to_csharp_type(false), optional);
        match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => one_of(base_type, format!("ObjectId{nullable}Filter")),
            FieldType::String => one_of(base_type, format!("String{nullable}Filter")),
            FieldType::Date => one_of(base_type, format!("DateOnly{nullable}Filter")),
            FieldType::DateTime => one_of(base_type, format!("DateTime{nullable}Filter")),
            FieldType::Bool => one_of(base_type, format!("Bool{nullable}Filter")),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 | FieldType::Decimal => {
                let number_type = self.to_csharp_type(false);
                one_of(base_type, format!("Number{nullable}Filter<{number_type}>"))
            },
            FieldType::Enum(_name) => {
                let enum_type = self.to_csharp_type(false);
                one_of(base_type, format!("Enum{nullable}Filter<{enum_type}>"))
            },
            FieldType::Vec(internal) => {
                let internal_type = internal.field_type().to_csharp_type(false);
                let prefix = array_prefix(&internal_type);
                one_of(base_type, format!("{prefix}Array{nullable}Filter<{internal_type}>"))
            },
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(_name) => "Unimplemented".to_string(),
        }
    }

    fn to_csharp_create_input_type(&self, optional: bool, no_question_mark: bool) -> String {
        let mut retval = self.to_csharp_type(false);
        if optional {
            retval = format!("Optional<{retval}>");
            if !no_question_mark {
                retval += "?";
            }
        }
        retval
    }

    fn to_csharp_update_input_type(&self, optional: bool, no_question_mark: bool) -> String {
        let create_input = self.to_csharp_create_input_type(optional, true);
        let operation_input = self.to_csharp_update_operation_input(optional);
        one_of(create_input, operation_input) + if no_question_mark { "" } else { "?" }
    }

    fn to_csharp_update_operation_input(&self, optional: bool) -> String {
        let prefix = if optional { "Nullable" } else { "" };
        match self {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => format!("{prefix}ObjectIdFieldUpdateOperationsInput"),
            FieldType::String => format!("{prefix}StringFieldUpdateOperationsInput"),
            FieldType::Date => format!("{prefix}DateOnlyFieldUpdateOperationsInput"),
            FieldType::DateTime => format!("{prefix}DateTimeFieldUpdateOperationsInput"),
            FieldType::Bool => format!("{prefix}BoolFieldUpdateOperationsInput"),
            FieldType::I8 | FieldType::I16 | FieldType::I32 | FieldType::I64 | FieldType::I128 | FieldType::U8 | FieldType::U16 | FieldType::U32 | FieldType::U64 | FieldType::U128 | FieldType::F32 | FieldType::F64 | FieldType::Decimal => {
                let number_type = self.to_csharp_type(false);
                format!("{prefix}NumberFieldUpdateOperationsInput<{number_type}>")
            },
            FieldType::Enum(_name) => {
                let enum_type = self.to_csharp_type(false);
                format!("{prefix}EnumFieldUpdateOperationsInput<{enum_type}>")
            },
            FieldType::Vec(internal) => {
                let internal_type = internal.field_type().to_csharp_type(false);
                let arr_prefix = array_prefix(&internal_type);
                format!("{prefix}{arr_prefix}ArrayFieldUpdateOperationsInput<{internal_type}>")
            },
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(_name) => "Unimplemented".to_string(),
        }
    }
}
