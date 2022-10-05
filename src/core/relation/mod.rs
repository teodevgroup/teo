use serde_json::{Value as JsonValue};
use crate::core::field::optionality::Optionality;
use crate::core::object::Object;
use crate::core::relation::delete_rule::DeleteRule;

pub mod builder;
pub mod delete_rule;
pub mod disconnect_rule;

#[derive(Debug, Clone)]
pub(crate) struct Relation {
    pub(self) name: String,
    pub(self) localized_name: String,
    pub(self) description: String,
    pub(self) optionality: Optionality,
    pub(self) model: String,
    pub(self) through: Option<String>,
    pub(self) is_vec: bool,
    pub(self) fields: Vec<String>,
    pub(self) references: Vec<String>,
    pub(self) delete_rule: DeleteRule,
}

impl Relation {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn localized_name(&self) -> &str {
        &self.localized_name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn optionality(&self) -> &Optionality { &self.optionality }

    pub(crate) fn is_optional(&self) -> bool {
        self.optionality.is_optional()
    }

    pub(crate) fn is_required(&self) -> bool {
        self.optionality.is_required()
    }

    pub(crate) fn model(&self) -> &str {
        &self.model
    }

    pub(crate) fn through(&self) -> Option<&str> {
        self.through.as_deref()
    }

    pub(crate) fn is_vec(&self) -> bool {
        self.is_vec
    }

    pub(crate) fn fields(&self) -> &Vec<String> {
        &self.fields
    }

    pub(crate) fn references(&self) -> &Vec<String> {
        &self.references
    }

    pub(crate) fn delete_rule(&self) -> DeleteRule {
        self.delete_rule
    }
}

#[derive(Debug)]
pub(crate) enum RelationManipulation {
    Create(JsonValue),
    ConnectOrCreate(JsonValue),
    Connect(JsonValue),
    Set(JsonValue),
    Update(JsonValue),
    Upsert(JsonValue),
    Disconnect(JsonValue),
    Delete(JsonValue),
}

#[derive(Debug)]
pub(crate) enum RelationConnection {
    Link(Object),
    Unlink(Object),
    UnlinkAndDelete(Object),
}
