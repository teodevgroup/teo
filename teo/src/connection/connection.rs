use crate::types::Schema;

pub trait SyncConnection {

    type Err;

    fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema;
}

pub trait AsyncConnection {

    type Err;

    fn migrate<S>(&self) -> impl Future<Output = Result<(), Self::Err>> + Send where S: Schema;
}
