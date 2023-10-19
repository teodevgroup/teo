use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;

#[derive(Default, Clone)]
pub struct ReqLocal {
    inner: Arc<ReqLocalInner>,
}

#[derive(Default, ToMut)]
struct ReqLocalInner {
    map: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl ReqLocal {

    #[inline]
    pub fn new() -> ReqLocal {
        Self {
            inner: Arc::new(ReqLocalInner {
                map: HashMap::default(),
            })
        }
    }

    pub fn insert<T: 'static + Send + Sync>(&self, key: impl Into<String>, val: T) {
        self.inner.to_mut().map.insert(key.into(), Box::new(val));
    }

    pub fn get<T: 'static + Send>(&self, key: &str) -> Option<&T> {
        self.inner.map.get(key).and_then(|boxed| boxed.downcast_ref())
    }

    pub fn get_mut<T: 'static + Send>(&self, key: &str) -> Option<&mut T> {
        self.inner.to_mut().map.get_mut(key).and_then(|boxed| boxed.downcast_mut())
    }

    pub fn contains<T: 'static + Send>(&self, key: &str) -> bool {
        self.inner.map.contains_key(key)
    }

    pub fn remove<T: 'static + Send>(&self, key: &str) -> Option<&T> {
        self.inner.to_mut().map.remove(key).and_then(|boxed| downcast_owned(boxed))
    }

    pub fn clear(&mut self) {
        self.inner.to_mut().map.clear()
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