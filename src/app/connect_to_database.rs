use std::sync::Arc;
use crate::app::ctx::AppCtx;
use crate::connectors::mongodb::connector::MongoDBConnector;
use crate::connectors::sql::connector::SQLConnector;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::connector::Connector;
use crate::core::database::name::DatabaseName;

pub(super) async fn connect_to_database() -> crate::app::new_app::new_result::Result<()> {
    let app_ctx = AppCtx::get_mut()?;
    let connector_conf = app_ctx.connector_conf()?;
    let connector: Box<dyn Connector> = match connector_conf.provider {
        DatabaseName::MySQL => {
            #[cfg(feature = "data-source-mysql")]
            Box::new(SQLConnector::new(SQLDialect::MySQL, url, false).await)
        },
        DatabaseName::PostgreSQL => {
            #[cfg(feature = "data-source-postgres")]
            Box::new(SQLConnector::new(SQLDialect::PostgreSQL, url, false).await)
        },
        #[cfg(feature = "data-source-sqlite")]
        DatabaseName::SQLite => {
            #[cfg(feature = "data-source-sqlite")]
            Box::new(SQLConnector::new(SQLDialect::SQLite, url, false).await)
        },
        DatabaseName::MongoDB => {
            #[cfg(feature = "data-source-mongodb")]
            Box::new(MongoDBConnector::new(url.clone()).await)
        },
    };
    app_ctx.set_connector(connector);
    Ok(())
}