use std::collections::{HashMap, HashSet};
use std::ptr::{addr_of, null};
use inflector::Inflector;
use sea_query::IndexType;
use crate::action::action::ActionType;
use crate::core::builders::model_builder::ModelBuilder;
use crate::core::field::{Field, FieldIndex, QueryAbility, Sort};
use crate::core::field::ReadRule::NoRead;
use crate::core::field::Store::{Calculated, Temp};
use crate::core::field::WriteRule::NoWrite;
use crate::core::permission::Permission;


#[derive(Copy, Clone)]
pub enum ModelIndexType {
    Primary,
    Index,
    Unique,
}

#[derive(Clone)]
pub(crate) struct CompoundIndexItem {
    pub(crate) field_name: String,
    pub(crate) sort: Sort,
    pub(crate) len: Option<usize>,
}

pub(crate) struct CompoundIndex {
    pub(crate) index_type: ModelIndexType,
    pub(crate) name: String,
    pub(crate) items: Vec<CompoundIndexItem>
}

#[derive(Debug)]
pub(crate) struct Model {
    pub(crate) name: &'static str,
    pub(crate) table_name: String,
    pub(crate) url_segment_name: String,
    pub(crate) localized_name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) identity: bool,
    pub(crate) actions: HashSet<ActionType>,
    pub(crate) permission: Option<Permission>,
    pub(crate) fields_vec: Vec<Field>,
    pub(crate) fields_map: HashMap<&'static str, * const Field>,
    pub(crate) primary_field: * const Field,
    pub(crate) index_fields: Vec<* const Field>,
    pub(crate) input_keys: Vec<&'static str>,
    pub(crate) save_keys: Vec<&'static str>,
    pub(crate) output_keys: Vec<&'static str>,
    pub(crate) get_value_keys: Vec<&'static str>,
    pub(crate) query_keys: Vec<&'static str>,
    pub(crate) unique_query_keys: Vec<&'static str>,
    pub(crate) auth_identity_keys: Vec<&'static str>,
    pub(crate) auth_by_keys: Vec<&'static str>,
}

impl Model {

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn table_name(&self) -> &String {
        &self.table_name
    }

    pub(crate) fn url_segment_name(&self) -> &String {
        &self.url_segment_name
    }

    pub(crate) fn localized_name(&self) -> &'static str {
        self.localized_name
    }

    pub(crate) fn description(&self) -> &'static str {
        self.description
    }

    pub(crate) fn identity(&self) -> bool {
        self.identity
    }

    pub(crate) fn fields(&self) -> &Vec<Field> {
        return &self.fields_vec
    }

    pub fn field(&self, name: &str) -> &Field {
        unsafe {
            &(**self.fields_map.get(name).unwrap())
        }
    }

    pub(crate) fn primary_field(&self) -> Option<&Field> {
        if self.primary_field == null() {
            None
        } else {
            Some(unsafe { &*self.primary_field })
        }
    }

    pub(crate) fn primary_field_name(&self) -> Option<&'static str> {
        match self.primary_field() {
            Some(field) => Some(field.name),
            None => None
        }
    }

    pub(crate) fn index_fields(&self) -> Vec<&Field> {
        self.index_fields.iter().map(|f| { unsafe { &**f } }).collect()
    }

    pub(crate) fn input_keys(&self) -> &Vec<&'static str> {
        &self.input_keys
    }

    pub(crate) fn save_keys(&self) -> &Vec<&'static str> {
        &self.save_keys
    }

    pub(crate) fn output_keys(&self) -> &Vec<&'static str> {
        &self.output_keys
    }

    pub(crate) fn get_value_keys(&self) -> &Vec<&'static str> {
        &self.get_value_keys
    }

    pub(crate) fn query_keys(&self) -> &Vec<&'static str> {
        &self.query_keys
    }

    pub(crate) fn unique_query_keys(&self) -> &Vec<&'static str> {
        &self.unique_query_keys
    }

    pub(crate) fn auth_identity_keys(&self) -> &Vec<&'static str> { &self.auth_identity_keys }

    pub(crate) fn auth_by_keys(&self) -> &Vec<&'static str> { &self.auth_by_keys }

    pub(crate) fn has_action(&self, action: ActionType) -> bool {
        self.actions.contains(&action)
    }

    pub(crate) fn actions(&self) -> &HashSet<ActionType> {
        &self.actions
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
