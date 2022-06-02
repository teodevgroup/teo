use chrono::{Date, DateTime, NaiveDate, Utc};
use serde_json::{Value as JsonValue};
use crate::core::argument::Argument;
use crate::core::builders::field_builder::FieldBuilder;
use crate::core::field_type::FieldType;
use crate::core::graph::{Graph};
use crate::core::permission::Permission;
use crate::core::pipeline::Pipeline;
use crate::core::value::Value;
use crate::error::ActionError;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Optionality {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QueryAbility {
    Queryable,
    Unqueryable,
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectAssignment {
    Reference,
    Copy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sort {
    Asc,
    Desc
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexSettings {
    pub(crate) name: Option<String>,
    pub(crate) sort: Sort,
    pub(crate) length: Option<usize>,
}

impl Default for IndexSettings {
    fn default() -> Self {
        IndexSettings {
            name: None,
            sort: Sort::Asc,
            length: None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldIndex {
    NoIndex,
    Index(IndexSettings),
    Unique(IndexSettings),
}

#[derive(Debug, Clone)]
pub(crate) struct Field {
    pub(crate) name: &'static str,
    pub(crate) field_type: FieldType,
    pub(crate) optionality: Optionality,
    pub(crate) store: Store,
    pub(crate) primary: bool,
    pub(crate) read_rule: ReadRule,
    pub(crate) write_rule: WriteRule,
    pub(crate) index: FieldIndex,
    pub(crate) query_ability: QueryAbility,
    pub(crate) object_assignment: ObjectAssignment,
    pub(crate) auto: bool,
    pub(crate) auto_increment: bool,
    pub(crate) auth_identity: bool,
    pub(crate) auth_by: bool,
    pub(crate) auth_by_arg: Option<Argument>,
    pub(crate) default: Option<Argument>,
    pub(crate) on_set_pipeline: Pipeline,
    pub(crate) on_save_pipeline: Pipeline,
    pub(crate) on_output_pipeline: Pipeline,
    pub(crate) permission: Option<Permission>,
    pub(crate) column_name: Option<&'static str>,
}

impl Field {
    pub(crate) fn new(builder: &FieldBuilder) -> Field {
        return Field {
            name: builder.name,
            field_type: builder.field_type.clone(),
            optionality: builder.optionality,
            store: builder.store,
            primary: builder.primary,
            read_rule: builder.read_rule,
            write_rule: builder.write_rule,
            index: builder.index.clone(),
            query_ability: builder.query_ability,
            object_assignment: builder.object_assignment,
            auto: builder.auto,
            auto_increment: builder.auto_increment,
            auth_identity: builder.auth_identity,
            auth_by: builder.auth_by,
            auth_by_arg: builder.auth_by_arg.clone(),
            default: builder.default.clone(),
            on_set_pipeline: builder.on_set_pipeline.build(),
            on_save_pipeline: builder.on_save_pipeline.build(),
            on_output_pipeline: builder.on_output_pipeline.build(),
            permission: if let Some(builder) = &builder.permission { Some(builder.build()) } else { None },
            column_name: builder.column_name
        }
    }
}
