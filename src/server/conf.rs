#[derive(Debug)]
pub struct ServerConf {
    pub(crate) bind: (String, u16),
    pub(crate) jwt_secret: Option<&'static str>,
    pub(crate) path_prefix: Option<&'static str>,
}