use std::borrow::Cow;
use crate::gen::internal::type_lookup::TypeLookup;
use crate::prelude::Graph;

pub(crate) struct DelegateAction<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) response: Cow<'a, str>,
    pub(crate) docs: Option<Cow<'a, str>>,
}

pub(crate) struct Delegate<'a> {
    pub(crate) model_name: Cow<'a, str>,
    pub(crate) actions: Vec<DelegateAction<'a>>,
}

pub(crate) fn delegates<T>(graph: &Graph, lookup: T) -> Vec<Delegate> where T: TypeLookup {
    graph.models().iter().map(|m| {
        Delegate {
            model_name: Cow::Borrowed(m.name()),
            actions: m.actions().iter().map(|a| DelegateAction {
                name: Cow::Borrowed(a.as_handler_str()),
                response: lookup.action_result_type(*a, m.name()),
                docs: None,
            }).collect(),
        }
    }).collect()
}
