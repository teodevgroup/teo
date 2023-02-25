pub(crate) mod source;

use std::collections::HashSet;
use std::slice::Iter;
use maplit::hashset;
use once_cell::sync::Lazy;

pub(crate) const CREATE: u32 = 1;
pub(crate) const UPDATE: u32 = 1 << 1;
pub(crate) const DELETE: u32 = 1 << 2;
pub(crate) const FIND: u32 = 1 << 3;
pub(crate) const FIRST: u32 = 1 << 4;
pub(crate) const CONNECT: u32 = 1 << 5;
pub(crate) const DISCONNECT: u32 = 1 << 6;
pub(crate) const SET: u32 = 1 << 7;
pub(crate) const JOIN: u32 = 1 << 8;
pub(crate) const SIGN_IN: u32 = 1 << 9;
pub(crate) const IDENTITY: u32 = 1 << 10;
pub(crate) const COUNT: u32 = 1 << 11;
pub(crate) const AGGREGATE: u32 = 1 << 12;
pub(crate) const GROUP_BY: u32 = 1 << 13;
pub(crate) const PROGRAM_CODE: u32 = 1 << 14;

pub(crate) const UPSERT: u32 = CREATE | UPDATE;
pub(crate) const CONNECT_OR_CREATE: u32 = CONNECT | CREATE;
pub(crate) const JOIN_CREATE: u32 = JOIN | CREATE;
pub(crate) const JOIN_DELETE: u32 = JOIN | DELETE;
pub(crate) const FIND_FIRST: u32 = FIND | FIRST;

pub(crate) const ENTRY: u32 = 1 << 15;
pub(crate) const NESTED: u32 = 1 << 16;
pub(crate) const INTERNAL_POSITION: u32 = 1 << 17;

pub(crate) const SINGLE: u32 = 1 << 18;
pub(crate) const MANY: u32 = 1 << 19;
pub(crate) const INTERNAL_AMOUNT: u32 = 1 << 20;

const ALL_NAMES: u32 = CREATE | UPDATE | UPSERT | DELETE | FIND | FIND_FIRST | CONNECT | CONNECT_OR_CREATE | DISCONNECT | SET | JOIN_CREATE | JOIN_DELETE | IDENTITY | SIGN_IN | COUNT | AGGREGATE | GROUP_BY;
const ALL_POSITIONS: u32 = ENTRY | NESTED | INTERNAL_POSITION;
const ALL_AMOUNTS: u32 = SINGLE | MANY | INTERNAL_AMOUNT;

const NOT_ALL_NAMES: u32 = !ALL_NAMES;
const NOT_ENTRY_NESTED: u32 = !ALL_POSITIONS;
const NOT_SINGLE_MANY: u32 = !ALL_AMOUNTS;

pub(crate) const FIND_UNIQUE_HANDLER: u32 = FIND | ENTRY | SINGLE;
pub(crate) const FIND_FIRST_HANDLER: u32 = FIND_FIRST | ENTRY | SINGLE;
pub(crate) const FIND_MANY_HANDLER: u32 = FIND | ENTRY | MANY;
pub(crate) const CREATE_HANDLER: u32 = CREATE | ENTRY | SINGLE;
pub(crate) const UPDATE_HANDLER: u32 = UPDATE | ENTRY | SINGLE;
pub(crate) const UPSERT_HANDLER: u32 = UPSERT | ENTRY | SINGLE;
pub(crate) const DELETE_HANDLER: u32 = DELETE | ENTRY | SINGLE;
pub(crate) const CREATE_MANY_HANDLER: u32 = CREATE | ENTRY | MANY;
pub(crate) const UPDATE_MANY_HANDLER: u32 = UPDATE | ENTRY | MANY;
pub(crate) const DELETE_MANY_HANDLER: u32 = DELETE | ENTRY | MANY;
pub(crate) const COUNT_HANDLER: u32 = COUNT | ENTRY;
pub(crate) const AGGREGATE_HANDLER: u32 = AGGREGATE | ENTRY;
pub(crate) const GROUP_BY_HANDLER: u32 = GROUP_BY | ENTRY;
pub(crate) const SIGN_IN_HANDLER: u32 = SIGN_IN | ENTRY;
pub(crate) const IDENTITY_HANDLER: u32 = IDENTITY | ENTRY;

