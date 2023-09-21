use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ops::BitOr;
use std::sync::Arc;
use array_tool::vec::Join;
use async_recursion::async_recursion;
use inflector::Inflector;
use maplit::{hashmap, hashset};
use to_mut::ToMut;
use to_mut_proc_macro::ToMut;
use crate::app::app_ctx::AppCtx;
use crate::core::action::{Action, CREATE_HANDLER, CREATE_MANY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER};
use crate::core::action::{FIND, IDENTITY, MANY, NESTED, SIGN_IN, SINGLE};
use crate::core::field::field::{Field, FieldIndex, PreviousValueRule};
use crate::core::field::r#type::FieldTypeOwner;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::model::migration::ModelMigration;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::relation::Relation;
use crate::core::pipeline::Pipeline;
use crate::core::property::Property;
use crate::core::relation::delete_rule::DeleteRule;
use crate::prelude::Value;
use crate::core::result::Result;
use crate::core::field::indexable::FieldIndexable;
use crate::teon;

#[derive(ToMut)]
pub struct Model {
    name: &'static str,
    ns_path: Vec<String>,
    full_path: Vec<String>,
    table_name: Cow<'static, str>,
    localized_name: Cow<'static, str>,
    description: Cow<'static, str>,
    internal: bool,
    identity: bool,
    r#virtual: bool,
    fields_vec: Vec<Arc<Field>>,
    fields_map: HashMap<&'static str, Arc<Field>>,
    dropped_fields_vec: Vec<Arc<Field>>,
    dropped_fields_map: HashMap<&'static str, Arc<Field>>,
    relations_vec: Vec<Arc<Relation>>,
    relations_map: HashMap<&'static str, Arc<Relation>>,
    properties_vec: Vec<Arc<Property>>,
    properties_map: HashMap<&'static str, Arc<Property>>,
    indices: Vec<Arc<ModelIndex>>,
    primary: Option<Arc<ModelIndex>>,
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
    all_keys: Vec<&'static str>,
    input_keys: Vec<&'static str>,
    save_keys: Vec<&'static str>,
    save_keys_and_virtual_keys: Vec<&'static str>,
    output_keys: Vec<&'static str>,
    query_keys: Vec<&'static str>,
    unique_query_keys: Vec<HashSet<&'static str>>,
    sort_keys: Vec<&'static str>,
    auth_identity_keys: Vec<&'static str>,
    auth_by_keys: Vec<&'static str>,
    auto_keys: Vec<&'static str>,
    deny_relation_keys: Vec<&'static str>,
    scalar_keys: Vec<&'static str>,
    scalar_number_keys: Vec<&'static str>,
    local_output_keys: Vec<&'static str>,
    relation_output_keys: Vec<&'static str>,
    field_property_map: HashMap<&'static str, Vec<&'static str>>,
    has_virtual_fields: bool,
}

impl Model {

    pub(crate) fn new(name: &'static str, ns_path: Vec<String>, localized_name: Option<&'static str>, description: Option<&'static str>) -> Self {
        Self {
            name,
            ns_path: ns_path.clone(),
            full_path: {
                let mut path = ns_path.clone();
                path.push(name.to_string());
                path
            },
            table_name: Cow::Owned(if ns_path.is_empty() { name.to_lowercase() } else { "_".to_owned() + &ns_path.join("_") + "_" + &name.to_lowercase() }),
            localized_name: localized_name.map_or_else(|| Cow::Owned(name.to_sentence_case()), |n| Cow::Borrowed(n)),
            description: description.map_or_else(|| Cow::Borrowed("This model doesn't have a description."), |n| Cow::Borrowed(n)),
            identity: false,
            internal: false,
            r#virtual: false,
            fields_vec: vec![],
            fields_map: hashmap!{},
            dropped_fields_vec: vec![],
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
            save_keys_and_virtual_keys: vec![],
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
            has_virtual_fields: false,
        }
    }

    pub(crate) fn path(&self) -> Vec<&str> {
        self.full_path.iter().map(|s| s.as_str()).collect()
    }

