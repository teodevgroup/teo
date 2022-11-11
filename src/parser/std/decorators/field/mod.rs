pub(crate) mod id;
pub(crate) mod db;
pub(crate) mod readonly;
pub(crate) mod writeonly;
pub(crate) mod readwrite;
pub(crate) mod internal;
pub(crate) mod write_on_create;
pub(crate) mod write_once;
pub(crate) mod write_nonnull;
pub(crate) mod read_if;
pub(crate) mod write_if;
pub(crate) mod atomic;
pub(crate) mod nonatomic;

use std::collections::HashMap;
use std::sync::Arc;
use crate::core::database::name::DatabaseName;
use crate::core::field::Field;
use crate::core::model::Model;
use crate::core::property::Property;
use crate::core::relation::Relation;
use crate::parser::ast::argument::Argument;
use crate::parser::ast::object::Object;
use crate::parser::std::decorators::field::atomic::{atomic_decorator};
use crate::parser::std::decorators::field::id::{id_decorator};
use crate::parser::std::decorators::field::internal::{internal_decorator};
use crate::parser::std::decorators::field::nonatomic::{nonatomic_decorator};
use crate::parser::std::decorators::field::read_if::read_if_decorator;
use crate::parser::std::decorators::field::readonly::{readonly_decorator};
use crate::parser::std::decorators::field::readwrite::{readwrite_decorator};
use crate::parser::std::decorators::field::write_if::write_if_decorator;
use crate::parser::std::decorators::field::write_nonnull::write_nonnull_decorator;
use crate::parser::std::decorators::field::write_on_create::write_on_create_decorator;
use crate::parser::std::decorators::field::write_once::write_once_decorator;
use crate::parser::std::decorators::field::writeonly::{writeonly_decorator};
use crate::prelude::Value;

pub(crate) struct GlobalFieldDecorators {
    objects: HashMap<String, Object>
}

impl GlobalFieldDecorators {
    pub(crate) fn new(database_name: DatabaseName) -> GlobalFieldDecorators {
        let mut objects: HashMap<String, Object> = HashMap::new();
        objects.insert("id".to_owned(), Object::FieldDecorator(id_decorator));
        objects.insert("internal".to_owned(), Object::FieldDecorator(internal_decorator));
        objects.insert("readonly".to_owned(), Object::FieldDecorator(readonly_decorator));
        objects.insert("writeonly".to_owned(), Object::FieldDecorator(writeonly_decorator));
        objects.insert("readwrite".to_owned(), Object::FieldDecorator(readwrite_decorator));
        objects.insert("writeNonnull".to_owned(), Object::FieldDecorator(write_nonnull_decorator));
        objects.insert("writeOnCreate".to_owned(), Object::FieldDecorator(write_on_create_decorator));
        objects.insert("writeOnce".to_owned(), Object::FieldDecorator(write_once_decorator));
        objects.insert("readIf".to_owned(), Object::FieldDecorator(read_if_decorator));
        objects.insert("writeIf".to_owned(), Object::FieldDecorator(write_if_decorator));
        objects.insert("atomic".to_owned(), Object::FieldDecorator(atomic_decorator));
        objects.insert("nonatomic".to_owned(), Object::FieldDecorator(nonatomic_decorator));
        Self { objects }
    }
}

impl GlobalFieldDecorators {

    fn get(&self, key: &str) -> &Object {
        match self.objects.get(key) {
            Some(o) => o.clone(),
            None => panic!("Object with key '{}' is not found.", key),
        }
    }
}
