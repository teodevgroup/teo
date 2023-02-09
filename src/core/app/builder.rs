use std::collections::HashMap;
use std::env;
use std::ffi::{OsString};
use std::fmt::{Debug};
use std::sync::{Arc, Mutex};
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use clap::{Arg, ArgAction, Command as ClapCommand};
use dotenvy::dotenv;
use crate::connectors::mongodb::connector::MongoDBConnector;
use crate::connectors::sql::connector::SQLConnector;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::app::command::{CLI, CLICommand, GenerateClientCommand, GenerateCommand, GenerateEntityCommand, MigrateCommand, ServeCommand};
use crate::core::app::conf::{ClientGeneratorConf, EntityGeneratorConf, ServerConf};
use crate::core::app::entrance::Entrance;
use crate::core::app::environment::EnvironmentVersion;
use crate::core::connector::Connector;
use crate::core::field::Field;
use crate::core::database::name::DatabaseName;
use crate::core::field::r#type::FieldType;
use crate::core::graph::builder::GraphBuilder;
use crate::parser::ast::field::FieldClass;
use crate::prelude::{App, Value};
use crate::core::pipeline::context::validity::Validity;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::function::compare::{CompareArgument, CompareModifier};
use crate::core::pipeline::modifiers::function::perform::{PerformArgument, PerformModifier};
use crate::core::pipeline::modifiers::function::transform::{TransformArgument, TransformModifier};
use crate::core::pipeline::modifiers::function::validate::{ValidateArgument, ValidateModifier};
use crate::core::property::Property;
use crate::core::relation::Relation;
use crate::parser::ast::r#type::Arity;
use crate::parser::parser::Parser;

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
    pub(crate) connector: Option<Arc<dyn Connector>>,
    pub(crate) graph_builder: GraphBuilder,
    pub(crate) server_conf: Option<ServerConf>,
    pub(crate) entity_generator_confs: Vec<EntityGeneratorConf>,
    pub(crate) client_generator_confs: Vec<ClientGeneratorConf>,
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
        let _ = dotenv(); // load dotenv file if exist. If the file is not exist, do nothing.
        Self {
            connector: None,
            graph_builder: GraphBuilder::new(),
            server_conf: None,
            entity_generator_confs: vec![],
            client_generator_confs: vec![],
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
                EnvironmentVersion::Python(_) | EnvironmentVersion::NodeJS(_) => env::args_os().enumerate().filter(|(i, _x)| *i != 1).map(|(_i, x)| x).collect::<Vec<OsString>>(),
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

    pub fn callback<T, F>(&mut self, name: impl Into<String>, f: F) -> &mut Self where
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

    async fn load(&mut self) {
        let mut parser = Parser::new(self.callback_lookup_table.clone());
        let main = match self.args.schema.as_ref() {
            Some(s) => Some(s.as_str()),
            None => None
        };
        parser.parse(main);
        self.load_config_from_parser(&parser).await;
    }

    pub async fn build(&mut self) -> App {
        self.load().await;
        App {
            server_conf: self.server_conf.clone().unwrap(),
            entity_generator_confs: self.entity_generator_confs.clone(),
            client_generator_confs: self.client_generator_confs.clone(),
            graph: self.graph_builder.build(self.connector.as_ref().unwrap().clone()).await,
            environment_version: self.environment_version.clone(),
            entrance: self.entrance.clone(),
            args: self.args.clone(),
        }
    }

    async fn load_config_from_parser(&mut self, parser: &Parser) {
        // connector
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let connector_declaration = source.get_connector(connector_ref.1);
        let url = connector_declaration.url.as_ref().unwrap();
        let connector: Arc<dyn Connector> = match connector_declaration.provider.unwrap() {
            DatabaseName::MySQL => {
                #[cfg(feature = "data-source-mysql")]
                Arc::new(SQLConnector::new(SQLDialect::MySQL, url.clone(), false).await)
            },
            DatabaseName::PostgreSQL => {
                #[cfg(feature = "data-source-postgres")]
                Arc::new(SQLConnector::new(SQLDialect::PostgreSQL, url.clone(), false).await)
            },
            DatabaseName::SQLite => {
                #[cfg(feature = "data-source-sqlite")]
                Arc::new(SQLConnector::new(SQLDialect::SQLite, url.clone(), false).await)
            },
            DatabaseName::MongoDB => {
                #[cfg(feature = "data-source-mongodb")]
                Arc::new(MongoDBConnector::new(url.clone()).await)
            },
        };
        // server config
        let config_ref = parser.config.unwrap();
        let source = parser.get_source(config_ref.0);
        let config = source.get_server_config(config_ref.1);
        let bind = config.bind.as_ref().unwrap();
        self.server_conf = Some(ServerConf {
            bind: bind.clone(),
            path_prefix: if let Some(path_prefix) = &config.path_prefix {
                Some(path_prefix.clone())
            } else {
                None
            },
            jwt_secret: if let Some(jwt_secret) = &config.jwt_secret {
                Some(jwt_secret.clone())
            } else {
                None
            }
        });
        // entity generators
        for entity_generator_ref in parser.generators.iter() {
            let source = parser.get_source(entity_generator_ref.0);
            let entity = source.get_entity(entity_generator_ref.1);
            self.entity_generator_confs.push(EntityGeneratorConf {
                name: Some(entity.identifier.name.clone()),
                provider: entity.provider.unwrap(),
                dest: entity.dest.clone().unwrap(),
            })
        }
        // client generators
        for client_generator_ref in parser.clients.iter() {
            let source = parser.get_source(client_generator_ref.0);
            let client = source.get_client(client_generator_ref.1);
            self.client_generator_confs.push(ClientGeneratorConf {
                name: Some(client.identifier.name.clone()),
                provider: client.provider.unwrap(),
                dest: client.dest.clone().unwrap(),
                package: client.package.unwrap(),
                host: client.host.clone().unwrap(),
                object_name: client.object_name.clone(),
            })
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
                            let mut model_field = Field::new(field.identifier.name.as_str().to_owned());
                            // type
                            match field.r#type.arity {
                                Arity::Scalar => {
                                    if field.r#type.item_required {
                                        model_field.set_required();
                                    } else {
                                        model_field.set_optional();
                                    }
                                    Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut model_field);
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
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner);
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
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner);
                                        inner
                                    })));
                                }
                            }
                            // decorators
                            for decorator in field.decorators.iter() {
                                let field_decorator = decorator.accessible.as_ref().unwrap().as_field_decorator().unwrap();
                                field_decorator(decorator.get_argument_list(), &mut model_field);
                            }
                            model_builder.field(model_field);
                        }
                        FieldClass::Relation => {
                            let mut model_relation = Relation::new(field.identifier.name.as_str().to_owned());
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
                            // type
                            match field.r#type.arity {
                                Arity::Scalar => {
                                    if field.r#type.item_required {
                                        model_property.set_required();
                                    } else {
                                        model_property.set_optional();
                                    }
                                    Self::install_types_to_property_builder(&field.r#type.identifier.name, &mut model_property);
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
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner);
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
                                        Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner);
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
    }

    fn install_types_to_field_builder(name: &str, field: &mut Field) {
        match name {
            "String" => field.field_type = Some(FieldType::String),
            "Bool" => field.field_type = Some(FieldType::Bool),
            "Int" | "Int32" => field.field_type = Some(FieldType::I32),
            "Int64" => field.field_type = Some(FieldType::I64),
            "Float32" => field.field_type = Some(FieldType::F32),
            "Float" | "Float64" => field.field_type = Some(FieldType::F64),
            "Date" => field.field_type = Some(FieldType::Date),
            "DateTime" => field.field_type = Some(FieldType::DateTime),
            #[cfg(feature = "data-source-mongodb")]
            "ObjectId" => field.field_type = Some(FieldType::ObjectId),
            // _ => panic!("Unrecognized type: '{}'.", name)
            _ => field.field_type = Some(FieldType::Enum(name.to_string())),
        };
    }

    fn install_types_to_property_builder(name: &str, property: &mut Property) {
        match name {
            "String" => property.field_type = Some(FieldType::String),
            "Bool" => property.field_type = Some(FieldType::Bool),
            "Int" | "Int32" => property.field_type = Some(FieldType::I32),
            "Int64" =>  property.field_type = Some(FieldType::I64),
            "Float32" =>  property.field_type = Some(FieldType::F32),
            "Float" | "Float64" =>  property.field_type = Some(FieldType::F64),
            "Date" =>  property.field_type = Some(FieldType::Date),
            "DateTime" =>  property.field_type = Some(FieldType::DateTime),
            #[cfg(feature = "data-source-mongodb")]
            "ObjectId" =>  property.field_type = Some(FieldType::ObjectId),
            _ => property.field_type = Some(FieldType::Enum(name.to_string())),
            // _ => panic!("Unrecognized type: '{}'.", name)
        };
    }
}

unsafe impl Send for AppBuilder { }
unsafe impl Sync for AppBuilder { }
