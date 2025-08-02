pub trait Insertable {

  #[cfg(feature = "mysql")]
  fn insert_as_mysql(&self) -> mysql_async::QueryWithParams<&'static str, mysql_async::Params>;

  #[cfg(feature = "postgres")]
  fn insert_as_postgres(&self) -> (&'static str, &[&(dyn tokio_postgres::types::ToSql + Sync)]);

  #[cfg(feature = "sqlite")]
  fn insert_as_sqlite(&self) -> (&'static str, &[&(dyn rusqlite::types::ToSql + Sync)]);

  #[cfg(feature = "mongo")]
  fn insert_as_mongo(&self) -> mongodb::bson::Bson;
}
