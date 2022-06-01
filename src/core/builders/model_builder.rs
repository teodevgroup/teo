use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::action::action::ActionType;
use crate::core::argument::{Argument, FnArgument};
use crate::core::argument::Argument::{PipelineArgument, ValueArgument};
use crate::core::connector::{ConnectorBuilder};
use crate::core::field::*;
use crate::core::pipeline::Pipeline;
use crate::core::value::Value;
use crate::core::builders::field_builder::FieldBuilder;
use crate::core::builders::action_builder::ActionBuilder;
use crate::core::builders::model_index_builder::{ModelIndexBuilder};
use crate::core::builders::permission_builder::PermissionBuilder;
use crate::core::model::{CompoundIndex, CompoundIndexItem, ModelIndexType};


pub struct ModelBuilder {
    pub(crate) name: &'static str,
    pub(crate) table_name: &'static str,
    pub(crate) url_segment_name: &'static str,
    pub(crate) localized_name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) identity: bool,
    pub(crate) fields: Vec<FieldBuilder>,
    pub(crate) actions: HashSet<ActionType>,
    pub(crate) permission: Option<PermissionBuilder>,
    pub(crate) primary: Option<CompoundIndex>,
    pub(crate) compound_indices: Vec<CompoundIndex>,
}

impl ModelBuilder {

    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            table_name: "",
            url_segment_name: "",
            localized_name: "",
            description: "",
            identity: false,
            fields: Vec::new(),
            actions: ActionType::default(),
            permission: None,
            primary: None,
            compound_indices: Vec::new(),
        }
    }

    pub fn table_name(&mut self, table_name: &'static str) -> &mut Self {
        self.table_name = table_name;
        self
    }

    pub fn url_segment_name(&mut self, url_segment_name: &'static str) -> &mut Self {
        self.url_segment_name = url_segment_name;
        self
    }

    pub fn localized_name(&mut self, localized_name: &'static str) -> &mut Self {
        self.localized_name = localized_name;
        self
    }

    pub fn description(&mut self, description: &'static str) -> &mut Self {
        self.description = description;
        self
    }

    pub fn identity(&mut self) -> &mut Self {
        self.identity = true;
        self.actions.insert(ActionType::SignIn);
        self
    }

    pub fn field<F: Fn(&mut FieldBuilder)>(&mut self, name: &'static str, build: F) -> &mut Self {
        let mut f = FieldBuilder::new(name);
        build(&mut f);
        self.fields.push(f);
        self
    }

    pub fn internal(&mut self) -> &mut Self {
        self.actions = HashSet::new();
        self
    }

    pub fn enable<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        self.internal();
        let mut action_builder = ActionBuilder::new();
        build(&mut action_builder);
        self.actions = action_builder.actions.clone();
        if self.identity {
            self.actions.insert(ActionType::SignIn);
        }
        self
    }

    pub fn disable<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        let mut action_builder = ActionBuilder::new();
        build(&mut action_builder);
        self.actions = HashSet::from_iter(self.actions.difference(&action_builder.actions).map(|x| *x));
        if self.identity {
            self.actions.insert(ActionType::SignIn);
        }
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
        let items = string_keys.iter().map(|k| {
            CompoundIndexItem { field_name: k.to_string(), sort: Sort::Asc, len: None }
        }).collect();
        let index = CompoundIndex {
            index_type: ModelIndexType::Primary,
            name,
            items
        };
        self.primary = Some(index);
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
        let items = string_keys.iter().map(|k| {
            CompoundIndexItem { field_name: k.to_string(), sort: Sort::Asc, len: None }
        }).collect();
        let index = CompoundIndex {
            index_type: ModelIndexType::Index,
            name,
            items
        };
        self.compound_indices.push(index);
        self
    }

    pub fn index_settings<F: Fn(&mut ModelIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ModelIndexBuilder::new(ModelIndexType::Index);
        build(&mut builder);
        self.compound_indices.push(builder.build());
        self
    }

    pub fn unique<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let string_keys: Vec<String> = keys.into_iter().map(Into::into).collect();
        let name = string_keys.join("_");
        let items = string_keys.iter().map(|k| {
            CompoundIndexItem { field_name: k.to_string(), sort: Sort::Asc, len: None }
        }).collect();
        let index = CompoundIndex {
            index_type: ModelIndexType::Unique,
            name,
            items
        };
        self.compound_indices.push(index);
        self
    }

    pub fn unique_settings<F: Fn(&mut ModelIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ModelIndexBuilder::new(ModelIndexType::Unique);
        build(&mut builder);
        self.compound_indices.push(builder.build());
        self
    }

}
