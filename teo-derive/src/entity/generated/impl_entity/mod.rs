#[macro_use]
mod table_def;

pub(crate) mod impl_entity;
#[cfg(feature = "mongo")]
mod mongo_table_def;
#[cfg(feature = "mysql")]
mod mysql_table_def;
#[cfg(feature = "postgres")]
mod postgres_table_def;
#[cfg(feature = "sqlite")]
mod sqlite_table_def;

pub(in crate::entity) use impl_entity::generate_impl_entity;
