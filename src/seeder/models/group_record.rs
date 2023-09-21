use std::fmt::{Debug, Display, Formatter};
use std::borrow::Borrow;
use std::sync::{Arc};
use crate::app::app_ctx::AppCtx;
use crate::core::cell::SyncUnsafeCell;
use crate::core::connector::connection::Connection;
use crate::prelude::{Object, Value, Result};

/// Group record
pub struct GroupRecord {
    pub(super) inner: Object,
    model_path: SyncUnsafeCell<Vec<String>>,
}

impl PartialEq for GroupRecord {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl GroupRecord {

    /// Find many group records.
    pub async fn find_many(query: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Result<Vec<GroupRecord>> {
        let model = AppCtx::get()?.model(vec!["__TeoGroupRecord"])?.unwrap();
        AppCtx::get()?.graph().find_many(model, query.borrow(), connection, None).await
    }

    /// Find a unique group record.
    pub async fn find_unique(query: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Result<Option<GroupRecord>> {
        let model = AppCtx::get()?.model(vec!["__TeoGroupRecord"])?.unwrap();
        AppCtx::get()?.graph().find_unique(model, query.borrow(), connection, None).await
    }

    /// Find a non unique group record.
    pub async fn find_first(query: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Result<Option<GroupRecord>> {
        let model = AppCtx::get()?.model(vec!["__TeoGroupRecord"])?.unwrap();
        AppCtx::get()?.graph().find_first(model, query.borrow(), connection, None).await
    }

    /// Create a new group record.
    pub async fn new(values: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Self {
        let model = AppCtx::get().unwrap().model(vec!["__TeoGroupRecord"]).unwrap().unwrap();
        Self {
            inner: AppCtx::get().unwrap().graph().create_object(model, values, connection, None).await.unwrap(),
            model_path: SyncUnsafeCell::new(vec![]),
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
    pub fn group(&self) -> Vec<String> {
        let group_string: String = self.inner.get("group").unwrap();
        group_string.split(".").map(|s| s.to_string()).collect()
    }

    pub fn set_group(&self, new_value: Vec<String>) {
        let new_value_string = new_value.join(".");
        self.inner.set("group", new_value_string).unwrap();
    }

    pub fn model_path(&self) -> Vec<&str> {
        let mut model_path = unsafe { &mut *self.model_path.get() };
        model_path.clear();
        model_path.extend(self.group());
        model_path.iter().map(|s| s.as_str()).collect()
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
        Self { inner: value, model_path: SyncUnsafeCell::new(vec![]) }
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
