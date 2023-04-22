use std::sync::{Arc};
use tokio::sync::Mutex;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use quaint_forked::{pooled::Quaint};
use crate::connectors::sql::connector::connection::SQLConnection;
use crate::connectors::sql::migration::migrate::SQLMigration;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::connectors::sql::url::url_utils;
use crate::core::connector::connection::Connection;
use crate::core::connector::connector::Connector;
use crate::core::error::{Error, RuntimeError};
use crate::core::result::Result;

pub(crate) struct SQLConnector {
    dialect: SQLDialect,
    pool: Quaint,
    memory_mode: bool,
}

impl SQLConnector {
    pub(crate) async fn new(dialect: SQLDialect, url: &str, reset: bool) -> Self {
        SQLMigration::create_database_if_needed(dialect, url, reset).await;
        let url = url_utils::normalized_url(dialect, url);
        let pool = Quaint::builder(url.as_str()).unwrap().build();
        Self { dialect, pool, memory_mode: url.to_string().contains(":memory:") }
    }
}

static UNIQUE_CONNECTION: Lazy<Mutex<Option<Arc<dyn Connection>>>> = Lazy::new(|| {
    Mutex::new(None)
});

#[async_trait]
impl Connector for SQLConnector {
    async fn connection(&self) -> Result<Arc<dyn Connection>> {
        if self.memory_mode {
            let mut connection = UNIQUE_CONNECTION.lock().await;
            if connection.is_none() {
                let result = self.normal_checkout_connection().await.unwrap();
                *connection = Some(result.clone());
                Ok(result)
            } else {
                Ok(connection.clone().unwrap())
            }
        } else {
            self.normal_checkout_connection().await
        }
    }
}

impl SQLConnector {
    async fn normal_checkout_connection(&self) -> Result<Arc<dyn Connection>> {
        let pooled_connection = self.pool.check_out().await;
        if pooled_connection.is_err() {
            Err(Error::RuntimeError(RuntimeError::CannotCreatePooledConnection(pooled_connection.err().unwrap().to_string())))
        } else {
            Ok(Arc::new(SQLConnection {
                dialect: self.dialect,
                conn: Arc::new(pooled_connection.unwrap()),
                tran: None,
            }))
        }
    }
}