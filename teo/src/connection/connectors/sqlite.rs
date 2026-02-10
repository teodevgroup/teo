use crate::{connection::Connection, types::Schema};

impl Connection for rusqlite::Connection {

    type Err = rusqlite::Error;

    async fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        todo!()
    }
}
