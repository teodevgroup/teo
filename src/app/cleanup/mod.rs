use crate::app::App;

pub trait Cleanup {
    fn call(&self, app: &App) -> ();
}
