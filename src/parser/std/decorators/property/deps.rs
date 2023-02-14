use crate::core::property::Property;
use crate::parser::ast::argument::Argument;

pub(crate) fn deps_decorator(args: Vec<Argument>, property: &mut Property) {
    let vec = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_vec().unwrap();
    let dependencies = vec.iter().map(|v| v.as_raw_enum_choice().unwrap().to_owned()).collect();
    property.dependencies = dependencies;
}
