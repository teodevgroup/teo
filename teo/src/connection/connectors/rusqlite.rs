use crate::{connection::SyncConnection, types::Schema};

impl SyncConnection for rusqlite::Connection {

    type Err = rusqlite::Error;

    fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        todo!()
    }
}