pub(crate) const NESTED_CREATE_ACTION: u32 = CREATE | NESTED | SINGLE;
pub(crate) const NESTED_UPDATE_ACTION: u32 = UPDATE | NESTED | SINGLE;
pub(crate) const NESTED_UPSERT_ACTION: u32 = UPSERT | NESTED | SINGLE;
pub(crate) const NESTED_DELETE_ACTION: u32 = DELETE | NESTED | SINGLE;
pub(crate) const NESTED_CONNECT_OR_CREATE_ACTION: u32 = CONNECT_OR_CREATE | NESTED | SINGLE;
pub(crate) const NESTED_CONNECT_ACTION: u32 = CONNECT | NESTED | SINGLE;
pub(crate) const NESTED_DISCONNECT_ACTION: u32 = DISCONNECT | NESTED | SINGLE;
pub(crate) const NESTED_SET_ACTION: u32 = SET | NESTED | SINGLE;
pub(crate) const NESTED_CREATE_MANY_ACTION: u32 = CREATE | NESTED | MANY;
pub(crate) const NESTED_UPDATE_MANY_ACTION: u32 = UPDATE | NESTED | MANY;
pub(crate) const NESTED_DELETE_MANY_ACTION: u32 = DELETE | NESTED | MANY;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct Action {
    value: u32
}

impl Action {

    pub(crate) fn empty() -> Self {
        Self { value: 0 }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.value == 0
    }

    pub(crate) fn from_name(name: &str) -> Self {
        Action {
            value: match name {
                "create" => CREATE,
                "update" => UPDATE,
                "delete" => DELETE,
                "find" => FIND,
                "connect" => CONNECT,
                "disconnect" => DISCONNECT,
                "set" => SET,
                "join" => JOIN,
                "first" => FIRST,
                "count" => COUNT,
                "aggregate" => AGGREGATE,
                "groupBy" => GROUP_BY,
                "entry" => ENTRY,
                "nested" => NESTED,
                "internalLocation" => INTERNAL_POSITION,
                "single" => SINGLE,
                "many" => MANY,
                "internalAmount" => INTERNAL_AMOUNT,
                "programCode" => PROGRAM_CODE,
                "identity" => IDENTITY,
                _ => panic!("Unrecognized action option name '{}'.", name)
            }
        }
    }

    pub(crate) const fn from_u32(value: u32) -> Self {
        Self { value }
    }

    pub(crate) fn to_u32(&self) -> u32 {
        self.value
    }

    pub(crate) fn finalized(&self) -> Self {
        let mut value = self.value;
        if !self.contains_name_bits() {
            value = value | ALL_NAMES;
        }
        if !self.contains_position_bits() {
            value = value | ALL_POSITIONS;
        }
        if !self.contains_amount_bits() {
            value = value | ALL_AMOUNTS;
        }
        Self { value }
    }

    pub(crate) fn redirect(&self, action: Action) -> Self {
        let mut result = self.value;
        let new_names_bits = action.value & ALL_NAMES;
        if new_names_bits != 0 {
            result = (result & !ALL_NAMES) | new_names_bits;
        }
        let new_position_bits = action.value & ALL_POSITIONS;
        if new_position_bits != 0 {
            result = (result & !ALL_POSITIONS) | new_position_bits;
        }
        let new_amount_bits = action.value & ALL_AMOUNTS;
        if new_amount_bits != 0 {
            result = (result & !ALL_AMOUNTS) | new_amount_bits;
        }
        Self { value: result }
    }

