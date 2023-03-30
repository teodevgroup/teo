use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::type_lookup::TypeLookup;
use crate::prelude::Graph;

pub(in crate::gen) struct Ctx<'a> {
    pub(in crate::gen) conf: &'a Conf,
    pub(in crate::gen) graph: &'a Graph,
    pub(in crate::gen) outline: Outline<'a>,
}

impl<'a> Ctx<'a> {
    pub(in crate::gen) fn build<L>(graph: &'a Graph, conf: &'a Conf, lookup: L) -> Self where L: TypeLookup {
        Self { conf, graph, outline: Outline::new(graph, lookup) }
    }
}
