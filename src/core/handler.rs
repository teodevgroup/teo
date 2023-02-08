use std::{collections::HashSet, slice::Iter};
use once_cell::sync::Lazy;
use maplit::hashset;

use self::Handler::*;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Handler {
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
pub enum ResMeta {
    PagingInfo,
    TokenInfo,
    NoMeta,
    Other,
}

#[derive(PartialEq)]
pub enum ResData {
    Single,
    Vec,
    Other,
    Number,
}

impl Handler {

    pub fn iter() -> Iter<'static, Handler> {
        static ACTION_TYPES: [Handler; 15] = [
            FindUnique, FindFirst, FindMany, Create, Update, Upsert, Delete, CreateMany,
            UpdateMany, DeleteMany, Count, Aggregate, GroupBy, SignIn, Identity
        ];
        ACTION_TYPES.iter()
    }

    pub(crate) fn default() -> HashSet<Handler> {
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
            FindUnique => "findUnique",
            FindFirst => "findFirst",
            FindMany => "findMany",
            Create => "create",
            Update => "update",
            Upsert => "upsert",
            Delete => "delete",
            CreateMany => "createMany",
            UpdateMany => "updateMany",
            DeleteMany => "deleteMany",
            Count => "count",
            Aggregate => "aggregate",
            GroupBy => "groupBy",
            SignIn => "signIn",
            Identity => "identity"
        }
    }

    pub(crate) fn from_str(str: &str) -> Option<Handler> {
        match str {
            "findUnique" => Some(FindUnique),
            "findFirst" => Some(FindFirst),
            "findMany" => Some(FindMany),
            "create" => Some(Create),
            "update" => Some(Update),
            "upsert" => Some(Upsert),
            "delete" => Some(Delete),
            "createMany" => Some(CreateMany),
            "updateMany" => Some(UpdateMany),
            "deleteMany" => Some(DeleteMany),
            "count" => Some(Count),
            "aggregate" => Some(Aggregate),
            "groupBy" => Some(GroupBy),
            "signIn" => Some(SignIn),
            "identity" => Some(Identity),
            _ => None
        }
    }

    pub(crate) fn res_meta(&self) -> ResMeta {
        match self {
            FindUnique => ResMeta::NoMeta,
            FindFirst => ResMeta::NoMeta,
            FindMany => ResMeta::PagingInfo,
            Create => ResMeta::NoMeta,
            Update => ResMeta::NoMeta,
            Upsert => ResMeta::NoMeta,
            Delete => ResMeta::NoMeta,
            CreateMany => ResMeta::NoMeta,
            UpdateMany => ResMeta::NoMeta,
            DeleteMany => ResMeta::NoMeta,
            Count => ResMeta::NoMeta,
            Aggregate => ResMeta::NoMeta,
            GroupBy => ResMeta::NoMeta,
            SignIn => ResMeta::TokenInfo,
            Identity => ResMeta::NoMeta,
        }
    }

    pub(crate) fn res_data(&self) -> ResData {
        match self {
            FindUnique => ResData::Single,
            FindFirst => ResData::Single,
            FindMany => ResData::Vec,
            Create => ResData::Single,
            Update => ResData::Single,
            Upsert => ResData::Single,
            Delete => ResData::Single,
            CreateMany => ResData::Vec,
            UpdateMany => ResData::Vec,
            DeleteMany => ResData::Vec,
            Count => ResData::Number,
            Aggregate => ResData::Other,
            GroupBy => ResData::Other,
            SignIn => ResData::Single,
            Identity => ResData::Single,
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
