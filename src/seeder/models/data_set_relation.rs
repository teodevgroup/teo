use std::fmt::{Debug, Display, Formatter};
use std::borrow::Borrow;
use key_path::path;
use teo_runtime::connection::transaction;
use teo_runtime::model;
use crate::prelude::{Value, Result};

/// Group relation
#[derive(Clone, PartialEq)]
pub struct DataSetRelation {
    pub(super) inner: model::Object,
}

impl DataSetRelation {

    /// Find many group records.
    pub async fn find_many(query: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Vec<DataSetRelation>> {
        let model = ctx.namespace().model_at_path(&vec!["std", "DataSetRelation"]).unwrap();
        Ok(ctx.find_many(model, query.borrow(), None, path![]).await?)
    }

    /// Find a unique group record.
    pub async fn find_unique(query: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Option<DataSetRelation>> {
        let model = ctx.namespace().model_at_path(&vec!["std", "DataSetRelation"]).unwrap();
        Ok(ctx.find_unique(model, query.borrow(), None, path![]).await?)
    }

    /// Find a non unique group record.
    pub async fn find_first(query: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Option<DataSetRelation>> {
        let model = ctx.namespace().model_at_path(&vec!["std", "DataSetRelation"]).unwrap();
        Ok(ctx.find_first(model, query.borrow(), None, path![]).await?)
    }

    /// Create a new group relation.
    pub async fn new(values: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Self> {
        let model = ctx.namespace().model_at_path(&vec!["std", "DataSetRelation"]).unwrap();
        Ok(ctx.create_object(model, values.borrow(), None).await?.into())
    }

    /// Whether this group relation is new.
    pub fn is_new(&self) -> bool {
        self.inner.is_new()
    }

    /// Whether this group relation is modified.
    pub fn is_modified(&self) -> bool {
        self.inner.is_modified()
    }

    /// Set new values to a group relation. Validations and transformations are
    /// triggered.
    pub async fn set(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.set_teon(values.as_ref()).await
    }

    /// Update new values to a group relation. Validations and transformations are
    /// not triggered.
    pub async fn update(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.update_teon(values.as_ref()).await
    }

    /// Save this group relation.
    pub async fn save(&self) -> Result<()> {
        self.inner.save().await
    }

    /// Delete this group relation.
    pub async fn delete(&self) -> Result<()> {
        self.inner.delete().await
    }

    /// Id
    pub fn id(&self) -> String {
        self.inner.get("id").unwrap()
    }

    pub fn set_id(&self, new_value: impl Into<String>) {
        self.inner.set("id", new_value.into()).unwrap();
    }

    /// Dataset
    pub fn dataset(&self) -> String {
        self.inner.get("dataset").unwrap()
    }

    pub fn set_dataset(&self, new_value: impl Into<String>) {
        self.inner.set("dataset", new_value.into()).unwrap();
    }

    /// Group a
    pub fn group_a(&self) -> String {
        self.inner.get("groupA").unwrap()
    }

    pub fn set_group_a(&self, new_value: impl Into<String>) {
        self.inner.set("groupA", new_value.into()).unwrap();
    }

    /// Relation a
    pub fn relation_a(&self) -> String {
        self.inner.get("relationA").unwrap()
    }

    pub fn set_relation_a(&self, new_value: impl Into<String>) {
        self.inner.set("relationA", new_value.into()).unwrap();
    }

    /// Name a
    pub fn name_a(&self) -> String {
        self.inner.get("nameA").unwrap()
    }

    pub fn set_name_a(&self, new_value: impl Into<String>) {
        self.inner.set("nameA", new_value.into()).unwrap();
    }

    /// Group b
    pub fn group_b(&self) -> String {
        self.inner.get("groupB").unwrap()
    }

    pub fn set_group_b(&self, new_value: impl Into<String>) {
        self.inner.set("groupB", new_value.into()).unwrap();
    }

    /// Relation b
    pub fn relation_b(&self) -> String {
        self.inner.get("relationB").unwrap()
    }

    pub fn set_relation_b(&self, new_value: impl Into<String>) {
        self.inner.set("relationB", new_value.into()).unwrap();
    }

    /// Name b
    pub fn name_b(&self) -> String {
        self.inner.get("nameB").unwrap()
    }

    pub fn set_name_b(&self, new_value: impl Into<String>) {
        self.inner.set("nameB", new_value.into()).unwrap();
    }
}

impl Into<model::Object> for DataSetRelation {
    fn into(self) -> model::Object {
        self.inner.clone()
    }
}

impl From<model::Object> for DataSetRelation {
    fn from(value: model::Object) -> Self {
        Self { inner: value }
    }
}

impl Debug for DataSetRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for DataSetRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

