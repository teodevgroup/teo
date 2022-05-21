use chrono::{Date, DateTime, NaiveDate, Utc};
use serde_json::{Value as JsonValue};
use crate::core::argument::Argument;
use crate::core::builders::FieldBuilder;
use crate::core::graph::{Graph};
use crate::core::pipeline::Pipeline;
use crate::core::value::Value;
use crate::error::ActionError;


#[derive(Debug)]
pub(crate) enum Type {
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
    String,
    Date,
    DateTime,
    Enum(&'static str),
    Vec(Box<Field>),
    Map(Box<Field>),
    Object(&'static str)
}

impl Type {

    pub(crate) fn decode_value(&self, json_value: &JsonValue, graph: &'static Graph) -> Result<Value, ActionError> {
        if *json_value == JsonValue::Null {
            Ok(Value::Null)
        } else {
            match self {
                Type::Undefined => { Ok(Value::Null) }
                Type::ObjectId => {
                    if json_value.is_string() {
                        Ok(Value::ObjectId(json_value.as_str().unwrap().to_string()))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::Bool => {
                    if json_value.is_boolean() {
                        Ok(Value::Bool(json_value.as_bool().unwrap()))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::I8 => {
                    if json_value.is_number() {
                        Ok(Value::I8(json_value.as_i64().unwrap() as i8))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::I16 => {
                    if json_value.is_number() {
                        Ok(Value::I16(json_value.as_i64().unwrap() as i16))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::I32 => {
                    if json_value.is_number() {
                        Ok(Value::I32(json_value.as_i64().unwrap() as i32))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::I64 => {
                    if json_value.is_number() {
                        Ok(Value::I64(json_value.as_i64().unwrap()))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::I128 => {
                    if json_value.is_number() {
                        Ok(Value::I128(json_value.as_i64().unwrap() as i128))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::U8 => {
                    if json_value.is_number() {
                        Ok(Value::U8(json_value.as_i64().unwrap() as u8))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::U16 => {
                    if json_value.is_number() {
                        Ok(Value::U16(json_value.as_i64().unwrap() as u16))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::U32 => {
                    if json_value.is_number() {
                        Ok(Value::U32(json_value.as_i64().unwrap() as u32))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::U64 => {
                    if json_value.is_number() {
                        Ok(Value::U64(json_value.as_i64().unwrap() as u64))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::U128 => {
                    if json_value.is_number() {
                        Ok(Value::U128(json_value.as_i64().unwrap() as u128))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::F32 => {
                    if json_value.is_number() {
                        Ok(Value::F32(json_value.as_f64().unwrap() as f32))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::F64 => {
                    if json_value.is_number() {
                        Ok(Value::F64(json_value.as_f64().unwrap() as f64))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::String => {
                    if json_value.is_string() {
                        Ok(Value::String(String::from(json_value.as_str().unwrap())))
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::Date => {
                    if json_value.is_string() {
                        match NaiveDate::parse_from_str(&json_value.as_str().unwrap(), "%Y-%m-%d") {
                            Ok(naive_date) => {
                                let date: Date<Utc> = Date::from_utc(naive_date, Utc);
                                Ok(Value::Date(date))
                            }
                            Err(error) => {
                                Err(ActionError::wrong_date_format())
                            }
                        }
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::DateTime => {
                    if json_value.is_string() {
                        match DateTime::parse_from_rfc3339(&json_value.as_str().unwrap()) {
                            Ok(fixed_offset_datetime) => {
                                let datetime: DateTime<Utc> = fixed_offset_datetime.with_timezone(&Utc);
                                Ok(Value::DateTime(datetime))
                            }
                            Err(error) => {
                                Err(ActionError::wrong_datetime_format())
                            }
                        }
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                Type::Enum(enum_name) => {
                    if json_value.is_string() {
                        let string = String::from(json_value.as_str().unwrap());
                        let enums = graph.enums();
                        let vals = enums.get(enum_name).unwrap();
                        if vals.contains(&&*string) {
                            Ok(Value::String(string))
                        } else {
                            Err(ActionError::wrong_enum_choice())
                        }
                    } else {
                        Err(ActionError::wrong_input_type())
                    }
                }
                _ => {
                    panic!()
                }
            }
        }
    }
}

impl Clone for Type {
    fn clone(&self) -> Self {
        match self {
            Type::Undefined => {
                return Type::Undefined
            }
            Type::ObjectId => {
                return Type::ObjectId
            }
            Type::Bool => {
                return Type::Bool
            }
            Type::I8 => {
                return Type::I8
            }
            Type::I16 => {
                return Type::I16
            }
            Type::I32 => {
                return Type::I32
            }
            Type::I64 => {
                return Type::I64
            }
            Type::I128 => {
                return Type::I128
            }
            Type::U8 => {
                return Type::U8
            }
            Type::U16 => {
                return Type::U16
            }
            Type::U32 => {
                return Type::U32
            }
            Type::U64 => {
                return Type::U64
            }
            Type::U128 => {
                return Type::U128
            }
            Type::F32 => {
                return Type::F32
            }
            Type::F64 => {
                return Type::F64
            }
            Type::String => {
                return Type::String
            }
            Type::Date => {
                return Type::Date
            }
            Type::DateTime => {
                return Type::DateTime
            }
            Type::Enum(e) => {
                return Type::Enum(e)
            }
            Type::Object(model) => {
                return Type::Object(model)
            }
            Type::Vec(field) => {
                return Type::Vec(field.clone())
            }
            Type::Map(field) => {
                return Type::Map(field.clone())
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Availability {
    Optional,
    Required
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Store {
    Embedded,
    LocalKey,
    ForeignKey(&'static str),
    JoinTableKey(&'static str, &'static str),
    Calculated,
    Temp
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadRule {
    Read,
    NoRead
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WriteRule {
    Write,
    NoWrite,
    WriteOnce,
    WriteOnCreate,
    WriteNonNull
}

#[derive(Debug, Clone, Copy)]
pub enum DeleteRule {
    Nullify,
    Cascade,
    Deny,
}

#[derive(Debug, Clone, Copy)]
pub enum QueryAbility {
    Queryable,
    Unqueryable,
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectAssignment {
    Reference,
    Copy,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldIndex {
    NoIndex,
    Index,
    CompoundIndex(&'static str),
    Unique,
    CompoundUnique(&'static str),
}

#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) name: &'static str,
    pub(crate) r#type: Type,
    pub(crate) availability: Availability,
    pub(crate) store: Store,
    pub(crate) primary: bool,
    pub(crate) read_rule: ReadRule,
    pub(crate) write_rule: WriteRule,
    pub(crate) index: FieldIndex,
    pub(crate) query_ability: QueryAbility,
    pub(crate) object_assignment: ObjectAssignment,
    pub(crate) assigned_by_database: bool,
    pub(crate) auth_identity: bool,
    pub(crate) default: Option<Argument>,
    pub(crate) on_set_pipeline: Pipeline,
    pub(crate) on_save_pipeline: Pipeline,
    pub(crate) on_output_pipeline: Pipeline,
}

impl Field {
    pub(crate) fn new(builder: &FieldBuilder) -> Field {
        return Field {
            name: builder.name,
            r#type: builder.r#type.clone(),
            availability: builder.availability,
            store: builder.store,
            primary: builder.primary,
            read_rule: builder.read_rule,
            write_rule: builder.write_rule,
            index: builder.index,
            query_ability: builder.query_ability,
            object_assignment: builder.object_assignment,
            assigned_by_database: builder.assigned_by_database,
            auth_identity: builder.auth_identity,
            default: builder.default.clone(),
            on_set_pipeline: builder.on_set_pipeline.clone(),
            on_save_pipeline: builder.on_save_pipeline.clone(),
            on_output_pipeline: builder.on_output_pipeline.clone(),
        }
    }
}

impl Clone for Field {
    fn clone(&self) -> Self {
        return Field {
            name: self.name,
            r#type: self.r#type.clone(),
            availability: self.availability,
            store: self.store,
            primary: self.primary,
            read_rule: self.read_rule,
            write_rule: self.write_rule,
            index: self.index,
            query_ability: self.query_ability,
            object_assignment: self.object_assignment,
            assigned_by_database: self.assigned_by_database,
            auth_identity: self.auth_identity,
            default: self.default.clone(),
            on_set_pipeline: self.on_set_pipeline.clone(),
            on_save_pipeline: self.on_save_pipeline.clone(),
            on_output_pipeline: self.on_output_pipeline.clone(),
        }
    }
}
