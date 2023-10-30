use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::identity::identity::IdentityItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn identity(args: &Vec<Argument>) -> Arc<dyn Item> {
    let pipeline = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    Arc::new(IdentityItem::new(pipeline.clone()))
}
