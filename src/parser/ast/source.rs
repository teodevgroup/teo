use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::parser::ast::constant::Constant;
use crate::parser::ast::import::Import;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;
use crate::parser::ast::top::Top;

pub(crate) struct Source {
    pub(crate) id: usize,
    pub(crate) path: PathBuf,
    pub(crate) tops: BTreeMap<usize, Top>,
    pub(crate) imports: BTreeSet<usize>,
    pub(crate) constants: BTreeSet<usize>,
    pub(crate) enums: BTreeSet<usize>,
    pub(crate) models: BTreeSet<usize>,
    pub(crate) resolved: bool,
}

impl Source {

    pub(crate) fn new(source_id: usize, path: PathBuf, tops: BTreeMap<usize, Top>, imports: BTreeSet<usize>, constants: BTreeSet<usize>, enums: BTreeSet<usize>, models: BTreeSet<usize>) -> Self {
        Self {
            id: source_id,
            path,
            tops,
            imports,
            constants,
            enums,
            models,
            resolved: false,
        }
    }

    pub(crate) fn imports(&self) -> Vec<&Import> {
        self.imports.iter().map(|id| {
            self.tops.get(id).unwrap().as_import().unwrap()
        }).collect::<Vec<&Import>>()
    }

    pub(crate) fn get_import(&self, id: &usize) -> &Import {
        self.tops.get(id).unwrap().as_import().unwrap()
    }

    pub(crate) fn get_constant(&self, id: &usize) -> &Constant {
        self.tops.get(id).unwrap().as_constant().unwrap()
    }

    pub(crate) fn get_constant_mut(&mut self, id: usize) -> &mut Constant {
        self.tops.get_mut(&id).unwrap().as_constant_mut().unwrap()
    }

    pub(crate) fn get_enum(&self, id: usize) -> &Enum {
        self.tops.get(&id).unwrap().as_enum().unwrap()
    }

    pub(crate) fn get_enum_mut(&mut self, id: usize) -> &mut Enum {
        self.tops.get_mut(&id).unwrap().as_enum_mut().unwrap()
    }

    pub(crate) fn get_model(&self, id: &usize) -> &Model {
        self.tops.get(id).unwrap().as_model().unwrap()
    }

    pub(crate) fn get_model_mut(&mut self, id: usize) -> &mut Model {
        self.tops.get_mut(&id).unwrap().as_model_mut().unwrap()
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
