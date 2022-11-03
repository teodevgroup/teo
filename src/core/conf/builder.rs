use crate::core::conf::client::builder::{CSharpClientBuilder, DartClientBuilder, KotlinClientBuilder, SwiftClientBuilder, TypeScriptClientBuilder};
use crate::core::conf::Conf;
use crate::core::conf::client::Client;

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

    pub fn typescript_client<F: Fn(&mut TypeScriptClientBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut builder = TypeScriptClientBuilder::new(name.into());
        build(&mut builder);
        self.clients.push(Client::TypeScriptClient(builder.build()));
        self
    }

    pub fn swift_client<F: Fn(&mut SwiftClientBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut builder = SwiftClientBuilder::new(name.into());
        build(&mut builder);
        self.clients.push(Client::SwiftClient(builder.build()));
        self
    }

    pub fn kotlin_client<F: Fn(&mut KotlinClientBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut builder = KotlinClientBuilder::new(name.into());
        build(&mut builder);
        self.clients.push(Client::KotlinClient(builder.build()));
        self
    }

    pub fn csharp_client<F: Fn(&mut CSharpClientBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut builder = CSharpClientBuilder::new(name.into());
        build(&mut builder);
        self.clients.push(Client::CSharpClientBuilder(builder.build()));
        self
    }

    pub fn dart_client<F: Fn(&mut DartClientBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let mut builder = DartClientBuilder::new(name.into());
        build(&mut builder);
        self.clients.push(Client::DartClient(builder.build()));
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