    #[inline(always)]
    fn contains_name_bits(&self) -> bool {
        self.value & ALL_NAMES != 0
    }

    #[inline(always)]
    fn contains_position_bits(&self) -> bool {
        self.value & ALL_POSITIONS != 0
    }

    #[inline(always)]
    fn contains_amount_bits(&self) -> bool {
        self.value & ALL_AMOUNTS != 0
    }

    pub(crate) fn neg(&self) -> Self {
        let restore_name_bits = !self.contains_name_bits();
        let restore_entry_nested_bits = !self.contains_position_bits();
        let restore_single_many_bits = !self.contains_amount_bits();
        let mut value = !self.value;
        if restore_name_bits {
            value = value & NOT_ALL_NAMES;
        }
        if restore_entry_nested_bits {
            value = value & NOT_ENTRY_NESTED;
        }
        if restore_single_many_bits {
            value = value & NOT_SINGLE_MANY;
        }
        Self { value }
    }

    pub(crate) fn and(&self, other: Action) -> Self {
        Self { value: self.value & other.value }
    }

    pub(crate) fn or(&self, other: Action) -> Self {
        Self { value: self.value | other.value }
    }

    pub(crate) fn xor(&self, other: Action) -> Self {
        Self { value: self.value ^ other.value }
    }

    pub(crate) fn passes(&self, matchers: &Vec<Action>) -> bool {
        for matcher in matchers {
            let copy = self.finalized();
            let finalized_matcher = matcher.finalized().value;
            let result = finalized_matcher & copy.value;
            return ((result & ALL_NAMES) != 0) &&
                ((result & ALL_POSITIONS) != 0) &&
                ((result & ALL_AMOUNTS) != 0)
        }
        false
    }

    // handler
    pub(crate) fn handler_allowed_input_json_keys(&self) -> &HashSet<&str> {
        match self.value {
            FIND_UNIQUE_HANDLER => &FIND_UNIQUE_INPUT_JSON_KEYS,
            FIND_FIRST_HANDLER => &FIND_FIRST_INPUT_JSON_KEYS,
            FIND_MANY_HANDLER => &FIND_MANY_INPUT_JSON_KEYS,
            CREATE_HANDLER => &CREATE_INPUT_JSON_KEYS,
            UPDATE_HANDLER => &UPDATE_INPUT_JSON_KEYS,
            UPSERT_HANDLER => &UPSERT_INPUT_JSON_KEYS,
            DELETE_HANDLER => &DELETE_INPUT_JSON_KEYS,
            CREATE_MANY_HANDLER => &CREATE_MANY_INPUT_JSON_KEYS,
            UPDATE_MANY_HANDLER => &UPDATE_MANY_INPUT_JSON_KEYS,
            DELETE_MANY_HANDLER => &DELETE_MANY_INPUT_JSON_KEYS,
            COUNT_HANDLER => &COUNT_INPUT_JSON_KEYS,
            AGGREGATE_HANDLER => &AGGREGATE_INPUT_JSON_KEYS,
            GROUP_BY_HANDLER => &GROUP_BY_INPUT_JSON_KEYS,
            SIGN_IN_HANDLER => &SIGN_IN_INPUT_JSON_KEYS,
            IDENTITY_HANDLER => &IDENTITY_INPUT_JSON_KEYS,
            _ => unreachable!()
        }
    }

    pub(crate) fn handler_requires_aggregates(&self) -> bool {
        self.value == GROUP_BY_HANDLER || self.value == AGGREGATE_HANDLER
    }

    pub(crate) fn handler_requires_by_and_having(&self) -> bool {
        self.value == GROUP_BY_HANDLER
    }

    pub(crate) fn handler_requires_credentials(&self) -> bool {
        self.value == SIGN_IN_HANDLER
    }


    pub(crate) fn handler_requires_update(&self) -> bool {
        match self.value {
            UPDATE_HANDLER | UPSERT_HANDLER | UPDATE_MANY_HANDLER => true,
            _ => false,
        }
    }


