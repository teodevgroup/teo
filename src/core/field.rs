use crate::core::argument::Argument;
use crate::core::database_type::DatabaseType;
use crate::core::field_type::FieldType;
use crate::core::object::Object;
use crate::core::permission::Permission;
use crate::core::pipeline::Pipeline;
use crate::core::stage::Stage;
use crate::core::value::Value;


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
    pub(crate) name: String,
    pub(crate) field_type: FieldType,
    pub(crate) database_type: DatabaseType,
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
    pub(crate) column_name: Option<String>,
}

impl Field {
    pub(crate) fn column_name(&self) -> &String {
        match &self.column_name {
            Some(column_name) => column_name,
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

    pub(crate) async fn perform_on_save_callback(&self, stage: Stage, object: &Object) -> Stage {
        let mut new_stage = stage;
        match &self.field_type {
            FieldType::Vec(inner) => {
                let val = new_stage.value().unwrap();
                let arr = val.as_vec();
                if !arr.is_none() {
                    let arr = arr.unwrap();
                    let mut new_arr: Vec<Value> = Vec::new();
                    for v in arr {
                        let inner_stage = Stage::Value(v.clone());
                        new_arr.push(inner.on_save_pipeline.process(inner_stage, object).await.value().unwrap());
                    }
                    new_stage = Stage::Value(Value::Vec(new_arr));
                }

            }
            _ => {}
        }
        self.on_save_pipeline.process(new_stage, object).await
    }
}
