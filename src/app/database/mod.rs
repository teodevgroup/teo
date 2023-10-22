use std::sync::Arc;
use teo_result::{Result};
use teo_runtime::config::connector::Connector;
use teo_runtime::connection::connection::Connection;
use teo_runtime::namespace::Namespace;

pub fn connect_databases(namespace: &mut Namespace) -> Result<()> {
    may_connect_database(namespace)?;
    for namespace in namespace.namespaces.values_mut() {
        may_connect_database(namespace)?;
    }
    Ok(())
}

pub fn may_connect_database(namespace: &mut Namespace) -> Result<()> {
    if namespace.connector.is_none() { return Ok(()) }
    let connector = namespace.connector.as_ref().unwrap();
    let connection = connection_for_connector(connector);
    namespace.connection = Some(connection);
    Ok(())
}

fn connection_for_connector(connector: &Connector) -> Arc<dyn Connection> {
    unreachable!()
}