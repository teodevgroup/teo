use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::math::abs::AbsItem;
use crate::core::items::math::add::AddItem;
use crate::core::items::math::cbrt::CbrtItem;
use crate::core::items::math::ceil::CeilItem;
use crate::core::items::math::divide::DivideItem;
use crate::core::items::math::floor::FloorItem;
use crate::core::items::math::max::MaxItem;
use crate::core::items::math::min::MinItem;
use crate::core::items::math::modular::ModularItem;
use crate::core::items::math::multiply::MultiplyItem;
use crate::core::items::math::pow::PowItem;
use crate::core::items::math::root::RootItem;
use crate::core::items::math::round::RoundItem;
use crate::core::items::math::sqrt::SqrtItem;
use crate::core::items::math::subtract::SubtractItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn abs(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(AbsItem::new())
}

pub(crate) fn add(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(AddItem::new(value.clone()))
}

pub(crate) fn subtract(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(SubtractItem::new(value))
}

pub(crate) fn divide(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(DivideItem::new(value))
}

pub(crate) fn multiply(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(MultiplyItem::new(value))
}

pub(crate) fn modular(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(ModularItem::new(value))
}

pub(crate) fn ceil(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(CeilItem::new())
}

pub(crate) fn floor(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(FloorItem::new())
}

pub(crate) fn round(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(RoundItem::new())
}

pub(crate) fn min(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( MinItem::new(value))
}

pub(crate) fn max(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( MaxItem::new(value))
}

pub(crate) fn pow(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( PowItem::new(value))
}

pub(crate) fn root(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new( RootItem::new(value))
}

pub(crate) fn sqrt(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new( SqrtItem::new())
}

pub(crate) fn cbrt(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new( CbrtItem::new())
}
