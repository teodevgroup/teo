use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::string::generation::cuid2::CUID2Item;
use crate::core::items::string::generation::cuid::CUIDItem;
use crate::core::items::string::generation::random_digits::RandomDigitsItem;
use crate::core::items::string::generation::slug::SlugItem;
use crate::core::items::string::generation::uuid::UUIDItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn cuid(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(CUIDItem::new())
}

pub(crate) fn cuid2(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(CUID2Item::new())
}

pub(crate) fn random_digits(args: &Vec<Argument>) -> Arc<dyn Item> {
    let arg = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(RandomDigitsItem::new(arg))
}

pub(crate) fn slug(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(SlugItem::new())
}

pub(crate) fn uuid(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(UUIDItem::new())
}
