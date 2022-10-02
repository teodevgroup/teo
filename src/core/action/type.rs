use std::{collections::HashSet, sync::Mutex, slice::Iter};
use once_cell::sync::Lazy;
use maplit::hashset;

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
    SignIn,
    Identity,
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
        static ACTION_TYPES: [ActionType; 15] = [
            FindUnique, FindFirst, FindMany, Create, Update, Upsert, Delete, CreateMany,
            UpdateMany, DeleteMany, Count, Aggregate, GroupBy, SignIn, Identity
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

    pub(crate) fn as_url_segment(&self) -> &'static str {
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
            GroupBy => "group-by",
            SignIn => "sign-in",
            Identity => "identity"
        }
    }

    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            FindUnique => "FindUnique",
            FindFirst => "FindFirst",
            FindMany => "FindMany",
            Create => "Create",
            Update => "Update",
            Upsert => "Upsert",
            Delete => "Delete",
            CreateMany => "CreateMany",
            UpdateMany => "UpdateMany",
            DeleteMany => "DeleteMany",
            Count => "Count",
            Aggregate => "Aggregate",
            GroupBy => "GroupBy",
            SignIn => "SignIn",
            Identity => "Identity"
        }
    }

    pub(crate) fn from_str(str: &str) -> Option<ActionType> {
        match str {
            "FindUnique" => Some(FindUnique),
            "FindFirst" => Some(FindFirst),
            "FindMany" => Some(FindMany),
            "Create" => Some(Create),
            "Update" => Some(Update),
            "Upsert" => Some(Upsert),
            "Delete" => Some(Delete),
            "CreateMany" => Some(CreateMany),
            "UpdateMany" => Some(UpdateMany),
            "DeleteMany" => Some(DeleteMany),
            "Count" => Some(Count),
            "Aggregate" => Some(Aggregate),
            "GroupBy" => Some(GroupBy),
            "SignIn" => Some(SignIn),
            "Identity" => Some(Identity),
            _ => None
        }
    }

    pub(crate) fn from_url_segment(str: &str) -> Option<ActionType> {
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
            "identity" => Some(Identity),
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
            Identity => ActionResultMeta::NoMeta,
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
            Identity => ActionResultData::Single,
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
            Identity => false,
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
            Identity => false,
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
            Identity => false,
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
            Identity => false,
        }
    }

    pub(crate) fn requires_credentials(&self) -> bool {
        self == &SignIn
    }

    pub(crate) fn requires_by_and_having(&self) -> bool {
        self == &GroupBy
    }

    pub(crate) fn requires_aggregates(&self) -> bool {
        self == &GroupBy || self == &Aggregate
    }

    pub(crate) fn allowed_input_json_keys(&self) -> &HashSet<&str> {
        match self {
            FindUnique => &FIND_UNIQUE_INPUT_JSON_KEYS,
            FindFirst => &FIND_FIRST_INPUT_JSON_KEYS,
            FindMany => &FIND_MANY_INPUT_JSON_KEYS,
            Create => &CREATE_INPUT_JSON_KEYS,
            Update => &UPDATE_INPUT_JSON_KEYS,
            Upsert => &UPSERT_INPUT_JSON_KEYS,
            Delete => &DELETE_INPUT_JSON_KEYS,
            CreateMany => &CREATE_MANY_INPUT_JSON_KEYS,
            UpdateMany => &UPDATE_MANY_INPUT_JSON_KEYS,
            DeleteMany => &DELETE_MANY_INPUT_JSON_KEYS,
            Count => &COUNT_INPUT_JSON_KEYS,
            Aggregate => &AGGREGATE_INPUT_JSON_KEYS,
            GroupBy => &GROUP_BY_INPUT_JSON_KEYS,
            SignIn => &SIGN_IN_INPUT_JSON_KEYS,
            Identity => &IDENTITY_INPUT_JSON_KEYS,
        }
    }
}

static FIND_UNIQUE_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "where"}
});
static FIND_FIRST_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "where", "orderBy", "skip", "cursor", "distinct"}
});
static FIND_MANY_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "where", "orderBy", "skip", "take", "pageSize", "pageNumber", "cursor", "distinct"}
});
static CREATE_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "create"}
});
static UPDATE_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "where", "update"}
});
static UPSERT_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "where", "create", "update"}
});
static DELETE_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"select", "where"}
});
static CREATE_MANY_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "create"}
});
static UPDATE_MANY_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "where", "update"}
});
static DELETE_MANY_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"select", "where"}
});
static COUNT_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"where", "orderBy", "skip", "take", "pageSize", "pageNumber", "cursor", "distinct"}
});
static AGGREGATE_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"_avg", "_count", "_sum", "_min", "_max", "where", "orderBy", "skip", "take", "pageSize", "pageNumber", "cursor"}
});
static GROUP_BY_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"_avg", "_count", "_sum", "_min", "_max", "by", "having", "where", "orderBy", "skip", "take", "pageSize", "pageNumber", "cursor"}
});
static SIGN_IN_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select", "credentials"}
});
static IDENTITY_INPUT_JSON_KEYS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset! {"include", "select"}
});
