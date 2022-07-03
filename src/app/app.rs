#[derive(Clone)]
pub struct ServerConfiguration {
    pub(crate) bind: (String, u16),
    pub(crate) jwt_secret: Option<String>,
    pub(crate) path_prefix: Option<String>,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        ServerConfiguration {
            bind: ("0.0.0.0".into(), 5000 as u16),
            jwt_secret: None,
            path_prefix: None,
        }
    }
}

pub struct ServerConfigurationBuilder {
    bind: (String, u16),
    jwt_secret: Option<String>,
    path_prefix: Option<String>,
}

impl ServerConfigurationBuilder {
    pub fn new() -> ServerConfigurationBuilder {
        ServerConfigurationBuilder {
            bind: ("0.0.0.0".into(), 5000u16),
            jwt_secret: None,
            path_prefix: None,
        }
    }

    pub fn bind(&mut self, bind: (impl Into<String>, impl Into<u16>)) -> &mut Self {
        self.bind = (bind.0.into(), bind.1.into());
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

    pub(crate) fn build(&self) -> ServerConfiguration {
        ServerConfiguration {
            bind: self.bind.clone(),
            jwt_secret: self.jwt_secret.clone(),
            path_prefix: self.path_prefix.clone(),
        }
    }
}

pub struct App {
    pub(crate) server: ServerConfiguration,
    pub(crate) client: ClientConfiguration,
}

impl App {
    pub fn new<F: Fn(&mut AppBuilder)>(build: F) -> App {
        let mut builder = AppBuilder::new();
        build(&mut builder);
        builder.build()
    }
}

pub struct AppBuilder {
    server: ServerConfiguration,
    client: ClientConfiguration,
}

impl AppBuilder {
    pub(crate) fn new() -> AppBuilder {
        AppBuilder {
            server: ServerConfiguration::default(),
            client: ClientConfiguration::default(),
        }
    }

    pub(crate) fn build(&self) -> App {
        App {
            server: self.server.clone(),
            client: self.client.clone(),
        }
    }

    pub fn server<F: Fn(&mut ServerConfigurationBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ServerConfigurationBuilder::new();
        build(&mut builder);
        self.server = builder.build();
        self
    }

    pub fn client<F: Fn(&mut ClientBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = ClientBuilder::new();
        build(&mut builder);
        self.client = builder.build();
        self
    }
}

pub struct ClientBuilder {
    type_script: Option<TypeScriptClient>,
    swift: Option<SwiftClient>,
    kotlin: Option<KotlinClient>,
    host_url: Option<String>,
}

impl ClientBuilder {
    pub(crate) fn new() -> ClientBuilder {
        ClientBuilder {
            type_script: None,
            swift: None,
            kotlin: None,
            host_url: None,
        }
    }

    pub(crate) fn build(&self) -> ClientConfiguration {
        ClientConfiguration {
            type_script: self.type_script.clone(),
            swift: self.swift.clone(),
            kotlin: self.kotlin.clone(),
            host_url: self.host_url.clone(),
        }
    }

    pub fn type_script<F: Fn(&mut TypeScriptClientBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = TypeScriptClientBuilder::new();
        build(&mut builder);
        self.type_script = Some(builder.build());
        self
    }

    pub fn swift<F: Fn(&mut SwiftClientBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = SwiftClientBuilder::new();
        build(&mut builder);
        self.swift = Some(builder.build());
        self
    }

    pub fn kotlin<F: Fn(&mut KotlinClientBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = KotlinClientBuilder::new();
        build(&mut builder);
        self.kotlin = Some(builder.build());
        self
    }
}

#[derive(Clone)]
pub struct ClientConfiguration {
    type_script: Option<TypeScriptClient>,
    swift: Option<SwiftClient>,
    kotlin: Option<KotlinClient>,
    pub(crate) host_url: Option<String>,
}

impl Default for ClientConfiguration {
    fn default() -> Self {
        ClientConfiguration {
            type_script: None,
            swift: None,
            kotlin: None,
            host_url: None,
        }
    }
}

#[derive(Clone)]
pub struct TypeScriptClient {
    react_queries: bool,
    react_forms: bool,
}

pub struct TypeScriptClientBuilder {
    react_queries: bool,
    react_forms: bool,
}

impl TypeScriptClientBuilder {
    pub(crate) fn new() -> TypeScriptClientBuilder {
        TypeScriptClientBuilder {
            react_queries: false,
            react_forms: false,
        }
    }

    pub(crate) fn build(&self) -> TypeScriptClient {
        TypeScriptClient {
            react_queries: self.react_queries,
            react_forms: self.react_forms,
        }
    }

    pub fn react_queries(&mut self) -> &mut Self {
        self.react_queries = true;
        self
    }

    pub fn react_forms(&mut self) -> &mut Self {
        self.react_forms = true;
        self
    }
}

#[derive(Clone)]
pub struct SwiftClient {
    combine_observable_objects: bool,
}

pub struct SwiftClientBuilder {
    combine_observable_objects: bool,
}

impl SwiftClientBuilder {
    pub(crate) fn new() -> SwiftClientBuilder {
        SwiftClientBuilder {
            combine_observable_objects: false,
        }
    }

    pub(crate) fn build(&self) -> SwiftClient {
        SwiftClient {
            combine_observable_objects: self.combine_observable_objects,
        }
    }

    pub fn combine_observable_objects(&mut self) -> &mut Self {
        self.combine_observable_objects = true;
        self
    }
}

#[derive(Clone)]
pub struct KotlinClient {
    jetpack_compose_states: bool,
}

pub struct KotlinClientBuilder {
    jetpack_compose_states: bool,
}

impl KotlinClientBuilder {
    pub(crate) fn new() -> KotlinClientBuilder {
        KotlinClientBuilder {
            jetpack_compose_states: false,
        }
    }

    pub(crate) fn build(&self) -> KotlinClient {
        KotlinClient {
            jetpack_compose_states: self.jetpack_compose_states,
        }
    }

    pub fn jetpack_compose_states(&mut self) -> &mut Self {
        self.jetpack_compose_states = true;
        self
    }
}
