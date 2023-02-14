use crate::core::property::Property;
use crate::parser::ast::argument::Argument;

pub(crate) fn getter_decorator(args: Vec<Argument>, property: &mut Property) {
    let pipeline = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    property.getter = Some(pipeline.clone());
}
