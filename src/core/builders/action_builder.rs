use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::action::action::ActionType;
use crate::core::argument::{Argument, FnArgument};
use crate::core::argument::Argument::{PipelineArgument, ValueArgument};
use crate::core::connector::{ConnectorBuilder};
use crate::core::field::*;
use crate::core::pipeline::Pipeline;
use crate::core::value::Value;


pub struct ActionBuilder {
    pub(crate) actions: HashSet<ActionType>
}

impl ActionBuilder {
    pub(crate) fn new() -> Self {
        ActionBuilder { actions: HashSet::new() }
    }

    pub fn find_unique(&mut self) -> &mut Self {
        self.actions.insert(ActionType::FindUnique);
        self
    }

    pub fn find_first(&mut self) -> &mut Self {
        self.actions.insert(ActionType::FindFirst);
        self
    }

    pub fn find_many(&mut self) -> &mut Self {
        self.actions.insert(ActionType::FindMany);
        self
    }

    pub fn create(&mut self) -> &mut Self {
        self.actions.insert(ActionType::Create);
        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.actions.insert(ActionType::Update);
        self
    }

    pub fn upsert(&mut self) -> &mut Self {
        self.actions.insert(ActionType::Upsert);
        self
    }

    pub fn delete(&mut self) -> &mut Self {
        self.actions.insert(ActionType::Delete);
        self
    }

    pub fn create_many(&mut self) -> &mut Self {
        self.actions.insert(ActionType::CreateMany);
        self
    }

    pub fn update_many(&mut self) -> &mut Self {
        self.actions.insert(ActionType::UpdateMany);
        self
    }

    pub fn delete_many(&mut self) -> &mut Self {
        self.actions.insert(ActionType::DeleteMany);
        self
    }

    pub fn count(&mut self) -> &mut Self {
        self.actions.insert(ActionType::Count);
        self
    }

    pub fn aggregate(&mut self) -> &mut Self {
        self.actions.insert(ActionType::Aggregate);
        self
    }

    pub fn group_by(&mut self) -> &mut Self {
        self.actions.insert(ActionType::GroupBy);
        self
    }
}
