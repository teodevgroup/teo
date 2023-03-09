pub(crate) mod r#type;
pub(crate) mod optionality;
pub(crate) mod read_rule;
pub(crate) mod write_rule;
pub(crate) mod migration;

use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use inflector::Inflector;
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use crate::core::connector::Connector;
use crate::core::database::r#type::DatabaseType;
use crate::core::field::migration::FieldMigration;
use crate::core::field::optionality::Optionality;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::field::read_rule::ReadRule;
use crate::core::field::write_rule::WriteRule;
use crate::core::pipeline::Pipeline;
use crate::core::pipeline::ctx::Ctx;
use crate::core::teon::Value;
use crate::core::result::Result;

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

#[derive(Clone, ToMut)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) localized_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) field_type: Option<FieldType>,
    pub(crate) database_type: Option<DatabaseType>,
    pub(crate) optionality: Optionality,
    pub(crate) r#virtual: bool,
    pub(crate) atomic: bool,
    pub(crate) primary: bool,
    pub(crate) read_rule: ReadRule,
    pub(crate) write_rule: WriteRule,
    pub(crate) previous_value_rule: PreviousValueRule,
    pub(crate) input_omissible: bool,
    pub(crate) output_omissible: bool,
    pub(crate) index: Option<FieldIndex>,
    pub(crate) query_ability: QueryAbility,
    pub(crate) auto: bool,
    pub(crate) auto_increment: bool,
    pub(crate) identity: bool,
    pub(crate) identity_checker: Option<Value>,
    pub(crate) default: Option<Value>,
    pub(crate) on_set_pipeline: Pipeline,
    pub(crate) on_save_pipeline: Pipeline,
    pub(crate) on_output_pipeline: Pipeline,
    pub(crate) can_mutate_pipeline: Pipeline,
    pub(crate) can_read_pipeline: Pipeline,
    pub(crate) column_name: Option<String>,
    pub(crate) foreign_key: bool,
    pub(crate) migration: Option<FieldMigration>,
    pub(crate) dropped: bool,
}

impl Debug for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_struct("Field");
        result.finish()
    }

}

impl Field {

    pub(crate) fn new(name: String) -> Self {
        return Self {
            name: name.into(),
            localized_name: None,
            description: None,
            field_type: None,
            database_type: None,
            optionality: Optionality::Required,
            r#virtual: false,
            atomic: false,
            primary: false,
            read_rule: ReadRule::Read,
            write_rule: WriteRule::Write,
            index: None,
            query_ability: QueryAbility::Queryable,
            auto: false,
            auto_increment: false,
            identity: false,
            identity_checker: None,
            default: None,
            on_set_pipeline: Pipeline::new(),
            on_save_pipeline: Pipeline::new(),
            on_output_pipeline: Pipeline::new(),
            can_mutate_pipeline: Pipeline::new(),
            can_read_pipeline: Pipeline::new(),
            column_name: None,
            previous_value_rule: PreviousValueRule::DontKeep,
            input_omissible: false,
            output_omissible: false,
            foreign_key: false,
            migration: None,
            dropped: false,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn localized_name(&self) -> String {
        if self.localized_name.is_some() {
            self.localized_name.clone().unwrap()
        } else {
            self.name.to_title_case()
        }
    }

    pub(crate) fn description(&self) -> Option<&str> {
        match self.description.as_ref() {
            Some(d) => Some(d),
            None => None
        }
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
        if self.on_save_pipeline.has_any_items() {
            return true;
        }
        return match self.field_type() {
            FieldType::Vec(inner) => inner.needs_on_save_callback(),
            _ => false
        }
    }

    pub(crate) fn database_type(&self) -> &DatabaseType {
        self.database_type.as_ref().unwrap()
    }

    pub(crate) fn needs_on_output_callback(&self) -> bool {
        if self.on_output_pipeline.has_any_items() {
            return true;
        }
        return match self.field_type() {
            FieldType::Vec(inner) => inner.needs_on_output_callback(),
            _ => false
        }
    }

    pub(crate) async fn perform_on_save_callback(&self, ctx: Ctx<'_>) -> Result<Value> {
        let mut new_ctx = ctx.clone();
        match self.field_type() {
            FieldType::Vec(inner) => {
                let val = &new_ctx.get_value();
                let arr = val.as_vec();
                if !arr.is_none() {
                    let arr = arr.unwrap();
                    let mut new_arr: Vec<Value> = Vec::new();
                    for (i, _v) in arr.iter().enumerate() {
                        let key_path = &ctx.path + i;
                        let arr_item_ctx = ctx.with_path(key_path);
                        new_arr.push(inner.on_save_pipeline.process(arr_item_ctx).await?);
                    }
                    new_ctx = new_ctx.with_value(Value::Vec(new_arr));
                }
            }
            _ => {}
        }
        self.on_save_pipeline.process(new_ctx.clone()).await
    }

    pub(crate) async fn perform_on_output_callback<'a>(&self, ctx: Ctx<'a>) -> Result<Value> {
        let mut new_ctx = ctx.clone();
        match self.field_type() {
            FieldType::Vec(inner) => {
                let val = &new_ctx.get_value();
                let arr = val.as_vec();
                if !arr.is_none() {
                    let arr = arr.unwrap();
                    let mut new_arr: Vec<Value> = Vec::new();
                    for (i, _v) in arr.iter().enumerate() {
                        let key_path = &ctx.path + i;
                        let arr_item_ctx = ctx.with_path(key_path);
                        new_arr.push(inner.on_output_pipeline.process(arr_item_ctx).await?);
                    }
                    new_ctx = new_ctx.with_value(Value::Vec(new_arr));
                }
            }
            _ => {}
        }
        self.on_output_pipeline.process(new_ctx.clone()).await
    }

    pub(crate) fn finalize(&mut self, connector: Arc<dyn Connector>) {
        self.database_type = Some(connector.default_database_type(self.field_type()));
    }

    pub(crate) fn set_required(&mut self) {
        self.optionality = Optionality::Required;
    }

    pub(crate) fn set_optional(&mut self) {
        self.optionality = Optionality::Optional;
        self.input_omissible = true;
        self.output_omissible = true;
    }
}

impl FieldTypeOwner for Field {
    fn field_type(&self) -> &FieldType {
        self.field_type.as_ref().unwrap()
    }

    fn is_optional(&self) -> bool {
        self.optionality.is_optional()
    }
}

unsafe impl Send for Field {}
unsafe impl Sync for Field {}
