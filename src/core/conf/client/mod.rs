pub mod builder;

#[derive(Clone)]
pub struct TypeScriptClient {
    pub(crate) react_queries: bool,
    pub(crate) react_forms: bool,
    pub(crate) object_name: String,
}

#[derive(Clone)]
pub struct SwiftClient {
    pub(crate) combine_observable_objects: bool,
}

#[derive(Clone)]
pub struct KotlinClient {
    pub(crate) jetpack_compose_states: bool,
}

#[derive(Clone)]
pub struct CSharpClient { }

#[derive(Clone)]
pub struct DartClient { }

#[derive(Clone)]
pub enum Client {
    TypeScriptClient(TypeScriptClient),
    SwiftClient(SwiftClient),
    KotlinClient(KotlinClient),
    CSharpClient(CSharpClient),
    DartClient(DartClient),
}
