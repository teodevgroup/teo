use std::sync::Arc;
use crate::core::builders::ModelBuilder;
use crate::core::graph::{Graph, GraphInner};
use crate::core::field::Field;
use crate::core::field::ReadRule::NoRead;
use crate::core::field::Store::{Calculated, Temp};
use crate::core::field::WriteRule::NoWrite;


#[derive(Debug)]
pub struct Model {
    pub(crate) name: &'static str,
    pub(crate) localized_name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) identity: bool,
    pub(crate) fields: Vec<Field>,
    pub(crate) graph: *const GraphInner,
    pub(crate) input_keys: Vec<&'static str>,
    pub(crate) save_keys: Vec<&'static str>,
    pub(crate) output_keys: Vec<&'static str>,
    pub(crate) all_getable_keys: Vec<&'static str>,
}

impl Model {

    pub(crate) fn new(builder: &ModelBuilder, graph_ptr: *const GraphInner) -> Self {
        let input_keys = Self::allowed_input_keys(builder);
        let save_keys = Self::allowed_save_keys(builder);
        let output_keys = Self::allowed_output_keys(builder);
        let all_getable_keys = Self::all_getable_keys(builder);
        return Model {
            name: builder.name,
            localized_name: builder.localized_name,
            description: builder.description,
            identity: builder.identity,
            fields: builder.fields.iter().map(|fb| { Field::new(fb) }).collect(),
            graph: graph_ptr,
            input_keys,
            save_keys,
            output_keys,
            all_getable_keys
        }
    }

    fn allowed_input_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.write_rule != NoWrite })
            .map(|f| { f.name })
            .collect()
    }

    fn allowed_save_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.store != Calculated && f.store != Temp })
            .map(|f| { f.name })
            .collect()
    }

    fn allowed_output_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.read_rule != NoRead })
            .map(|f| { f.name })
            .collect()
    }

    fn all_getable_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .map(|f| { f.name })
            .collect()
    }

    pub(crate) fn graph(&self) -> &GraphInner {
        unsafe {
            &(*self.graph)
        }
    }

    pub fn field(&self, name: &str) -> &Field {
        self.fields.iter().find(|f| { f.name == name}).unwrap()
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}
