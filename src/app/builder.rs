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
