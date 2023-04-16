use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ops::BitOr;
use std::sync::Arc;
use async_recursion::async_recursion;
use inflector::Inflector;
use maplit::{hashmap, hashset};
use crate::core::action::{Action, FIND, IDENTITY, MANY, NESTED, SIGN_IN, SINGLE};
use crate::core::field::Field;
use crate::core::field::field::Field;
use crate::core::model::migration::ModelMigration;
use crate::core::pipeline::ctx::Ctx;
use crate::core::relation::Relation;
use crate::core::pipeline::Pipeline;
use crate::core::property::Property;
use crate::prelude::{Graph, Value};
use crate::core::result::Result;
use crate::teon;
use self::index::ModelIndex;

pub struct Model {
    name: &'static str,
    table_name: Cow<'static, str>,
    localized_name: Cow<'static, str>,
    description: Cow<'static, str>,
    identity: bool,
    r#virtual: bool,
    fields_vec: Vec<Arc<Field>>,
    fields_map: HashMap<String, Arc<Field>>,
    dropped_fields: Vec<Arc<Field>>,
    dropped_fields_map: HashMap<String, Arc<Field>>,
    relations_vec: Vec<Arc<Relation>>,
    relations_map: HashMap<String, Arc<Relation>>,
    properties_vec: Vec<Arc<Property>>,
    properties_map: HashMap<String, Arc<Property>>,
    indices: Vec<ModelIndex>,
    primary: Option<ModelIndex>,
    before_save_pipeline: Pipeline,
    after_save_pipeline: Pipeline,
    before_delete_pipeline: Pipeline,
    after_delete_pipeline: Pipeline,
    can_read_pipeline: Pipeline,
    can_mutate_pipeline: Pipeline,
    handler_actions: HashSet<Action>,
    disabled_actions: Option<Vec<Action>>,
    action_transformers: Vec<Pipeline>,
    migration: Option<ModelMigration>,
    teo_internal: bool,
    all_keys: Vec<String>,
    input_keys: Vec<String>,
    save_keys: Vec<String>,
    output_keys: Vec<String>,
    query_keys: Vec<String>,
    unique_query_keys: Vec<HashSet<String>>,
    sort_keys: Vec<String>,
    auth_identity_keys: Vec<String>,
    auth_by_keys: Vec<String>,
    auto_keys: Vec<String>,
    deny_relation_keys: Vec<String>,
    scalar_keys: Vec<String>,
    scalar_number_keys: Vec<String>,
    local_output_keys: Vec<String>,
    relation_output_keys: Vec<String>,
    field_property_map: HashMap<String, Vec<String>>,
}

impl Model {

    pub(crate) fn new(name: &'static str, table_name: Option<&'static str>, localized_name: Option<&'static str>, description: Option<&'static str>) -> Self {
        Self {
            name,
            table_name: table_name.map_or_else(|| Cow::Owned(name.to_lowercase().to_plural()), |n| Cow::Borrowed(n)),
            localized_name: localized_name.map_or_else(|| Cow::Owned(name.to_sentence_case()), |n| Cow::Borrowed(n)),
            description: description.map_or_else(|| Cow::Borrowed("This model doesn't have a description."), |n| Cow::Borrowed(n)),
            identity: false,
            r#virtual: false,
            fields_vec: vec![],
            fields_map: hashmap!{},
            dropped_fields: vec![],
            dropped_fields_map: hashmap!{},
            relations_vec: vec![],
            relations_map: hashmap!{},
            properties_vec: vec![],
            properties_map: hashmap!{},
            indices: vec![],
            primary: None,
            before_save_pipeline: Pipeline::new(),
            after_save_pipeline: Pipeline::new(),
            before_delete_pipeline: Pipeline::new(),
            after_delete_pipeline: Pipeline::new(),
            can_read_pipeline: Pipeline::new(),
            can_mutate_pipeline: Pipeline::new(),
            all_keys: vec![],
            input_keys: vec![],
            save_keys: vec![],
            output_keys: vec![],
            query_keys: vec![],
            unique_query_keys: vec![],
            sort_keys: vec![],
            auth_identity_keys: vec![],
            auth_by_keys: vec![],
            auto_keys: vec![],
            deny_relation_keys: vec![],
            scalar_keys: vec![],
            scalar_number_keys: vec![],
            local_output_keys: vec![],
            relation_output_keys: vec![],
            field_property_map: hashmap!{},
            handler_actions: hashset!{},
            disabled_actions: None,
            action_transformers: vec![],
            migration: None,
            teo_internal: false,
        }
    }

