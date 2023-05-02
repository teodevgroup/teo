use std::sync::Arc;
use key_path::KeyPath;
use crate::core::action::Action;
use crate::core::connector::connection::Connection;
use crate::core::ctx::user::UserCtx;
use crate::core::object::Object;
use crate::core::result::Result;
use crate::core::teon::Value;
use crate::prelude::Error;

#[derive(Clone)]
pub struct PipelineCtx<'a> {
    pub(crate) value: Value,
    pub(crate) object: Option<Object>,
    pub(crate) path: KeyPath<'a>,
    pub(crate) action: Action,
    pub(crate) conn: Arc<dyn Connection>,
}

impl<'a> PipelineCtx<'a> {

    pub(crate) fn user_ctx(&self) -> UserCtx {
        UserCtx::new(self.conn.clone())
    }

    pub(crate) fn initial_state_with_value(value: Value, conn: Arc<dyn Connection>) -> Self {
        Self {
            value,
            object: None,
            path: KeyPath::default(),
            action: Action::empty(),
            conn,
        }
    }

    pub(crate) fn initial_state_with_object(object: Object, conn: Arc<dyn Connection>) -> Self {
        Self {
            value: Value::Null,
            object: Some(object),
            path: KeyPath::default(),
            action: Action::empty(),
            conn,
        }
    }

    pub(crate) fn with_path(&self, path: impl AsRef<KeyPath<'a>>) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            path: path.as_ref().clone(),
            action: self.action.clone(),
            conn: self.conn.clone(),
        }
    }

    pub(crate) fn with_value(&self, value: Value) -> Self {
        Self {
            value,
            object: self.object.clone(),
            path: self.path.clone(),
            action: self.action.clone(),
            conn: self.conn.clone(),
        }
    }

    pub(crate) fn with_value_result(&self, result: Result<Value>) -> Result<Self> {
        match result {
            Ok(value) => Ok(Self {
                value,
                object: self.object.clone(),
                path: self.path.clone(),
                action: self.action.clone(),
                conn: self.conn.clone()
            }),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn with_invalid(&self, reason: impl Into<String>) -> Error {
        Error::validation_error(&self.path, reason.into())
    }

    pub(crate) fn with_action(&self, action: Action) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            path: self.path.clone(),
            action,
            conn: self.conn.clone(),
        }
    }

    pub(crate) fn get_value(&self) -> Value {
        self.value.clone()
    }

    pub(crate) fn get_object(&self) -> Result<Object> {
        match &self.object {
            Some(object) => Ok(object.clone()),
            None => Err(Error::internal_server_error_with_path(&self.path, "ctx object is null"))
        }
    }

    pub(crate) fn internal_server_error(&self, reason: impl Into<String>) -> Error {
        Error::internal_server_error_with_path(&self.path, reason.into())
    }

    pub(crate) fn unwrap_custom_error(&self, error: Error) -> Error {
        if error.is_custom_validation_error() {
            Error::validation_error(&self.path, error.message())
        } else if error.is_custom_internal_server_error() {
            Error::internal_server_error_with_path(&self.path, error.message())
        } else {
            error
        }
    }

    pub(crate) fn redirect(&self, action: Action) -> Self {
        Self {
            value: self.value.clone(),
            object: self.object.clone(),
            path: self.path.clone(),
            action: self.action.redirect(action),
            conn: self.conn.clone(),
        }
    }
}
