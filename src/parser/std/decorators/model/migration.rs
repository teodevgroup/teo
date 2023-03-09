use crate::core::model::builder::ModelBuilder;
use crate::core::model::migration::ModelMigration;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn migration_decorator(args: Vec<Argument>, model: &mut ModelBuilder) {
    let mut migration = ModelMigration { renamed: vec![], version: None };
    for arg in args {
        if arg.name.is_none() {
            panic!("@migration requires argument name.");
        }
        if (arg.name.as_ref().unwrap().name.as_str() != "renamed") || (arg.name.as_ref().unwrap().name.as_str() != "version") {
            panic!("Unknown argument name: {}", arg.name.as_ref().unwrap().name.as_str());
        }
        match arg.name.as_ref().unwrap().name.as_str() {
            "renamed" => {
                let value = arg.resolved.as_ref().unwrap().as_value().unwrap();
                match value {
                    Value::String(s) => {
                        migration.renamed = vec![s.clone()];
                    }
                    Value::Vec(v) => {
                        migration.renamed = v.iter().map(|v| v.as_str().unwrap().to_owned()).collect();
                    }
                    _ => unreachable!()
                }
            }
            "version" => {
                let value = arg.resolved.as_ref().unwrap().as_value().unwrap();
                let str = value.as_str().unwrap();
                migration.version = Some(str.to_owned());
            }
            _ => unreachable!()
        }

    }
    model.migration = Some(migration);
}
