use crate::core::field::Field;
use crate::core::field::migration::FieldMigration;
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

static VALID_NAMES: [&str; 5] = ["renamed", "default", "version", "action", "priority"];

pub(crate) fn migration_decorator(args: Vec<Argument>, field: &mut Field) {
    let mut migration = FieldMigration { renamed: vec![], default: None, version: None, action: None, priority: None };
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
            "default" => {
                let value = arg.resolved.as_ref().unwrap().as_value().unwrap();
                migration.default = Some(value.clone());
            }
            "action" => {
                let value = arg.resolved.as_ref().unwrap().as_value().unwrap();
                let p = value.as_pipeline().unwrap();
                migration.action = Some(p.clone());
            }
            "priority" => {
                let value = arg.resolved.as_ref().unwrap().as_value().unwrap();
                let u = value.as_usize().unwrap();
                migration.priority = Some(u);
            }
            _ => unreachable!()
        }
        field.migration = Some(migration.clone());
    }
}
