use crate::app::ctx::AppCtx;
use crate::connectors::mongodb::connector::connector::MongoDBConnector;
use crate::connectors::sql::connector::connector::SQLConnector;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::connector::connector::Connector;
use crate::core::database::name::DatabaseName;
use crate::core::result::Result;

pub(super) async fn connect_to_database() -> Result<()> {
    let app_ctx = AppCtx::get()?;
    let connector_conf = app_ctx.connector_conf()?;
    let connector: Box<dyn Connector> = match connector_conf.provider {
        DatabaseName::MySQL => {
            #[cfg(feature = "data-source-mysql")]
            Box::new(SQLConnector::new(SQLDialect::MySQL, connector_conf.url, false).await)
        },
        DatabaseName::PostgreSQL => {
            #[cfg(feature = "data-source-postgres")]
            Box::new(SQLConnector::new(SQLDialect::PostgreSQL, connector_conf.url, false).await)
        },
        #[cfg(feature = "data-source-sqlite")]
        DatabaseName::SQLite => {
            #[cfg(feature = "data-source-sqlite")]
            Box::new(SQLConnector::new(SQLDialect::SQLite, connector_conf.url, false).await)
        },
        DatabaseName::MongoDB => {
            #[cfg(feature = "data-source-mongodb")]
            Box::new(MongoDBConnector::new(connector_conf.url.to_string()).await)
        },
    };
    app_ctx.set_connector(connector);
    Ok(())
}