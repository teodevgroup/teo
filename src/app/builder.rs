use std::collections::HashMap;
use std::env;
use std::ffi::{OsString};
use std::sync::{Arc, Mutex};
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use clap::{Arg, ArgAction, Command as ClapCommand};
use dotenvy::dotenv;
use futures_util::future::BoxFuture;
use std::future::Future;
use crate::core::result::Result;
use crate::connectors::mongodb::connector::MongoDBConnector;
use crate::connectors::sql::connector::SQLConnector;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::app::command::{CLI, CLICommand, GenerateClientCommand, GenerateCommand, GenerateEntityCommand, MigrateCommand, PurgeCommand, SeedCommand, SeedCommandAction, ServeCommand};
use crate::app::conf::{DebugConf, EntityGeneratorConf, ServerConf, TestConf};
use crate::app::entrance::Entrance;
use crate::app::program::Program;
use crate::core::callbacks::lookup::CallbackLookup;
use crate::core::connector::Connector;
use crate::core::field::Field;
use crate::core::database::name::DatabaseName;
use crate::core::field::r#type::FieldType;
use crate::core::graph::builder::GraphBuilder;
use crate::parser::ast::field::FieldClass;
use crate::prelude::{App, Graph, Value};
use crate::core::callbacks::types::compare::CompareArgument;
use crate::core::callbacks::types::callback::{CallbackArgument, CallbackResult};
use crate::core::callbacks::types::transform::{TransformResult, TransformArgument};
use crate::core::callbacks::types::validate::{ValidateArgument, ValidateResult};
use crate::core::items::function::compare::CompareItem;
use crate::core::items::function::perform::CallbackItem;
use crate::core::items::function::transform::TransformItem;
use crate::core::items::function::validate::ValidateItem;
use crate::core::property::Property;
use crate::core::r#enum::{Enum, EnumVariant};
use crate::core::relation::Relation;
use crate::gen::interface::client::conf::{Conf as ClientConf};
use crate::parser::ast::r#type::Arity;
use crate::parser::parser::parser::ASTParser;
use crate::seeder::data_set::{DataSet, Group, Record};
use crate::seeder::models::define::define_seeder_models;

#[derive(ToMut, Clone)]
pub struct AppBuilder {
    pub(crate) connector: Option<Arc<dyn Connector>>,
    pub(crate) graph_builder: GraphBuilder,
    pub(crate) server_conf: Option<ServerConf>,
    pub(crate) debug_conf: Option<&'static DebugConf>,
    pub(crate) test_conf: Option<&'static TestConf>,
    pub(crate) entity_generator_confs: Vec<EntityGeneratorConf>,
    pub(crate) client_generator_confs: Vec<ClientConf>,
    pub(crate) callback_lookup_table: Arc<Mutex<CallbackLookup>>,
    pub(crate) before_server_start: Option<Arc<dyn AsyncCallbackWithoutArgs>>,
    pub(crate) environment_version: Program,
    pub(crate) entrance: Entrance,
    pub(crate) args: Arc<CLI>,
    pub(crate) data_sets: Vec<DataSet>,
}

impl AppBuilder {

    pub fn new() -> Self {
        Self::new_with_environment_version_and_entrance(Program::Rust("deprecated"), Entrance::APP)
    }

    pub fn new_with_environment_version(environment_version: Program) -> Self {
        Self::new_with_environment_version_and_entrance(environment_version, Entrance::APP)
    }

    pub fn new_with_entrance(entrance: Entrance) -> Self {
        Self::new_with_environment_version_and_entrance(Program::Rust("deprecated"), entrance)
    }

    pub fn new_with_environment_version_and_entrance(environment_version: Program, entrance: Entrance) -> Self {
        let _ = dotenv(); // load dotenv file if exist. If the file does not exist, do nothing.
        Self {
            connector: None,
            graph_builder: GraphBuilder::new(),
            server_conf: None,
            debug_conf: None,
            test_conf: None,
            entity_generator_confs: vec![],
            client_generator_confs: vec![],
            callback_lookup_table: Arc::new(Mutex::new(CallbackLookup::new())),
            before_server_start: None,
            environment_version: environment_version.clone(),
            entrance,
            args: Arc::new(Self::parse_cli_args(environment_version.clone(), entrance.clone())),
            data_sets: vec![],
        }
    }




