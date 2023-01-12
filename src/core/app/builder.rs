use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use crate::core::conf::builder::ConfBuilder;
use crate::core::database::name::DatabaseName;
use crate::core::graph::builder::GraphBuilder;
use crate::parser::ast::field::FieldClass;
use crate::parser::parser::Parser;
use crate::prelude::{App, Value};
use futures_util::future::BoxFuture;
use std::future::Future;
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use clap::{Arg, arg, ArgAction, Command as ClapCommand, Parser as ClapParser};
use crate::core::app::command::{CLI, CLICommand, GenerateClientCommand, GenerateCommand, GenerateEntityCommand, MigrateCommand, ServeCommand};
use crate::core::app::entrance::Entrance;
use crate::core::app::environment::EnvironmentVersion;
use crate::core::field::builder::FieldBuilder;
use crate::core::pipeline::context::Context;
use crate::core::pipeline::context::validity::Validity;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::function::compare::{CompareArgument, CompareModifier};
use crate::core::pipeline::modifiers::function::perform::{PerformArgument, PerformModifier};
use crate::core::pipeline::modifiers::function::transform::{TransformArgument, TransformModifier};
use crate::core::pipeline::modifiers::function::validate::{ValidateArgument, ValidateModifier};
use crate::core::property::builder::PropertyBuilder;
use crate::parser::ast::r#type::Arity;

#[derive(Debug)]
pub(crate) struct CallbackLookupTable {
    pub(crate) transforms: HashMap<String, Arc<dyn Modifier>>,
    pub(crate) validators: HashMap<String, Arc<dyn Modifier>>,
    pub(crate) callbacks: HashMap<String, Arc<dyn Modifier>>,
    pub(crate) compares: HashMap<String, Arc<dyn Modifier>>,
}

impl CallbackLookupTable {
    fn new() -> Self {
        Self { transforms: HashMap::new(), validators: HashMap::new(), callbacks: HashMap::new(), compares: HashMap::new() }
    }
}

#[derive(ToMut)]
pub struct AppBuilder {
    pub(crate) graph_builder: GraphBuilder,
    pub(crate) conf_builder: ConfBuilder,
    pub(crate) callback_lookup_table: Arc<Mutex<CallbackLookupTable>>,
    pub(crate) environment_version: EnvironmentVersion,
    pub(crate) entrance: Entrance,
    pub(crate) args: Arc<CLI>,
}

impl AppBuilder {

    pub fn new() -> Self {
        Self::new_with_environment_version_and_entrance(Self::rust_environment_version(), Entrance::APP)
    }

    pub fn new_with_environment_version(environment_version: EnvironmentVersion) -> Self {
        Self::new_with_environment_version_and_entrance(environment_version, Entrance::APP)
    }

    pub fn new_with_entrance(entrance: Entrance) -> Self {
        Self::new_with_environment_version_and_entrance(Self::rust_environment_version(), entrance)
    }

    pub fn new_with_environment_version_and_entrance(environment_version: EnvironmentVersion, entrance: Entrance) -> Self {
        Self {
            graph_builder: GraphBuilder::new(),
            conf_builder: ConfBuilder::new(),
            callback_lookup_table: Arc::new(Mutex::new(CallbackLookupTable::new())),
            environment_version: environment_version.clone(),
            entrance,
            args: Arc::new(Self::parse_cli_args(environment_version.clone(), entrance.clone())),
        }
    }