    pub(crate) fn set_table_name(&mut self, table_name: &'static str) {
        if self.ns_path.is_empty() {
            self.table_name = Cow::Borrowed(table_name);
        } else {
            self.table_name = Cow::Owned("_".to_owned() + &self.ns_path.join("_") + "_" + table_name);
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

    pub(crate) fn deny_relation_keys(&self) -> &Vec<&str> {
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

    pub(crate) fn save_keys_and_virtual_keys(&self) -> &Vec<&str> {
        &self.save_keys_and_virtual_keys
    }

    pub(crate) fn all_keys(&self) -> &Vec<&str> { &self.all_keys }

    pub(crate) fn input_keys(&self) -> &Vec<&str> {
        &self.input_keys
    }

    pub(crate) fn save_keys(&self) -> &Vec<&str> {
        &self.save_keys
    }

    pub(crate) fn output_keys(&self) -> &Vec<&str> {
        &self.output_keys
    }

    pub(crate) fn query_keys(&self) -> &Vec<&str> {
        &self.query_keys
    }

    pub(crate) fn unique_query_keys(&self) -> &Vec<HashSet<&'static str>> {
        &self.unique_query_keys
    }

    pub(crate) fn sort_keys(&self) -> &Vec<&str> {
        &self.sort_keys
    }

    pub(crate) fn auth_identity_keys(&self) -> &Vec<&str> { &self.auth_identity_keys }

    pub(crate) fn auth_by_keys(&self) -> &Vec<&str> { &self.auth_by_keys }

    pub(crate) fn auto_keys(&self) -> &Vec<&str> { &self.auto_keys }

    pub(crate) fn scalar_keys(&self) -> &Vec<&str> { &self.scalar_keys }

    pub(crate) fn scalar_number_keys(&self) -> &Vec<&str> { &self.scalar_number_keys }

    pub(crate) fn allowed_keys_for_aggregate(&self, name: &str) -> HashSet<&str> {
        match name {
            "_count" => self.scalar_keys().iter().map(|k| *k).collect::<HashSet<&str>>().bitor(&hashset!{"_all"}),
            "_min" | "_max" => self.scalar_keys().iter().map(|k| *k).collect(),
            _ => self.scalar_number_keys().iter().map(|k| *k).collect(),
        }
    }

    pub(crate) fn local_output_keys(&self) -> &Vec<&'static str> {
        &self.local_output_keys
    }

    pub(crate) fn relation_output_keys(&self) -> &Vec<&'static str> {
        &self.relation_output_keys
    }

    pub(crate) fn field_property_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
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

