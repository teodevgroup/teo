use mongodb::{Collection, Database, bson::{Bson, doc}, error::Error};
use crate::{connection::AsyncConnection, migration::{AsyncMigration, ColumnDef, EnumDef, IndexColumnDef, IndexDef, TableDef}, types::Schema};
use teo_column_type::mongo;

impl AsyncConnection for Database {

    type Err = Error;

    async fn migrate<S>(&mut self) -> Result<(), Error> where S: Schema {
        AsyncMigration::migrate::<S>(self).await
    }
}

impl AsyncMigration for Database {

    type Err = Error;

    type ColumnType = mongo::ColumnType;

    async fn execute_without_params(&mut self, q: &str) -> Result<(), Self::Err> {
        unreachable!()
    }

    fn ident_quote_char() -> &'static str {
        unreachable!()
    }

    fn string_quote_char() -> &'static str {
        unreachable!()
    }

    async fn exist_enum_names(&mut self) -> Result<Vec<String>, Self::Err> {
        unreachable!()
    }

    fn enum_create_statement(&self, enum_def: &EnumDef) -> String {
        unreachable!()
    }

    fn enum_drop_statement(&self, enum_name: &str) -> String {
        unreachable!()
    }

    fn add_enum_variant_statement(&self, enum_name: &str, variant_name: &str) -> String {
        unreachable!()
    }

    async fn exist_enum_def(&mut self, enum_name: &'static str) -> Result<EnumDef, Self::Err> {
        unreachable!()
    }

    fn defined_table_defs<S>(&self) -> Vec<TableDef<Self::ColumnType>> where S: Schema {
        S::mongo_table_defs()
    }

    async fn exist_table_names(&mut self) -> Result<Vec<String>, Self::Err> {
        self.list_collection_names().await
    }

    fn drop_table_statement(&self, table_name: &str) -> String {
        unreachable!()
    }

    fn create_table_statement(&self, table_def: &TableDef<Self::ColumnType>) -> String {
        unreachable!()
    }

    fn column_statement(&self, column_def: &ColumnDef<Self::ColumnType>) -> String {
        unreachable!()
    }

    fn index_column_statement(&self, index_column_def: &IndexColumnDef) -> String {
        unreachable!()
    }

    fn create_index_statement(&self, table_name: &str, index_def: &IndexDef) -> String {
        unreachable!()
    }

    async fn exist_table_def(&mut self, table_name: &'static str) -> Result<TableDef<Self::ColumnType>, Self::Err> {
        let collections: Collection<TableDef<Self::ColumnType>> = self.collection("_Collections");
        let cursor = collections.find(doc! {}).await?;
        while let Some(doc) = cursor.try_next().await? {
            println!("{:?}", doc);
        }
    }

}
