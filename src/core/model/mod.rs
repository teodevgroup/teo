use std::collections::{HashMap, HashSet};
use std::ops::{BitAnd, BitOr};
use std::sync::Arc;
use maplit::hashset;
use crate::core::action::Action;
use crate::core::action::r#type::ActionType;
use crate::core::field::Field;
use crate::core::permission::Permission;
use crate::core::relation::Relation;
use crate::core::pipeline::Pipeline;
use crate::core::property::Property;

use self::index::ModelIndex;

pub(crate) mod builder;
pub(crate) mod index;

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
    pub(crate) properties_vec: Vec<Arc<Property>>,
    pub(crate) properties_map: HashMap<String, Arc<Property>>,
    pub(crate) indices: Vec<ModelIndex>,
    pub(crate) primary: Option<ModelIndex>,
    pub(crate) before_save_pipeline: Pipeline,
    pub(crate) after_save_pipeline: Pipeline,
    pub(crate) before_delete_pipeline: Pipeline,
    pub(crate) after_delete_pipeline: Pipeline,
    pub(crate) all_keys: Vec<String>,
    pub(crate) input_keys: Vec<String>,
    pub(crate) save_keys: Vec<String>,
    pub(crate) output_keys: Vec<String>,
    pub(crate) query_keys: Vec<String>,
    pub(crate) unique_query_keys: Vec<HashSet<String>>,
    pub(crate) auth_identity_keys: Vec<String>,
    pub(crate) auth_by_keys: Vec<String>,
    pub(crate) auto_keys: Vec<String>,
    pub(crate) deny_relation_keys: Vec<String>,
    pub(crate) scalar_keys: Vec<String>,
    pub(crate) scalar_number_keys: Vec<String>,
    pub(crate) local_output_keys: Vec<String>,
    pub(crate) relation_output_keys: Vec<String>,
    pub(crate) field_property_map: HashMap<String, Vec<String>>
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

    pub(crate) fn r#virtual(&self) -> bool {
        self.inner.r#virtual
    }

    pub(crate) fn actions(&self) -> &HashSet<ActionType> {
        &self.inner.actions
    }

    pub(crate) fn fields(&self) -> &Vec<Arc<Field>> {
        return &self.inner.fields_vec
    }

    pub(crate) fn properties(&self) -> &Vec<Arc<Property>> {
        return &self.inner.properties_vec
    }

    pub(crate) fn relations(&self) -> &Vec<Arc<Relation>> {
        return &self.inner.relations_vec
    }

    pub(crate) fn deny_relation_keys(&self) -> &Vec<String> {
        return &self.inner.deny_relation_keys
    }

    pub(crate) fn field(&self, name: &str) -> Option<&Field> {
        match self.inner.fields_map.get(name) {
            Some(f) => Some(f.as_ref()),
            None => None
        }
    }

    pub(crate) fn field_with_column_name(&self, name: &str) -> Option<&Field> {
        match self.inner.fields_vec.iter().find(|f| { f.column_name() == name }) {
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

    pub(crate) fn property(&self, name: &str) -> Option<&Property> {
        match self.inner.properties_map.get(name) {
            Some(p) => Some(p.as_ref()),
            None => None
        }
    }

    pub(crate) fn primary_field_names(&self) -> Vec<&str> {
        self.primary_index().items().iter().map(|i| i.field_name()).collect::<Vec<&str>>()
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

    pub(crate) fn query_keys(&self) -> &Vec<String> {
        &self.inner.query_keys
    }

    pub(crate) fn unique_query_keys(&self) -> &Vec<HashSet<String>> {
        &self.inner.unique_query_keys
    }

    pub(crate) fn auth_identity_keys(&self) -> &Vec<String> { &self.inner.auth_identity_keys }

    pub(crate) fn auth_by_keys(&self) -> &Vec<String> { &self.inner.auth_by_keys }

    pub(crate) fn auto_keys(&self) -> &Vec<String> { &self.inner.auto_keys }

    pub(crate) fn scalar_keys(&self) -> &Vec<String> { &self.inner.scalar_keys }

    pub(crate) fn scalar_number_keys(&self) -> &Vec<String> { &self.inner.scalar_number_keys }

    pub(crate) fn allowed_keys_for_aggregate(&self, name: &str) -> HashSet<&str> {
        match name {
            "_count" => self.scalar_number_keys().iter().map(|k| k.as_str()).collect::<HashSet<&str>>().bitor(&hashset!{"_all"}),
            _ => self.scalar_number_keys().iter().map(|k| k.as_str()).collect(),
        }
    }

    pub(crate) fn local_output_keys(&self) -> &Vec<String> {
        &self.inner.local_output_keys
    }

    pub(crate) fn relation_output_keys(&self) -> &Vec<String> {
        &self.inner.relation_output_keys
    }

    pub(crate) fn field_property_map(&self) -> &HashMap<String, Vec<String>> {
        &self.inner.field_property_map
    }

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

    pub(crate) fn primary_index(&self) -> &ModelIndex {
        self.inner.primary.as_ref().unwrap()
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

    pub(crate) fn permission(&self) -> Option<&Permission> {
        self.inner.permission.as_ref()
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
