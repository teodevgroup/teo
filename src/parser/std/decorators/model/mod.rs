pub(crate) mod map;
pub(crate) mod id;
pub(crate) mod index;
pub(crate) mod unique;
pub(crate) mod url;
pub(crate) mod identity;
pub(crate) mod r#virtual;

use std::collections::HashMap;
use crate::parser::ast::accessible::Accessible;
use crate::parser::std::decorators::model::id::id_decorator;
use crate::parser::std::decorators::model::identity::identity_decorator;
use crate::parser::std::decorators::model::index::index_decorator;
use crate::parser::std::decorators::model::map::map_decorator;
use crate::parser::std::decorators::model::r#virtual::virtual_decorator;
use crate::parser::std::decorators::model::unique::unique_decorator;
use crate::parser::std::decorators::model::url::url_decorator;

pub(crate) struct GlobalModelDecorators {
    objects: HashMap<String, Accessible>
}

impl GlobalModelDecorators {

    pub(crate) fn new() -> Self {
        let mut objects: HashMap<String, Accessible> = HashMap::new();
        objects.insert("map".to_owned(), Accessible::ModelDecorator(map_decorator));
        objects.insert("url".to_owned(), Accessible::ModelDecorator(url_decorator));
        objects.insert("identity".to_owned(), Accessible::ModelDecorator(identity_decorator));
        objects.insert("id".to_owned(), Accessible::ModelDecorator(id_decorator));
        objects.insert("unique".to_owned(), Accessible::ModelDecorator(unique_decorator));
        objects.insert("index".to_owned(), Accessible::ModelDecorator(index_decorator));
        objects.insert("virtual".to_owned(), Accessible::ModelDecorator(virtual_decorator));
        Self { objects }
    }

    fn get(&self, key: &str) -> &Accessible {
        match self.objects.get(key) {
            Some(o) => o,
            None => panic!("Object with key '{}' is not found.", key),
        }
    }
}
