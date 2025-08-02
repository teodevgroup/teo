use crate::model::Insertable;
use super::Connection;
use async_trait::async_trait;
use mongodb::Database;

#[async_trait]
impl Connection for Database {

  type Error = mongodb::error::Error;

  async fn insert_silently<I>(&mut self, insertable: &I) -> Result<(), Self::Error> where I: Insertable + Sync {
    let collection = self.collection("abc");
    let _ = collection.insert_one(insertable.insert_as_mongo()).await?;
    Ok(())
  }
}
