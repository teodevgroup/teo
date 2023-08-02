use std::any::Any;
use std::collections::HashMap;
use std::fmt;

#[derive(Default)]
pub struct ReqLocal {
    /// Use AHasher with a std HashMap with for faster lookups on the small `TypeId` keys.
    map: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl ReqLocal {

    #[inline]
    pub fn new() -> ReqLocal {
        Self {
            map: HashMap::default(),
        }
    }

    pub fn insert<T: 'static + Send + Sync>(&mut self, key: impl Into<String>, val: T) {
        self.map.insert(key.into(), Box::new(val));
    }

    pub fn get<T: 'static + Send>(&self, key: &str) -> Option<&T> {
        self.map.get(key).and_then(|boxed| boxed.downcast_ref())
    }

    pub fn get_mut<T: 'static + Send>(&mut self, key: &str) -> Option<&mut T> {
        self.map.get_mut(key).and_then(|boxed| boxed.downcast_mut())
    }

    pub fn contains<T: 'static + Send>(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn remove<T: 'static + Send>(&mut self, key: &str) -> Option<&T> {
        self.map.remove(key).and_then(|boxed| downcast_owned(boxed))
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }

    pub fn extend(&mut self, other: Self) {
        self.map.extend(other.map);
    }
}

impl fmt::Debug for ReqLocal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReqLocal").finish()
    }
}

fn downcast_owned<T: 'static>(boxed: Box<dyn Any>) -> Option<T> {
    boxed.downcast().ok().map(|boxed| *boxed)
}