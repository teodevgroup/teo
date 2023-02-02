// tson_set
// tson_get
// tson_set_default

use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::teon::tson_get::TsonGetModifier;
use crate::core::pipeline::modifiers::teon::tson_set::TsonSetModifier;
use crate::core::pipeline::modifiers::teon::tson_set_default::TsonSetDefaultModifier;
use crate::parser::ast::argument::Argument;

// pub(crate) fn tson_set(args: Vec<Argument>) -> Arc<dyn Modifier> {
//     let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
//     Arc::new(TsonSetModifier::new(value))
// }
//
// pub(crate) fn tson_get(args: Vec<Argument>) -> Arc<dyn Modifier> {
//     let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
//     Arc::new(TsonSetDefaultModifier::new(value))
// }
//
// pub(crate) fn tson_get(args: Vec<Argument>) -> Arc<dyn Modifier> {
//     let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
//     Arc::new(TsonGetModifier::new(value))
// }
