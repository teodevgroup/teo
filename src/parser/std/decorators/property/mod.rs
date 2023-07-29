pub(crate) mod setter;
pub(crate) mod getter;
pub(crate) mod cached;
pub(crate) mod deps;
pub(crate) mod index;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::parser::ast::accessible::Accessible;
use crate::parser::std::decorators::property::cached::cached_decorator;
use crate::parser::std::decorators::property::deps::deps_decorator;
use crate::parser::std::decorators::property::getter::getter_decorator;
use crate::parser::std::decorators::property::index::{index_decorator, unique_decorator};
use crate::parser::std::decorators::property::setter::setter_decorator;


pub(crate) struct GlobalPropertyDecorators {
    objects: HashMap<String, Accessible>
}

impl Debug for GlobalPropertyDecorators {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlobalPropertyDecorators")
    }
}

impl GlobalPropertyDecorators {

    pub(crate) fn new() -> Self {
        let mut objects: HashMap<String, Accessible> = HashMap::new();
        objects.insert("setter".to_owned(), Accessible::PropertyDecorator(setter_decorator));
        objects.insert("getter".to_owned(), Accessible::PropertyDecorator(getter_decorator));
        objects.insert("cached".to_owned(), Accessible::PropertyDecorator(cached_decorator));
        objects.insert("dependencies".to_owned(), Accessible::PropertyDecorator(deps_decorator));
        objects.insert("index".to_owned(), Accessible::PropertyDecorator(index_decorator));
        objects.insert("unique".to_owned(), Accessible::PropertyDecorator(unique_decorator));
        Self { objects }
    }

    pub(crate) fn get(&self, key: &str) -> &Accessible {
        match self.objects.get(key) {
            Some(o) => o,
            None => panic!("Object with key '{}' is not found.", key),
        }
    }
}
