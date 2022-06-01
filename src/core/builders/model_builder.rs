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
use crate::core::builders::permission_builder::PermissionBuilder;


pub struct ModelBuilder {
    pub name: &'static str,
    pub table_name: &'static str,
    pub url_segment_name: &'static str,
    pub localized_name: &'static str,
    pub description: &'static str,
    pub identity: bool,
    pub fields: Vec<FieldBuilder>,
    pub actions: HashSet<ActionType>,
    pub permission: Option<PermissionBuilder>,
    pub primary: Option<Vec<String>>,
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
            primary: None
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

    pub fn primary<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String>, {
        self.primary = Some(keys.into_iter().map(Into::into).collect());
        self
    }
}
