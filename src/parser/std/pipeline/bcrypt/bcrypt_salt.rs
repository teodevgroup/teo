use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::bcrypt::bcrypt_salt::BcryptSaltItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn bcrypt_salt(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(BcryptSaltItem::new())
}
