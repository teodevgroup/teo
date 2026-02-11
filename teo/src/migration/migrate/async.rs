use crate::{connection::AsyncConnection, types::Schema};

pub async fn migrate_async<C, S>(connection: &C) -> Result<(), C::Err> where C: AsyncConnection, S: Schema {
    connection.migrate::<S>().await
}
