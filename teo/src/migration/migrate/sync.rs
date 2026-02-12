use crate::{connection::SyncConnection, types::Schema};

pub fn migrate<C, S>(connection: &mut C) -> Result<(), C::Err> where C: SyncConnection, S: Schema {
    connection.migrate::<S>()
}
