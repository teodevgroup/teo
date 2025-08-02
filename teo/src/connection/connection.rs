use async_trait::async_trait;
use crate::model::{Insertable, Orderable, Queryable, Selectable, Updatable};

#[async_trait]
pub trait Connection {

  type Error;

  async fn insert_silently<I>(&mut self, insertable: &I) -> Result<(), Self::Error> where I: Insertable + Sync;

  // async fn insert<I, S>(&mut self, insertable: &I) -> Result<S, Self::Error> where I: Insertable, S: Selectable;

  // async fn insert_or_ignore_silently<I>(&mut self, insertable: &I) -> Result<(), Self::Error> where I: Insertable;

  // async fn insert_or_ignore<I, S>(&mut self, insertable: &I) -> Result<S, Self::Error> where I: Insertable, S: Selectable;

  // async fn update_silently<Q, U>(&mut self, queryable: &Q, updatable: &U) -> Result<(), Self::Error> where Q: Queryable, U: Updatable;

  // async fn update<Q, U, S>(&mut self, queryable: &Q, updatable: &U) -> Result<S, Self::Error> where Q: Queryable, U: Updatable, S: Selectable;

  // async fn update_or_ignore_silently<Q, U>(&mut self, queryable: &Q, updatable: &U) -> Result<(), Self::Error> where Q: Queryable, U: Updatable;

  // async fn update_or_ignore<Q, U, S>(&mut self, queryable: &Q, updatable: &U) -> Result<Option<S>, Self::Error> where Q: Queryable, U: Updatable, S: Selectable;

  // async fn update_many_silently<Q, U>(&mut self, queryable: &Q, updatable: &U) -> Result<(), Self::Error> where Q: Queryable, U: Updatable;

  // async fn update_many<Q, U, S>(&mut self, queryable: &Q, updatable: &U) -> Result<Vec<S>, Self::Error> where Q: Queryable, U: Updatable, S: Selectable;

  // async fn find_unique<Q, S>(&mut self, queryable: &Q) -> Result<Option<S>, Self::Error> where Q: Queryable, S: Selectable;

  // async fn find_many<Q, O, S>(&mut self, queryable: &Q, orderable: &O, limit: u64, offset: u64) -> Result<Vec<S>, Self::Error> where Q: Queryable, O: Orderable, S: Selectable;

  // async fn find_one<Q, O, S>(&mut self, queryable: &Q, orderable: &O, offset: u64) -> Result<Option<S>, Self::Error> where Q: Queryable, O: Orderable, S: Selectable;

  // async fn delete<Q>(&mut self, queryable: &Q) -> Result<(), Self::Error> where Q: Queryable;

  // async fn delete_or_ignore<Q>(&mut self, queryable: &Q) -> Result<(), Self::Error> where Q: Queryable;

  // async fn count_all_records(&mut self) -> Result<usize, Self::Error>;

  // async fn count_records<Q>(&mut self, queryable: &Q) -> Result<usize, Self::Error> where Q: Queryable;
}
