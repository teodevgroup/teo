use crate::model::Insertable;
use super::Connection;
use async_trait::async_trait;
use mysql_async::prelude::Query;

#[async_trait]
impl Connection for mysql_async::Conn {

  type Error = mysql_async::Error;

  async fn insert_silently<I>(&mut self, insertable: &I) -> Result<(), Self::Error> where I: Insertable + Sync {
    let query_with_params = insertable.insert_as_mysql();
    query_with_params.ignore(self).await
  }
}
