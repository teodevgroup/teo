use std::borrow::Cow;
use crate::core::action::Action;
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

pub(crate) fn delegates<'a, F>(graph: &'a Graph, action_result: F) -> Vec<Delegate> where F: Fn(Action, &'a str) -> Cow<'a, str> {
    graph.models().iter().map(|m| {
        Delegate {
            model_name: Cow::Borrowed(m.name()),
            actions: m.actions().iter().map(|a| DelegateAction {
                name: Cow::Borrowed(a.as_handler_str()),
                response: action_result(*a, m.name()),
                docs: None,
            }).collect(),
        }
    }).collect()
}
