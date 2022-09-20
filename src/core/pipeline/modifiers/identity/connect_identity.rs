use async_trait::async_trait;
use cuid::cuid;
use crate::core::pipeline::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::pipeline::context::Context;
use crate::core::relation::RelationManipulation;

#[derive(Debug, Copy, Clone)]
pub struct ConnectIdentityModifier {}

impl ConnectIdentityModifier {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Modifier for ConnectIdentityModifier {

    fn name(&self) -> &'static str {
        "connectIdentity"
    }

    async fn call(&self, ctx: Context) -> Context {
        if !ctx.identity.is_some() {
            return ctx;
        }
        let model = ctx.object.model();
        let relation_name = ctx.key_path[0].as_string().unwrap();
        let relation = model.relation(relation_name).unwrap();
        let relation_model_name = relation.model.as_str();
        let identity = ctx.identity.as_ref().unwrap().clone();
        let identity_model_name = identity.model().name();
        if relation_model_name != identity_model_name {
            return ctx;
        }
        let mut map = ctx.object.inner.relation_mutation_map.lock().unwrap();
        let mutations = map.get(relation_name);
        if mutations.is_none() {
            map.insert(relation_name.to_string(), Vec::new());
            map.get_mut(relation_name).unwrap().push(RelationManipulation::Connect(identity.clone()));
        }
        ctx.clone()
    }
}
