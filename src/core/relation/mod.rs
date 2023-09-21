pub mod update_rule;
pub mod delete_rule;
pub mod disconnect_rule;

use std::collections::{HashSet};
use std::sync::Arc;
use inflector::Inflector;
use maplit::hashset;
use once_cell::sync::Lazy;
use crate::app::app_ctx::AppCtx;
use crate::core::field::field::Field;
use crate::core::field::optionality::Optionality;
use crate::core::model::model::Model;
use crate::core::relation::delete_rule::DeleteRule;

#[derive(Debug, Clone)]
pub struct Relation {
    pub(self) name: &'static str,
    pub(crate) localized_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(self) optionality: Optionality,
    pub(self) model: Vec<String>,
    pub(self) through: Option<Vec<String>>,
    pub(self) is_vec: bool,
    pub(self) fields: Vec<String>,
    pub(self) references: Vec<String>,
    pub(self) delete_rule: DeleteRule,
    pub(self) has_foreign_key: bool,
}

impl Relation {

    pub(crate) fn new(name: &'static str) -> Self {
        return Self {
            name,
            localized_name: None,
            description: None,
            optionality: Optionality::Required,
            model: vec![],
            through: None,
            is_vec: false,
            fields: Vec::new(),
            references: Vec::new(),
            delete_rule: DeleteRule::Default,
            has_foreign_key: false,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn localized_name(&self) -> String {
        if let Some(ln) = &self.localized_name {
            ln.clone()
        } else {
            self.name.to_title_case()
        }
    }

    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|d| d.as_str())
    }

    pub(crate) fn description_with_default(&self) -> &str {
        self.description().unwrap_or("This relation doesn't have a description.")
    }

    pub(crate) fn optionality(&self) -> &Optionality { &self.optionality }

    pub(crate) fn is_optional(&self) -> bool {
        self.optionality.is_optional()
    }

    pub(crate) fn is_required(&self) -> bool {
        self.optionality.is_required()
    }

    pub(crate) fn model(&self) -> &Model {
        AppCtx::get().unwrap().model(self.model_path()).unwrap().unwrap()
    }

    pub(crate) fn model_path(&self) -> Vec<&str> {
        self.model.iter().map(|s| s.as_str()).collect()
    }

    pub(crate) fn set_through(&mut self, through: Vec<String>) {
        self.through = Some(through);
    }

    pub(crate) fn through(&self) -> Option<&Model> {
        self.through_path().map(|p| AppCtx::get().unwrap().model(p).unwrap().unwrap())
    }

    pub(crate) fn through_path(&self) -> Option<Vec<&str>> {
        self.through.map(|t| t.iter().map(|s| s.as_str()).collect())
    }

    pub fn is_vec(&self) -> bool {
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

    pub(crate) fn set_model(&mut self, model: Vec<String>) {
        self.model = model;
    }

    pub(crate) fn finalize(&mut self, fields: &Vec<Arc<Field>>) {
        self.has_foreign_key = if self.through.is_some() {
            false
        } else {
            self.fields.iter().find(|name| fields.iter().find(|f| f.name() == name.as_str() && f.foreign_key).is_some()).is_some()
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
