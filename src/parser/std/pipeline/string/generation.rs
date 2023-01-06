use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::string::generation::cuid::CUIDModifier;
use crate::core::pipeline::modifiers::string::generation::random_digits::RandomDigitsModifier;
use crate::core::pipeline::modifiers::string::generation::slug::SlugModifier;
use crate::core::pipeline::modifiers::string::generation::uuid::UUIDModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn cuid(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(CUIDModifier::new())
}

pub(crate) fn random_digits(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let arg = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(RandomDigitsModifier::new(arg))
}

pub(crate) fn slug(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(SlugModifier::new())
}

pub(crate) fn uuid(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(UUIDModifier::new())
}
