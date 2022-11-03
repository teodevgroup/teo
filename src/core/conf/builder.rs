use crate::core::conf::Conf;

pub struct ConfBuilder {
    pub(crate) bind: (String, u16),
    pub(crate) jwt_secret: Option<String>,
    pub(crate) path_prefix: Option<String>,
    pub(crate) clients: Vec<Client>,
}

impl ConfBuilder {
    pub(crate) fn new() -> Self {
        Self {
            bind: ("0.0.0.0".into(), 5100u16),
            jwt_secret: None,
            path_prefix: None,
            clients: vec![],
        }
    }

    pub fn bind(&mut self, bind: (impl Into<String>, i32)) -> &mut Self {
        self.bind = (bind.0.into(), (bind.1 as u16).into());
        self
    }

    pub fn jwt_secret(&mut self, jwt_secret: impl Into<String>) -> &mut Self {
        self.jwt_secret = Some(jwt_secret.into());
        self
    }

    pub fn path_prefix(&mut self, path_prefix: impl Into<String>) -> &mut Self {
        self.path_prefix = Some(path_prefix.into());
        self
    }

    pub(crate) fn build(&self) -> Conf {
        Conf {
            bind: self.bind.clone(),
            jwt_secret: self.jwt_secret.clone(),
            path_prefix: self.path_prefix.clone(),
            clients: self.clients.clone(),
        }
    }
}
