use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::value::eq::EqModifier;
use crate::core::pipeline::modifiers::value::gt::GtModifier;
use crate::core::pipeline::modifiers::value::gte::GteModifier;
use crate::core::pipeline::modifiers::value::exists::ExistsModifier;
use crate::core::pipeline::modifiers::value::is_false::IsFalseModifier;
use crate::core::pipeline::modifiers::value::is_null::IsNullModifier;
use crate::core::pipeline::modifiers::value::is_true::IsTrueModifier;
use crate::core::pipeline::modifiers::value::lt::LtModifier;
use crate::core::pipeline::modifiers::value::lte::LteModifier;
use crate::core::pipeline::modifiers::value::neq::NeqModifier;
use crate::core::pipeline::modifiers::value::one_of::OneOfModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn eq(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(EqModifier::new(value))
}

pub(crate) fn gt(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(GtModifier::new(value))
}

pub(crate) fn gte(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(GteModifier::new(value))
}

pub(crate) fn exists(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(ExistsModifier::new())
}

pub(crate) fn is_false(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(IsFalseModifier::new())
}

pub(crate) fn is_null(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(IsNullModifier::new())
}

pub(crate) fn is_true(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(IsTrueModifier::new())
}

pub(crate) fn lt(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(LtModifier::new(value))
}

pub(crate) fn lte(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(LteModifier::new(value))
}

pub(crate) fn neq(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(NeqModifier::new(value))
}

pub(crate) fn one_of(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(OneOfModifier::new(value))
}
