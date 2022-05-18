use crate::core::argument::Argument;
use crate::core::builders::FieldBuilder;
use crate::core::pipeline::Pipeline;


#[derive(Debug)]
pub enum Type {
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
pub struct Field {
    pub name: &'static str,
    pub r#type: Type,
    pub availability: Availability,
    pub store: Store,
    pub primary: bool,
    pub read_rule: ReadRule,
    pub write_rule: WriteRule,
    pub index: FieldIndex,
    pub query_ability: QueryAbility,
    pub object_assignment: ObjectAssignment,
    pub assigned_by_database: bool,
    pub auth_identity: bool,
    pub default: Option<Argument>,
    pub on_set_pipeline: Pipeline,
    pub on_save_pipeline: Pipeline,
    pub on_output_pipeline: Pipeline,
}

impl Field {
    pub fn new(builder: &FieldBuilder) -> Field {
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
