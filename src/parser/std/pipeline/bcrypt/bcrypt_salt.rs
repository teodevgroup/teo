use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::modifiers::bcrypt::bcrypt_salt::BcryptSaltModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn bcrypt_salt(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(BcryptSaltModifier::new())
}
