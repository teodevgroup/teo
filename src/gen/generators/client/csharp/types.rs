use std::borrow::Cow;
use crate::core::action::{Action, AGGREGATE_HANDLER, COUNT_HANDLER, CREATE_HANDLER, CREATE_MANY_HANDLER, DELETE_HANDLER, DELETE_MANY_HANDLER, FIND_FIRST_HANDLER, FIND_MANY_HANDLER, FIND_UNIQUE_HANDLER, GROUP_BY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::type_lookup::ClientTypeLookup;

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

fn with_agg(agg: bool) -> &'static str {
    if agg {
        "WithAggregates"
    } else {
        ""
    }
}

fn one_of(t0: impl AsRef<str>, t1: impl AsRef<str>) -> String {
    format!("OneOf<{}, {}>", t0.as_ref(), t1.as_ref())
}

fn array_prefix(t: &str) -> &str {
    if t == "string" {
        "Ref"
    } else {
        "Value"
    }
}

pub(crate) struct CSharpTypes { }

impl CSharpTypes {
    pub(crate) fn new() -> Self { Self { } }

    fn update_operation_input(&self, field_type: &FieldType, optional: bool) -> String {
        let prefix = if optional { "Nullable" } else { "" };
        match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => format!("{prefix}ObjectIdFieldUpdateOperationsInput"),
            FieldType::String => format!("{prefix}StringFieldUpdateOperationsInput"),
            FieldType::Date => format!("{prefix}DateOnlyFieldUpdateOperationsInput"),
            FieldType::DateTime => format!("{prefix}DateTimeFieldUpdateOperationsInput"),
            FieldType::Bool => format!("{prefix}BoolFieldUpdateOperationsInput"),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 | FieldType::Decimal => {
                let number_type = self.field_type_to_result_type(field_type);
                format!("{prefix}NumberFieldUpdateOperationsInput<{number_type}>")
            },
            FieldType::Enum(_name) => {
                let enum_type = self.field_type_to_result_type(field_type);
                format!("{prefix}EnumFieldUpdateOperationsInput<{enum_type}>")
            },
            FieldType::Vec(internal) => {
                let internal_type = self.field_type_to_result_type(internal.field_type());
                let arr_prefix = array_prefix(&internal_type);
                format!("{prefix}{arr_prefix}ArrayFieldUpdateOperationsInput<{internal_type}>")
            },
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(_name) => "Unimplemented".to_string(),
        }
    }

    fn internal_field_type_to_filter_type<'a>(&self, field_type: &'a FieldType, optional: bool, with_aggregates: bool) -> Cow<'a, str> {
        let nullable = nullable_if_optional(optional);
        let aggregates = with_agg(with_aggregates);
        let infix = nullable.to_owned() + aggregates;
        let base_type = to_optional(&self.field_type_to_result_type(field_type), optional);
        Cow::Owned(match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => one_of(base_type, format!("RefComparable{infix}Filter<string>")),
            FieldType::String => one_of(base_type, format!("String{infix}Filter")),
            FieldType::Date => one_of(base_type, format!("Comparable{infix}Filter<DateOnly>")),
            FieldType::DateTime => one_of(base_type, format!("Comparable{infix}Filter<DateTime>")),
            FieldType::Bool => one_of(base_type, format!("Bool{infix}Filter")),
            FieldType::I32 | FieldType::I64 | FieldType::F32 | FieldType::F64 | FieldType::Decimal => {
                let number_type = self.field_type_to_result_type(field_type);
                if with_aggregates {
                    one_of(base_type, format!("Number{infix}Filter<{number_type}>"))
                } else {
                    one_of(base_type, format!("Comparable{infix}Filter<{number_type}>"))
                }
            },
            FieldType::Enum(_name) => {
                let enum_type = self.field_type_to_result_type(field_type);
                one_of(base_type, format!("Enum{infix}Filter<{enum_type}>"))
            },
            FieldType::Vec(internal) => {
                let internal_type = self.field_type_to_result_type(internal.field_type());
                let prefix = array_prefix(internal_type.as_ref());
                one_of(base_type, format!("{prefix}Array{infix}Filter<{internal_type}>"))
            },
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(_name) => "Unimplemented".to_string(),
        })
    }
}

