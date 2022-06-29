use std::collections::HashSet;
use std::slice::Iter;
use self::ActionType::*;


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

#[derive(PartialEq)]
pub enum ActionResultMeta {
    PagingInfo,
    TokenInfo,
    NoMeta,
    Other,
}

#[derive(PartialEq)]
pub enum ActionResultData {
    Single,
    Vec,
    Other,
    Number,
}

impl ActionType {

    pub fn iter() -> Iter<'static, ActionType> {
        static ACTION_TYPES: [ActionType; 14] = [
            FindUnique, FindFirst, FindMany, Create, Update, Upsert, Delete, CreateMany,
            UpdateMany, DeleteMany, Count, Aggregate, GroupBy, SignIn
        ];
        ACTION_TYPES.iter()
    }

    pub(crate) fn default() -> HashSet<ActionType> {
        HashSet::from_iter(vec![
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
        ].iter().map(|x| *x))
    }

    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            FindUnique => "find-unique",
            FindFirst => "find-first",
            FindMany => "find-many",
            Create => "create",
            Update => "update",
            Upsert => "upsert",
            Delete => "delete",
            CreateMany => "create-many",
            UpdateMany => "update-many",
            DeleteMany => "delete-many",
            Count => "count",
            Aggregate => "aggregate",
            GroupBy => "groupBy",
            SignIn => "sign-in",
        }
    }

    pub(crate) fn from_str(str: &str) -> Option<ActionType> {
        match str {
            "find-unique" => Some(FindUnique),
            "find-first" => Some(FindFirst),
            "find-many" => Some(FindMany),
            "create" => Some(Create),
            "update" => Some(Update),
            "upsert" => Some(Upsert),
            "delete" => Some(Delete),
            "create-many" => Some(CreateMany),
            "update-many" => Some(UpdateMany),
            "delete-many" => Some(DeleteMany),
            "count" => Some(Count),
            "aggregate" => Some(Aggregate),
            "group-by" => Some(GroupBy),
            "sign-in" => Some(SignIn),
            _ => None
        }
    }

    pub(crate) fn result_meta(&self) -> ActionResultMeta {
        match self {
            FindUnique => ActionResultMeta::NoMeta,
            FindFirst => ActionResultMeta::NoMeta,
            FindMany => ActionResultMeta::PagingInfo,
            Create => ActionResultMeta::NoMeta,
            Update => ActionResultMeta::NoMeta,
            Upsert => ActionResultMeta::NoMeta,
            Delete => ActionResultMeta::NoMeta,
            CreateMany => ActionResultMeta::NoMeta,
            UpdateMany => ActionResultMeta::NoMeta,
            DeleteMany => ActionResultMeta::NoMeta,
            Count => ActionResultMeta::NoMeta,
            Aggregate => ActionResultMeta::NoMeta,
            GroupBy => ActionResultMeta::NoMeta,
            SignIn => ActionResultMeta::TokenInfo,
        }
    }

    pub(crate) fn result_data(&self) -> ActionResultData {
        match self {
            FindUnique => ActionResultData::Single,
            FindFirst => ActionResultData::Single,
            FindMany => ActionResultData::Vec,
            Create => ActionResultData::Single,
            Update => ActionResultData::Single,
            Upsert => ActionResultData::Single,
            Delete => ActionResultData::Single,
            CreateMany => ActionResultData::Vec,
            UpdateMany => ActionResultData::Vec,
            DeleteMany => ActionResultData::Vec,
            Count => ActionResultData::Number,
            Aggregate => ActionResultData::Other,
            GroupBy => ActionResultData::Other,
            SignIn => ActionResultData::Single,
        }
    }

    pub(crate) fn requires_where(&self) -> bool {
        match self {
            FindUnique => false,
            FindFirst => true,
            FindMany => true,
            Create => false,
            Update => false,
            Upsert => false,
            Delete => false,
            CreateMany => false,
            UpdateMany => true,
            DeleteMany => true,
            Count => false,
            Aggregate => false,
            GroupBy => false,
            SignIn => false,
        }
    }

    pub(crate) fn requires_where_unique(&self) -> bool {
        match self {
            FindUnique => true,
            FindFirst => false,
            FindMany => false,
            Create => false,
            Update => true,
            Upsert => true,
            Delete => true,
            CreateMany => false,
            UpdateMany => false,
            DeleteMany => false,
            Count => false,
            Aggregate => false,
            GroupBy => false,
            SignIn => false,
        }
    }

    pub(crate) fn requires_create(&self) -> bool {
        match self {
            FindUnique => false,
            FindFirst => false,
            FindMany => false,
            Create => true,
            Update => false,
            Upsert => true,
            Delete => false,
            CreateMany => true,
            UpdateMany => false,
            DeleteMany => false,
            Count => false,
            Aggregate => false,
            GroupBy => false,
            SignIn => false,
        }
    }

    pub(crate) fn requires_update(&self) -> bool {
        match self {
            FindUnique => false,
            FindFirst => false,
            FindMany => false,
            Create => false,
            Update => true,
            Upsert => true,
            Delete => false,
            CreateMany => false,
            UpdateMany => true,
            DeleteMany => false,
            Count => false,
            Aggregate => false,
            GroupBy => false,
            SignIn => false,
        }
    }
}
