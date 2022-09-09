use std::collections::HashMap;

use crate::core::data_source::builder::DataSourceBuilder;
use crate::core::r#enum::builder::EnumBuilder;
use crate::core::connector::{ConnectorBuilder};
use crate::core::model::builder::ModelBuilder;

use crate::core::r#enum::Enum;

pub struct GraphBuilder {
    pub(crate) enum_builders: HashMap<String, EnumBuilder>,
    pub(crate) models: Vec<ModelBuilder>,
    pub(crate) connector_builder: Option<Box<dyn ConnectorBuilder>>,
    pub(crate) reset_database: bool,
}

impl GraphBuilder {

    pub(crate) fn new() -> Self {
        GraphBuilder {
            enum_builders: HashMap::new(),
            models: Vec::new(),
            connector_builder: None,
            reset_database: false,
        }
    }

    pub fn r#enum<F: Fn(&mut EnumBuilder)>(&mut self, name: impl Into<String>, build: F) -> &mut Self {
        let name = name.into();
        let mut enum_builder = EnumBuilder::new(name.clone());
        build(&mut enum_builder);
        self.enum_builders.insert(name.clone(), enum_builder);
        self
    }

    pub fn model<F: Fn(&mut ModelBuilder)>(&mut self, name: &'static str, build: F) -> &mut Self {
        let mut model: ModelBuilder = ModelBuilder::new(name, self.connector_builder());
        build(&mut model);
        self.models.push(model);
        self
    }

    pub fn reset_database(&mut self) -> &mut Self {
        self.reset_database = true;
        self
    }

    pub fn data_source(&mut self) -> DataSourceBuilder {
        DataSourceBuilder { graph_builder: self }
    }

    pub(crate) fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        match &self.connector_builder {
            Some(connector_builder) => connector_builder,
            None => panic!("Graph doesn't have a database connector.")
        }
    }

    pub(crate) fn build_enums(&self) -> HashMap<String, Enum> {
        let mut retval: HashMap<String, Enum> = HashMap::new();
        for (k, v) in &self.enum_builders {
            retval.insert(k.clone(), v.into());
        }
        retval
    }
}