impl ClientTypeLookup for CSharpTypes {
    fn field_type_to_filter_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        self.internal_field_type_to_filter_type(field_type, optional, false)
    }

    fn field_type_to_filter_with_aggregates_type<'a>(&self, field_type: &'a FieldType, nullable: bool) -> Cow<'a, str> {
        self.internal_field_type_to_filter_type(field_type, nullable, true)
    }

    fn field_type_to_create_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        let mut retval = self.field_type_to_result_type(field_type);
        if optional {
            retval = Cow::Owned(format!("Optional<{}>", retval.as_ref()));
        }
        retval
    }

    fn field_type_to_update_type<'a>(&self, field_type: &'a FieldType, optional: bool) -> Cow<'a, str> {
        let create_input = self.field_type_to_create_type(field_type, optional);
        let operation_input = self.update_operation_input(field_type, optional);
        Cow::Owned(one_of(create_input.as_ref(), operation_input))
    }

    fn field_type_to_result_type<'a>(&self, field_type: &'a FieldType) -> Cow<'a, str> {
        match field_type {
            #[cfg(feature = "data-source-mongodb")]
            FieldType::ObjectId => Cow::Borrowed("string"),
            FieldType::String => Cow::Borrowed("string"),
            FieldType::Bool => Cow::Borrowed("bool"),
            FieldType::I32 => Cow::Borrowed("int"),
            FieldType::I64 => Cow::Borrowed("long"),
            FieldType::F32 => Cow::Borrowed("float"),
            FieldType::F64 => Cow::Borrowed("double"),
            FieldType::Decimal => Cow::Borrowed("decimal"),
            FieldType::Date => Cow::Borrowed("DateOnly"),
            FieldType::DateTime => Cow::Borrowed("DateTime"),
            FieldType::Enum(enum_def) => Cow::Owned(enum_def.name().to_string()),
            FieldType::Vec(inner) => Cow::Owned(self.field_type_to_result_type(inner.field_type()).as_ref().to_owned() + "[]"),
            FieldType::HashMap(_) => panic!(),
            FieldType::BTreeMap(_) => panic!(),
            FieldType::Object(name) => Cow::Owned(name.to_string()),
        }
    }

    fn generated_type_to_vec<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(generated_type.as_ref().to_owned() + "[]")
    }

    fn generated_type_to_enumerate<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("Enumerable<".to_owned() + generated_type.as_ref() + ">")
    }

    fn generated_type_to_optional<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned(generated_type.as_ref().to_owned() + "?")
    }

    fn generated_type_to_or_null<'a>(&self, generated_type: Cow<'a, str>) -> Cow<'a, str> {
        Cow::Owned("Optional<".to_owned() + generated_type.as_ref() + ">")
    }

    fn action_result_type<'a>(&self, action: Action, model_name: &'a str) -> Cow<'a, str> {
        Cow::Owned(match action.to_u32() {
            FIND_UNIQUE_HANDLER => format!("Response<{model_name}>"),
            FIND_FIRST_HANDLER => format!("Response<{model_name}>"),
            FIND_MANY_HANDLER => format!("Response<PagingInfo, {model_name}[]>"),
            CREATE_HANDLER => format!("Response<{model_name}>"),
            UPDATE_HANDLER => format!("Response<{model_name}>"),
            UPSERT_HANDLER => format!("Response<{model_name}>"),
            DELETE_HANDLER => format!("Response<{model_name}>"),
            CREATE_MANY_HANDLER => format!("Response<PagingInfo, {model_name}[]>"),
            UPDATE_MANY_HANDLER => format!("Response<PagingInfo, {model_name}[]>"),
            DELETE_MANY_HANDLER => format!("Response<PagingInfo, {model_name}[]>"),
            COUNT_HANDLER => format!("Response<long>"),
            AGGREGATE_HANDLER => format!("Response<{model_name}>"),
            GROUP_BY_HANDLER => format!("Response<{model_name}>"),
            SIGN_IN_HANDLER => format!("Response<TokenInfo, {model_name}>"),
            IDENTITY_HANDLER => format!("Response<{model_name}>"),
            _ => unreachable!()
        })
    }

    fn number_type(&self) -> &'static str {
        "int"
    }

    fn bool_type(&self) -> &'static str {
        "bool"
    }
}
