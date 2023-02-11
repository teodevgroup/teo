use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::bcrypt::bcrypt_salt::BcryptSaltItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn bcrypt_salt(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(BcryptSaltItem::new())
}
