use std::borrow::Cow;

pub(crate) struct DelegateAction<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) response: Cow<'a, str>,
    pub(crate) docs: Option<Cow<'a, str>>,
}

pub(crate) struct Delegate<'a> {
    pub(crate) model_name: Cow<'a, str>,
    pub(crate) actions: Vec<DelegateAction<'a>>,
}
