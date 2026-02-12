use crate::types::Schema;

pub trait SyncConnection {

    type Err;

    fn migrate<S>(&mut self) -> Result<(), Self::Err> where S: Schema;
}

pub trait AsyncConnection {

    type Err;

    fn migrate<S>(&mut self) -> impl Future<Output = Result<(), Self::Err>> + Send where S: Schema;
}
