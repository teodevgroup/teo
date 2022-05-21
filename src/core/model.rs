use std::sync::Arc;
use inflector::Inflector;
use crate::core::builders::ModelBuilder;
use crate::core::field::Field;
use crate::core::field::ReadRule::NoRead;
use crate::core::field::Store::{Calculated, Temp};
use crate::core::field::WriteRule::NoWrite;


#[derive(Debug, Clone)]
pub(crate) struct Model {
    inner: Arc<ModelInner>
}

impl Model {
    pub(crate) fn new(builder: &ModelBuilder) -> Self {
        Model { inner: Arc::new(ModelInner::new(builder))}
    }

    pub(crate) fn name(&self) -> &'static str {
        self.inner.name
    }

    pub(crate) fn table_name(&self) -> &String {
        &self.inner.table_name
    }

    pub(crate) fn localized_name(&self) -> &'static str {
        self.inner.localized_name
    }

    pub(crate) fn description(&self) -> &'static str {
        self.inner.description
    }

    pub(crate) fn identity(&self) -> bool {
        self.inner.identity
    }

    pub(crate) fn fields(&self) -> &Vec<Field> {
        return &self.inner.fields
    }

    pub(crate) fn input_keys(&self) -> &Vec<&'static str> {
        &self.inner.input_keys
    }

    pub(crate) fn save_keys(&self) -> &Vec<&'static str> {
        &self.inner.save_keys
    }

    pub(crate) fn output_keys(&self) -> &Vec<&'static str> {
        &self.inner.output_keys
    }

    pub(crate) fn all_getable_keys(&self) -> &Vec<&'static str> {
        &self.inner.all_getable_keys
    }

    pub(crate) fn field(&self, name: &str) -> &Field {
        self.inner.fields.iter().find(|f| { f.name == name}).unwrap()
    }
}

#[derive(Debug)]
struct ModelInner {
    pub(crate) name: &'static str,
    pub(crate) table_name: String,
    pub(crate) localized_name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) identity: bool,
    pub(crate) fields: Vec<Field>,
    pub(crate) input_keys: Vec<&'static str>,
    pub(crate) save_keys: Vec<&'static str>,
    pub(crate) output_keys: Vec<&'static str>,
    pub(crate) all_getable_keys: Vec<&'static str>,
}

impl ModelInner {

    pub(crate) fn new(builder: &ModelBuilder) -> Self {
        let input_keys = Self::allowed_input_keys(builder);
        let save_keys = Self::allowed_save_keys(builder);
        let output_keys = Self::allowed_output_keys(builder);
        let all_getable_keys = Self::all_getable_keys(builder);
        return ModelInner {
            name: builder.name,
            table_name: if builder.table_name == "" { builder.name.to_lowercase().to_plural() } else { builder.table_name.to_string() },
            localized_name: builder.localized_name,
            description: builder.description,
            identity: builder.identity,
            fields: builder.fields.iter().map(|fb| { Field::new(fb) }).collect(),
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

    pub fn field(&self, name: &str) -> &Field {
        self.fields.iter().find(|f| { f.name == name}).unwrap()
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}