    pub(crate) fn set_is_teo_internal(&mut self) {
        self.teo_internal = true;
    }

    pub(crate) fn is_teo_internal(&self) -> bool {
        self.teo_internal
    }

    pub fn fields(&self) -> &Vec<Arc<Field>> {
        return &self.fields_vec
    }

    pub fn properties(&self) -> &Vec<Arc<Property>> {
        return &self.properties_vec
    }

    pub fn relations(&self) -> &Vec<Arc<Relation>> {
        return &self.relations_vec
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub(crate) fn table_name(&self) -> &str {
        self.table_name.as_ref()
    }

    pub(crate) fn localized_name(&self) -> &str {
        self.localized_name.as_ref()
    }

    pub(crate) fn description(&self) -> &str {
        self.description.as_ref()
    }

    pub(crate) fn is_identity(&self) -> bool {
        self.identity
    }

    pub(crate) fn is_virtual(&self) -> bool {
        self.r#virtual
    }

    pub(crate) fn actions(&self) -> &HashSet<Action> {
        &self.handler_actions
    }

    pub(crate) fn deny_relation_keys(&self) -> &Vec<String> {
        return &self.deny_relation_keys
    }

    pub(crate) fn dropped_field(&self, name: &str) -> Option<&Field> {
        match self.dropped_fields_map.get(name) {
            Some(f) => Some(f.as_ref()),
            None => None
        }
    }

    pub(crate) fn field(&self, name: &str) -> Option<&Field> {
        match self.fields_map.get(name) {
            Some(f) => Some(f.as_ref()),
            None => None
        }
    }

    pub(crate) fn field_with_column_name(&self, name: &str) -> Option<&Field> {
        match self.fields_vec.iter().find(|f| { f.column_name() == name }) {
            Some(f) => Some(f.as_ref()),
            None => None
        }
    }

    pub(crate) fn relation(&self, name: &str) -> Option<&Relation> {
        match self.relations_map.get(name) {
            Some(r) => Some(r.as_ref()),
            None => None
        }
    }

    pub(crate) fn property(&self, name: &str) -> Option<&Property> {
        match self.properties_map.get(name) {
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

    pub(crate) fn query_keys(&self) -> &Vec<String> {
        &self.query_keys
    }

    pub(crate) fn unique_query_keys(&self) -> &Vec<HashSet<String>> {
        &self.unique_query_keys
    }

    pub(crate) fn sort_keys(&self) -> &Vec<String> {
        &self.sort_keys
    }

    pub(crate) fn auth_identity_keys(&self) -> &Vec<String> { &self.auth_identity_keys }

    pub(crate) fn auth_by_keys(&self) -> &Vec<String> { &self.auth_by_keys }

    pub(crate) fn auto_keys(&self) -> &Vec<String> { &self.auto_keys }

    pub(crate) fn scalar_keys(&self) -> &Vec<String> { &self.scalar_keys }

    pub(crate) fn scalar_number_keys(&self) -> &Vec<String> { &self.scalar_number_keys }

    pub(crate) fn allowed_keys_for_aggregate(&self, name: &str) -> HashSet<&str> {
        match name {
            "_count" => self.scalar_keys().iter().map(|k| k.as_str()).collect::<HashSet<&str>>().bitor(&hashset!{"_all"}),
            "_min" | "_max" => self.scalar_keys().iter().map(|k| k.as_str()).collect(),
            _ => self.scalar_number_keys().iter().map(|k| k.as_str()).collect(),
        }
    }

    pub(crate) fn local_output_keys(&self) -> &Vec<String> {
        &self.local_output_keys
    }

    pub(crate) fn relation_output_keys(&self) -> &Vec<String> {
        &self.relation_output_keys
    }

    pub(crate) fn field_property_map(&self) -> &HashMap<String, Vec<String>> {
        &self.field_property_map
    }

    pub(crate) fn has_action(&self, action: Action) -> bool {
        if let Some(disabled_actions) = self.disabled_actions() {
            if action.passes(disabled_actions) {
                return false;
            }
        }
        if ((action.to_u32() & IDENTITY) != 0) || ((action.to_u32() & SIGN_IN) != 0) {
            return self.identity;
        }
        true
    }

    pub(crate) fn has_field(&self, name: &str) -> bool {
        self.fields_map.get(name).is_some()
    }

    pub(crate) fn has_relation(&self, name: &str) -> bool {
        self.relations_map.get(name).is_some()
    }

    pub(crate) fn indices(&self) -> &Vec<ModelIndex> {
        &self.indices
    }

    pub(crate) fn primary_index(&self) -> &ModelIndex {
        self.primary.as_ref().unwrap()
    }

    pub(crate) fn before_save_pipeline(&self) -> &Pipeline {
        &self.before_save_pipeline
    }

    pub(crate) fn after_save_pipeline(&self) -> &Pipeline {
        &self.after_save_pipeline
    }

    pub(crate) fn before_delete_pipeline(&self) -> &Pipeline {
        &self.before_delete_pipeline
    }

    pub(crate) fn after_delete_pipeline(&self) -> &Pipeline {
        &self.after_delete_pipeline
    }

    pub(crate) fn can_mutate_pipeline(&self) -> &Pipeline { &self.can_mutate_pipeline }

    pub(crate) fn can_read_pipeline(&self) -> &Pipeline { &self.can_read_pipeline }

    pub(crate) fn migration(&self) -> Option<&ModelMigration> {
        self.migration.as_ref()
    }

    pub(crate) fn disabled_actions(&self) -> Option<&Vec<Action>> {
        self.disabled_actions.as_ref()
    }

    pub(crate) fn has_action_transformers(&self) -> bool {
        self.action_transformers.len() > 0
    }

    #[async_recursion]
    pub(crate) async fn transformed_action<'a: 'async_recursion>(&self, ctx: Ctx<'a>) -> Result<(Value, Action)> {
        let mut ctx = ctx;
        for transformer in self.action_transformers.iter() {
            ctx = transformer.process_with_ctx_result(ctx).await?;
        }
        let mut surface_value = ctx.value;
        if let Some(surface_map) = surface_value.as_hashmap_mut() {
            if let Some(include) = surface_map.get("include") {
                let mut transformed_include = teon!({});
                for (key, included_value) in include.as_hashmap().unwrap() {
                    let relation = self.relation(key).unwrap();
                    let (opposite_model, _opposite_relation) = Graph::current().opposite_relation(relation);
                    let find_action = if relation.is_vec() {
                        Action::from_u32(NESTED | FIND | MANY)
                    } else {
                        Action::from_u32(NESTED | FIND | SINGLE)
                    };
                    let inner = Ctx::initial_state_with_value(if included_value.is_bool() { teon!({}) } else {included_value.clone()}).with_action(find_action);
                    let result = opposite_model.transformed_action(inner).await?.0;
                    transformed_include.as_hashmap_mut().unwrap().insert(key.clone(), result);
                }
                surface_value.as_hashmap_mut().unwrap().insert("include".to_owned(), transformed_include);
            }
        }
        Ok((surface_value, ctx.action))
    }

    pub(crate) fn allows_drop_when_migrate(&self) -> bool {
        self.migration.map_or(false, |m| m.drop)
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.inner.name
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}
unsafe impl Send for ModelInner {}
unsafe impl Sync for ModelInner {}
