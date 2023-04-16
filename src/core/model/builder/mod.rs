use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use inflector::Inflector;
use to_mut::ToMut;
use crate::core::action::{Action, CREATE_HANDLER, CREATE_MANY_HANDLER, IDENTITY_HANDLER, SIGN_IN_HANDLER};
use crate::core::connector::Connector;
use crate::core::field::*;
use crate::core::field::Field;
use crate::core::field::r#type::FieldTypeOwner;
use crate::core::relation::Relation;
use crate::core::property::Property;
use crate::core::relation::delete_rule::DeleteRule;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::model::index::builder::{ModelIndexBuilder};
use crate::core::model::{Model, ModelInner};
use crate::core::model::migration::ModelMigration;
use crate::core::pipeline::Pipeline;


    pub(crate) fn build(&self, connector: Arc<dyn Connector>) -> Model {
        // generate indices from fields
        if field.index.is_some() {
            match &field.index.as_ref().unwrap() {
                FieldIndex::Index(settings) => {
                    indices.push(ModelIndex::new(ModelIndexType::Index, if settings.name.is_some() { Some(settings.name.as_ref().unwrap().clone()) } else { None }, vec![
                        ModelIndexItem::new(field.name(), settings.sort, settings.length)
                    ]));

                }
                FieldIndex::Unique(settings) => {
                    indices.push(ModelIndex::new(ModelIndexType::Unique, if settings.name.is_some() { Some(settings.name.as_ref().unwrap().clone()) } else { None }, vec![
                        ModelIndexItem::new(field.name(), settings.sort, settings.length)
                    ]));
                }
                FieldIndex::Primary(settings) => {
                    primary = Some(ModelIndex::new(ModelIndexType::Primary, if settings.name.is_some() { Some(settings.name.as_ref().unwrap().clone()) } else { None }, vec![
                        ModelIndexItem::new(field.name(), settings.sort, settings.length)
                    ]));
                    indices.push(primary.as_ref().unwrap().clone());
                }
            }
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
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            identity: self.identity,
            r#virtual: self.r#virtual,
            fields_vec,
            fields_map,
            dropped_fields: dropped_fields_vec,
            dropped_fields_map,
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
            sort_keys: self.sort_keys(),
            auth_identity_keys: self.get_auth_identity_keys(),
            auth_by_keys: self.get_auth_by_keys(),
            auto_keys: self.get_auto_keys(),
            deny_relation_keys: self.get_deny_relation_keys(),
            scalar_keys: self.get_scalar_keys(),
            scalar_number_keys: self.get_scalar_number_keys(),
            local_output_keys: self.output_field_keys_and_property_keys(),
            relation_output_keys: self.output_relation_keys(),
            field_property_map: self.get_field_property_map(),
            handler_actions: self.figure_out_actions(),
            disabled_actions: self.disabled_actions.clone(),
            action_transformers: self.action_transformers.clone(),
            migration: self.migration.clone(),
            teo_internal: self.teo_internal,
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

    pub(crate) fn sort_keys(&self) -> Vec<String> {
        self.fields.iter().filter(|f| f.sortable).map(|f| f.name().to_owned()).collect()
    }

    pub(crate) fn query_keys(&self) -> Vec<String> {
        let mut fields: Vec<String> = self.fields.iter()
            .filter(|f| f.queryable)
            .map(|f| f.name.clone())
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

    pub(crate) fn add_action_transformer(&mut self, pipeline: Pipeline) {
        self.action_transformers.push(pipeline);
    }

    pub(crate) fn figure_out_actions(&self) -> HashSet<Action> {
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
