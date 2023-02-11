use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::string::validation::has_prefix::HasPrefixItem;
use crate::core::pipeline::items::string::validation::has_suffix::HasSuffixItem;
use crate::core::pipeline::items::string::validation::is_alphabetic::IsAlphabeticItem;
use crate::core::pipeline::items::string::validation::is_alphanumeric::IsAlphanumericItem;
use crate::core::pipeline::items::string::validation::is_email::IsEmailItem;
use crate::core::pipeline::items::string::validation::is_hex_color::IsHexColorItem;
use crate::core::pipeline::items::string::validation::is_numeric::IsNumericItem;
use crate::core::pipeline::items::string::validation::is_prefix_of::IsPrefixOfItem;
use crate::core::pipeline::items::string::validation::is_secure_password::IsSecurePasswordItem;
use crate::core::pipeline::items::string::validation::is_suffix_of::IsSuffixOfItem;
use crate::core::pipeline::items::string::validation::regex_match::RegexMatchItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn is_email(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsEmailItem::new())
}

pub(crate) fn is_alphabetic(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsAlphabeticItem::new())
}

pub(crate) fn is_numeric(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsNumericItem::new())
}

pub(crate) fn is_alphanumeric(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsAlphanumericItem::new())
}

pub(crate) fn is_secure_password(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsSecurePasswordItem::new())
}

pub(crate) fn regex_match(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(RegexMatchItem::new(value))
}

pub(crate) fn has_prefix(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(HasPrefixItem::new(value))
}

pub(crate) fn has_suffix(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(HasSuffixItem::new(value))
}

pub(crate) fn is_prefix_of(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(IsPrefixOfItem::new(value))
}

pub(crate) fn is_suffix_of(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(IsSuffixOfItem::new(value))
}

pub(crate) fn is_hex_color(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsHexColorItem::new())
}
