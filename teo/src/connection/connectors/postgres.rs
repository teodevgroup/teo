use tokio_postgres::{Client, Error};
use crate::{connection::Connection, types::Schema};

impl Connection for Client {

    type Err = Error;

    async fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        todo!()
    }
}
