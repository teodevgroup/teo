use std::sync::Arc;
use crate::app::app_ctx::AppCtx;
use crate::core::connector::connection::Connection;
use crate::core::model::model::Model;
use crate::prelude::{Object, Req, Value};
use crate::core::result::Result;

#[derive(Clone)]
pub struct ModelCtx {
    pub(super) conn: Arc<dyn Connection>,
    pub(super) model: &'static Model,
    pub(super) req: Option<Req>,
}

impl ModelCtx {

    fn req(&self) -> Option<Req> {
        self.req.clone()
    }

    pub async fn find_unique<T: From<Object>>(&self, finder: &Value) -> Result<Option<T>> {
        AppCtx::get()?.graph()?.find_unique(self.model.name(), finder, Some(self.conn.clone()), self.req()).await
    }

    pub async fn find_first<T: From<Object>>(&self, finder: &Value) -> Result<Option<T>> {
        AppCtx::get()?.graph()?.find_first(self.model.name(), finder, Some(self.conn.clone()), self.req()).await
    }

    pub async fn find_many<T: From<Object>>(&self, finder: &Value) -> Result<Vec<T>> {
        AppCtx::get()?.graph()?.find_many(self.model.name(), finder, Some(self.conn.clone()), self.req()).await
    }

    pub async fn create_object<T: From<Object>>(&self, values: &Value) -> Result<T> {
        AppCtx::get()?.graph()?.create_object(self.model.name(), values, Some(self.conn.clone()), self.req()).await.map(|v| v.into())
    }

    pub async fn count(&self, finder: &Value) -> Result<usize> {
        AppCtx::get()?.graph()?.count(self.model.name(), finder, Some(self.conn.clone())).await
    }

    pub async fn aggregate<T: From<Value>>(&self, finder: &Value) -> Result<T> {
        AppCtx::get()?.graph()?.aggregate(self.model.name(), finder, Some(self.conn.clone())).await.map(|v| v.into())
    }

    pub async fn group_by<T: From<Value>>(&self, finder: &Value) -> Result<Vec<T>> {
        let vec_value = AppCtx::get()?.graph()?.group_by(self.model.name(), finder, Some(self.conn.clone())).await?;
        let vec = vec_value.into_vec().unwrap();
        Ok(vec.into_iter().map(|v| v.into()).collect())
    }
}