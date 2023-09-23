use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::generators::client::typescript::gen::TsGenerationConf;
use crate::gen::internal::type_lookup::TypeLookup;

pub(crate) struct TsTypes {
    conf: TsGenerationConf
}

impl TsTypes {
    pub(in crate::gen) fn new(conf: TsGenerationConf) -> Self { Self { conf } }
}

impl TsTypes {
    fn update_operation_input(&self, field_type: &FieldType, nullable: bool) -> String {
        let mut generic = "".to_owned();
        let base: &str = match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "ObjectId",
            FieldType::String => "String",
            FieldType::Date => "Date",
            FieldType::DateTime => "DateTime",
            FieldType::Decimal => "Decimal",
            FieldType::Bool => "Bool",
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "Number",
            FieldType::Enum(_) => {
                let name = field_type.unwrap_enum().name();
                generic = format!("<{name}>");
                "Enum"
            },
            FieldType::Vec(inner) => {
                let create_type = "(".to_owned() + &self.field_type_to_result_type(inner.field_type()) + " | null)";
                generic = format!("<{create_type}>");
                "Array"
            },
            _ => panic!(),
        };
        let suffix = "FieldUpdateOperationsInput";
        let prefix = if nullable { "Nullable" } else { "" };
        format!("{prefix}{base}{suffix}{generic}")
    }
}

impl TypeLookup for TsTypes {
    fn field_type_to_filter_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        let mut with_generic = false;
        let base: String = match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "string | ObjectId".to_string(),
            FieldType::String => "string | String".to_string(),
            FieldType::Date => "string | Date".to_string(),
            FieldType::DateTime => "string | Date | DateTime".to_string(),
            FieldType::Bool => "boolean | Bool".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number | Number".to_string(),
            FieldType::Decimal => self.conf.decimal_input.to_string() + " | Decimal",
            FieldType::Enum(_) => {
                let name = field_type.unwrap_enum().name();
                with_generic = true;
                if nullable {
                    format!(r#"{name} | EnumNullableFilter<{name}> | null"#)
                } else {
                    format!(r#"{name} | EnumFilter<{name}>"#)
                }
            },
            FieldType::Vec(internal) => {
                with_generic = true;
                let create_type = self.field_type_to_create_type(internal.field_type(), false);
                if nullable {
                    format!("{create_type}[] | ArrayNullableFilter<{create_type}> | null")
                } else {
                    format!("{create_type}[] | ArrayFilter<{create_type}>")
                }
            },
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(_name) => "undefined | Unimplemented".to_string(),
        };
        Cow::Owned(if !with_generic {
            if nullable {
                base + "NullableFilter | null"
            } else {
                base + "Filter"
            }
        } else {
            base
        })
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        // TODO: see the C# version
        self.field_type_to_filter_type(field_type, nullable)
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        let base: String = match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "string".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Decimal => self.conf.decimal_input.to_string(),
            FieldType::Date | FieldType::DateTime => "string".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Enum(_) => field_type.unwrap_enum().name().to_string(),
            FieldType::Vec(internal) => (if internal.is_optional() {
                "(".to_owned() + &self.field_type_to_result_type(internal.field_type()) + " | null)"
            } else {
                self.field_type_to_result_type(internal.field_type()).to_string()
            }) + "[]",
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => name.to_string(),
        };
        Cow::Owned(if nullable {
            base + " | null"
        } else {
            base
        })
    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        let update_operation = self.update_operation_input(field_type, nullable);
        let create_input = self.field_type_to_create_type(field_type, nullable);
        return Cow::Owned(format!("{update_operation} | {create_input}"));
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
        Cow::Owned(match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => "string".to_string(),
            FieldType::String | FieldType::Date => "string".to_string(),
            FieldType::DateTime => "Date".to_string(),
            FieldType::Bool => "boolean".to_string(),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 => "number".to_string(),
            FieldType::Decimal => "Decimal".to_string(),
            FieldType::Enum(_) => field_type.unwrap_enum().name().to_string(),
            FieldType::Vec(internal) => (if internal.is_optional() {
                "(".to_owned() + &self.field_type_to_result_type(internal.field_type()) + " | null)"
            } else {
                self.field_type_to_result_type(internal.field_type()).to_string()
            }) + "[]",
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => name.to_string(),
        })
    }

    fn generated_type_to_vec<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(format!("{generated_type}[]"))
    }

    fn generated_type_to_enumerate<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(format!("Enumerable<{generated_type}>"))
    }

    fn generated_type_to_optional<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(generated_type.as_ref().to_owned() + " | undefined")
    }

    fn generated_type_to_or_null<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(generated_type.as_ref().to_owned() + " | null")
    }

    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str> {
        Cow::Owned(match action.to_u32() {
            FIND_UNIQUE_HANDLER => format!("Response<undefined, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            FIND_FIRST_HANDLER => format!("Response<undefined, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            FIND_MANY_HANDLER => format!("Response<PagingInfo, CheckSelectInclude<T, {model_name}[], {model_name}GetPayload<T>[]>>"),
            CREATE_HANDLER => format!("Response<undefined, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            UPDATE_HANDLER => format!("Response<undefined, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            UPSERT_HANDLER => format!("Response<undefined, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            DELETE_HANDLER => format!("Response<undefined, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            CREATE_MANY_HANDLER => format!("Response<PagingInfo, CheckSelectInclude<T, {model_name}[], {model_name}GetPayload<T>[]>>"),
            UPDATE_MANY_HANDLER => format!("Response<PagingInfo, CheckSelectInclude<T, {model_name}[], {model_name}GetPayload<T>[]>>"),
            DELETE_MANY_HANDLER => format!("Response<PagingInfo, CheckSelectInclude<T, {model_name}[], {model_name}GetPayload<T>[]>>"),
            COUNT_HANDLER => format!("Response<undefined, CheckSelectInclude<T, number, {model_name}GetPayload<T>>>"),
            AGGREGATE_HANDLER => format!("Response<undefined, CheckSelectInclude<T, never, {model_name}GetPayload<T>>>"),
            GROUP_BY_HANDLER => format!("Response<undefined, CheckSelectInclude<T, never, {model_name}GetPayload<T>>>"),
            SIGN_IN_HANDLER => format!("Response<TokenInfo, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            IDENTITY_HANDLER => format!("Response<undefined, CheckSelectInclude<T, {model_name}, {model_name}GetPayload<T>>>"),
            _ => unreachable!()
        })
    }

    fn number_type(&self) -> &'static str {
        "number"
    }

    fn bool_type(&self) -> &'static str {
        "boolean"
    }
}
