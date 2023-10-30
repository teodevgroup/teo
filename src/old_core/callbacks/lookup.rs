use std::collections::HashMap;
use std::sync::Arc;
use maplit::hashmap;
use crate::core::item::Item;

#[derive(Debug)]
pub(crate) struct CallbackLookup {
    transforms: HashMap<&'static str, Arc<dyn Item>>,
    validators: HashMap<&'static str, Arc<dyn Item>>,
    callbacks: HashMap<&'static str, Arc<dyn Item>>,
    compares: HashMap<&'static str, Arc<dyn Item>>,
}

impl CallbackLookup {
    pub(crate) fn new() -> Self {
        Self {
            transforms: hashmap!{},
            validators: hashmap!{},
            callbacks: hashmap!{},
            compares: hashmap!{}
        }
    }

    pub(crate) fn add_transform(&mut self, name: &'static str, item: Arc<dyn Item>) {
        self.transforms.insert(name, item);
    }

    pub(crate) fn add_validator(&mut self, name: &'static str, item: Arc<dyn Item>) {
        self.validators.insert(name, item);
    }

    pub(crate) fn add_callback(&mut self, name: &'static str, item: Arc<dyn Item>) {
        self.callbacks.insert(name, item);
    }

    pub(crate) fn add_compare(&mut self, name: &'static str, item: Arc<dyn Item>) {
        self.compares.insert(name, item);
    }

    pub(crate) fn transform(&self, name: &str) -> Option<Arc<dyn Item>> {
        self.transforms.get(name).cloned()
    }

    pub(crate) fn validator(&self, name: &str) -> Option<Arc<dyn Item>> {
        self.validators.get(name).cloned()
    }

    pub(crate) fn callback(&self, name: &str) -> Option<Arc<dyn Item>> {
        self.callbacks.get(name).cloned()
    }

    pub(crate) fn compare(&self, name: &str) -> Option<Arc<dyn Item>> {
        self.compares.get(name).cloned()
    }
}