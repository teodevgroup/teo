
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