    pub(crate) fn indices(&self) -> &Vec<Arc<ModelIndex>> {
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
    pub(crate) async fn transformed_action<'a: 'async_recursion>(&self, ctx: PipelineCtx<'a>) -> Result<(Value, Action)> {
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
                    let (opposite_model, _opposite_relation) = AppCtx::get()?.graph().opposite_relation(relation);
                    let find_action = if relation.is_vec() {
                        Action::from_u32(NESTED | FIND | MANY)
                    } else {
                        Action::from_u32(NESTED | FIND | SINGLE)
                    };
                    let inner = PipelineCtx::initial_state_with_value(if included_value.is_bool() { teon!({}) } else {included_value.clone()}, ctx.conn.clone(), ctx.req.clone()).with_action(find_action);
                    let result = opposite_model.transformed_action(inner).await?.0;
                    transformed_include.as_hashmap_mut().unwrap().insert(key.clone(), result);
                }
                surface_value.as_hashmap_mut().unwrap().insert("include".to_owned(), transformed_include);
            }
        }
        Ok((surface_value, ctx.action))
    }

    pub(crate) fn allows_drop_when_migrate(&self) -> bool {
        self.migration.as_ref().map_or(false, |m| m.drop)
    }

    pub(crate) fn install_field_index(&mut self, field_name: &'static str, field_index: &FieldIndex) {
        match field_index {
            FieldIndex::Index(settings) => {
                self.indices.push(Arc::new(ModelIndex::new(ModelIndexType::Index, if settings.name.is_some() { Some(settings.name.as_ref().unwrap().clone()) } else { None }, vec![
                    ModelIndexItem::new(field_name, settings.sort, settings.length)
                ])));
            }
            FieldIndex::Unique(settings) => {
                self.indices.push(Arc::new(ModelIndex::new(ModelIndexType::Unique, if settings.name.is_some() { Some(settings.name.as_ref().unwrap().clone()) } else { None }, vec![
                    ModelIndexItem::new(field_name, settings.sort, settings.length)
                ])));
            }
            FieldIndex::Primary(settings) => {
                let primary = Arc::new(ModelIndex::new(ModelIndexType::Primary, if settings.name.is_some() { Some(settings.name.as_ref().unwrap().clone()) } else { None }, vec![
                    ModelIndexItem::new(field_name, settings.sort, settings.length)
                ]));
                self.primary = Some(primary.clone());
                self.indices.push(primary.clone());
            }
        }
    }

    pub(crate) fn finalize(&'static mut self) {
        // generate indices from fields
        let fields_vec = self.fields_vec.clone();
        let properties_vec = self.properties_vec.clone();
        for field in &fields_vec {
            let field_name = Box::leak(Box::new(field.name().to_string())).as_str();
            if let Some(field_index) = field.index() {
                self.install_field_index(field_name, field_index);
            }
        }
        // generate indices from properties
        for property in &properties_vec {
            let field_name = Box::leak(Box::new(property.name().to_string())).as_str();
            if let Some(field_index) = property.index() {
                self.install_field_index(field_name, field_index);
            }
        }
        if self.primary.is_none() && !self.r#virtual {
            panic!("Model '{}' must has a primary field.", self.name);
        }
        // install recordPrevious for primary
        for key in self.primary.as_ref().unwrap().keys() {
            let field = self.fields_map.get(key).unwrap();
            field.as_ref().to_mut().previous_value_rule = PreviousValueRule::Keep;
        }
        //
        let indices = self.indices.clone();
        // load caches
        let all_field_keys: Vec<&str> = self.fields_vec.iter().map(|f| f.name()).collect();
        let all_relation_keys: Vec<&str> = self.relations_vec.iter().map(|r| r.name()).collect();
        let all_property_keys: Vec<&str> = self.properties_vec.iter().map(|p| p.name()).collect();
        let mut all_keys = vec![];
        all_keys.extend(all_field_keys.clone());
        all_keys.extend(all_relation_keys.clone());
        all_keys.extend(all_property_keys.clone());
        let input_field_keys: Vec<&str> = self.fields_vec.iter().filter(|&f| !f.write_rule.is_no_write()).map(|f| f.name).collect();
        let input_relation_keys = all_relation_keys.clone();
        let input_property_keys: Vec<&str> = self.properties_vec.iter().filter(|p| p.setter.is_some()).map(|p| p.name).collect();
        let mut input_keys = vec![];
        input_keys.extend(input_field_keys);
        input_keys.extend(input_relation_keys);
        input_keys.extend(input_property_keys);
        let field_save_keys: Vec<&str> = self.fields_vec.iter().filter(|f| { !f.r#virtual }).map(|f| f.name).collect();
        let property_save_keys: Vec<&str> = self.properties_vec.iter().filter(|p| p.cached).map(|p| p.name).collect();
        let mut save_keys = vec![];
        save_keys.extend(field_save_keys.clone());
        save_keys.extend(property_save_keys.clone());
        let mut save_keys_and_virtual_keys = vec![];
        save_keys_and_virtual_keys.extend(all_field_keys.clone());
        save_keys_and_virtual_keys.extend(property_save_keys);
        let output_field_keys: Vec<&str> = self.fields().iter().filter(|&f| { !f.read_rule.is_no_read() }).map(|f| { f.name }).collect();
        let output_relation_keys = all_relation_keys.clone();
        let output_property_keys: Vec<&str> = self.properties().iter().filter(|p| p.getter.is_some()).map(|p| p.name).collect();
        let mut output_keys = vec![];
        output_keys.extend(output_field_keys.iter());
        output_keys.extend(output_relation_keys.iter());
        output_keys.extend(output_property_keys.iter());
        let mut output_field_keys_and_property_keys = vec![];
        output_field_keys_and_property_keys.extend(output_field_keys);
        output_field_keys_and_property_keys.extend(output_property_keys);
        let sort_keys: Vec<&str> = self.fields_vec.iter().filter(|f| f.sortable).map(|f| f.name()).collect();
        let query_keys: Vec<&str> = {
            let mut query_keys: Vec<&str> = self.fields().iter().filter(|f| f.queryable).map(|f| f.name()).collect();
            query_keys.extend(all_relation_keys.iter());
            query_keys
        };
        let unique_query_keys: Vec<HashSet<&str>> = {
            let mut result = vec![];
            for index in &indices {
                let set = HashSet::from_iter(index.items().iter().map(|i| {
                    Box::leak(Box::new(i.field_name().to_owned())).as_str()
                }));
                result.push(set);
            }
            if let Some(primary) = &self.primary {
                result.push(HashSet::from_iter(primary.items().iter().map(|i| i.field_name())));
            }
            result
        };
        let auth_identity_keys: Vec<&str> = self.fields_vec.iter()
            .filter(|&f| { f.identity == true })
            .map(|f| { f.name })
            .collect();
        let auth_by_keys: Vec<&str> = self.fields_vec.iter()
            .filter(|&f| { f.identity_checker.is_some() })
            .map(|f| { f.name })
            .collect();
        let auto_keys: Vec<&str> = self.fields_vec
            .iter()
            .filter(|&f| { f.auto || f.auto_increment })
            .map(|f| f.name.clone())
            .collect();
        let deny_relation_keys: Vec<&str> = self.relations_vec
            .iter()
            .filter(|&r| { r.delete_rule() == DeleteRule::Deny })
            .map(|r| r.name())
            .collect();
        let scalar_keys: Vec<&str> = self.fields_vec
            .iter()
            .map(|f| f.name)
            .collect();
        let scalar_number_keys: Vec<&str> = self.fields_vec
            .iter()
            .filter(|f| f.field_type().is_number())
            .map(|f| f.name.clone())
            .collect();
        // assign cache keys
        self.all_keys = all_keys.clone();
        self.input_keys = input_keys;
        self.save_keys = save_keys;
        self.save_keys_and_virtual_keys = save_keys_and_virtual_keys;
        self.output_keys = output_keys;
        self.query_keys = query_keys;
        self.sort_keys = sort_keys;
        self.unique_query_keys = unique_query_keys;
        self.auth_identity_keys = auth_identity_keys;
        self.auth_by_keys = auth_by_keys;
        self.auto_keys = auto_keys;
        self.deny_relation_keys = deny_relation_keys;
        self.scalar_keys = scalar_keys;
        self.scalar_number_keys = scalar_number_keys;
        self.local_output_keys = output_field_keys_and_property_keys;
        self.relation_output_keys = output_relation_keys;

        // figure out actions
        self.handler_actions = {
            let mut default = if self.internal {
                HashSet::new()
            } else if self.r#virtual {
                HashSet::from([Action::from_u32(CREATE_HANDLER), Action::from_u32(CREATE_MANY_HANDLER)])
            } else {
                Action::handlers_default()
            };
            if self.identity {
                default.insert(Action::from_u32(SIGN_IN_HANDLER));
                default.insert(Action::from_u32(IDENTITY_HANDLER));
            }
            if let Some(disabled) = &self.disabled_actions {
                default.iter().filter(|a| {
                    !a.passes(disabled)
                }).cloned().collect()
            } else {
                default
            }
        };
        // field property map
        self.field_property_map = {
            let mut map = HashMap::new();
            for property in self.properties_vec.iter() {
                if property.cached {
                    for dependency in &property.dependencies {
                        if map.get(dependency).is_none() {
                            map.insert(*dependency, vec![]);
                        }
                        map.get_mut(dependency).unwrap().push(property.name)
                    }
                }
            }
            map
        };
    }

    pub(crate) fn add_action_transformer(&mut self, pipeline: Pipeline) {
        self.action_transformers.push(pipeline);
    }

    pub(crate) fn set_before_save_pipeline(&mut self, p: Pipeline) {
        self.before_save_pipeline = p;
    }

    pub(crate) fn set_after_save_pipeline(&mut self, p: Pipeline) {
        self.after_save_pipeline = p;
    }

    pub(crate) fn set_before_delete_pipeline(&mut self, p: Pipeline) {
        self.before_delete_pipeline = p;
    }

    pub(crate) fn set_after_delete_pipeline(&mut self, p: Pipeline) {
        self.after_delete_pipeline = p;
    }

    pub(crate) fn set_can_read_pipeline(&mut self, p: Pipeline) {
        self.can_read_pipeline = p;
    }

    pub(crate) fn set_can_mutate_pipeline(&mut self, p: Pipeline) {
        self.can_mutate_pipeline = p;
    }

    pub(crate) fn set_disabled_actions(&mut self, actions: Vec<Action>) {
        self.disabled_actions = Some(actions);
    }

    pub(crate) fn add_index(&mut self, index: ModelIndex) {
        if index.r#type().is_primary() {
            self.primary = Some(index.clone().into());
        }
        self.indices.push(index.into());
    }

    pub(crate) fn set_identity(&mut self, identity: bool) {
        self.identity = identity;
    }

    pub(crate) fn set_migration(&mut self, migration: ModelMigration) {
        self.migration = Some(migration);
    }

    pub(crate) fn set_virtual(&mut self, r#virtual: bool) {
        self.r#virtual = r#virtual;
    }

    pub(crate) fn add_field(&mut self, field: Field, name: &'static str) {
        let arc = Arc::new(field);
        self.fields_vec.push(arc.clone());
        self.fields_map.insert(name, arc.clone());
        if arc.r#virtual {
            self.has_virtual_fields = true;
        }
    }

    pub(crate) fn add_dropped_field(&mut self, field: Field, name: &'static str) {
        let arc = Arc::new(field);
        self.dropped_fields_vec.push(arc.clone());
        self.dropped_fields_map.insert(name, arc);
    }

    pub(crate) fn add_relation(&mut self, relation: Relation, name: &'static str) {
        let arc = Arc::new(relation);
        self.relations_vec.push(arc.clone());
        self.relations_map.insert(name, arc);
    }

    pub(crate) fn add_property(&mut self, property: Property, name: &'static str) {
        let arc = Arc::new(property);
        self.properties_vec.push(arc.clone());
        self.properties_map.insert(name, arc);
    }

    pub(crate) fn has_virtual_fields(&self) -> bool {
        self.has_virtual_fields
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}
