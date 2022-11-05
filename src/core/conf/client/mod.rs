pub mod builder;

#[derive(Clone)]
pub struct TypeScriptClient {
    pub(crate) host_url: String,
    pub(crate) react_queries: bool,
    pub(crate) react_forms: bool,
    pub(crate) object_name: String,
}

#[derive(Clone)]
pub struct SwiftClient {
    pub(crate) host_url: String,
    pub(crate) combine_observable_objects: bool,
}

#[derive(Clone)]
pub struct KotlinClient {
    pub(crate) host_url: String,
    pub(crate) jetpack_compose_states: bool,
}

#[derive(Clone)]
pub struct CSharpClient {
    pub(crate) host_url: String,
}

#[derive(Clone)]
pub struct DartClient {
    pub(crate) host_url: String,
}

#[derive(Clone)]
pub enum Client {
    TypeScriptClient(TypeScriptClient),
    SwiftClient(SwiftClient),
    KotlinClient(KotlinClient),
    CSharpClient(CSharpClient),
    DartClient(DartClient),
}

impl Client {
    pub(crate) fn as_typescript(&self) -> &TypeScriptClient {
        match self {
            Client::TypeScriptClient(c) => c,
            _ => panic!()
        }
    }

    pub(crate) fn as_swift(&self) -> &SwiftClient {
        match self {
            Client::SwiftClient(c) => c,
            _ => panic!()
        }
    }

    pub(crate) fn as_kotlin(&self) -> &KotlinClient {
        match self {
            Client::KotlinClient(c) => c,
            _ => panic!()
        }
    }

    pub(crate) fn as_csharp(&self) -> &CSharpClient {
        match self {
            Client::CSharpClient(c) => c,
            _ => panic!()
        }
    }

    pub(crate) fn as_dart(&self) -> &DartClient {
        match self {
            Client::DartClient(c) => c,
            _ => panic!()
        }
    }
}