    pub(crate) fn handler_requires_create(&self) -> bool {
        match self.value {
            CREATE_HANDLER | UPSERT_HANDLER | CREATE_MANY_HANDLER => true,
            _ => false,
        }
    }

    pub(crate) fn handler_requires_where_unique(&self) -> bool {
        match self.value {
            FIND_UNIQUE_HANDLER | UPDATE_HANDLER | UPSERT_HANDLER | DELETE_HANDLER => true,
            _ => false,
        }
    }

    pub(crate) fn handler_requires_where(&self) -> bool {
        match self.value {
            FIND_FIRST_HANDLER | FIND_MANY_HANDLER | UPDATE_MANY_HANDLER | DELETE_MANY_HANDLER => true,
            _ => false,
        }
    }

    pub(crate) fn handler_res_meta(&self) -> ResMeta {
        match self.value {
            FIND_UNIQUE_HANDLER => ResMeta::NoMeta,
            FIND_FIRST_HANDLER => ResMeta::NoMeta,
            FIND_MANY_HANDLER => ResMeta::PagingInfo,
            CREATE_HANDLER => ResMeta::NoMeta,
            UPDATE_HANDLER => ResMeta::NoMeta,
            UPSERT_HANDLER => ResMeta::NoMeta,
            DELETE_HANDLER => ResMeta::NoMeta,
            CREATE_MANY_HANDLER => ResMeta::NoMeta,
            UPDATE_MANY_HANDLER => ResMeta::NoMeta,
            DELETE_MANY_HANDLER => ResMeta::NoMeta,
            COUNT_HANDLER => ResMeta::NoMeta,
            AGGREGATE_HANDLER => ResMeta::NoMeta,
            GROUP_BY_HANDLER => ResMeta::NoMeta,
            SIGN_IN_HANDLER => ResMeta::TokenInfo,
            IDENTITY_HANDLER => ResMeta::NoMeta,
            _ => unreachable!()
        }
    }

    pub(crate) fn handler_res_data(&self) -> ResData {
        match self.value {
            FIND_UNIQUE_HANDLER => ResData::Single,
            FIND_FIRST_HANDLER => ResData::Single,
            FIND_MANY_HANDLER => ResData::Vec,
            CREATE_HANDLER => ResData::Single,
            UPDATE_HANDLER => ResData::Single,
            UPSERT_HANDLER => ResData::Single,
            DELETE_HANDLER => ResData::Single,
            CREATE_MANY_HANDLER => ResData::Vec,
            UPDATE_MANY_HANDLER => ResData::Vec,
            DELETE_MANY_HANDLER => ResData::Vec,
            COUNT_HANDLER => ResData::Number,
            AGGREGATE_HANDLER => ResData::Other,
            GROUP_BY_HANDLER => ResData::Other,
            SIGN_IN_HANDLER => ResData::Single,
            IDENTITY_HANDLER => ResData::Single,
            _ => unreachable!()
        }
    }

