pub(crate) mod relation;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::parser::ast::accessible::Accessible;
use crate::parser::std::decorators::relation::relation::relation_decorator;

pub(crate) struct GlobalRelationDecorators {
    objects: HashMap<String, Accessible>
}

impl Debug for GlobalRelationDecorators {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlobalRelationDecorators")
    }
}

impl GlobalRelationDecorators {

    pub(crate) fn new() -> Self {
        let mut objects: HashMap<String, Accessible> = HashMap::new();
        objects.insert("relation".to_owned(), Accessible::RelationDecorator(relation_decorator));
        Self { objects }
    }

    pub(crate) fn get(&self, key: &str) -> &Accessible {
        match self.objects.get(key) {
            Some(o) => o,
            None => panic!("Object with key '{}' is not found.", key),
        }
    }
}
