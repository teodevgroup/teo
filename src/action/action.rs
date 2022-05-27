use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum ActionType {
    FindUnique,
    FindFirst,
    FindMany,
    Create,
    Update,
    Upsert,
    Delete,
    CreateMany,
    UpdateMany,
    DeleteMany,
    Count,
    Aggregate,
    GroupBy,
    SignIn
}

impl ActionType {
    pub(crate) fn default() -> HashSet<ActionType> {
        HashSet::from_iter(vec![
            ActionType::FindUnique,
            ActionType::FindFirst,
            ActionType::FindMany,
            ActionType::Create,
            ActionType::Update,
            ActionType::Upsert,
            ActionType::Delete,
            ActionType::CreateMany,
            ActionType::UpdateMany,
            ActionType::DeleteMany,
            ActionType::Count,
            ActionType::Aggregate,
            ActionType::GroupBy
        ].iter().map(|x| *x))
    }

    pub(crate) fn from_str(str: &str) -> Option<ActionType> {
        match str {
            "FindUnique" => Some(ActionType::FindUnique),
            "FindFirst" => Some(ActionType::FindFirst),
            "FindMany" => Some(ActionType::FindMany),
            "Create" => Some(ActionType::Create),
            "Update" => Some(ActionType::Update),
            "Upsert" => Some(ActionType::Upsert),
            "Delete" => Some(ActionType::Delete),
            "CreateMany" => Some(ActionType::CreateMany),
            "UpdateMany" => Some(ActionType::UpdateMany),
            "DeleteMany" => Some(ActionType::DeleteMany),
            "Count" => Some(ActionType::Count),
            "Aggregate" => Some(ActionType::Aggregate),
            "GroupBy" => Some(ActionType::GroupBy),
            "SignIn" => Some(ActionType::SignIn),
            _ => None
        }
    }
}
