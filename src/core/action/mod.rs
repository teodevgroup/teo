pub(crate) mod source;

pub(crate) const CREATE: u32 = 1;
pub(crate) const UPDATE: u32 = 1 << 1;
pub(crate) const UPSERT: u32 = 1 << 2;
pub(crate) const DELETE: u32 = 1 << 3;
pub(crate) const FIND: u32 = 1 << 4;
pub(crate) const CONNECT: u32 = 1 << 5;
pub(crate) const CONNECT_OR_CREATE: u32 = 1 << 6;
pub(crate) const DISCONNECT: u32 = 1 << 7;
pub(crate) const SET: u32 = 1 << 8;
pub(crate) const JOIN_CREATE: u32 = 1 << 9;
pub(crate) const JOIN_DELETE: u32 = 1 << 10;
pub(crate) const IDENTITY: u32 = 1 << 11;
pub(crate) const PROGRAM_CODE: u32 = 1 << 12;
pub(crate) const INTENT: u32 = 1 << 13;
pub(crate) const ACTUAL: u32 = 1 << 14;
pub(crate) const ENTRY: u32 = 1 << 15;
pub(crate) const NESTED: u32 = 1 << 16;
pub(crate) const INTERNAL_LOCATION: u32 = 1 << 17;
pub(crate) const SINGLE: u32 = 1 << 18;
pub(crate) const MANY: u32 = 1 << 19;
pub(crate) const INTERNAL_AMOUNT: u32 = 1 << 20;

const ALL_NAMES: u32 = CREATE | UPDATE | UPSERT | DELETE | FIND | CONNECT | CONNECT_OR_CREATE | DISCONNECT | SET | JOIN_CREATE | JOIN_DELETE;
const INTENT_ACTUAL: u32 = INTENT | ACTUAL;
const ENTRY_NESTED: u32 = ENTRY | NESTED | INTERNAL_LOCATION;
const SINGLE_MANY: u32 = SINGLE | MANY | INTERNAL_AMOUNT;

const NOT_ALL_NAMES: u32 = !ALL_NAMES;
const NOT_INTENT_ACTUAL: u32 = !INTENT_ACTUAL;
const NOT_ENTRY_NESTED: u32 = !ENTRY_NESTED;
const NOT_SINGLE_MANY: u32 = !SINGLE_MANY;

const DISABLE_INTENT: u32 = !(UPSERT | CONNECT_OR_CREATE | JOIN_CREATE | JOIN_DELETE);
const DISABLE_CREATE_UPDATE: u32 = !(CREATE | UPDATE);
const DISABLE_CONNECT_CREATE: u32 = !(CONNECT | CREATE);
const DISABLE_CREATE: u32 = !CREATE;
const DISABLE_DELETE: u32 = !DELETE;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Action {
    value: u32
}

impl Action {

    pub(crate) fn program_code_create() -> Self {
        Self { value: PROGRAM_CODE | CREATE | SINGLE | ENTRY }
    }

    pub(crate) fn from_name(name: &str) -> Self {
        Action {
            value: match name {
                "create" => CREATE,
                "update" => UPDATE,
                "upsert" => UPSERT,
                "delete" => DELETE,
                "find" => FIND,
                "connect" => CONNECT,
                "connectOrCreate" => CONNECT_OR_CREATE,
                "disconnect" => DISCONNECT,
                "set" => SET,
                "joinCreate" => JOIN_CREATE,
                "joinDelete" => JOIN_DELETE,
                "intent" => INTENT,
                "actual" => ACTUAL,
                "entry" => ENTRY,
                "nested" => NESTED,
                "internalLocation" => INTERNAL_LOCATION,
                "single" => SINGLE,
                "many" => MANY,
                "internalAmount" => INTERNAL_AMOUNT,
                "programCode" => PROGRAM_CODE,
                "identity" => IDENTITY,
                _ => panic!("Unrecognized action option name '{}'.", name)
            }
        }
    }

    pub(crate) fn from_u32(value: u32) -> Self {
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
        if !self.contains_intent_actual_bits() {
            value = value | INTENT_ACTUAL;
        }
        if !self.contains_entry_nested_bits() {
            value = value | ENTRY_NESTED;
        }
        if !self.contains_single_many_bits() {
            value = value | SINGLE_MANY;
        }
        Self { value }
    }

    #[inline(always)]
    fn contains_name_bits(&self) -> bool {
        self.value & ALL_NAMES != 0
    }

    #[inline(always)]
    fn contains_intent_actual_bits(&self) -> bool {
        self.value & INTENT_ACTUAL != 0
    }

    #[inline(always)]
    fn contains_entry_nested_bits(&self) -> bool {
        self.value & ENTRY_NESTED != 0
    }

    #[inline(always)]
    fn contains_single_many_bits(&self) -> bool {
        self.value & SINGLE_MANY != 0
    }

    pub(crate) fn neg(&self) -> Self {
        let restore_name_bits = !self.contains_name_bits();
        let restore_intent_actual_bits = !self.contains_intent_actual_bits();
        let restore_entry_nested_bits = !self.contains_entry_nested_bits();
        let restore_single_many_bits = !self.contains_single_many_bits();
        let mut value = !self.value;
        if restore_name_bits {
            value = value & NOT_ALL_NAMES;
        }
        if restore_intent_actual_bits {
            value = value & NOT_INTENT_ACTUAL;
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

    fn disable_intent(&mut self) {
        self.value = self.value & DISABLE_INTENT;
    }

    fn disable_actual(&mut self) {
        if (self.value & UPSERT) != 0 {
            self.value = self.value & DISABLE_CREATE_UPDATE;
        }
        if (self.value & CONNECT_OR_CREATE) != 0 {
            self.value = self.value & DISABLE_CONNECT_CREATE;
        }
        if (self.value & JOIN_CREATE) != 0 {
            self.value = self.value & DISABLE_CREATE;
        }
        if (self.value & JOIN_DELETE) != 0 {
            self.value = self.value & DISABLE_DELETE;
        }
    }

    pub(crate) fn passes(&self, matchers: &Vec<Action>) -> bool {
        for matcher in matchers {
            let finalized_matcher = matcher.finalized().value;
            let mut copy = self.finalized();
            if ((finalized_matcher & ACTUAL) != 0) && ((finalized_matcher & INTENT) == 0) {
                copy.disable_intent();
            }
            if ((finalized_matcher & ACTUAL) == 0) && ((finalized_matcher & INTENT) != 0) {
                copy.disable_actual();
            }
            let result = finalized_matcher & copy.value;
            return ((result & ALL_NAMES) != 0) &&
                ((result & ENTRY_NESTED) != 0) &&
                ((result & SINGLE_MANY) != 0) &&
                ((result & INTENT_ACTUAL) != 0)
        }
        false
    }
}
