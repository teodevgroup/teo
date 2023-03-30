use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::outline::outline::Outline;
use crate::prelude::Graph;

pub(in crate::gen) struct Ctx<'a> {
    pub(in crate::gen) conf: &'a Conf,
    pub(in crate::gen) graph: &'a Graph,
    pub(in crate::gen) outline: &'a Outline,
}
