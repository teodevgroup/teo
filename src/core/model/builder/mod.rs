use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use inflector::Inflector;
use to_mut::ToMut;
use crate::core::handler::Handler;
use crate::core::connector::Connector;
use crate::core::field::*;
use crate::core::field::Field;
use crate::core::relation::Relation;
use crate::core::property::Property;
use crate::core::relation::delete_rule::DeleteRule;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::model::index::builder::{ModelIndexBuilder};
use crate::core::model::{Model, ModelInner};
use crate::core::pipeline::Pipeline;

pub struct ModelBuilder {
    pub(crate) name: String,
    pub(crate) table_name: String,
    pub(crate) url_segment_name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) identity: bool,
    pub(crate) internal: bool,
    pub(crate) r#virtual: bool,
    pub(crate) fields: Vec<Field>,
    pub(crate) relations: Vec<Relation>,
    pub(crate) properties: Vec<Property>,
    pub(crate) primary: Option<ModelIndex>,
    pub(crate) indices: Vec<ModelIndex>,
    pub(crate) before_save_pipeline: Pipeline,
    pub(crate) after_save_pipeline: Pipeline,
    pub(crate) before_delete_pipeline: Pipeline,
    pub(crate) after_delete_pipeline: Pipeline,
    pub(crate) can_read_pipeline: Pipeline,
    pub(crate) can_mutate_pipeline: Pipeline,
}

impl ModelBuilder {

    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            table_name: "".to_string(),
            url_segment_name: "".to_string(),
            localized_name: "".to_string(),
            description: "".to_string(),
            identity: false,
            internal: false,
            r#virtual: false,
            fields: vec![],
            relations: vec![],
            properties: vec![],
            primary: None,
            indices: Vec::new(),
            before_save_pipeline: Pipeline::new(),
            after_save_pipeline: Pipeline::new(),
            before_delete_pipeline: Pipeline::new(),
            after_delete_pipeline: Pipeline::new(),
            can_read_pipeline: Pipeline::new(),
            can_mutate_pipeline: Pipeline::new(),
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

