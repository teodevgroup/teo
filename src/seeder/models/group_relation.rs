use std::fmt::{Debug, Display, Formatter};
use std::borrow::Borrow;
use std::sync::Arc;
use crate::app::app_ctx::AppCtx;
use crate::core::connector::connection::Connection;
use crate::prelude::{Object, Value, Result};

/// Group relation
#[derive(Clone, PartialEq)]
pub struct GroupRelation {
    pub(super) inner: Object
}

impl GroupRelation {

    /// Find many group relations.
    pub async fn find_many(query: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Result<Vec<GroupRelation>> {
        AppCtx::get()?.graph()?.find_many("__TeoGroupRelation", query.borrow(), connection, None).await
    }

    /// Find a unique group relation.
    pub async fn find_unique(query: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Result<Option<GroupRelation>> {
        AppCtx::get()?.graph()?.find_unique("__TeoGroupRelation", query.borrow(), connection, None).await
    }

    /// Find a non unique group relation.
    pub async fn find_first(query: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Result<Option<GroupRelation>> {
        AppCtx::get()?.graph()?.find_first("__TeoGroupRelation", query.borrow(), connection, None).await
    }

    /// Create a new group relation.
    pub async fn new(values: impl Borrow<Value>, connection: Arc<dyn Connection>) -> Self {
        Self {
            inner: AppCtx::get().unwrap().graph().unwrap().create_object("__TeoGroupRelation", values, connection, None).await.unwrap(),
        }
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

impl Into<Object> for GroupRelation {
    fn into(self) -> Object {
        self.inner.clone()
    }
}

impl From<Object> for GroupRelation {
    fn from(value: Object) -> Self {
        Self { inner: value }
    }
}

impl Into<Value> for GroupRelation {
    fn into(self) -> Value {
        Value::Object(self.into())
    }
}

impl From<Value> for GroupRelation {
    fn from(value: Value) -> Self {
        Self::from(value.as_object().unwrap().clone())
    }
}

impl Debug for GroupRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for GroupRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

