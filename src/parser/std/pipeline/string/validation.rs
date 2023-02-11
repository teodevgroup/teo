use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::string::validation::has_prefix::HasPrefixModifier;
use crate::core::pipeline::items::string::validation::has_suffix::HasSuffixModifier;
use crate::core::pipeline::items::string::validation::is_alphabetic::IsAlphabeticModifier;
use crate::core::pipeline::items::string::validation::is_alphanumeric::IsAlphanumericModifier;
use crate::core::pipeline::items::string::validation::is_email::IsEmailModifier;
use crate::core::pipeline::items::string::validation::is_hex_color::IsHexColorModifier;
use crate::core::pipeline::items::string::validation::is_numeric::IsNumericModifier;
use crate::core::pipeline::items::string::validation::is_prefix_of::IsPrefixOfModifier;
use crate::core::pipeline::items::string::validation::is_secure_password::IsSecurePasswordModifier;
use crate::core::pipeline::items::string::validation::is_suffix_of::IsSuffixOfModifier;
use crate::core::pipeline::items::string::validation::regex_match::RegexMatchModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn is_email(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsEmailModifier::new())
}

pub(crate) fn is_alphabetic(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsAlphabeticModifier::new())
}

pub(crate) fn is_numeric(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsNumericModifier::new())
}

pub(crate) fn is_alphanumeric(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsAlphanumericModifier::new())
}

pub(crate) fn is_secure_password(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsSecurePasswordModifier::new())
}

pub(crate) fn regex_match(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(RegexMatchModifier::new(value))
}

pub(crate) fn has_prefix(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(HasPrefixModifier::new(value))
}

pub(crate) fn has_suffix(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(HasSuffixModifier::new(value))
}

pub(crate) fn is_prefix_of(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(IsPrefixOfModifier::new(value))
}

pub(crate) fn is_suffix_of(args: Vec<Argument>) -> Arc<dyn Item> {
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(IsSuffixOfModifier::new(value))
}

pub(crate) fn is_hex_color(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(IsHexColorModifier::new())
}
