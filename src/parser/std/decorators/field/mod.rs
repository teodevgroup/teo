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
use crate::parser::ast::accessible::Accessible;
use crate::parser::std::decorators::field::atomic::{atomic_decorator};
use crate::parser::std::decorators::field::db::db_container;
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
    objects: HashMap<String, Accessible>
}

impl GlobalFieldDecorators {
    pub(crate) fn new(database_name: DatabaseName) -> GlobalFieldDecorators {
        let mut objects: HashMap<String, Accessible> = HashMap::new();
        objects.insert("db".to_owned(), Accessible::Container(db_container(database_name)));
        objects.insert("id".to_owned(), Accessible::FieldDecorator(id_decorator));
        objects.insert("internal".to_owned(), Accessible::FieldDecorator(internal_decorator));
        objects.insert("readonly".to_owned(), Accessible::FieldDecorator(readonly_decorator));
        objects.insert("writeonly".to_owned(), Accessible::FieldDecorator(writeonly_decorator));
        objects.insert("readwrite".to_owned(), Accessible::FieldDecorator(readwrite_decorator));
        objects.insert("writeNonnull".to_owned(), Accessible::FieldDecorator(write_nonnull_decorator));
        objects.insert("writeOnCreate".to_owned(), Accessible::FieldDecorator(write_on_create_decorator));
        objects.insert("writeOnce".to_owned(), Accessible::FieldDecorator(write_once_decorator));
        objects.insert("readIf".to_owned(), Accessible::FieldDecorator(read_if_decorator));
        objects.insert("writeIf".to_owned(), Accessible::FieldDecorator(write_if_decorator));
        objects.insert("atomic".to_owned(), Accessible::FieldDecorator(atomic_decorator));
        objects.insert("nonatomic".to_owned(), Accessible::FieldDecorator(nonatomic_decorator));
        Self { objects }
    }
}

impl GlobalFieldDecorators {

    fn get(&self, key: &str) -> &Accessible {
        match self.objects.get(key) {
            Some(o) => o,
            None => panic!("Object with key '{}' is not found.", key),
        }
    }
}
