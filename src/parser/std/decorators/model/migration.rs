use crate::core::model::builder::ModelBuilder;
use crate::core::model::migration::ModelMigration;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn migration_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    let arg = args.get(0).unwrap();
    if arg.name.is_none() {
        panic!("@migration requires argument name.");
    }
    if arg.name.as_ref().unwrap().name.as_str() != "renamed" {
        panic!("Unknown argument name: {}", arg.name.as_ref().unwrap().name.as_str());
    }
    let value = arg.resolved.as_ref().unwrap().as_value().unwrap();
    match value {
        Value::String(s) => {
            model.migration = Some(ModelMigration { renamed: vec![s.clone()] });
        }
        Value::Vec(v) => {
            model.migration = Some(ModelMigration { renamed: v.iter().map(|v| v.as_str().unwrap().to_owned()).collect() });
        }
        _ => unreachable!()
    }
}
