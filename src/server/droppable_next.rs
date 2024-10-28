use teo_runtime::middleware::next::Next;

pub(super) struct DroppableNext {
    next: * mut dyn Next,
}

impl DroppableNext {

    pub fn new<T>(next: T) -> Self where T: Next + 'static {
        Self {
            next: Box::into_raw(Box::new(next))
        }
    }

    pub fn get_next(&self) -> &'static dyn Next {
        unsafe { &*(self.next as * const dyn Next) }
    }
}

impl Drop for DroppableNext {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.next) };
    }
}

unsafe impl Send for DroppableNext { }
unsafe impl Sync for DroppableNext { }