    pub(crate) fn field(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);
        self
    }

    pub(crate) fn relation(&mut self, relation: Relation) -> &mut Self {
        self.relations.push(relation);
        self
    }

    pub(crate) fn property(&mut self, property: Property) -> &mut Self {
        self.properties.push(property);
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

    pub fn primary<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let string_keys: Vec<String> = keys.into_iter().map(Into::into).collect();
        let name = string_keys.join("_");
        let items: Vec<ModelIndexItem> = string_keys.iter().map(|k| {
            ModelIndexItem::new(k, Sort::Asc, None)
        }).collect();
        let primary_index = ModelIndex::new(ModelIndexType::Primary, name, items);
        self.indices.push(primary_index.clone());
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
            ModelIndexItem::new(k, Sort::Asc, None)
        }).collect();
        let index = ModelIndex::new(ModelIndexType::Index, name, items);
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
            ModelIndexItem::new(k, Sort::Asc, None)
        }).collect();
        let index = ModelIndex::new(ModelIndexType::Unique, name, items);
        self.indices.push(index);
        self
    }

    pub fn unique_settings<F: Fn(&mut ModelIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ModelIndexBuilder::new(ModelIndexType::Unique);
        build(&mut builder);
        self.indices.push(builder.build());
        self
    }

    pub(crate) fn build(&self, connector: Arc<dyn Connector>) -> Model {
        let fields_vec: Vec<Arc<Field>> = self.fields.clone().iter_mut().map(|fb| { Arc::new({ fb.finalize(connector.clone()); fb.clone()}) }).collect();
        let properties_vec: Vec<Arc<Property>> = self.properties.clone().iter_mut().map(|pb| { Arc::new({ pb.finalize(connector.clone()); pb.clone() }) }).collect();
        let mut fields_map: HashMap<String, Arc<Field>> = HashMap::new();
        let mut properties_map: HashMap<String, Arc<Property>> = HashMap::new();
        let mut primary = self.primary.clone();
        let mut indices = self.indices.clone();
        for field in fields_vec.iter() {
            fields_map.insert(field.name.clone(), field.clone());
            if field.primary {
                primary = Some(ModelIndex::new(ModelIndexType::Primary, field.name(), vec![
                    ModelIndexItem::new(field.name(), Sort::Asc, None)
                ]));
                indices.push(primary.as_ref().unwrap().clone());
            }
            if field.index.is_some() {
                match &field.index.as_ref().unwrap() {
                    FieldIndex::Index(settings) => {
                        indices.push(ModelIndex::new(ModelIndexType::Index, if settings.name.is_some() { settings.name.as_ref().unwrap().clone() } else { field.name.clone() }, vec![
                            ModelIndexItem::new(field.name(), settings.sort, settings.length)
                        ]));

                    }
                    FieldIndex::Unique(settings) => {
                        indices.push(ModelIndex::new(ModelIndexType::Unique, if settings.name.is_some() { settings.name.as_ref().unwrap().clone() } else { field.name.clone() }, vec![
                            ModelIndexItem::new(field.name(), settings.sort, settings.length)
                        ]));
                    }
                }
            }
        }
        let mut relations_map: HashMap<String, Arc<Relation>> = HashMap::new();
        let relations_vec: Vec<Arc<Relation>> = self.relations.iter().map(|rb| { Arc::new(rb.clone()) }).collect();
        for relation in relations_vec.iter() {
            relations_map.insert(relation.name().to_owned(), relation.clone());
        }
        for property in properties_vec.iter() {
            properties_map.insert(property.name.clone(), property.clone());
        }

        if primary.is_none() && !self.r#virtual {
            panic!("Model '{}' must has a primary field.", self.name);
        }
        // install recordPrevious for primary
        for key in primary.as_ref().unwrap().keys() {
            let field = fields_map.get(key).unwrap();
            field.as_ref().to_mut().previous_value_rule = PreviousValueRule::Keep;
        }
        let unique_query_keys = Self::unique_query_keys(self, &indices, primary.as_ref());
        let inner = ModelInner {
            name: self.name.clone(),
            table_name: if self.table_name == "" { self.name.to_lowercase().to_plural() } else { self.table_name.to_string() },
            url_segment_name: if self.url_segment_name == "" { self.name.to_kebab_case().to_plural() } else { self.url_segment_name.to_string() },
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            identity: self.identity,
            r#virtual: self.r#virtual,
            actions: self.figure_out_actions(),
            fields_vec,
            fields_map,
            relations_map,
            relations_vec,
            properties_vec,
            properties_map,
            primary,
            indices: indices.clone(),
            before_save_pipeline: self.before_save_pipeline.clone(),
            after_save_pipeline: self.after_save_pipeline.clone(),
            before_delete_pipeline: self.before_delete_pipeline.clone(),
            after_delete_pipeline: self.after_delete_pipeline.clone(),
            can_read_pipeline: self.can_read_pipeline.clone(),
            can_mutate_pipeline: self.can_mutate_pipeline.clone(),
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
            scalar_keys: self.get_scalar_keys(),
            scalar_number_keys: self.get_scalar_number_keys(),
            local_output_keys: self.output_field_keys_and_property_keys(),
            relation_output_keys: self.output_relation_keys(),
            field_property_map: self.get_field_property_map(),
        };
        Model::new_with_inner(Arc::new(inner))
    }

    fn all_field_keys(&self) -> Vec<String> {
        self.fields.iter().map(|f| f.name.clone()).collect()
    }

    fn all_relation_keys(&self) -> Vec<String> {
        self.relations.iter().map(|r| r.name().to_owned()).collect()
    }

    fn all_property_keys(&self) -> Vec<String> {
        self.properties.iter().map(|p| p.name.clone()).collect()
    }

    fn all_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.all_field_keys());
        fields.extend(self.all_relation_keys());
        fields.extend(self.all_property_keys());
        fields
    }

    fn input_field_keys(&self) -> Vec<String> {
        self.fields.iter().filter(|&f| !f.write_rule.is_no_write()).map(|f| f.name.clone()).collect()
    }

    fn input_relation_keys(&self) -> Vec<String> {
        // todo: relation can also use readwrite rule
        self.relations.iter().map(|r| r.name().to_owned()).collect()
    }

    fn input_property_keys(&self) -> Vec<String> {
        self.properties.iter().filter(|p| p.setter.is_some()).map(|p| p.name.clone()).collect()
    }

    fn input_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.input_field_keys());
        fields.extend(self.input_relation_keys());
        fields.extend(self.input_property_keys());
        fields
    }

    fn field_save_keys(&self) -> Vec<String> {
        self.fields.iter()
            .filter(|f| { !f.r#virtual })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn property_save_keys(&self) -> Vec<String> {
        self.properties.iter().filter(|p| p.cached).map(|p| p.name.clone()).collect()
    }

    fn save_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.field_save_keys());
        fields.extend(self.property_save_keys());
        fields
    }

    fn output_field_keys(&self) -> Vec<String> {
        self.fields.iter()
            .filter(|&f| { !f.read_rule.is_no_read() })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn output_relation_keys(&self) -> Vec<String> {
        self.all_relation_keys()
    }

    fn output_property_keys(&self) -> Vec<String> {
        self.properties.iter().filter(|p| p.getter.is_some()).map(|p| p.name.clone()).collect()
    }

    fn output_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.output_field_keys());
        fields.extend(self.output_relation_keys());
        fields.extend(self.output_property_keys());
        fields
    }

    fn output_field_keys_and_property_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = vec![];
        fields.extend(self.output_field_keys());
        fields.extend(self.output_property_keys());
        fields
    }

    pub(crate) fn query_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = self.fields.iter()
            .filter(|&f| { f.query_ability == QueryAbility::Queryable })
            .map(|f| { f.name.clone() })
            .collect();
        fields.extend(self.all_relation_keys());
        fields
    }

    pub(crate) fn unique_query_keys(&self, indices: &Vec<ModelIndex>, primary: Option<&ModelIndex>) -> Vec<HashSet<String>> {
        let mut result: Vec<HashSet<String>> = Vec::new();
        for index in indices {
            let set = HashSet::from_iter(index.items().iter().map(|i| {
                i.field_name().to_string()
            }));
            result.push(set);
        }
        if let Some(primary) = primary {
            result.push(HashSet::from_iter(primary.items().iter().map(|i| i.field_name().to_string())));
        }
        result
    }

    pub(crate) fn get_auth_identity_keys(&self) -> Vec<String> {
        self.fields.iter()
            .filter(|&f| { f.identity == true })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn get_auth_by_keys(&self) -> Vec<String> {
        self.fields.iter()
            .filter(|&f| { f.identity_checker.is_some() })
            .map(|f| { f.name.clone() })
            .collect()
    }

    fn get_auto_keys(&self) -> Vec<String> {
        self.fields
            .iter()
            .filter(|&f| { f.auto || f.auto_increment })
            .map(|f| f.name.clone())
            .collect()
    }

    fn get_deny_relation_keys(&self) -> Vec<String> {
        self.relations
            .iter()
            .filter(|&r| { r.delete_rule() == DeleteRule::Deny })
            .map(|r| r.name().to_owned())
            .collect()
    }

    fn get_scalar_keys(&self) -> Vec<String> {
        self.fields
            .iter()
            .map(|f| f.name.clone())
            .collect()
    }

    fn get_scalar_number_keys(&self) -> Vec<String> {
        self.fields
            .iter()
            .filter(|f| f.field_type().is_number())
            .map(|f| f.name.clone())
            .collect()
    }

    pub(crate) fn figure_out_actions(&self) -> HashSet<Handler> {
        let mut default = if self.internal {
            HashSet::new()
        } else if self.r#virtual {
            HashSet::from([Handler::Create, Handler::CreateMany])
        } else {
            Handler::default()
        };
        if self.identity {
            default.insert(Handler::SignIn);
            default.insert(Handler::Identity);
        }
        default
        // let disabled = self.actions_builder.get_disabled_list();
        // HashSet::from_iter(default.difference(disabled).map(|x| *x))
    }

    fn get_field_property_map(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for property in self.properties.iter() {
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

unsafe impl Send for ModelBuilder { }
unsafe impl Sync for ModelBuilder { }
