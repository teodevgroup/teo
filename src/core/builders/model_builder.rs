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


pub struct ModelBuilder {
    pub name: &'static str,
    pub table_name: &'static str,
    pub url_segment_name: &'static str,
    pub localized_name: &'static str,
    pub description: &'static str,
    pub identity: bool,
    pub fields: Vec<FieldBuilder>,
    pub actions: HashSet<ActionType>
}

impl ModelBuilder {

    pub fn new(name: &'static str) -> ModelBuilder {
        return ModelBuilder {
            name,
            table_name: "",
            url_segment_name: "",
            localized_name: "",
            description: "",
            identity: false,
            fields: Vec::new(),
            actions: ActionType::default(),
        }
    }

    pub fn table_name(&mut self, table_name: &'static str) {
        self.table_name = table_name;
    }

    pub fn url_segment_name(&mut self, url_segment_name: &'static str) {
        self.url_segment_name = url_segment_name;
    }

    pub fn localized_name(&mut self, localized_name: &'static str) {
        self.localized_name = localized_name;
    }

    pub fn description(&mut self, description: &'static str) {
        self.description = description;
    }

    pub fn identity(&mut self) {
        self.identity = true;
    }

    pub fn field<F: Fn(&mut FieldBuilder)>(&mut self, name: &'static str, build: F) {
        let mut f = FieldBuilder::new(name);
        build(&mut f);
        self.fields.push(f);
    }

    pub fn internal(&mut self) {
        self.actions = HashSet::new();
    }

    pub fn enable<F: Fn(&mut ActionBuilder)>(&mut self, build: F) {
        self.internal();
        let mut action_builder = ActionBuilder::new();
        build(&mut action_builder);
        self.actions = action_builder.actions.clone();
    }

    pub fn disable<F: Fn(&mut ActionBuilder)>(&mut self, build: F) {
        let mut action_builder = ActionBuilder::new();
        build(&mut action_builder);
        self.actions = HashSet::from_iter(self.actions.difference(&action_builder.actions).map(|x| *x));
    }
}
