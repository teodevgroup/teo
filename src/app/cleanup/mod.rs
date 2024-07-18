use crate::app::App;

pub trait Cleanup {
    fn call(&self, app: &App) -> ();
}

impl<F> Cleanup for F where F: Fn(&App) -> () {
    fn call(&self, app: &App) -> () {
        self(app)
    }
}