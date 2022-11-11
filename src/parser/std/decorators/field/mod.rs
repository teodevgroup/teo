pub(crate) mod id;
pub(crate) mod db;
pub(crate) mod readonly;
pub(crate) mod writeonly;
pub(crate) mod readwrite;
pub(crate) mod internal;
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
use crate::parser::std::decorators::field::atomic::{atomic_decorator, AtomicDecorator};
use crate::parser::std::decorators::field::id::{id_decorator, IdDecorator};
use crate::parser::std::decorators::field::internal::{internal_decorator, InternalDecorator};
use crate::parser::std::decorators::field::nonatomic::{nonatomic_decorator, NonatomicDecorator};
use crate::parser::std::decorators::field::readonly::{readonly_decorator, ReadonlyDecorator};
use crate::parser::std::decorators::field::readwrite::{readwrite_decorator, ReadWriteDecorator};
use crate::parser::std::decorators::field::writeonly::{writeonly_decorator, WriteonlyDecorator};
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
