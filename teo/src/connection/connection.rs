use crate::types::Schema;

pub trait Connection {

    type Err;

    fn migrate<S>(&self) -> impl Future<Output = Result<(), Self::Err>> + Send where S: Schema;
}
