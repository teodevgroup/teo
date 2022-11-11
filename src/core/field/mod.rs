use std::fmt::{Debug, Formatter};
use crate::core::pipeline::argument::Argument;
use crate::core::database::r#type::DatabaseType;
use crate::core::field::optionality::Optionality;
use crate::core::field::r#type::FieldType;
use crate::core::field::read_rule::ReadRule;
use crate::core::field::write_rule::WriteRule;
use crate::core::permission::Permission;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::context::Context;
use crate::core::tson::Value;

pub(crate) mod r#type;
pub(crate) mod builder;
pub mod optionality;
pub mod read_rule;
pub mod write_rule;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PreviousValueRule {
    DontKeep,
    Keep,
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

impl FieldIndex {
    pub fn is_unique(&self) -> bool {
        match self {
            FieldIndex::Unique(_) => true,
            _ => false
        }
    }
}

#[derive(Clone)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) field_type: FieldType,
    pub(crate) database_type: DatabaseType,
    pub(crate) optionality: Optionality,
    pub(crate) r#virtual: bool,
    pub(crate) atomic: bool,
    pub(crate) primary: bool,
    pub(crate) read_rule: ReadRule,
    pub(crate) write_rule: WriteRule,
    pub(crate) previous_value_rule: PreviousValueRule,
    pub(crate) input_omissible: bool,
    pub(crate) output_omissible: bool,
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
    pub(crate) column_name: Option<String>,
    pub(crate) foreign_key: bool,
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("Field");
        result.finish()
    }

}

impl Field {

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn localized_name(&self) -> &str {
        &self.localized_name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn r#type(&self) -> &FieldType {
        &self.field_type
    }

    pub(crate) fn is_optional(&self) -> bool {
        self.optionality.is_optional()
    }

    pub(crate) fn is_required(&self) -> bool {
        self.optionality.is_required()
    }

    pub(crate) fn column_name(&self) -> &str {
        match &self.column_name {
            Some(column_name) => column_name.as_str(),
            None => &self.name
        }
    }

    pub(crate) fn needs_on_save_callback(&self) -> bool {
        if self.on_save_pipeline.has_any_modifier() {
            return true;
        }
        return match &self.field_type {
            FieldType::Vec(inner) => inner.needs_on_save_callback(),
            _ => false
        }
    }

    pub(crate) fn needs_on_output_callback(&self) -> bool {
        if self.on_output_pipeline.has_any_modifier() {
            return true;
        }
        return match &self.field_type {
            FieldType::Vec(inner) => inner.needs_on_output_callback(),
            _ => false
        }
    }

    pub(crate) async fn perform_on_save_callback<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let mut new_ctx = ctx.clone();
        match &self.field_type {
            FieldType::Vec(inner) => {
                let val = &new_ctx.value;
                let arr = val.as_vec();
                if !arr.is_none() {
                    let arr = arr.unwrap();
                    let mut new_arr: Vec<Value> = Vec::new();
                    for (i, _v) in arr.iter().enumerate() {
                        let key_path = &ctx.key_path + i;
                        let arr_item_ctx = ctx.alter_key_path(key_path);
                        new_arr.push(inner.on_save_pipeline.process(arr_item_ctx).await.value);
                    }
                    new_ctx = new_ctx.alter_value(Value::Vec(new_arr));
                }
            }
            _ => {}
        }
        self.on_save_pipeline.process(new_ctx.clone()).await
    }

    pub(crate) async fn perform_on_output_callback<'a>(&self, ctx: Context<'a>) -> Context<'a> {
        let mut new_ctx = ctx.clone();
        match &self.field_type {
            FieldType::Vec(inner) => {
                let val = &new_ctx.value;
                let arr = val.as_vec();
                if !arr.is_none() {
                    let arr = arr.unwrap();
                    let mut new_arr: Vec<Value> = Vec::new();
                    for (i, _v) in arr.iter().enumerate() {
                        let key_path = &ctx.key_path + i;
                        let arr_item_ctx = ctx.alter_key_path(key_path);
                        new_arr.push(inner.on_output_pipeline.process(arr_item_ctx).await.value);
                    }
                    new_ctx = new_ctx.alter_value(Value::Vec(new_arr));
                }
            }
            _ => {}
        }
        self.on_output_pipeline.process(new_ctx.clone()).await
    }

    pub(crate) fn permission(&self) -> Option<&Permission> {
        self.permission.as_ref()
    }
}

unsafe impl Send for Field {}
unsafe impl Sync for Field {}
