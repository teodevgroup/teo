use std::collections::{HashMap, HashSet};
use crate::core::action::Action;
use crate::core::action::builder::list_builder::ActionListBuilder;
use crate::core::action::builder::action_builder::ActionBuilder;
use crate::core::action::r#type::ActionType;

pub mod list_builder;
pub mod action_builder;

pub(crate) struct ActionsBuilder {
    disabled: HashSet<ActionType>,
    actions: HashMap<ActionType, Action>,
}

impl ActionsBuilder {
    pub(crate) fn new() -> Self {
        ActionsBuilder {
            disabled: HashSet::new(),
            actions: HashMap::new(),
        }
    }

    pub fn disable_all(&mut self) -> &mut Self {
        self.disabled.extend(ActionType::iter());
        self
    }

    pub fn enable<F: Fn(&mut ActionListBuilder)>(&mut self, build: F) -> &mut Self {
        self.disable_all();
        let mut list_builder = ActionListBuilder::new();
        build(&mut list_builder);
        self.disabled = HashSet::from_iter(self.disabled.difference(&list_builder.actions).map(|x| *x));
        self
    }

    pub fn disable<F: Fn(&mut ActionListBuilder)>(&mut self, build: F) -> &mut Self {
        let mut list_builder = ActionListBuilder::new();
        build(&mut list_builder);
        self.disabled = list_builder.actions.clone();
        self
    }

    pub fn find_unique<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::FindUnique) {
            self.actions.insert(ActionType::FindUnique, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::FindUnique).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::FindUnique, action_builder.action.clone());
        self
    }

    pub fn find_first<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::FindFirst) {
            self.actions.insert(ActionType::FindFirst, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::FindFirst).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::FindFirst, action_builder.action.clone());
        self
    }

    pub fn find_many<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::FindMany) {
            self.actions.insert(ActionType::FindMany, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::FindMany).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::FindMany, action_builder.action.clone());
        self
    }

    pub fn create<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::Create) {
            self.actions.insert(ActionType::Create, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::Create).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::Create, action_builder.action.clone());
        self
    }

    pub fn update<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::Update) {
            self.actions.insert(ActionType::Update, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::Update).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::Update, action_builder.action.clone());
        self
    }

    pub fn upsert<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::Upsert) {
            self.actions.insert(ActionType::Upsert, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::Upsert).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::Upsert, action_builder.action.clone());
        self
    }

    pub fn delete<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::Delete) {
            self.actions.insert(ActionType::Delete, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::Delete).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::Delete, action_builder.action.clone());
        self
    }

    pub fn create_many<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::CreateMany) {
            self.actions.insert(ActionType::CreateMany, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::CreateMany).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::CreateMany, action_builder.action.clone());
        self
    }

    pub fn update_many<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::UpdateMany) {
            self.actions.insert(ActionType::UpdateMany, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::UpdateMany).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::UpdateMany, action_builder.action.clone());
        self
    }

    pub fn delete_many<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::DeleteMany) {
            self.actions.insert(ActionType::DeleteMany, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::DeleteMany).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::DeleteMany, action_builder.action.clone());
        self
    }

    pub fn count<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::Count) {
            self.actions.insert(ActionType::Count, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::Count).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::Count, action_builder.action.clone());
        self
    }

    pub fn aggregate<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::Aggregate) {
            self.actions.insert(ActionType::Aggregate, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::Aggregate).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::Aggregate, action_builder.action.clone());
        self
    }

    pub fn group_by<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::GroupBy) {
            self.actions.insert(ActionType::GroupBy, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::GroupBy).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::GroupBy, action_builder.action.clone());
        self
    }

    pub fn sign_in<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::SignIn) {
            self.actions.insert(ActionType::SignIn, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::SignIn).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::SignIn, action_builder.action.clone());
        self
    }

    pub fn identity<F: Fn(&mut ActionBuilder)>(&mut self, build: F) -> &mut Self {
        if !self.actions.contains_key(&ActionType::Identity) {
            self.actions.insert(ActionType::Identity, Action::new());
        }
        let action = self.actions.get_mut(&ActionType::Identity).unwrap();
        let mut action_builder = ActionBuilder::new(action.clone());
        build(&mut action_builder);
        self.actions.insert(ActionType::Identity, action_builder.action.clone());
        self
    }
}
