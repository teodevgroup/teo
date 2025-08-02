use crate::model::Insertable;
use super::Connection;
use async_trait::async_trait;
use tokio_postgres::{Client, Error};

#[async_trait]
impl Connection for Client {

  type Error = Error;

  async fn insert_silently<I>(&mut self, insertable: &I) -> Result<(), Self::Error> where I: Insertable + Sync {
    let (statement, bindings) = insertable.insert_as_postgres();
    let _ = self.execute(statement, bindings).await?;
    Ok(())
  }
}
