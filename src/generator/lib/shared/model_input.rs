use std::borrow::Cow;
use crate::prelude::Graph;

pub(crate) struct ModelInclude<'a> {
    pub(crate) relation_name: Cow<'a, str>,
    pub(crate) model_name: Cow<'a, str>,
    pub(crate) many: bool,
}

pub(crate) struct ModelInput<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) select: Vec<Cow<'a, str>>,
    pub(crate) includes: Vec<ModelInclude<'a>>,
}

pub(crate) fn model_inputs(graph: &Graph) -> Vec<ModelInput> {
    graph.models().iter().map(|m| {
        ModelInput {
            name: Cow::Borrowed(m.name()),
            select: m.output_keys().iter().filter(|k| m.field(k).is_some()).map(|k| Cow::Borrowed(k.as_str())).collect(),
            includes: m.relations().iter().map(|r| ModelInclude { relation_name: Cow::Borrowed(r.name()), model_name: Cow::Borrowed(r.model()), many: r.is_vec() }).collect(),
        }
    }).collect()
}
