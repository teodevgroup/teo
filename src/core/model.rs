use std::collections::{HashMap, HashSet};
use std::ptr::null;
use std::rc::Rc;
use crate::action::action::ActionType;
use crate::core::field::{Field, Sort};
use crate::core::permission::Permission;
use crate::core::relation::Relation;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ModelIndexType {
    Primary,
    Index,
    Unique,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ModelIndexItem {
    pub(crate) field_name: String,
    pub(crate) sort: Sort,
    pub(crate) len: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ModelIndex {
    pub(crate) index_type: ModelIndexType,
    pub(crate) name: String,
    pub(crate) items: Vec<ModelIndexItem>
}

#[derive(Debug)]
pub(crate) struct Model {
    pub(crate) name: String,
    pub(crate) table_name: String,
    pub(crate) url_segment_name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) identity: bool,
    pub(crate) actions: HashSet<ActionType>,
    pub(crate) permission: Option<Permission>,
    pub(crate) fields_vec: Vec<Field>,
    pub(crate) fields_map: HashMap<String, * const Field>,
    pub(crate) relations_vec: Vec<Relation>,
    pub(crate) relations_map: HashMap<String, * const Relation>,
    pub(crate) indices: Vec<ModelIndex>,
    pub(crate) primary: ModelIndex,
    pub(crate) primary_field: * const Field,
    pub(crate) index_fields: Vec<* const Field>,
    pub(crate) all_keys: Vec<String>,
    pub(crate) input_keys: Vec<String>,
    pub(crate) save_keys: Vec<String>,
    pub(crate) output_keys: Vec<String>,
    pub(crate) get_value_keys: Vec<String>,
    pub(crate) query_keys: Vec<String>,
    pub(crate) unique_query_keys: Vec<HashSet<String>>,
    pub(crate) auth_identity_keys: Vec<String>,
    pub(crate) auth_by_keys: Vec<String>,
}

impl Model {

    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn table_name(&self) -> &String {
        &self.table_name
    }

    pub(crate) fn url_segment_name(&self) -> &String {
        &self.url_segment_name
    }

    pub(crate) fn localized_name(&self) -> &String {
        &self.localized_name
    }

    pub(crate) fn description(&self) -> &String {
        &self.description
    }

    pub(crate) fn identity(&self) -> bool {
        self.identity
    }

    pub(crate) fn fields(&self) -> &Vec<Field> {
        return &self.fields_vec
    }

    pub fn field(&self, name: &str) -> Option<&Field> {
        match self.fields_map.get(name) {
            Some(f) => Some(unsafe { &**f }),
            None => None
        }
    }

    pub(crate) fn primary_field(&self) -> Option<&Field> {
        if self.primary_field == null() {
            None
        } else {
            Some(unsafe { &*self.primary_field })
        }
    }

    pub(crate) fn primary_field_name(&self) -> Option<String> {
        match self.primary_field() {
            Some(field) => Some(field.name.clone()),
            None => None
        }
    }

    pub(crate) fn index_fields(&self) -> Vec<&Field> {
        self.index_fields.iter().map(|f| { unsafe { &**f } }).collect()
    }

    pub(crate) fn all_keys(&self) -> &Vec<String> { &self.all_keys }

    pub(crate) fn input_keys(&self) -> &Vec<String> {
        &self.input_keys
    }

    pub(crate) fn save_keys(&self) -> &Vec<String> {
        &self.save_keys
    }

    pub(crate) fn output_keys(&self) -> &Vec<String> {
        &self.output_keys
    }

    pub(crate) fn get_value_keys(&self) -> &Vec<String> {
        &self.get_value_keys
    }

    pub(crate) fn query_keys(&self) -> &Vec<String> {
        &self.query_keys
    }

    pub(crate) fn unique_query_keys(&self) -> &Vec<HashSet<String>> {
        &self.unique_query_keys
    }

    pub(crate) fn auth_identity_keys(&self) -> &Vec<String> { &self.auth_identity_keys }

    pub(crate) fn auth_by_keys(&self) -> &Vec<String> { &self.auth_by_keys }

    pub(crate) fn has_action(&self, action: ActionType) -> bool {
        self.actions.contains(&action)
    }

    pub(crate) fn actions(&self) -> &HashSet<ActionType> {
        &self.actions
    }

    pub(crate) fn has_field(&self, name: &str) -> bool {
        self.fields_map.get(name).is_some()
    }

    pub(crate) fn has_relation(&self, name: &str) -> bool {
        self.relations_map.get(name).is_some()
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
