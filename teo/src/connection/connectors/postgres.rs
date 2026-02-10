use tokio_postgres::{Client, Error};
use crate::{connection::AsyncConnection, types::Schema};

impl AsyncConnection for Client {

    type Err = Error;

    async fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema {
        todo!()
    }
}
