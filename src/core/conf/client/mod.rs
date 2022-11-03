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
