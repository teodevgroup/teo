pub mod builder;
pub mod client;

use crate::core::conf::client::Client;

#[derive(Clone)]
pub struct Conf {
    pub(crate) bind: (String, u16),
    pub(crate) jwt_secret: Option<String>,
    pub(crate) path_prefix: Option<String>,
    pub(crate) clients: Vec<Client>,
}
