use crate::model::Insertable;
use super::Connection;
use async_trait::async_trait;

#[async_trait]
impl Connection for rusqlite::Connection {

  type Error = rusqlite::Error;

  async fn insert_silently<I>(&mut self, insertable: &I) -> Result<(), Self::Error> where I: Insertable + Sync {
    let (statement, bindings) = insertable.insert_as_sqlite();
    let _ = self.execute(statement, ("s", &2))?;
    Ok(())
  }
}
