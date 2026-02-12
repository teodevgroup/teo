#[macro_use]
mod table_defs;

pub(in crate::schema) mod impl_schema;
mod mongo_table_defs;
mod mysql_table_defs;
mod postgres_table_defs;
mod sqlite_table_defs;

pub(in crate::schema) use impl_schema::generate_impl_schema;
