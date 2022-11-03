pub struct TypeScriptClientBuilder {
    react_queries: bool,
    react_forms: bool,
    object_name: String,
}

impl TypeScriptClientBuilder {
    pub(crate) fn new() -> TypeScriptClientBuilder {
        TypeScriptClientBuilder {
            react_queries: false,
            react_forms: false,
            object_name: "teo".to_string(),
        }
    }

    pub(crate) fn build(&self) -> TypeScriptClient {
        TypeScriptClient {
            react_queries: self.react_queries,
            react_forms: self.react_forms,
            object_name: self.object_name.clone(),
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

    pub fn object_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.object_name = name.into();
        self
    }
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

pub struct CSharpClientBuilder { }

impl CSharpClientBuilder {
    pub(crate) fn new() -> CSharpClientBuilder {
        CSharpClientBuilder { }
    }

    pub(crate) fn build(&self) -> CSharpClient {
        CSharpClient { }
    }
}
