use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::string::transform::ellipsis::EllipsisModifier;
use crate::core::pipeline::items::string::transform::pad_end::PadEndModifier;
use crate::core::pipeline::items::string::transform::pad_start::PadStartModifier;
use crate::core::pipeline::items::string::transform::regex_replace::RegexReplaceModifier;
use crate::core::pipeline::items::string::transform::split::SplitModifier;
use crate::core::pipeline::items::string::transform::trim::TrimModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn regex_replace(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(RegexReplaceModifier::new(arg1, arg2))
}

pub(crate) fn trim(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(TrimModifier::new())
}

pub(crate) fn split(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(SplitModifier::new(arg))
}

pub(crate) fn pad_start(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(PadStartModifier::new(arg1.as_str().unwrap().chars().nth(0).unwrap(), arg2))
}

pub(crate) fn pad_end(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(PadEndModifier::new(arg1.as_str().unwrap().chars().nth(0).unwrap(), arg2))
}

pub(crate) fn ellipsis(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(EllipsisModifier::new(arg1.clone(), arg2))
}
