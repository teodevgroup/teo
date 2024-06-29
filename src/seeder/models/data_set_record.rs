use std::fmt::{Debug, Display, Formatter};
use std::borrow::Borrow;
use key_path::path;
use teo_runtime::connection::transaction;
use teo_runtime::model;
use crate::prelude::{Value, Result};

/// Group record
pub struct DataSetRecord {
    pub(super) inner: model::Object,
}

impl PartialEq for DataSetRecord {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl DataSetRecord {

    /// Find many group records.
    pub async fn find_many(query: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Vec<DataSetRecord>> {
        let model = ctx.namespace().model_at_path(&vec!["std".to_owned(), "DataSetRecord".to_owned()]).unwrap();
        Ok(ctx.find_many(model, query.borrow(), None, path![]).await?)
    }

    /// Find a unique group record.
    pub async fn find_unique(query: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Option<DataSetRecord>> {
        let model = ctx.namespace().model_at_path(&vec!["std".to_owned(), "DataSetRecord".to_owned()]).unwrap();
        Ok(ctx.find_unique(model, query.borrow(), None, path![]).await?)
    }

    /// Find a non unique group record.
    pub async fn find_first(query: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Option<DataSetRecord>> {
        let model = ctx.namespace().model_at_path(&vec!["std".to_owned(), "DataSetRecord".to_owned()]).unwrap();
        Ok(ctx.find_first(model, query.borrow(), None, path![]).await?)
    }

    /// Create a new group record.
    pub async fn new(values: impl Borrow<Value>, ctx: transaction::Ctx) -> Result<Self> {
        let model = ctx.namespace().model_at_path(&vec!["std".to_owned(), "DataSetRecord".to_owned()]).unwrap();
        Ok(ctx.create_object(model, values.borrow(), None).await?.into())
    }

    /// Whether this group record is new.
    pub fn is_new(&self) -> bool {
        self.inner.is_new()
    }

    /// Whether this group record is modified.
    pub fn is_modified(&self) -> bool {
        self.inner.is_modified()
    }

    /// Set new values to a group record. Validations and transformations are
    /// triggered.
    pub async fn set(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.set_teon(values.as_ref()).await
    }

    /// Update new values to a group record. Validations and transformations are
    /// not triggered.
    pub async fn update(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.update_teon(values.as_ref()).await
    }

    /// Save this group record.
    pub async fn save(&self) -> Result<()> {
        self.inner.save().await
    }

    /// Delete this group record.
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
        self.inner.get("dataSet").unwrap()
    }

    pub fn set_dataset(&self, new_value: impl Into<String>) {
        self.inner.set("dataSet", new_value.into()).unwrap();
    }

    /// Group
    pub fn group(&self) -> Vec<String> {
        let group_string: String = self.inner.get("group").unwrap();
        group_string.split(".").map(|s| s.to_string()).collect()
    }

    pub fn set_group(&self, new_value: Vec<String>) {
        let new_value_string = new_value.join(".");
        self.inner.set("group", new_value_string).unwrap();
    }

    /// Name
    pub fn name(&self) -> String {
        self.inner.get("name").unwrap()
    }

    pub fn set_name(&self, new_value: impl Into<String>) {
        self.inner.set("name", new_value.into()).unwrap();
    }

    /// Record
    pub fn record(&self) -> String {
        self.inner.get("record").unwrap()
    }

    pub fn set_record(&self, new_value: impl Into<String>) {
        self.inner.set("record", new_value.into()).unwrap();
    }
}

impl Into<model::Object> for DataSetRecord {
    fn into(self) -> model::Object {
        self.inner.clone()
    }
}

impl From<model::Object> for DataSetRecord {
    fn from(value: model::Object) -> Self {
        Self { inner: value }
    }
}

impl Debug for DataSetRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for DataSetRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
