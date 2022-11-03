pub struct TypeScriptClientBuilder {
    host_url: String,
    react_queries: bool,
    react_forms: bool,
    object_name: String,
}

impl TypeScriptClientBuilder {
    pub(crate) fn new() -> TypeScriptClientBuilder {
        TypeScriptClientBuilder {
            host_url: "".to_string(),
            react_queries: false,
            react_forms: false,
            object_name: "teo".to_string(),
        }
    }

    pub(crate) fn build(&self) -> TypeScriptClient {
        TypeScriptClient {
            host_url: self.host_url.clone(),
            react_queries: self.react_queries,
            react_forms: self.react_forms,
            object_name: self.object_name.clone(),
        }
    }

    pub fn host_url(&mut self, url: String) -> &mut Self {
        self.host_url = url;
        self
    }

    pub fn react_queries(&mut self) -> &mut Self {
        self.react_queries = true;
        self
    }

    pub fn react_forms(&mut self) -> &mut Self {
        self.react_forms = true;
        self
    }

    pub fn object_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.object_name = name.into();
        self
    }
}

pub struct SwiftClientBuilder {
    host_url: String,
    combine_observable_objects: bool,
}

impl SwiftClientBuilder {
    pub(crate) fn new() -> SwiftClientBuilder {
        SwiftClientBuilder {
            host_url: "".to_string(),
            combine_observable_objects: false,
        }
    }

    pub(crate) fn build(&self) -> SwiftClient {
        SwiftClient {
            host_url: self.host_url.clone(),
            combine_observable_objects: self.combine_observable_objects,
        }
    }

    pub fn host_url(&mut self, url: String) -> &mut Self {
        self.host_url = url;
        self
    }

    pub fn combine_observable_objects(&mut self) -> &mut Self {
        self.combine_observable_objects = true;
        self
    }
}

pub struct KotlinClientBuilder {
    host_url: String,
    jetpack_compose_states: bool,
}

impl KotlinClientBuilder {
    pub(crate) fn new() -> KotlinClientBuilder {
        KotlinClientBuilder {
            host_url: "".to_string(),
            jetpack_compose_states: false,
        }
    }

    pub(crate) fn build(&self) -> KotlinClient {
        KotlinClient {
            host_url: self.host_url.clone(),
            jetpack_compose_states: self.jetpack_compose_states,
        }
    }

    pub fn host_url(&mut self, url: String) -> &mut Self {
        self.host_url = url;
        self
    }

    pub fn jetpack_compose_states(&mut self) -> &mut Self {
        self.jetpack_compose_states = true;
        self
    }
}

pub struct CSharpClientBuilder {
    host_url: String,
}

impl CSharpClientBuilder {
    pub(crate) fn new() -> CSharpClientBuilder {
        CSharpClientBuilder {
            host_url: "".to_string(),
        }
    }

    pub(crate) fn build(&self) -> CSharpClient {
        CSharpClient {
            host_url: self.host_url.clone(),
        }
    }

    pub fn host_url(&mut self, url: String) -> &mut Self {
        self.host_url = url;
        self
    }
}

pub struct DartClientBuilder {
    host_url: String,
}

impl DartClientBuilder {
    pub(crate) fn new() -> DartClientBuilder {
        DartClientBuilder {
            host_url: "".to_string(),
        }
    }

    pub(crate) fn build(&self) -> DartClient {
        DartClient {
            host_url: self.host_url.clone(),
        }
    }

    pub fn host_url(&mut self, url: String) -> &mut Self {
        self.host_url = url;
        self
    }
}
