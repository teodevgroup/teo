use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::PathBuf;
use to_mut::ToMut;
use to_mut_proc_macro::ToMut;
use crate::parser::ast::client::Client;
use crate::parser::ast::config::ServerConfig;
use crate::parser::ast::connector::Connector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::generator::Generator;
use crate::parser::ast::import::Import;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;
use crate::parser::ast::top::Top;

#[derive(ToMut)]
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

pub(crate) struct SourceImportIter<'a> {
    source: &'a Source,
    index: usize,
}

impl<'a> Iterator for SourceImportIter<'a> {
    type Item = &'a Import;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.imports.iter().skip(self.index).next() {
            Some(index) => {
                let top = self.source.tops.get(index).unwrap();
                self.index += 1;
                Some(top.as_import().unwrap())
            }
            None => None,
        }

    }
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

    pub(crate) fn imports(&self) -> SourceImportIter {
        SourceImportIter { source: self, index: 0 }
    }

    pub(crate) fn get_import(&self, id: usize) -> &Import {
        self.tops.get(&id).unwrap().as_import().unwrap()
    }

    pub(crate) fn get_constant(&self, id: usize) -> &Constant {
        self.tops.get(&id).unwrap().as_constant().unwrap()
    }

    pub(crate) fn get_enum(&self, id: usize) -> &Enum {
        self.tops.get(&id).unwrap().as_enum().unwrap()
    }

    pub(crate) fn get_model(&self, id: usize) -> &Model {
        self.tops.get(&id).unwrap().as_model().unwrap()
    }

    pub(crate) fn get_connector(&self, id: usize) -> &Connector {
        self.tops.get(&id).unwrap().as_connector().unwrap()
    }

    pub(crate) fn get_server_config(&self, id: usize) -> &ServerConfig {
        self.tops.get(&id).unwrap().as_server_config().unwrap()
    }

    pub(crate) fn get_entity(&self, id: usize) -> &Generator {
        self.tops.get(&id).unwrap().as_generator().unwrap()
    }

    pub(crate) fn get_client(&self, id: usize) -> &Client {
        self.tops.get(&id).unwrap().as_client().unwrap()
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
