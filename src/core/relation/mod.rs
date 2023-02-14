use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use maplit::hashset;
use once_cell::sync::Lazy;
use crate::core::field::Field;
use crate::core::field::optionality::Optionality;
use crate::core::relation::delete_rule::DeleteRule;

pub mod update_rule;
pub mod delete_rule;
pub mod disconnect_rule;

#[derive(Debug, Clone)]
pub(crate) struct Relation {
    pub(self) name: String,
    pub(self) localized_name: Option<String>,
    pub(self) description: Option<String>,
    pub(self) optionality: Optionality,
    pub(self) model: String,
    pub(self) through: Option<String>,
    pub(self) is_vec: bool,
    pub(self) fields: Vec<String>,
    pub(self) references: Vec<String>,
    pub(self) delete_rule: DeleteRule,
    pub(self) has_foreign_key: bool,
}

impl Relation {

    pub(crate) fn new(name: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            localized_name: None,
            description: None,
            optionality: Optionality::Required,
            model: "".into(),
            through: None,
            is_vec: false,
            fields: Vec::new(),
            references: Vec::new(),
            delete_rule: DeleteRule::Default,
            has_foreign_key: false,
        }
    }
    
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn localized_name(&self) -> &str {
        self.localized_name.as_ref().unwrap()
    }

    pub(crate) fn description(&self) -> &str {
        self.description.as_ref().unwrap()
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

    pub(crate) fn set_through(&mut self, through: String) {
        self.through = Some(through);
    }

    pub(crate) fn through(&self) -> Option<&str> {
        self.through.as_deref()
    }

    pub(crate) fn is_vec(&self) -> bool {
        self.is_vec
    }

    pub(crate) fn set_fields(&mut self, fields: Vec<String>) {
        self.fields = fields;
    }

    pub(crate) fn fields(&self) -> &Vec<String> {
        &self.fields
    }

    pub(crate) fn set_references(&mut self, references: Vec<String>) {
        self.references = references;
    }

    pub(crate) fn references(&self) -> &Vec<String> {
        &self.references
    }

    pub(crate) fn set_local(&mut self, local: String) {
        self.fields = vec![local];
    }

    pub(crate) fn local(&self) -> &str {
        self.fields.get(0).unwrap()
    }

    pub(crate) fn set_foreign(&mut self, foreign: String) {
        self.references = vec![foreign];
    }

    pub(crate) fn foreign(&self) -> &str {
        self.references.get(0).unwrap()
    }

    pub(crate) fn delete_rule(&self) -> DeleteRule {
        self.delete_rule
    }

    pub(crate) fn has_foreign_key(&self) -> bool {
        self.has_foreign_key
    }

    pub(crate) fn has_join_table(&self) -> bool {
        self.through().is_some()
    }

    pub(crate) fn iter(&self) -> RelationIter {
        RelationIter { index: 0, relation: self }
    }

    pub(crate) fn len(&self) -> usize {
        self.fields().len()
    }

    pub(crate) fn filters(&self) -> &HashSet<&str> {
        if self.is_vec {
            &VEC_FILTERS
        } else {
            &OBJECT_FILTERS
        }
    }

    pub(crate) fn set_required(&mut self) {
        self.optionality = Optionality::Required;
    }

    pub(crate) fn set_optional(&mut self) {
        self.optionality = Optionality::Optional;
    }

    pub(crate) fn set_is_vec(&mut self, is_vec: bool) {
        self.is_vec = is_vec
    }

    pub(crate) fn set_model(&mut self, model: String) {
        self.model = model;
    }

    pub(crate) fn finalize(&mut self, fields: &HashMap<String, Arc<Field>>) {
        self.has_foreign_key = if self.through.is_some() {
            false
        } else {
            self.fields.iter().find(|name| fields.get(name.as_str()).unwrap().foreign_key).is_some()
        }
    }
}

pub(crate) struct RelationIter<'a> {
    index: usize,
    relation: &'a Relation,
}

impl<'a> Iterator for RelationIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(f) = self.relation.fields().get(self.index) {
            let result = Some((f.as_str(), self.relation.references().get(self.index).unwrap().as_str()));
            self.index += 1;
            result
        } else {
            None
        }
    }
}

static VEC_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"some", "none", "every"}
});
static OBJECT_FILTERS: Lazy<HashSet<&str>> = Lazy::new(|| {
    hashset!{"is", "isNot"}
});
