use crate::gen::interface::server::conf::EntityConf;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::server::outline::outline::EntityOutline;
use crate::gen::internal::server_type_lookup::ServerTypeLookup;
use crate::gen::internal::type_lookup::TypeLookup;
use crate::prelude::Graph;

pub(in crate::gen) struct Ctx<'a> {
    pub(in crate::gen) conf: &'a EntityConf,
    pub(in crate::gen) graph: &'a Graph,
    pub(in crate::gen) client_outline: Outline<'a>,
    pub(in crate::gen) entity_outline: EntityOutline<'a>,
}

impl<'a> Ctx<'a> {
    pub(in crate::gen) fn build<L1, L2>(graph: &'a Graph, conf: &'a EntityConf, lookup: L1, server_lookup: L2) -> Self where L1: TypeLookup, L2: ServerTypeLookup {
        Self {
            conf, graph,
            client_outline: Outline::new(graph, lookup),
            entity_outline: EntityOutline::new(graph, server_lookup),
        }
    }
}
