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
    csharp: Option<CSharpClient>,
    host_url: Option<String>,
}

impl ClientBuilder {
    pub(crate) fn new() -> ClientBuilder {
        ClientBuilder {
            type_script: None,
            swift: None,
            kotlin: None,
            csharp: None,
            host_url: None,
        }
    }

    pub(crate) fn build(&self) -> ClientConfiguration {
        ClientConfiguration {
            type_script: self.type_script.clone(),
            swift: self.swift.clone(),
            kotlin: self.kotlin.clone(),
            csharp: self.csharp.clone(),
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

    pub fn csharp<F: Fn(&mut CSharpClientBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = CSharpClientBuilder::new();
        build(&mut builder);
        self.csharp = Some(builder.build());
        self
    }

    pub fn host_url(&mut self, url: impl Into<String>) -> &mut Self {
        self.host_url = Some(url.into());
        self
    }
}

#[derive(Clone)]
pub struct ClientConfiguration {
    pub(crate) type_script: Option<TypeScriptClient>,
    pub(crate) swift: Option<SwiftClient>,
    pub(crate) kotlin: Option<KotlinClient>,
    pub(crate) csharp: Option<CSharpClient>,
    pub(crate) host_url: Option<String>,
}

impl Default for ClientConfiguration {
    fn default() -> Self {
        ClientConfiguration {
            type_script: None,
            swift: None,
            kotlin: None,
            csharp: None,
            host_url: None,
        }
    }
}

unsafe impl Send for App {}
unsafe impl Sync for App {}
