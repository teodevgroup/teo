use crate::core::builders::ModelBuilder;
use crate::core::graph::Graph;
use crate::core::field::Field;
use crate::core::field::ReadRule::NoRead;
use crate::core::field::Store::{Calculated, Temp};
use crate::core::field::WriteRule::NoWrite;


#[derive(Debug)]
pub struct Model {
    pub name: &'static str,
    pub localized_name: &'static str,
    pub description: &'static str,
    pub identity: bool,
    pub fields: Vec<Field>,
    pub graph: *const Graph,
    pub input_keys: Vec<&'static str>,
    pub save_keys: Vec<&'static str>,
    pub output_keys: Vec<&'static str>,
    pub all_getable_keys: Vec<&'static str>,
}

impl Model {

    pub fn new(builder: &ModelBuilder, graph_ptr: *const Graph) -> Self {
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

    pub(crate) fn graph(&self) -> &Graph {
        unsafe {
            return &(*self.graph);
        }
    }

    pub fn field(&self, name: &str) -> &Field {
        self.fields.iter().find(|f| { f.name == name}).unwrap()
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}
