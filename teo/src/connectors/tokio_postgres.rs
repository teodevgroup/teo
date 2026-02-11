use tokio_postgres::{Client, Error};
use crate::{connection::AsyncConnection, migration::AsyncMigration, types::Schema};

impl AsyncConnection for Client {

    type Err = Error;

    async fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        AsyncMigration::migrate(self)
    }
}

impl AsyncMigration for Client {

    type Err = Error;

    #[inline]
    async fn execute_without_params(&self, q: &str) -> Result<(), Self::Err> {
        self.execute(q, &[]).await.map(|_| ())
    }

    #[inline]
    fn ident_quote_char() -> &'static str {
        "\""
    }

    #[inline]
    fn string_quote_char() -> &'static str {
        "'"
    }
}
