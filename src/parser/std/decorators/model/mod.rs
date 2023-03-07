pub(crate) mod map;
pub(crate) mod id;
pub(crate) mod index;
pub(crate) mod unique;
pub(crate) mod url;
pub(crate) mod identity;
pub(crate) mod r#virtual;
pub(crate) mod before_save;
pub(crate) mod after_save;
pub(crate) mod before_delete;
pub(crate) mod after_delete;
pub(crate) mod can_read;
pub(crate) mod can_mutate;
pub(crate) mod disable;
pub(crate) mod action;
pub(crate) mod migration;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::parser::ast::accessible::Accessible;
use crate::parser::std::decorators::model::action::action_decorator;
use crate::parser::std::decorators::model::after_delete::after_delete_decorator;
use crate::parser::std::decorators::model::after_save::after_save_decorator;
use crate::parser::std::decorators::model::before_delete::before_delete_decorator;
use crate::parser::std::decorators::model::before_save::before_save_decorator;
use crate::parser::std::decorators::model::can_mutate::can_mutate_decorator;
use crate::parser::std::decorators::model::can_read::can_read_decorator;
use crate::parser::std::decorators::model::disable::disable_decorator;
use crate::parser::std::decorators::model::id::id_decorator;
use crate::parser::std::decorators::model::identity::identity_decorator;
use crate::parser::std::decorators::model::index::index_decorator;
use crate::parser::std::decorators::model::map::map_decorator;
use crate::parser::std::decorators::model::migration::migration_decorator;
use crate::parser::std::decorators::model::r#virtual::virtual_decorator;
use crate::parser::std::decorators::model::unique::unique_decorator;
use crate::parser::std::decorators::model::url::url_decorator;

pub(crate) struct GlobalModelDecorators {
    objects: HashMap<String, Accessible>
}

impl Debug for GlobalModelDecorators {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlobalModelDecorators")
    }
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
        objects.insert("beforeSave".to_owned(), Accessible::ModelDecorator(before_save_decorator));
        objects.insert("afterSave".to_owned(), Accessible::ModelDecorator(after_save_decorator));
        objects.insert("beforeDelete".to_owned(), Accessible::ModelDecorator(before_delete_decorator));
        objects.insert("afterDelete".to_owned(), Accessible::ModelDecorator(after_delete_decorator));
        objects.insert("disable".to_owned(), Accessible::ModelDecorator(disable_decorator));
        objects.insert("action".to_owned(), Accessible::ModelDecorator(action_decorator));
        objects.insert("canRead".to_owned(), Accessible::ModelDecorator(can_read_decorator));
        objects.insert("canMutate".to_owned(), Accessible::ModelDecorator(can_mutate_decorator));
        objects.insert("migration".to_owned(), Accessible::ModelDecorator(migration_decorator));
        Self { objects }
    }

    pub(crate) fn get(&self, key: &str) -> &Accessible {
        match self.objects.get(key) {
            Some(o) => o,
            None => panic!("Object with key '{}' is not found.", key),
        }
    }
}
