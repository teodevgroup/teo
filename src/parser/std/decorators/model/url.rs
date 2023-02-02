use crate::core::model::builder::ModelBuilder;

use crate::parser::ast::argument::Argument;

pub(crate) fn url_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    model.url_segment_name(args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap());
}