    fn parse_cli_args(environment_version: EnvironmentVersion, entrance: Entrance) -> CLI {
        let version = Box::leak(Box::new(format!("Teo {} ({}) [{}]", env!("CARGO_PKG_VERSION"), environment_version.to_string(), entrance.to_str())));
        let about = Box::leak(Box::new(match entrance {
            Entrance::CLI => format!("{version}\n\nRun Teo application with CLI."),
            Entrance::APP => format!("{version}\n\nRun Teo application with custom code loaded."),
        }));
        let matches = ClapCommand::new("teo")
            .version(version.as_str())
            .disable_version_flag(true)
            .disable_help_subcommand(true)
            .arg_required_else_help(true)
            .about(about.as_str())
            .subcommand_required(true)
            .arg(Arg::new("SCHEMA_FILE")
                .short('s')
                .long("schema")
                .help("The schema file to load").action(ArgAction::Set)
                .required(false)
                .num_args(1))
            .arg(Arg::new("version")
                .short('v')
                .long("version")
                .help("Print version information")
                .action(ArgAction::Version))
            .subcommand(ClapCommand::new("serve")
                .about("Run migration and start the server")
                .arg_required_else_help(false)
                .arg(Arg::new("no-migration")
                    .short('M')
                    .long("no-migration")
                    .help("Start server without running migration")
                    .action(ArgAction::SetTrue)))
            .subcommand(ClapCommand::new("generate")
                .about("Generate code")
                .arg_required_else_help(true)
                .subcommand(ClapCommand::new("client")
                    .about("Generate client")
                    .arg(Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Generate all clients")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("NAME"))
                    .arg(Arg::new("NAME")
                        .action(ArgAction::Append)
                        .conflicts_with("all")
                        .help("Client names to generate")
                        .num_args(1..)))
                .subcommand(ClapCommand::new("entity")
                    .about("Generate model entities")
                    .arg_required_else_help(false)
                    .arg(Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Generate all clients")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("NAME"))
                    .arg(Arg::new("NAME")
                        .action(ArgAction::Append)
                        .conflicts_with("all")
                        .help("Entity names to generate")
                        .num_args(1..))))
            .subcommand(ClapCommand::new("migrate")
                .about("Run migration")
                .arg(Arg::new("dry")
                    .short('d')
                    .long("dry")
                    .help("Dry run")
                    .action(ArgAction::SetTrue)))
            .get_matches_from(match environment_version {
                EnvironmentVersion::Python(_) | EnvironmentVersion::NodeJS(_) => env::args_os().enumerate().filter(|(i, x)| *i != 1).map(|(i, x)| x).collect::<Vec<OsString>>(),
                _ => env::args_os().collect::<Vec<OsString>>(),
            });
        let schema: Option<&String> = matches.get_one("SCHEMA_FILE");
        let command = match matches.subcommand() {
            Some(("serve", submatches)) => {
                CLICommand::Serve(ServeCommand { no_migration: submatches.get_flag("no-migration") })
            }
            Some(("generate", submatches)) => {
                match submatches.subcommand() {
                    Some(("client", submatches)) => {
                        let names: Option<Vec<String>> = submatches.get_many::<String>("NAME").map(|s| s.map(|v| v.to_string()).collect::<Vec<String>>());
                        CLICommand::Generate(GenerateCommand::GenerateClientCommand(GenerateClientCommand { all: false, names }))
                    }
                    Some(("entity", submatches)) => {
                        let names: Option<Vec<String>> = submatches.get_many::<String>("NAME").map(|s| s.map(|v| v.to_string()).collect::<Vec<String>>());
                        CLICommand::Generate(GenerateCommand::GenerateEntityCommand(GenerateEntityCommand { all: false, names }))
                    }
                    _ => unreachable!()
                }
            }
            Some(("migrate", submatches)) => {
                CLICommand::Migrate(MigrateCommand { dry: submatches.get_flag("dry") })
            }
            _ => unreachable!()
        };
        CLI { command, schema: schema.map(|s| s.to_string()) }
    }

    fn rust_environment_version() -> EnvironmentVersion {
        EnvironmentVersion::Rust(env!("TEO_RUSTC_VERSION").to_string())
    }

    pub fn transform<T, F>(&mut self, name: impl Into<String>, f: F) -> &mut Self where
        T: From<Value> + Into<Value> + Send + Sync + 'static,
        F: TransformArgument<T> + 'static {
        self.callback_lookup_table.lock().unwrap().transforms.insert(name.into(), Arc::new(TransformModifier::new(f)));
        self
    }

    pub fn perform<T, F>(&mut self, name: impl Into<String>, f: F) -> &mut Self where
        T: From<Value> + Send + Sync + 'static,
        F: PerformArgument<T> + 'static {
        self.callback_lookup_table.lock().unwrap().callbacks.insert(name.into(), Arc::new(PerformModifier::new(f)));
        self
    }

