use teo_column_type::postgres;
use tokio_postgres::{Client, Error};
use crate::{connection::AsyncConnection, migration::AsyncMigration, types::Schema};

impl AsyncConnection for Client {

    type Err = Error;

    async fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        AsyncMigration::migrate::<S>(self).await
    }
}

impl AsyncMigration for Client {

    type Err = Error;

    type ColumnType = postgres::ColumnType;

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

    #[inline]
    fn exist_table_defs<S>(&self) -> Vec<crate::migration::TableDef<postgres::ColumnType>> where S: Schema {
        S::postgres_table_defs()
    }
}