    pub(crate) fn as_handler_str(&self) -> &'static str {
        match self.to_u32() {
            FIND_UNIQUE_HANDLER => "findUnique",
            FIND_FIRST_HANDLER => "findFirst",
            FIND_MANY_HANDLER => "findMany",
            CREATE_HANDLER => "create",
            UPDATE_HANDLER => "update",
            UPSERT_HANDLER => "upsert",
            DELETE_HANDLER => "delete",
            CREATE_MANY_HANDLER => "createMany",
            UPDATE_MANY_HANDLER => "updateMany",
            DELETE_MANY_HANDLER => "deleteMany",
            COUNT_HANDLER => "count",
            AGGREGATE_HANDLER => "aggregate",
            GROUP_BY_HANDLER => "groupBy",
            SIGN_IN_HANDLER => "signIn",
            IDENTITY_HANDLER => "identity",
            _ => unreachable!()
        }
    }

    pub(crate) fn handler_from_name(name: &str) -> Option<Self> {
        Some(Action {
            value: match name {
                "findUnique" => FIND_UNIQUE_HANDLER,
                "findFirst" => FIND_FIRST_HANDLER,
                "findMany" => FIND_MANY_HANDLER,
                "create" => CREATE_HANDLER,
                "update" => UPDATE_HANDLER,
                "upsert" => UPSERT_HANDLER,
                "delete" => DELETE_HANDLER,
                "createMany" => CREATE_MANY_HANDLER,
                "updateMany" => UPDATE_MANY_HANDLER,
                "deleteMany" => DELETE_MANY_HANDLER,
                "count" => COUNT_HANDLER,
                "aggregate" => AGGREGATE_HANDLER,
                "groupBy" => GROUP_BY_HANDLER,
                "signIn" => SIGN_IN_HANDLER,
                "identity" => IDENTITY_HANDLER,
                _ => None?
            }
        })
    }

    pub(crate) fn handlers_iter() -> Iter<'static, Action> {
        static HANDLER_TYPES: [Action; 15] = [
            Action::from_u32(FIND_UNIQUE_HANDLER),
            Action::from_u32(FIND_FIRST_HANDLER),
            Action::from_u32(FIND_MANY_HANDLER),
            Action::from_u32(CREATE_HANDLER),
            Action::from_u32(UPDATE_HANDLER),
            Action::from_u32(UPSERT_HANDLER),
            Action::from_u32(DELETE_HANDLER),
            Action::from_u32(CREATE_MANY_HANDLER),
            Action::from_u32(UPDATE_MANY_HANDLER),
            Action::from_u32(DELETE_MANY_HANDLER),
            Action::from_u32(COUNT_HANDLER),
            Action::from_u32(AGGREGATE_HANDLER),
            Action::from_u32(GROUP_BY_HANDLER),
            Action::from_u32(SIGN_IN_HANDLER),
            Action::from_u32(IDENTITY_HANDLER),
        ];
        HANDLER_TYPES.iter()
    }

    pub(crate) fn handlers_default() -> HashSet<Action> {
        HashSet::from_iter(vec![
            Action::from_u32(FIND_UNIQUE_HANDLER),
            Action::from_u32(FIND_FIRST_HANDLER),
            Action::from_u32(FIND_MANY_HANDLER),
            Action::from_u32(CREATE_HANDLER),
            Action::from_u32(UPDATE_HANDLER),
            Action::from_u32(UPSERT_HANDLER),
            Action::from_u32(DELETE_HANDLER),
            Action::from_u32(CREATE_MANY_HANDLER),
            Action::from_u32(UPDATE_MANY_HANDLER),
            Action::from_u32(DELETE_MANY_HANDLER),
            Action::from_u32(COUNT_HANDLER),
            Action::from_u32(AGGREGATE_HANDLER),
            Action::from_u32(GROUP_BY_HANDLER),
            Action::from_u32(SIGN_IN_HANDLER),
            Action::from_u32(IDENTITY_HANDLER),
        ].iter().map(|x| *x))
    }

    pub(crate) fn nested_action_from_name(name: &str) -> Option<Self> {
        Some(Action {
            value: match name {
                "create" => NESTED_CREATE_ACTION,
                "update" => NESTED_UPDATE_ACTION,
                "upsert" => NESTED_UPSERT_ACTION,
                "delete" => NESTED_DELETE_ACTION,
                "connect" => NESTED_CONNECT_ACTION,
                "connectOrCreate" => NESTED_CONNECT_OR_CREATE_ACTION,
                "disconnect" => NESTED_DISCONNECT_ACTION,
                "set" => NESTED_SET_ACTION,
                "createMany" => NESTED_CREATE_ACTION,
                "updateMany" => NESTED_UPDATE_MANY_ACTION,
                "deleteMany" => NESTED_DELETE_MANY_ACTION,
                _ => None?
            }
        })
    }
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
