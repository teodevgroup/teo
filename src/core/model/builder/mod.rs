use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use inflector::Inflector;
use crate::core::action::Action;
use crate::core::action::builder::ActionsBuilder;
use crate::core::action::r#type::ActionType;
use crate::core::connector::{ConnectorBuilder};
use crate::core::field::*;
use crate::core::field::builder::FieldBuilder;
use crate::core::permission::builder::PermissionBuilder;
use crate::core::relation::builder::RelationBuilder;
use crate::core::relation::Relation;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::property::builder::PropertyBuilder;
use crate::core::property::Property;
use crate::core::relation::delete_rule::DeleteRule;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::model::index::builder::{ModelIndexBuilder};

pub struct ModelBuilder {
    pub(crate) name: String,
    pub(crate) table_name: String,
    pub(crate) url_segment_name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) identity: bool,
    pub(crate) internal: bool,
    pub(crate) r#virtual: bool,
    pub(crate) field_builders: Vec<FieldBuilder>,
    pub(crate) relation_builders: Vec<RelationBuilder>,
    pub(crate) property_builders: Vec<PropertyBuilder>,
    pub(crate) permission: Option<PermissionBuilder>,
    pub(crate) primary: Option<ModelIndex>,
    pub(crate) indices: Vec<ModelIndex>,
    pub(crate) before_save_pipeline: PipelineBuilder,
    pub(crate) after_save_pipeline: PipelineBuilder,
    pub(crate) before_delete_pipeline: PipelineBuilder,
    pub(crate) after_delete_pipeline: PipelineBuilder,
    pub(crate) actions_builder: ActionsBuilder,
    connector_builder: * const Box<dyn ConnectorBuilder>,
}

impl ModelBuilder {

    pub(crate) fn new(name: impl Into<String>, connector_builder: &Box<dyn ConnectorBuilder>) -> Self {
        Self {
            name: name.into(),
            table_name: "".to_string(),
            url_segment_name: "".to_string(),
            localized_name: "".to_string(),
            description: "".to_string(),
            identity: false,
            internal: false,
            r#virtual: false,
            field_builders: vec![],
            relation_builders: vec![],
            property_builders: vec![],
            permission: None,
            primary: None,
            indices: Vec::new(),
            before_save_pipeline: PipelineBuilder::new(),
            after_save_pipeline: PipelineBuilder::new(),
            before_delete_pipeline: PipelineBuilder::new(),
            after_delete_pipeline: PipelineBuilder::new(),
            actions_builder: ActionsBuilder::new(),
            connector_builder
        }
    }

    fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        unsafe {
            &*self.connector_builder
        }
    }

    pub fn table_name(&mut self, table_name: impl Into<String>) -> &mut Self {
        self.table_name = table_name.into();
        self
    }

    pub fn url_segment_name(&mut self, url_segment_name: impl Into<String>) -> &mut Self {
        self.url_segment_name = url_segment_name.into();
        self
    }

    pub fn localized_name(&mut self, localized_name: impl Into<String>) -> &mut Self {
        self.localized_name = localized_name.into();
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = description.into();
        self
    }

    pub fn identity(&mut self) -> &mut Self {
        self.identity = true;
        self
    }

    pub fn field<F: Fn(&mut FieldBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut f = FieldBuilder::new(name, self.connector_builder());
        build(&mut f);
        self.field_builders.push(f);
        self
    }

    pub fn relation<F: Fn(&mut RelationBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut f = RelationBuilder::new(name, self.connector_builder());
        build(&mut f);
        self.relation_builders.push(f);
        self
    }

    pub fn property<F: Fn(&mut PropertyBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut p = PropertyBuilder::new(name.into(), self.connector_builder());
        build(&mut p);
        self.property_builders.push(p);
        self
    }

    pub fn internal(&mut self) -> &mut Self {
        self.internal = true;
        self
    }

    pub fn r#virtual(&mut self) -> &mut Self {
        self.r#virtual = true;
        self
    }

    pub fn permissions<F: Fn(&mut PermissionBuilder)>(&mut self, build: F) -> &mut Self {
        let mut permission_builder = PermissionBuilder::new();
        build(&mut permission_builder);
        self.permission = Some(permission_builder);
        self
    }

    pub fn primary<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let string_keys: Vec<String> = keys.into_iter().map(Into::into).collect();
        let name = string_keys.join("_");
        let items: Vec<ModelIndexItem> = string_keys.iter().map(|k| {
            ModelIndexItem { field_name: k.to_string(), sort: Sort::Asc, len: None }
        }).collect();
        let keys: Vec<String> = items.iter().map(|i| i.field_name().to_string()).collect();
        let index = ModelIndex {
            index_type: ModelIndexType::Primary,
            name,
            items,
            keys,
        };
        let primary_index = index.clone();
        self.indices.push(index);
        self.primary = Some(primary_index);
        self
    }

    pub fn primary_settings<F: Fn(&mut ModelIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ModelIndexBuilder::new(ModelIndexType::Primary);
        build(&mut builder);
        self.primary = Some(builder.build());
        self
    }

    pub fn index<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let string_keys: Vec<String> = keys.into_iter().map(Into::into).collect();
        let name = string_keys.join("_");
        let items: Vec<ModelIndexItem> = string_keys.iter().map(|k| {
            ModelIndexItem { field_name: k.to_string(), sort: Sort::Asc, len: None }
        }).collect();
        let keys: Vec<String> = items.iter().map(|i| i.field_name().to_string()).collect();
        let index = ModelIndex {
            index_type: ModelIndexType::Index,
            name,
            items,
            keys,
        };
        self.indices.push(index);
        self
    }

    pub fn index_settings<F: Fn(&mut ModelIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ModelIndexBuilder::new(ModelIndexType::Index);
        build(&mut builder);
        self.indices.push(builder.build());
        self
    }

    pub fn unique<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let string_keys: Vec<String> = keys.into_iter().map(Into::into).collect();
        let name = string_keys.join("_");
        let items: Vec<ModelIndexItem> = string_keys.iter().map(|k| {
            ModelIndexItem { field_name: k.to_string(), sort: Sort::Asc, len: None }
        }).collect();
        let keys: Vec<String> = items.iter().map(|i| i.field_name().to_string()).collect();
        let index = ModelIndex {
            index_type: ModelIndexType::Unique,
            name,
            items,
            keys,
        };
        self.indices.push(index);
        self
    }

    pub fn unique_settings<F: Fn(&mut ModelIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ModelIndexBuilder::new(ModelIndexType::Unique);
        build(&mut builder);
        self.indices.push(builder.build());
        self
    }

    pub fn before_save<F: Fn(&mut PipelineBuilder)>(&mut self, callback: F) -> &mut Self {
        callback(&mut self.before_save_pipeline);
        self
    }

    pub fn after_save<F: Fn(&mut PipelineBuilder)>(&mut self, callback: F) -> &mut Self {
        callback(&mut self.after_save_pipeline);
        self
    }

    pub fn before_delete<F: Fn(&mut PipelineBuilder)>(&mut self, callback: F) -> &mut Self {
        callback(&mut self.before_delete_pipeline);
        self
    }

    pub fn after_delete<F: Fn(&mut PipelineBuilder)>(&mut self, callback: F) -> &mut Self {
        callback(&mut self.after_delete_pipeline);
        self
    }

    pub fn actions<F: Fn(&mut ActionsBuilder)>(&mut self, build: F) -> &mut Self {
        build(&mut self.actions_builder);
        self
    }

    pub(crate) fn build(&self, connector_builder: &Box<dyn ConnectorBuilder>) -> Model {
        let fields_vec: Vec<Arc<Field>> = self.field_builders.iter().map(|fb| { Arc::new(fb.build(connector_builder)) }).collect();
        let properties_vec: Vec<Arc<Property>> = self.property_builders.iter().map(|pb| { Arc::new(pb.build(connector_builder)) }).collect();
        let mut fields_map: HashMap<String, Arc<Field>> = HashMap::new();
        let mut properties_map: HashMap<String, Arc<Property>> = HashMap::new();
        let mut primary_field: Option<Arc<Field>> = None;
        let mut primary = self.primary.clone();
        let mut indices = self.indices.clone();
        for field in fields_vec.iter() {
            fields_map.insert(field.name.clone(), field.clone());
            if field.primary {
                primary_field = Some(field.clone());
                primary = Some(ModelIndex {
                    index_type: ModelIndexType::Primary,
                    name: "".to_string(),
                    items: vec![
                        ModelIndexItem {
                            field_name: field.name.clone(),
                            sort: Sort::Asc,
                            len: None
                        }
                    ]
                });
                indices.push(primary.as_ref().unwrap().clone());
            }
            if field.index != FieldIndex::NoIndex {
                match &field.index {
                    FieldIndex::Index(settings) => {
                        indices.push(ModelIndex {
                            index_type: ModelIndexType::Index,
                            name: if settings.name.is_some() { settings.name.as_ref().unwrap().clone() } else { field.name.clone() },
                            items: vec![
                                ModelIndexItem {
                                    field_name: field.name.clone(),
                                    sort: settings.sort,
                                    len: settings.length
                                }
                            ]
                        })
                    }
                    FieldIndex::Unique(settings) => {
                        indices.push(ModelIndex {
                            index_type: ModelIndexType::Unique,
                            name: if settings.name.is_some() { settings.name.as_ref().unwrap().clone() } else { field.name.clone() },
                            items: vec![
                                ModelIndexItem {
                                    field_name: field.name.clone(),
                                    sort: settings.sort,
                                    len: settings.length
                                }
                            ]
                        })
                    }
                    _ => { }
                }
            }
        }
        let mut relations_map: HashMap<String, Arc<Relation>> = HashMap::new();
        let relations_vec: Vec<Arc<Relation>> = self.relation_builders.iter().map(|rb| { Arc::new(rb.build(connector_builder, &fields_map )) }).collect();
        for relation in relations_vec.iter() {
            relations_map.insert(relation.name().to_owned(), relation.clone());
        }
        for property in properties_vec.iter() {
            properties_map.insert(property.name.clone(), property.clone());
        }

        if primary.is_none() && !self.r#virtual {
            panic!("Model '{}' must has a primary field.", self.name);
        }

        let unique_query_keys = Self::unique_query_keys(self, &indices, primary.as_ref());
        let _a = if self.url_segment_name == "" { self.name.to_kebab_case().to_plural() } else { self.url_segment_name.to_string() };
        let inner = ModelInner {
            name: self.name.clone(),
            table_name: if self.table_name == "" { self.name.to_lowercase().to_plural() } else { self.table_name.to_string() },
            url_segment_name: if self.url_segment_name == "" { self.name.to_kebab_case().to_plural() } else { self.url_segment_name.to_string() },
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            identity: self.identity,
            r#virtual: self.r#virtual,
            actions: self.figure_out_actions(),
            action_defs: self.figure_out_action_defs(),
            permission: if let Some(builder) = &self.permission { Some(builder.build()) } else { None },
            fields_vec,
            fields_map,
            relations_map,
            relations_vec,
            properties_vec,
            properties_map,
            primary,
            indices: indices.clone(),
            before_save_pipeline: self.before_save_pipeline.build(),
            after_save_pipeline: self.after_save_pipeline.build(),
            before_delete_pipeline: self.before_delete_pipeline.build(),
            after_delete_pipeline: self.after_delete_pipeline.build(),
            all_keys: self.all_keys(),
            input_keys: self.input_keys(),
            save_keys: self.save_keys(),
            output_keys: self.output_keys(),
            query_keys: self.query_keys(),
            unique_query_keys,
            auth_identity_keys: self.get_auth_identity_keys(),
            auth_by_keys: self.get_auth_by_keys(),
            auto_keys: self.get_auto_keys(),
            deny_relation_keys: self.get_deny_relation_keys(),
            field_property_map: self.get_field_property_map(),
        };
        Model::new_with_inner(Arc::new(inner))
    }

    fn all_field_keys(&self) -> Vec<String> {
        self.field_builders.iter().map(|f| f.name.clone()).collect()
    }

    fn all_relation_keys(&self) -> Vec<String> {
        self.relation_builders.iter().map(|r| r.name.clone()).collect()
    }

    fn all_property_keys(&self) -> Vec<String> {
        self.property_builders.iter().map(|p| p.name.clone()).collect()
    }

    fn all_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.all_field_keys());
        fields.extend(self.all_relation_keys());
        fields.extend(self.all_property_keys());
        fields
    }

    fn input_field_keys(&self) -> Vec<String> {
        self.field_builders.iter().filter(|&f| !f.write_rule.is_no_write()).map(|f| f.name.clone()).collect()
    }

    fn input_relation_keys(&self) -> Vec<String> {
        // todo: relation can also use readwrite rule
        self.relation_builders.iter().map(|r| r.name.clone()).collect()
    }

    fn input_property_keys(&self) -> Vec<String> {
        self.property_builders.iter().filter(|p| p.setter.is_some()).map(|p| p.name.clone()).collect()
    }

    fn input_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.input_field_keys());
        fields.extend(self.input_relation_keys());
        fields.extend(self.input_property_keys());
        fields
    }

    fn field_save_keys(&self) -> Vec<String> {
        self.field_builders.iter()
            .filter(|f| { !f.r#virtual })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn property_save_keys(&self) -> Vec<String> {
        self.property_builders.iter().filter(|p| p.cached).map(|p| p.name.clone()).collect()
    }

    fn save_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.field_save_keys());
        fields.extend(self.property_save_keys());
        fields
    }

    fn output_field_keys(&self) -> Vec<String> {
        self.field_builders.iter()
            .filter(|&f| { !f.read_rule.is_no_read() })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn output_relation_keys(&self) -> Vec<String> {
        self.all_relation_keys()
    }

    fn output_property_keys(&self) -> Vec<String> {
        self.property_builders.iter().filter(|p| p.getter.is_some()).map(|p| p.name.clone()).collect()
    }

    fn output_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.output_field_keys());
        fields.extend(self.output_relation_keys());
        fields.extend(self.output_property_keys());
        fields
    }

    pub(crate) fn query_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = self.field_builders.iter()
            .filter(|&f| { f.query_ability == QueryAbility::Queryable })
            .map(|f| { f.name.clone() })
            .collect();
        fields.extend(self.all_relation_keys());
        fields
    }

    pub(crate) fn unique_query_keys(&self, indices: &Vec<ModelIndex>, primary: Option<&ModelIndex>) -> Vec<HashSet<String>> {
        let mut result: Vec<HashSet<String>> = Vec::new();
        for index in indices {
            let set = HashSet::from_iter(index.items.iter().map(|i| {
                i.field_name.clone()
            }));
            result.push(set);
        }
        if let Some(primary) = primary {
            result.push(HashSet::from_iter(primary.items.iter().map(|i| i.field_name.clone())));
        }
        result
    }

    pub(crate) fn get_auth_identity_keys(&self) -> Vec<String> {
        self.field_builders.iter()
            .filter(|&f| { f.auth_identity == true })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn get_auth_by_keys(&self) -> Vec<String> {
        self.field_builders.iter()
            .filter(|&f| { f.auth_by == true })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn get_auto_keys(&self) -> Vec<String> {
        self.field_builders
            .iter()
            .filter(|&f| { f.auto || f.auto_increment })
            .map(|f| f.name.clone())
            .collect()
    }

    fn get_deny_relation_keys(&self) -> Vec<String> {
        self.relation_builders
            .iter()
            .filter(|&r| { r.delete_rule == DeleteRule::Deny })
            .map(|r| r.name.clone())
            .collect()
    }

    pub(crate) fn figure_out_actions(&self) -> HashSet<ActionType> {
        let mut default = if self.internal {
            HashSet::new()
        } else if self.r#virtual {
            HashSet::from([ActionType::Create, ActionType::CreateMany])
        } else {
            ActionType::default()
        };
        if self.identity {
            default.insert(ActionType::SignIn);
            default.insert(ActionType::Identity);
        }
        let disabled = self.actions_builder.get_disabled_list();
        HashSet::from_iter(default.difference(disabled).map(|x| *x))
    }

    pub(crate) fn figure_out_action_defs(&self) -> HashMap<ActionType, Action> {
        self.actions_builder.get_action_defs().clone()
    }

    fn get_field_property_map(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for property in self.property_builders.iter() {
            if property.cached {
                for dependency in property.dependencies.iter() {
                    if map.get(dependency).is_none() {
                        map.insert(dependency.clone(), vec![]);
                    }
                    map.get_mut(dependency).unwrap().push(property.name.clone())
                }
            }
        }
        map
    }
}
