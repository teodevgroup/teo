use std::collections::HashSet;
use crate::core::action::r#type::ActionType;

pub struct ActionListBuilder {
    pub(crate) actions: HashSet<ActionType>
}

impl ActionListBuilder {

    pub(crate) fn new() -> Self {
        ActionListBuilder { actions: HashSet::new() }
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

    pub fn sign_in(&mut self) -> &mut Self {
        self.actions.insert(ActionType::SignIn);
        self
    }

    pub fn identity(&mut self) -> &mut Self {
        self.actions.insert(ActionType::Identity);
        self
    }
}
