use crate::core::model::model::Model;
use crate::core::model::migration::ModelMigration;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

static VALID_NAMES: [&str; 3] = ["renamed", "version", "drop"];

pub(crate) fn migration_decorator(args: Option<&Vec<Argument>>, model: &mut Model) {
    let args = args.unwrap();
    let mut migration = ModelMigration { renamed: vec![], version: None, drop: false };
    for arg in args {
        if arg.name.is_none() {
            panic!("@migration requires argument name.");
        }
        if !VALID_NAMES.contains(&arg.name.as_ref().unwrap().name.as_str()) {
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
            "drop" => {
                let value = arg.resolved.as_ref().unwrap().as_value().unwrap();
                let b = value.as_bool().unwrap();
                migration.drop = b;
            }
            _ => unreachable!()
        }

    }
    model.set_migration(migration);
}
