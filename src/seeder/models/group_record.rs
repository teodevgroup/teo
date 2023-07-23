use std::fmt::{Debug, Display, Formatter};
use std::borrow::Borrow;
use std::sync::Arc;
use crate::app::app_ctx::AppCtx;
use crate::core::connector::connection::Connection;
use crate::prelude::{Object, Value, Result};

/// Group record
#[derive(Clone, PartialEq)]
pub struct GroupRecord {
    pub(super) inner: Object
}

impl GroupRecord {

    /// Find many group records.
    pub async fn find_many(query: impl Borrow<Value>) -> Result<Vec<GroupRecord>> {
        AppCtx::get()?.graph()?.find_many("__TeoGroupRecord", query.borrow(), None).await
    }

    /// Find a unique group record.
    pub async fn find_unique(query: impl Borrow<Value>) -> Result<Option<GroupRecord>> {
        AppCtx::get()?.graph()?.find_unique("__TeoGroupRecord", query.borrow(), None).await
    }

    /// Find a non unique group record.
    pub async fn find_first(query: impl Borrow<Value>) -> Result<Option<GroupRecord>> {
        AppCtx::get()?.graph()?.find_first("__TeoGroupRecord", query.borrow(), None).await
    }

    /// Create a new group record.
    pub async fn new(values: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Self {
        Self {
            inner: AppCtx::get().unwrap().graph().unwrap().create_object("__TeoGroupRecord", values, Some(connection)).await.unwrap(),
        }
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
        self.inner.get("dataset").unwrap()
    }

    pub fn set_dataset(&self, new_value: impl Into<String>) {
        self.inner.set("dataset", new_value.into()).unwrap();
    }

    /// Group
    pub fn group(&self) -> String {
        self.inner.get("group").unwrap()
    }

    pub fn set_group(&self, new_value: impl Into<String>) {
        self.inner.set("group", new_value.into()).unwrap();
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

impl Into<Object> for GroupRecord {
    fn into(self) -> Object {
        self.inner.clone()
    }
}

impl From<Object> for GroupRecord {
    fn from(value: Object) -> Self {
        Self { inner: value }
    }
}

impl Into<Value> for GroupRecord {
    fn into(self) -> Value {
        Value::Object(self.into())
    }
}

impl From<Value> for GroupRecord {
    fn from(value: Value) -> Self {
        Self::from(value.as_object().unwrap().clone())
    }
}

impl Debug for GroupRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for GroupRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