    async fn load(&mut self) {
        let mut parser = ASTParser::new(self.callback_lookup_table.clone());
        let main = match self.args.schema.as_ref() {
            Some(s) => Some(s.as_str()),
            None => None
        };
        parser.parse(main);
        self.load_config_from_parser(&parser).await;
    }

    pub async fn build(mut self) -> App {
        self.load().await;
        let graph = Box::leak(Box::new(self.graph_builder.build(self.connector.as_ref().unwrap().clone()).await));
        Graph::set_current(graph);
        let server_conf = Box::leak(Box::new(self.server_conf.unwrap()));
        App {
            server_conf,
            debug_conf: self.debug_conf,
            test_conf: self.test_conf,
            entity_generator_confs: self.entity_generator_confs,
            client_generator_confs: self.client_generator_confs,
            graph,
            environment_version: self.environment_version,
            entrance: self.entrance.clone(),
            args: self.args.clone(),
            before_server_start: self.before_server_start,
            datasets: self.data_sets,
        }
    }

    async fn load_config_from_parser(&mut self, parser: &ASTParser) {
        // connector
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let connector_declaration = source.get_connector(connector_ref.1);
        let url = connector_declaration.url.as_ref().unwrap();
        let connector: Arc<dyn Connector> = match connector_declaration.provider.unwrap() {
            DatabaseName::MySQL => {
                #[cfg(feature = "data-source-mysql")]
                Arc::new(SQLConnector::new(SQLDialect::MySQL, url, false).await)
            },
            DatabaseName::PostgreSQL => {
                #[cfg(feature = "data-source-postgres")]
                Arc::new(SQLConnector::new(SQLDialect::PostgreSQL, url, false).await)
            },
            #[cfg(feature = "data-source-sqlite")]
            DatabaseName::SQLite => {
                #[cfg(feature = "data-source-sqlite")]
                Arc::new(SQLConnector::new(SQLDialect::SQLite, url, false).await)
            },
            DatabaseName::MongoDB => {
                #[cfg(feature = "data-source-mongodb")]
                Arc::new(MongoDBConnector::new(url.clone()).await)
            },
        };
        self.connector = Some(connector.clone());

        // load enums
        for enum_ref in parser.enums.clone() {
            let source = parser.get_source(enum_ref.0);
            let ast_enum = source.get_enum(enum_ref.1);
            let enum_def = Enum::new(
                ast_enum.identifier.name.clone(),
                None,
                None,
                ast_enum.choices.iter().map(|ast_choice| {
                    EnumVariant::new(ast_choice.identifier.name.clone(), None, None)
                }).collect()
            );
            self.graph_builder.r#enum(enum_def);
        }
        let enums = self.graph_builder.clone_enums();
        // load models
        for model_ref in parser.models.clone() {
            let source = parser.get_source(model_ref.0);
            let model = source.get_model(model_ref.1);
            self.graph_builder.model(&model.identifier.name, |model_builder| {
                if let Some(comment) = &model.comment_block {
                    if let Some(name) = comment.name.as_ref() {
                        model_builder.localized_name(name);
                    }
                    if let Some(desc) = comment.desc.as_ref() {
                        model_builder.description(desc);
                    }
                }
                for decorator in model.decorators.iter() {
                   let model_decorator = decorator.accessible.as_ref().unwrap().as_model_decorator().unwrap();
                    model_decorator(decorator.get_argument_list(), model_builder);
                }
                for field in model.fields.iter() {
                    match &field.field_class {
                        FieldClass::Field | FieldClass::DroppedField => {
                            let mut model_field = Field::new(field.identifier.name.as_str().to_owned());
                            if let Some(comment) = &field.comment_block {
                                if let Some(name) = comment.name.as_ref() {
                                    model_field.localized_name = Some(name.to_owned());
                                }
                                if let Some(desc) = comment.desc.as_ref() {
                                    model_field.description = Some(desc.to_owned());
                                }
                            }
                            // type
                            match field.r#type.arity {
                                Arity::Scalar => {
                                    if field.r#type.item_required {
                                        model_field.set_required();
                                    } else {
                                        model_field.set_optional();
                                    }
                                    Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut model_field, &enums);
                                }
                                Arity::Array => {
                                    if field.r#type.collection_required {
                                        model_field.set_required();
                                    } else {
                                        model_field.set_optional();
                                    }
                                    model_field.field_type = Some(FieldType::Vec(Box::new({
                                        let mut inner = Field::new("".to_owned());
                                        if field.r#type.item_required {
                                            inner.set_required();
                                        } else {
                                            inner.set_optional();
                                        }
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner, &enums);
                                        inner
                                    })));
                                }
                                Arity::Dictionary => {
                                    if field.r#type.collection_required {
                                        model_field.set_required();
                                    } else {
                                        model_field.set_optional();
                                    }
                                    model_field.field_type = Some(FieldType::HashMap(Box::new({
                                        let mut inner = Field::new("".to_owned());
                                        if field.r#type.item_required {
                                            inner.set_required();
                                        } else {
                                            inner.set_optional();
                                        }
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner, &enums);
                                        inner
                                    })));
                                }
                            }
                            // decorators
                            for decorator in field.decorators.iter() {
                                let field_decorator = decorator.accessible.as_ref().unwrap().as_field_decorator().unwrap();
                                field_decorator(decorator.get_argument_list(), &mut model_field);
                            }
                            match &field.field_class {
                                FieldClass::DroppedField => {
                                    model_builder.dropped_field(model_field);
                                }
                                _ => {
                                    model_builder.field(model_field);
                                }
                            }
                        }
                        FieldClass::Relation => {
                            let mut model_relation = Relation::new(field.identifier.name.as_str().to_owned());
                            if let Some(comment) = &field.comment_block {
                                if let Some(name) = comment.name.as_ref() {
                                    model_relation.localized_name = Some(name.to_owned());
                                }
                                if let Some(desc) = comment.desc.as_ref() {
                                    model_relation.description = Some(desc.to_owned());
                                }
                            }
                            match field.r#type.arity {
                                Arity::Scalar => {
                                    if field.r#type.item_required {
                                        model_relation.set_required();
                                    } else {
                                        model_relation.set_optional();
                                    }
                                    model_relation.set_is_vec(false);
                                    model_relation.set_model(field.r#type.identifier.name.clone());
                                }
                                Arity::Array => {
                                    if !field.r#type.item_required {
                                        panic!("Relation cannot have optional items.")
                                    }
                                    model_relation.set_is_vec(true);
                                    model_relation.set_model(field.r#type.identifier.name.clone());
                                }
                                Arity::Dictionary => panic!("Relations cannot be dictionary.")
                            }
                            // handle decorators
                            for decorator in field.decorators.iter() {
                                let relation_decorator = decorator.accessible.as_ref().unwrap().as_relation_decorator().unwrap();
                                relation_decorator(decorator.get_argument_list(), &mut model_relation);
                            }
                            model_builder.relation(model_relation);
                        }
                        FieldClass::Property => {
                            let mut model_property = Property::new(field.identifier.name.clone());
                            if let Some(comment) = &field.comment_block {
                                if let Some(name) = comment.name.as_ref() {
                                    model_property.localized_name = Some(name.to_owned());
                                }
                                if let Some(desc) = comment.desc.as_ref() {
                                    model_property.description = Some(desc.to_owned());
                                }
                            }
                            // type
                            match field.r#type.arity {
                                Arity::Scalar => {
                                    if field.r#type.item_required {
                                        model_property.set_required();
                                    } else {
                                        model_property.set_optional();
                                    }
                                    Self::install_types_to_property_builder(&field.r#type.identifier.name, &mut model_property, &enums);
                                }
                                Arity::Array => {
                                    if field.r#type.collection_required {
                                        model_property.set_required();
                                    } else {
                                        model_property.set_optional();
                                    }
                                    model_property.field_type = Some(FieldType::Vec(Box::new({
                                        let mut inner = Field::new("".to_owned());
                                        if field.r#type.item_required {
                                            inner.set_required();
                                        } else {
                                            inner.set_optional();
                                        }
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner, &enums);
                                        inner
                                    })));
                                }
                                Arity::Dictionary => {
                                    if field.r#type.collection_required {
                                        model_property.set_required();
                                    } else {
                                        model_property.set_optional();
                                    }
                                    model_property.field_type = Some(FieldType::HashMap(Box::new({
                                        let mut inner = Field::new("".to_owned());
                                        if field.r#type.item_required {
                                            inner.set_required();
                                        } else {
                                            inner.set_optional();
                                        }
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner, &enums);
                                        inner
                                    })));
                                }
                            }
                            for decorator in field.decorators.iter() {
                                let property_decorator = decorator.accessible.as_ref().unwrap().as_property_decorator().unwrap();
                                property_decorator(decorator.get_argument_list(), &mut model_property);
                            }
                            model_builder.property(model_property);
                        }
                        FieldClass::Unresolved => panic!()
                    }
                }
            });
        }
        // internal model used by teo
        // load seeder models
        define_seeder_models(&mut self.graph_builder);
        // load data sets
        for data_set_ref in parser.data_sets.clone() {
            let source = parser.get_source(data_set_ref.0);
            let parser_data_set = source.get_data_set(data_set_ref.1);
            let seeder_data_set = DataSet {
                name: parser_data_set.identifier.name.clone(),
                groups: parser_data_set.groups.iter().map(|g| Group {
                    name: g.identifier.name.clone(),
                    records: g.records.iter().map(|r| Record {
                        name: r.identifier.name.clone(),
                        value: r.resolved.as_ref().unwrap().clone()
                    }).collect(),
                }).collect(),
                autoseed: parser_data_set.auto_seed,
                notrack: parser_data_set.notrack,
            };
            self.data_sets.push(seeder_data_set);
        }
    }

    fn install_types_to_field_builder(name: &str, field: &mut Field, enums: &HashMap<String, Enum>) {
        match name {
            "String" => field.field_type = Some(FieldType::String),
            "Bool" => field.field_type = Some(FieldType::Bool),
            "Int" | "Int32" => field.field_type = Some(FieldType::I32),
            "Int64" => field.field_type = Some(FieldType::I64),
            "Float32" => field.field_type = Some(FieldType::F32),
            "Float" | "Float64" => field.field_type = Some(FieldType::F64),
            "Date" => field.field_type = Some(FieldType::Date),
            "DateTime" => field.field_type = Some(FieldType::DateTime),
            "Decimal" => field.field_type = Some(FieldType::Decimal),
            #[cfg(feature = "data-source-mongodb")]
            "ObjectId" => field.field_type = Some(FieldType::ObjectId),
            // _ => panic!("Unrecognized type: '{}'.", name)
            _ => field.field_type = Some(FieldType::Enum(enums.get(name).unwrap().clone())),
        };
    }

    fn install_types_to_property_builder(name: &str, property: &mut Property, enums: &HashMap<String, Enum>) {
        match name {
            "String" => property.field_type = Some(FieldType::String),
            "Bool" => property.field_type = Some(FieldType::Bool),
            "Int" | "Int32" => property.field_type = Some(FieldType::I32),
            "Int64" =>  property.field_type = Some(FieldType::I64),
            "Float32" =>  property.field_type = Some(FieldType::F32),
            "Float" | "Float64" =>  property.field_type = Some(FieldType::F64),
            "Date" =>  property.field_type = Some(FieldType::Date),
            "DateTime" =>  property.field_type = Some(FieldType::DateTime),
            "Decimal" => property.field_type = Some(FieldType::Decimal),
            #[cfg(feature = "data-source-mongodb")]
            "ObjectId" =>  property.field_type = Some(FieldType::ObjectId),
            _ => property.field_type = Some(FieldType::Enum(enums.get(name).unwrap().clone())),
            // _ => panic!("Unrecognized type: '{}'.", name)
        };
    }
}

unsafe impl Send for AppBuilder { }
unsafe impl Sync for AppBuilder { }
