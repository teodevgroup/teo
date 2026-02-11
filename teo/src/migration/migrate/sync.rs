use crate::{connection::SyncConnection, types::Schema};

pub fn migrate_sync<C, S>(connection: &C) -> Result<(), C::Err> where C: SyncConnection, S: Schema {
    connection.migrate::<S>()
}
