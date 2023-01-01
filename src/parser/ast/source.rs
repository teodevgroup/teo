use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::import::Import;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;
use crate::parser::ast::top::Top;

pub(crate) struct Source {
    pub(crate) id: usize,
    pub(crate) path: PathBuf,
    pub(crate) tops: BTreeMap<usize, Rc<RefCell<Top>>>,
    pub(crate) imports: BTreeSet<usize>,
    pub(crate) constants: BTreeSet<usize>,
    pub(crate) enums: BTreeSet<usize>,
    pub(crate) models: BTreeSet<usize>,
    pub(crate) resolved: AtomicBool,
}

pub(crate) struct SourceImportIter<'a> {
    source: &'a Source,
    index: usize,
}

impl<'a> Iterator for SourceImportIter<'a> {
    type Item = &'a Import;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.imports.get(&self.index) {
            Some(index) => unsafe {
                let borrow = self.source.tops.get(index).unwrap().as_ref().borrow();
                let a = borrow.deref().as_import().unwrap();
                let b: * const Import = a;
                self.index += 1;
                Some(&*b)
            }
            None => None,
        }

    }
}

impl Source {

    pub(crate) fn new(source_id: usize, path: PathBuf, tops: BTreeMap<usize, Rc<RefCell<Top>>>, imports: BTreeSet<usize>, constants: BTreeSet<usize>, enums: BTreeSet<usize>, models: BTreeSet<usize>) -> Self {
        Self {
            id: source_id,
            path,
            tops,
            imports,
            constants,
            enums,
            models,
            resolved: AtomicBool::new(false),
        }
    }

    pub(crate) fn imports(&self) -> SourceImportIter {
        SourceImportIter { source: self, index: 0 }
    }

    pub(crate) fn get_import(&self, id: &usize) -> &Import {
        unsafe {
            let borrow = self.tops.get(id).unwrap().as_ref().borrow();
            let a = borrow.as_import().unwrap();
            let b: * const Import = a;
            &*b
        }
    }

    pub(crate) fn get_constant(&self, id: &usize) -> &Constant {
        unsafe {
            let borrow = self.tops.get(id).unwrap().as_ref().borrow();
            let a = borrow.as_constant().unwrap();
            let b: * const Constant = a;
            &*b
        }
    }

    // pub(crate) fn get_constant_mut(&mut self, id: usize) -> &mut Constant {
    //     self.tops.get_mut(&id).unwrap().as_ref().borrow_mut().as_constant_mut().unwrap()
    // }

    pub(crate) fn get_enum(&self, id: usize) -> &Enum {
        unsafe {
            let borrow = self.tops.get(&id).unwrap().as_ref().borrow();
            let a = borrow.as_enum().unwrap();
            let b: * const Enum = a;
            &*b
        }
    }

    // pub(crate) fn get_enum_mut(&mut self, id: usize) -> &mut Enum {
    //     self.tops.get_mut(&id).unwrap().as_ref().borrow_mut().as_enum_mut().unwrap()
    // }

    pub(crate) fn get_model(&self, id: &usize) -> &Model {
        unsafe {
            let borrow = self.tops.get(&id).unwrap().as_ref().borrow();
            let a = borrow.as_model().unwrap();
            let b: * const Model = a;
            &*b
        }
    }

    // pub(crate) fn get_model_mut(&mut self, id: usize) -> &mut Model {
    //     self.tops.get_mut(&id).unwrap().as_ref().borrow_mut().as_model_mut().unwrap()
    // }
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
