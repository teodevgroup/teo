use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::parser::ast::top::Top;

#[derive(Clone)]
pub(crate) struct Source {
    pub(crate) id: usize,
    pub(crate) path: PathBuf,
    pub(crate) tops: Vec<Arc<Mutex<Top>>>,
    pub(crate) imports: Vec<Arc<Mutex<Top>>>,
    pub(crate) constants: Vec<Arc<Mutex<Top>>>,
}

impl fmt::Debug for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source(\"{}\")", self.path.to_str().unwrap())
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}
