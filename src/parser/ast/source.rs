use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::parser::ast::constant::Constant;
use crate::parser::ast::top::Top;

#[derive(Clone)]
pub(crate) struct Source {
    pub(crate) id: usize,
    pub(crate) path: PathBuf,
    pub(crate) tops: Vec<Arc<Mutex<Top>>>,
    pub(crate) imports: BTreeMap<usize, Arc<Mutex<Top>>>,
    pub(crate) constants: BTreeMap<usize, Arc<Mutex<Top>>>,
    pub(crate) models: BTreeMap<usize, Arc<Mutex<Top>>>,
}

impl Source {
    pub(crate) fn get_constant_with_reference(&self, id: usize) -> &Arc<Mutex<Top>> {
        self.constants.get(&id).unwrap()
    }

    pub(crate) fn get_model_with_reference(&self, id: usize) -> &Arc<Mutex<Top>> {
        self.models.get(&id).unwrap()
    }
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
