use std::sync::Arc;
use array_tool::vec::Join;
use teo_result::{Result};
use teo_runtime::config::connector::Connector;
use teo_runtime::connection::connection::Connection;
use teo_runtime::database::database::Database;
use teo_runtime::namespace::Namespace;
use teo_sql_connector::connector::SQLConnection;
use teo_sql_connector::schema::dialect::SQLDialect;
use teo_mongodb_connector::connector::MongoDBConnection;
use teo_runtime::connection::Ctx as ConnCtx;
use crate::app::App;
use crate::message::info_message;

pub async fn connect_databases(app: &App, namespace: &Namespace, silent: bool) -> Result<()> {
    may_connect_database(namespace, silent).await?;
    for namespace in namespace.namespaces().values() {
        may_connect_database(namespace, silent).await?;
    }
    let ctx = ConnCtx::from_namespace(app.compiled_main_namespace());
    app.replace_conn_ctx(ctx);
    Ok(())
}

pub async fn may_connect_database(namespace: &Namespace, silent: bool) -> Result<()> {
    if namespace.connector().is_none() { return Ok(()) }
    let connector = namespace.connector().unwrap();
    let connection = connection_for_connector(connector, silent).await;
    if !silent {
        info_message(format!("{} connector connected for `{}` at \"{}\"", connector.provider().lowercase_desc(), if namespace.path().is_empty() { "main".to_string() } else { namespace.path().join(".") }, connector.url()));
    }
    namespace.set_connection(Some(connection));
    Ok(())
}

async fn connection_for_connector(connector: &Connector, silent: bool) -> Arc<dyn Connection> {
    if connector.provider().is_mongo() {
        Arc::new(MongoDBConnection::new(connector.url(), |content| {
            if !silent {
                info_message(content);
            }
        }).await)
    } else {
        Arc::new(SQLConnection::new(
            match connector.provider() {
                Database::MongoDB => unreachable!(),
                Database::MySQL => SQLDialect::MySQL,
                Database::PostgreSQL => SQLDialect::PostgreSQL,
                Database::SQLite => SQLDialect::SQLite,
            },
            connector.url(),
            false,
        ).await)
    }
}