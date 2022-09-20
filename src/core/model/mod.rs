use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::core::action::Action;
use crate::core::action::r#type::ActionType;
use crate::core::field::{Field, Sort};


use crate::core::permission::Permission;
use crate::core::relation::Relation;


use crate::core::pipeline::Pipeline;

pub(crate) mod builder;

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

pub struct ModelInner {
    pub(crate) name: String,
    pub(crate) table_name: String,
    pub(crate) url_segment_name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) identity: bool,
    pub(crate) r#virtual: bool,
    pub(crate) actions: HashSet<ActionType>,
    pub(crate) action_defs: HashMap<ActionType, Action>,
    pub(crate) permission: Option<Permission>,
    pub(crate) fields_vec: Vec<Arc<Field>>,
    pub(crate) fields_map: HashMap<String, Arc<Field>>,
    pub(crate) relations_vec: Vec<Arc<Relation>>,
    pub(crate) relations_map: HashMap<String, Arc<Relation>>,
    pub(crate) indices: Vec<ModelIndex>,
    pub(crate) primary: ModelIndex,
    pub(crate) before_save_pipeline: Pipeline,
    pub(crate) after_save_pipeline: Pipeline,
    pub(crate) before_delete_pipeline: Pipeline,
    pub(crate) after_delete_pipeline: Pipeline,
    pub(crate) primary_field: Option<Arc<Field>>,
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

#[derive(Clone)]
pub struct Model {
    inner: Arc<ModelInner>
}

impl Model {

    pub(crate) fn new_with_inner(inner: Arc<ModelInner>) -> Model {
        Model { inner }
    }

    pub(crate) fn name(&self) -> &str {
        &self.inner.name
    }

    pub(crate) fn table_name(&self) -> &str {
        &self.inner.table_name
    }

    pub(crate) fn url_segment_name(&self) -> &str {
        &self.inner.url_segment_name
    }

    pub(crate) fn localized_name(&self) -> &str {
        &self.inner.localized_name
    }

    pub(crate) fn description(&self) -> &str {
        &self.inner.description
    }

    pub(crate) fn identity(&self) -> bool {
        self.inner.identity
    }

    pub(crate) fn actions(&self) -> &HashSet<ActionType> {
        &self.inner.actions
    }

    pub(crate) fn fields(&self) -> &Vec<Arc<Field>> {
        return &self.inner.fields_vec
    }

    pub(crate) fn relations(&self) -> &Vec<Arc<Relation>> {
        return &self.inner.relations_vec
    }

    pub(crate) fn field(&self, name: &str) -> Option<&Field> {
        match self.inner.fields_map.get(name) {
            Some(f) => Some(f.as_ref()),
            None => None
        }
    }

    pub(crate) fn relation(&self, name: &str) -> Option<&Relation> {
        match self.inner.relations_map.get(name) {
            Some(r) => Some(r.as_ref()),
            None => None
        }
    }

    pub(crate) fn primary_index(&self) -> &ModelIndex {
        &self.inner.primary
    }

    pub(crate) fn primary_field(&self) -> Option<&Field> {
        match &self.inner.primary_field {
            Some(f) => Some(f.as_ref()),
            None => None
        }
    }

    pub(crate) fn primary_field_name(&self) -> Option<&str> {
        match self.primary_field() {
            Some(field) => Some(&field.name),
            None => None
        }
    }

    pub(crate) fn column_name_for_field_name(&self, column_name: &str) -> Option<&str> {
        for field in self.fields().iter() {
            if field.column_name() == column_name {
                return Some(&field.name);
            }
        }
        None
    }

    pub(crate) fn all_keys(&self) -> &Vec<String> { &self.inner.all_keys }

    pub(crate) fn input_keys(&self) -> &Vec<String> {
        &self.inner.input_keys
    }

    pub(crate) fn save_keys(&self) -> &Vec<String> {
        &self.inner.save_keys
    }

    pub(crate) fn output_keys(&self) -> &Vec<String> {
        &self.inner.output_keys
    }

    pub(crate) fn get_value_keys(&self) -> &Vec<String> {
        &self.inner.get_value_keys
    }

    pub(crate) fn query_keys(&self) -> &Vec<String> {
        &self.inner.query_keys
    }

    pub(crate) fn unique_query_keys(&self) -> &Vec<HashSet<String>> {
        &self.inner.unique_query_keys
    }

    pub(crate) fn auth_identity_keys(&self) -> &Vec<String> { &self.inner.auth_identity_keys }

    pub(crate) fn auth_by_keys(&self) -> &Vec<String> { &self.inner.auth_by_keys }

    pub(crate) fn has_action(&self, action: ActionType) -> bool {
        self.inner.actions.contains(&action)
    }

    pub(crate) fn get_action_def(&self, action: ActionType) -> Option<&Action> {
        self.inner.action_defs.get(&action)
    }

    pub(crate) fn has_field(&self, name: &str) -> bool {
        self.inner.fields_map.get(name).is_some()
    }

    pub(crate) fn has_relation(&self, name: &str) -> bool {
        self.inner.relations_map.get(name).is_some()
    }

    pub(crate) fn indices(&self) -> &Vec<ModelIndex> {
        &self.inner.indices
    }

    pub(crate) fn primary(&self) -> &ModelIndex {
        &self.inner.primary
    }

    pub(crate) fn before_save_pipeline(&self) -> &Pipeline {
        &self.inner.before_save_pipeline
    }

    pub(crate) fn after_save_pipeline(&self) -> &Pipeline {
        &self.inner.after_save_pipeline
    }

    pub(crate) fn before_delete_pipeline(&self) -> &Pipeline {
        &self.inner.before_delete_pipeline
    }

    pub(crate) fn after_delete_pipeline(&self) -> &Pipeline {
        &self.inner.after_delete_pipeline
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.inner.name == other.inner.name
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}
unsafe impl Send for ModelInner {}
unsafe impl Sync for ModelInner {}
