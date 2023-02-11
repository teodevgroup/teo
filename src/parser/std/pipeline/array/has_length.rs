use std::sync::Arc;
use crate::core::pipeline::item::Item;

use crate::core::pipeline::items::array::has_length::HasLengthModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn has_length(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(HasLengthModifier::new(value.clone()))
}
