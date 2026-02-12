use mongodb::{Collection, Database, bson::{Bson, doc}, error::Error};
use serde::de::DeserializeOwned;
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

    async fn exist_table_def(&mut self, table_name: &str) -> Result<TableDef<mongo::ColumnType>, Self::Err> {
        let collections: Collection<TableDef<Self::ColumnType>> = self.collection("_Collections");
        let table_def = collections.find_one(doc!{ "name": table_name }).await?.unwrap();
        Ok(table_def)
    }

    fn drop_table_column_statement(&self, table_name: &str, column_name: &str) -> String {
        unreachable!()
    }

    fn add_table_column_statement(&self, table_name: &str, column_def: &ColumnDef<Self::ColumnType>) -> String {
        unreachable!()
    }

    fn alter_table_column_type_statement(&self, table_name: &str, column_name: &str, column_ty: &Self::ColumnType) -> String {
        unreachable!()
    }

    fn alter_table_column_set_not_null_statement(&self, table_name: &str, column_name: &str) -> String {
        unreachable!()
    }

    fn alter_table_column_drop_not_null_statement(&self, table_name: &str, column_name: &str) -> String {
        unreachable!()
    }

    fn alter_table_column_set_default_statement(&self, table_name: &str, column_name: &str, default: &str) -> String {
        unreachable!()
    }

    fn alter_table_column_drop_default_statement(&self, table_name: &str, column_name: &str) -> String {
        unreachable!()
    }

    fn drop_index_statement(&self, index_name: &str) -> String {
        unreachable!()
    }

    async fn create_enum(&mut self, enum_def: &EnumDef) -> Result<(), Self::Err> {
        Ok(())
    }

    async fn diff_enum(&mut self, defined_enum_def: &EnumDef) -> Result<(), Self::Err> {
        Ok(())
    }

    async fn delete_enum(&mut self, enum_name: &str) -> Result<(), Self::Err> {
        Ok(())
    }

    async fn add_enum_variant(&mut self, enum_name: &str, variant_name: &str) -> Result<(), Self::Err> {
        Ok(())
    }

    async fn delete_table(&mut self, table_name: &str) -> Result<(), Self::Err> {
        self.collection::<Bson>(table_name).drop().await?;
        let collections: Collection<TableDef<Self::ColumnType>> = self.collection("_Collections");
        collections.delete_one(doc!{ "name": table_name }).await?;
        Ok(())
    }

    async fn create_table(&mut self, table_def: &TableDef<Self::ColumnType>) -> Result<(), Self::Err> {
        self.create_collection(table_def.name.as_ref()).await?;
        let collections: Collection<TableDef<Self::ColumnType>> = self.collection("_Collections");
        collections.insert_one(table_def).await?;
        Ok(())
    }

    async fn drop_table_column(&mut self, table_name: &str, column_name: &str) -> Result<(), Self::Err> {
        let table: Collection<Bson> = self.collection(table_name);
        table.update_many(doc!{}, doc!{"$unset": {column_name: 1}}).await?;
        let collections: Collection<TableDef<Self::ColumnType>> = self.collection("_Collections");
        collections.update_one(doc!{"name": table_name}, doc!{

        }).await?;
        Ok(())
    }
}
