use std::sync::Arc;
use crate::core::pipeline::item::Item;


use crate::core::items::bcrypt::bcrypt_verify::BcryptVerifyItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn bcrypt_verify(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(BcryptVerifyItem::new(value.as_pipeline().unwrap().clone()))
}
