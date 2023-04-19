use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::value::eq::EqItem;
use crate::core::items::value::gt::GtItem;
use crate::core::items::value::gte::GteItem;
use crate::core::items::value::exists::ExistsItem;
use crate::core::items::value::is_false::IsFalseItem;
use crate::core::items::value::is_null::IsNullItem;
use crate::core::items::value::is_true::IsTrueItem;
use crate::core::items::value::lt::LtItem;
use crate::core::items::value::lte::LteItem;
use crate::core::items::value::neq::NeqItem;
use crate::core::items::value::one_of::OneOfItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn eq(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(EqItem::new(value))
}

pub(crate) fn gt(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(GtItem::new(value))
}

pub(crate) fn gte(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(GteItem::new(value))
}

pub(crate) fn exists(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ExistsItem::new())
}

pub(crate) fn is_false(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsFalseItem::new())
}

pub(crate) fn is_null(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsNullItem::new())
}

pub(crate) fn is_true(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsTrueItem::new())
}

pub(crate) fn lt(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(LtItem::new(value))
}

pub(crate) fn lte(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(LteItem::new(value))
}

pub(crate) fn neq(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(NeqItem::new(value))
}

pub(crate) fn one_of(args: &Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(OneOfItem::new(value.clone()))
}
