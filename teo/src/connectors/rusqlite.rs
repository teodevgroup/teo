use rusqlite::{Connection, Error};
use crate::{connection::SyncConnection, migration::SyncMigration, types::Schema};

impl SyncConnection for Connection {

    type Err = Error;

    fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        todo!()
    }
}

impl SyncMigration for Connection {

    type Err = Error;

    fn execute_without_params(&self, q: &str) -> Result<(), Self::Err> {
        self.execute(q, ()).map(|_| ())
    }

    fn ident_quote_char() -> &'static str {
        "\""
    }

    fn string_quote_char() -> &'static str {
        "'"
    }
}
