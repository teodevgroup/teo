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
    GroupBy
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
}
