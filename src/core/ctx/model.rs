use std::sync::Arc;
use crate::app::ctx::AppCtx;
use crate::core::connector::connection::Connection;
use crate::core::model::model::Model;
use crate::prelude::{Object, Value};
use crate::core::result::Result;

pub struct ModelCtx {
    pub(super) conn: Arc<dyn Connection>,
    pub(super) model: &'static Model,
}

impl ModelCtx {

    pub async fn find_unique<T: From<Object>>(&self, finder: &Value) -> Result<Option<T>> {
        AppCtx::get()?.graph()?.find_unique(self.model.name(), finder, Some(self.conn.clone())).await
    }

    pub async fn find_first<T: From<Object>>(&self, finder: &Value) -> Result<Option<T>> {
        AppCtx::get()?.graph()?.find_first(self.model.name(), finder, Some(self.conn.clone())).await
    }

    pub async fn find_many<T: From<Object>>(&self, finder: &Value) -> Result<Vec<T>> {
        AppCtx::get()?.graph()?.find_many(self.model.name(), finder, Some(self.conn.clone())).await
    }
}