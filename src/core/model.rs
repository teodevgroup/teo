use inflector::Inflector;
use crate::core::builders::ModelBuilder;
use crate::core::field::Field;
use crate::core::field::ReadRule::NoRead;
use crate::core::field::Store::{Calculated, Temp};
use crate::core::field::WriteRule::NoWrite;


#[derive(Debug)]
pub(crate) struct Model {
    name: &'static str,
    table_name: String,
    url_segment_name: String,
    localized_name: &'static str,
    description: &'static str,
    identity: bool,
    fields: Vec<Field>,
    input_keys: Vec<&'static str>,
    save_keys: Vec<&'static str>,
    output_keys: Vec<&'static str>,
    get_value_keys: Vec<&'static str>,
}

impl Model {

    pub(crate) fn new(builder: &ModelBuilder) -> Self {
        let input_keys = Self::allowed_input_keys(builder);
        let save_keys = Self::allowed_save_keys(builder);
        let output_keys = Self::allowed_output_keys(builder);
        let get_value_keys = Self::get_get_value_keys(builder);
        return Model {
            name: builder.name,
            table_name: if builder.table_name == "" { builder.name.to_lowercase().to_plural() } else { builder.table_name.to_string() },
            url_segment_name: if builder.url_segment_name == "" { builder.name.to_kebab_case().to_plural() } else { builder.url_segment_name.to_string() },
            localized_name: builder.localized_name,
            description: builder.description,
            identity: builder.identity,
            fields: builder.fields.iter().map(|fb| { Field::new(fb) }).collect(),
            input_keys,
            save_keys,
            output_keys,
            get_value_keys
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn table_name(&self) -> &String {
        &self.table_name
    }

    pub(crate) fn url_segment_name(&self) -> &String {
        &self.url_segment_name
    }

    pub(crate) fn localized_name(&self) -> &'static str {
        self.localized_name
    }

    pub(crate) fn description(&self) -> &'static str {
        self.description
    }

    pub(crate) fn identity(&self) -> bool {
        self.identity
    }

    pub(crate) fn fields(&self) -> &Vec<Field> {
        return &self.fields
    }

    pub(crate) fn input_keys(&self) -> &Vec<&'static str> {
        &self.input_keys
    }

    pub(crate) fn save_keys(&self) -> &Vec<&'static str> {
        &self.save_keys
    }

    pub(crate) fn output_keys(&self) -> &Vec<&'static str> {
        &self.output_keys
    }

    pub(crate) fn get_value_keys(&self) -> &Vec<&'static str> {
        &self.get_value_keys
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

    pub(crate) fn get_get_value_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .map(|f| { f.name })
            .collect()
    }

    pub fn field(&self, name: &str) -> &Field {
        self.fields.iter().find(|f| { f.name == name }).unwrap()
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}