    pub fn validate<T, O, F>(&mut self, name: impl Into<String>, f: F) -> &mut Self where
        T: From<Value> + Send + Sync + 'static,
        O: Into<Validity> + Send + Sync + 'static,
        F: ValidateArgument<T, O> + 'static {
        self.callback_lookup_table.lock().unwrap().validators.insert(name.into(), Arc::new(ValidateModifier::new(f)));
        self
    }

    pub fn compare<T, O, F>(&mut self, name: impl Into<String>, f: F) -> &mut Self where
        T: From<Value> + Send + Sync + 'static,
        O: Into<Validity> + Send + Sync + 'static,
        F: CompareArgument<T, O> + 'static {
        self.callback_lookup_table.lock().unwrap().compares.insert(name.into(), Arc::new(CompareModifier::new(f)));
        self
    }

    fn load(&mut self) {
        let mut parser = Parser::new(self.callback_lookup_table.clone());
        let main = match self.args.schema.as_ref() {
            Some(s) => Some(s.as_str()),
            None => None
        };
        parser.parse(main);
        self.load_config_from_parser(&parser);
    }

    pub async fn build(&mut self) -> App {
        self.load();
        App {
            conf: self.conf_builder.build(),
            graph: self.graph_builder.build().await,
            environment_version: self.environment_version.clone(),
            entrance: self.entrance.clone(),
            args: self.args.clone(),
        }
    }

    fn graph_builder(&mut self) -> &mut GraphBuilder {
        &mut self.graph_builder
    }

    fn conf_builder(&mut self) -> &mut ConfBuilder {
        &mut self.conf_builder
    }

    fn load_config_from_parser(&mut self, parser: &Parser) {
        // connector
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let connector = source.get_connector(connector_ref.1);
        let url = connector.url.as_ref().unwrap();
        match connector.provider.unwrap() {
            DatabaseName::MySQL => {
                #[cfg(feature = "data-source-mysql")]
                self.graph_builder.data_source().mysql(url)
            },
            DatabaseName::PostgreSQL => {
                #[cfg(feature = "data-source-postgres")]
                self.graph_builder.data_source().postgres(url)
            },
            DatabaseName::SQLite => {
                #[cfg(feature = "data-source-sqlite")]
                self.graph_builder.data_source().sqlite(url)
            },
            DatabaseName::MongoDB => {
                #[cfg(feature = "data-source-mongodb")]
                self.graph_builder.data_source().mongodb(url)
            },
        }
        // config
        let config_ref = parser.config.unwrap();
        let source = parser.get_source(config_ref.0);
        let config = source.get_config(config_ref.1);
        let bind = config.bind.as_ref().unwrap();
        self.conf_builder().bind(bind.clone());
        if let Some(path_prefix) = &config.path_prefix {
            self.conf_builder().path_prefix(path_prefix);
        }
        if let Some(jwt_secret) = &config.jwt_secret {
            self.conf_builder().jwt_secret(jwt_secret);
        }
        // load enums
        for enum_ref in parser.enums.clone() {
            let source = parser.get_source(enum_ref.0);
            let r#enum = source.get_enum(enum_ref.1);
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
                    model_decorator(decorator.get_argument_list(), model_builder);
                }
                for field in model.fields.iter() {
                    match &field.field_class {
                        FieldClass::Field => {
                            model_builder.field(field.identifier.name.as_str(), |field_builder| {
                                // handle types here
                                match field.r#type.arity {
                                    Arity::Scalar => {
                                        if field.r#type.item_required {
                                            field_builder.required();
                                        } else {
                                            field_builder.optional();
                                        }
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, field_builder);
                                    }
                                    Arity::Array => {
                                        if field.r#type.collection_required {
                                            field_builder.required();
                                        } else {
                                            field_builder.optional();
                                        }
                                        field_builder.vec(|builder| {
                                            if field.r#type.item_required {
                                                builder.required();
                                            } else {
                                                builder.optional();
                                            }
                                            Self::install_types_to_field_builder(&field.r#type.identifier.name, builder);
                                        });
                                    }
                                    Arity::Dictionary => {
                                        if field.r#type.collection_required {
                                            field_builder.required();
                                        } else {
                                            field_builder.optional();
                                        }
                                        field_builder.vec(|builder| {
                                            if field.r#type.item_required {
                                                builder.required();
                                            } else {
                                                builder.optional();
                                            }
                                            Self::install_types_to_field_builder(&field.r#type.identifier.name, builder);
                                        });
                                    }
                                }
                                // handle decorators
                                for decorator in field.decorators.iter() {
                                    let field_decorator = decorator.accessible.as_ref().unwrap().as_field_decorator().unwrap();
                                    field_decorator(decorator.get_argument_list(), field_builder);
                                }
                            });
                        }
                        FieldClass::Relation => {
                            model_builder.relation(field.identifier.name.as_str(), |relation_builder| {
                                // handle types here
                                match field.r#type.arity {
                                    Arity::Scalar => {
                                        if field.r#type.item_required {
                                            relation_builder.required();
                                        } else {
                                            relation_builder.optional();
                                        }
                                        relation_builder.object(&field.r#type.identifier.name);
                                    }
                                    Arity::Array => {
                                        if !field.r#type.item_required {
                                            panic!("Relation cannot have optional items.")
                                        }
                                        relation_builder.vec(&field.r#type.identifier.name);
                                    }
                                    Arity::Dictionary => {
                                        panic!("Relations cannot be dictionary.")
                                    }
                                }
                                // handle decorators
                                for decorator in field.decorators.iter() {
                                    let relation_decorator = decorator.accessible.as_ref().unwrap().as_relation_decorator().unwrap();
                                    relation_decorator(decorator.get_argument_list(), relation_builder);
                                }
                            });
                        }
                        FieldClass::Property => {
                            model_builder.property(field.identifier.name.as_str(), |property_builder| {
                                // handle types here
                                match field.r#type.arity {
                                    Arity::Scalar => {
                                        if field.r#type.item_required {
                                            property_builder.required();
                                        } else {
                                            property_builder.optional();
                                        }
                                        Self::install_types_to_property_builder(&field.r#type.identifier.name, property_builder);
                                    }
                                    Arity::Array => {
                                        if field.r#type.collection_required {
                                            property_builder.required();
                                        } else {
                                            property_builder.optional();
                                        }
                                        property_builder.vec(|builder| {
                                            if field.r#type.item_required {
                                                builder.required();
                                            } else {
                                                builder.optional();
                                            }
                                            Self::install_types_to_field_builder(&field.r#type.identifier.name, builder);
                                        });
                                    }
                                    Arity::Dictionary => {
                                        if field.r#type.collection_required {
                                            property_builder.required();
                                        } else {
                                            property_builder.optional();
                                        }
                                        property_builder.vec(|builder| {
                                            if field.r#type.item_required {
                                                builder.required();
                                            } else {
                                                builder.optional();
                                            }
                                            Self::install_types_to_field_builder(&field.r#type.identifier.name, builder);
                                        });
                                    }
                                }
                                // handle decorators
                                for decorator in field.decorators.iter() {
                                    let property_decorator = decorator.accessible.as_ref().unwrap().as_property_decorator().unwrap();
                                    property_decorator(decorator.get_argument_list(), property_builder);
                                }
                            });
                        }
                        FieldClass::Unresolved => panic!()
                    }
                }
            });
        }
    }

    fn install_types_to_field_builder(name: &str, field_builder: &mut FieldBuilder) {
        match name {
            "String" => field_builder.string(),
            "Bool" => field_builder.bool(),
            "Int" | "Int32" => field_builder.i32(),
            "Int64" => field_builder.i64(),
            "Int8" => field_builder.i8(),
            "Int16" => field_builder.i16(),
            "UInt" | "UInt32" => field_builder.u32(),
            "UInt64" => field_builder.u64(),
            "UInt8" => field_builder.u8(),
            "UInt16" => field_builder.u16(),
            "Float32" => field_builder.f32(),
            "Float" | "Float64" => field_builder.f64(),
            "Date" => field_builder.date(),
            "DateTime" => field_builder.datetime(),
            #[cfg(feature = "data-source-mongodb")]
            "ObjectId" => field_builder.object_id(),
            // _ => panic!("Unrecognized type: '{}'.", name)
            _ => field_builder.r#enum(name),
        };
    }

    fn install_types_to_property_builder(name: &str, property_builder: &mut PropertyBuilder) {
        match name {
            "String" => property_builder.string(),
            "Bool" => property_builder.bool(),
            "Int" | "Int32" => property_builder.i32(),
            "Int64" => property_builder.i64(),
            "Int8" => property_builder.i8(),
            "Int16" => property_builder.i16(),
            "UInt" | "UInt32" => property_builder.u32(),
            "UInt64" => property_builder.u64(),
            "UInt8" => property_builder.u8(),
            "UInt16" => property_builder.u16(),
            "Float32" => property_builder.f32(),
            "Float" | "Float64" => property_builder.f64(),
            "Date" => property_builder.date(),
            "DateTime" => property_builder.datetime(),
            #[cfg(feature = "data-source-mongodb")]
            "ObjectId" => property_builder.object_id(),
            _ => property_builder.r#enum(name),
            // _ => panic!("Unrecognized type: '{}'.", name)
        };
    }
}

unsafe impl Send for AppBuilder { }
unsafe impl Sync for AppBuilder { }
