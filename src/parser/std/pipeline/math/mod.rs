use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::math::abs::AbsModifier;
use crate::core::pipeline::items::math::add::AddModifier;
use crate::core::pipeline::items::math::cbrt::CbrtModifier;
use crate::core::pipeline::items::math::ceil::CeilModifier;
use crate::core::pipeline::items::math::divide::DivideModifier;
use crate::core::pipeline::items::math::floor::FloorModifier;
use crate::core::pipeline::items::math::max::MaxModifier;
use crate::core::pipeline::items::math::min::MinModifier;
use crate::core::pipeline::items::math::modular::ModularModifier;
use crate::core::pipeline::items::math::multiply::MultiplyModifier;
use crate::core::pipeline::items::math::pow::PowModifier;
use crate::core::pipeline::items::math::root::RootModifier;
use crate::core::pipeline::items::math::round::RoundModifier;
use crate::core::pipeline::items::math::sqrt::SqrtModifier;
use crate::core::pipeline::items::math::subtract::SubtractModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn abs(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(AbsModifier::new())
}

pub(crate) fn add(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(AddModifier::new(value))
}

pub(crate) fn subtract(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(SubtractModifier::new(value))
}

pub(crate) fn divide(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(DivideModifier::new(value))
}

pub(crate) fn multiply(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(MultiplyModifier::new(value))
}

pub(crate) fn modular(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(ModularModifier::new(value))
}

pub(crate) fn ceil(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(CeilModifier::new())
}

pub(crate) fn floor(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(FloorModifier::new())
}

pub(crate) fn round(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(RoundModifier::new())
}

pub(crate) fn min(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( MinModifier::new(value))
}

pub(crate) fn max(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( MaxModifier::new(value))
}

pub(crate) fn pow(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( PowModifier::new(value))
}

pub(crate) fn root(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( RootModifier::new(value))
}

pub(crate) fn sqrt(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new( SqrtModifier::new())
}

pub(crate) fn cbrt(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new( CbrtModifier::new())
}
