use crate::core::conf::builder::ConfBuilder;
use crate::core::database::name::DatabaseName;
use crate::core::graph::builder::GraphBuilder;
use crate::parser::ast::field::FieldClass;
use crate::parser::parser::Parser;
use crate::prelude::App;

pub struct AppBuilder {
    pub(crate) graph_builder: GraphBuilder,
    pub(crate) conf_builder: ConfBuilder,
}

impl AppBuilder {

    pub fn new() -> Self {
        Self {
            graph_builder: GraphBuilder::new(),
            conf_builder: ConfBuilder::new(),
        }
    }

    pub fn load(&mut self, schema_file_name: Option<&str>) {
        let mut parser = Parser::new();
        parser.parse(schema_file_name);
        self.load_config_from_parser(&parser);
    }

    pub fn graph_builder(&mut self) -> &mut GraphBuilder {
        &mut self.graph_builder
    }

    pub fn conf_builder(&mut self) -> &mut ConfBuilder {
        &mut self.conf_builder
    }

    pub async fn build(&self) -> App {
        App { conf: self.conf_builder.build(), graph: self.graph_builder.build().await }
    }

    fn load_config_from_parser(&mut self, parser: &Parser) {
        // connector
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let connector = source.get_connector(connector_ref.1);
        let url = connector.url.as_ref().unwrap();
        match connector.provider.unwrap() {
            DatabaseName::MySQL => self.graph_builder.data_source().mysql(url),
            DatabaseName::PostgreSQL => self.graph_builder.data_source().postgres(url),
            DatabaseName::SQLite => self.graph_builder.data_source().sqlite(url),
            DatabaseName::MongoDB => self.graph_builder.data_source().mongodb(url),
        }
        // config
        let config_ref = parser.connector.unwrap();
        let source = parser.get_source(config_ref.0);
        let config = source.get_config(config_ref.1);
        let bind = config.bind.as_ref().unwrap();
        self.conf_builder().bind(bind.clone());
        if let Some(pathPrefix) = &config.pathPrefix {
            self.conf_builder().path_prefix(pathPrefix);
        }
        if let Some(jwtSecret) = &config.jwtSecret {
            self.conf_builder().jwt_secret(jwtSecret);
        }
        // load enums
        for enum_ref in parser.enums.clone() {
            let source = parser.get_source(enum_ref.0);
            let r#enum = source.get_enum(connector_ref.1);
            self.graph_builder.r#enum(&r#enum.identifier.name, |enum_builder| {
               for choice in r#enum.choices.iter() {
                    enum_builder.choice(&choice.identifier.name, |_| {});
               }
            });
        }
        // load models
        for model_ref in parser.models.clone() {
            let source = parser.get_source(model_ref.0);
            let model = source.get_model(model_ref.1);
            self.graph_builder.model(&model.identifier.name, |model_builder| {
                for decorator in model.decorators.iter() {
                   let model_decorator = decorator.accessible.as_ref().unwrap().as_model_decorator().unwrap();
                    model_decorator(decorator.arguments.clone().unwrap().arguments, model_builder);
                }
                for field in model.fields.iter() {
                    match &field.field_class {
                        FieldClass::Field => {
                            model_builder.field(field.identifier.name.as_str(), |field_builder| {
                                // handle types here
                                for decorator in field.decorators.iter() {
                                    let field_decorator = decorator.accessible.as_ref().unwrap().as_field_decorator().unwrap();
                                    field_decorator(decorator.arguments.clone().unwrap().arguments, field_builder);
                                }
                            });
                        }
                        FieldClass::Relation => {
                            model_builder.relation(field.identifier.name.as_str(), |relation_builder| {
                                // handle types here
                                for decorator in field.decorators.iter() {
                                    let relation_decorator = decorator.accessible.as_ref().unwrap().as_relation_decorator().unwrap();
                                    relation_decorator(decorator.arguments.clone().unwrap().arguments, relation_builder);
                                }
                            });
                        }
                        FieldClass::Property => {
                            model_builder.property(field.identifier.name.as_str(), |property_builder| {
                                // handle types here
                                for decorator in field.decorators.iter() {
                                    let property_decorator = decorator.accessible.as_ref().unwrap().as_property_decorator().unwrap();
                                    property_decorator(decorator.arguments.clone().unwrap().arguments, property_builder);
                                }
                            });
                        }
                        FieldClass::Unresolved => panic!()
                    }
                }
            });
        }
    }
}
