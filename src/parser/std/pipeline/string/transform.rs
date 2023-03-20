use std::sync::Arc;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::items::string::transform::ellipsis::EllipsisItem;
use crate::core::pipeline::items::string::transform::to_upper_case::ToUpperCaseItem;
use crate::core::pipeline::items::string::transform::to_lower_case::ToLowerCaseItem;
use crate::core::pipeline::items::string::transform::pad_end::PadEndItem;
use crate::core::pipeline::items::string::transform::pad_start::PadStartItem;
use crate::core::pipeline::items::string::transform::regex_replace::RegexReplaceItem;
use crate::core::pipeline::items::string::transform::split::SplitItem;
use crate::core::pipeline::items::string::transform::trim::TrimItem;
use crate::core::pipeline::items::string::transform::to_word_case::ToWordCaseItem;
use crate::core::pipeline::items::string::transform::to_sentence_case::ToSentenceCaseItem;
use crate::core::pipeline::items::string::transform::to_title_case::ToTitleCaseItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn regex_replace(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(RegexReplaceItem::new(arg1, arg2))
}

pub(crate) fn trim(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(TrimItem::new())
}

pub(crate) fn split(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(SplitItem::new(arg))
}

pub(crate) fn pad_start(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(PadStartItem::new(arg1.as_str().unwrap().chars().nth(0).unwrap(), arg2))
}

pub(crate) fn pad_end(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(PadEndItem::new(arg1.as_str().unwrap().chars().nth(0).unwrap(), arg2))
}

pub(crate) fn ellipsis(args: Vec<Argument>) -> Arc<dyn Item> {
    let arg1 = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let arg2 = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    Arc::new(EllipsisItem::new(arg1.clone(), arg2))
}

pub(crate) fn to_word_case(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ToWordCaseItem::new())
}

pub(crate) fn to_lower_case(args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ToLowerCaseItem::new())
}

pub(crate) fn to_upper_case(args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ToUpperCaseItem::new())
}

pub(crate) fn to_sentence_case(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ToSentenceCaseItem::new())
}

pub(crate) fn to_title_case(_args: Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ToTitleCaseItem::new())
}
