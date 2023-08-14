use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use maplit::hashmap;
use path_absolutize::Absolutize;
use regex::Regex;
use snailquote::unescape;
use crate::core::database::name::DatabaseName;
use crate::core::teon::range::Range;
use crate::parser::ast::accessible::{Accessible, ASTResolvedPipeline, ASTPipelineItem, Container};
use crate::parser::ast::argument::{ArgumentList};
use crate::parser::ast::config::ASTServer;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::decorator::ASTDecorator;
use crate::parser::ast::entity::Entity;
use crate::parser::ast::expression::{ArrayLiteral, BitwiseNegation, BoolLiteral, DictionaryLiteral, EnumChoiceLiteral, Expression, ExpressionKind, Negation, NullishCoalescing, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::parser::ast::field::{ASTField, ASTFieldClass};
use crate::parser::ast::group::Group;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::import::ASTImport;
use crate::parser::ast::model::ASTModel;
use crate::parser::ast::pipeline::ASTPipeline;
use crate::parser::ast::r#enum::{ASTEnum, EnumChoice};
use crate::parser::ast::reference::{Reference};
use crate::parser::ast::source::Source;
use crate::parser::ast::subscript::Subscript;
use crate::parser::ast::top::Top;
use crate::parser::ast::unit::Unit;
use crate::parser::parser::parser::ASTParser;
use crate::parser::std::decorators::field::GlobalFieldDecorators;
use crate::parser::std::decorators::model::GlobalModelDecorators;
use crate::parser::std::decorators::property::GlobalPropertyDecorators;
use crate::parser::std::decorators::relation::GlobalRelationDecorators;
use crate::prelude::Value;
use to_mut::ToMut;
use crate::core::action::Action;
use crate::app::program::ProgramLang;
use crate::core::interface::{ResolvedInterfaceField, ResolvedInterfaceFieldType};
use crate::gen::interface::client::kind::Kind;
use crate::parser::ast::action::{ActionDeclaration, ActionGroupDeclaration};
use crate::parser::ast::arith_expr::{ArithExpr, Op};
use crate::parser::ast::client::{ASTClient};
use crate::parser::ast::data_set::DataSet;
use crate::parser::ast::debug_conf::ASTDebugConf;
use crate::parser::ast::generator::ASTEntity;
use crate::parser::ast::interface::{InterfaceDeclaration, InterfaceItemDeclaration};
use crate::parser::ast::test_conf::ASTTestConf;
use crate::parser::ast::interface_type::InterfaceType;
use crate::parser::ast::r#type::Arity;
use crate::parser::std::pipeline::global::{GlobalFunctionInstallers, GlobalPipelineInstallers};

pub(crate) struct Resolver { }

impl Resolver {

    pub(crate) fn resolve_parser(parser: &ASTParser) {
        let database_name = Self::resolve_connector(parser);
        parser.set_global_model_decorators(GlobalModelDecorators::new());
        parser.set_global_field_decorators(GlobalFieldDecorators::new(database_name));
        parser.set_global_relation_decorators(GlobalRelationDecorators::new());
        parser.set_global_property_decorators(GlobalPropertyDecorators::new());
        parser.set_global_pipeline_installers(GlobalPipelineInstallers::new());
        parser.set_global_function_installers(GlobalFunctionInstallers::new());
        let main = parser.get_source(1);
        Self::resolve_source(parser, main);
        for (index, source) in parser.sources.iter() {
            if *index == 1 { continue }
            Self::resolve_source(parser, source);
        }
        parser.to_mut().resolved = true;
    }

    pub(crate) fn resolve_source(parser: &ASTParser, source: &Source) {
        if source.resolved { return }
        for (_item_id, top) in source.to_mut().tops.iter_mut() {
            match top {
                Top::Import(import) => {
                    Self::resolve_import(parser, source, import);
                }
                Top::Constant(constant) => {
                    Self::resolve_constant(parser, source, constant);
                }
                Top::Enum(r#enum) => {
                    Self::resolve_enum(parser, source, r#enum);
                }
                Top::Model(model) => {
                    Self::resolve_model(parser, source, model);
                }
                Top::Connector(_connector) => {
                    continue;
                }
                Top::Generator(generator) => {
                    Self::resolve_model_entity_generator(parser, source, generator);
                }
                Top::Client(client) => {
                    Self::resolve_client_generator(parser, source, client);
                }
                Top::ServerConfig(config) => {
                    Self::resolve_server_config_block(parser, source, config);
                }
                Top::DataSet(data_set) => {
                    Self::resolve_data_set(parser, source, data_set);
                }
                Top::DebugConf(debug_conf) => {
                    Self::resolve_debug_conf(parser, source, debug_conf);
                }
                Top::TestConf(test_conf) => {
                    Self::resolve_test_conf(parser, source, test_conf);
                }
                Top::MiddlewareDeclaration(middleware_declaration) => {
                    continue;
                }
                Top::ActionGroupDeclaration(action_group_declaration) => {
                    Self::resolve_action_group(parser, source, action_group_declaration);
                }
                Top::InterfaceDeclaration(interface_declaration) => {
                    continue;
                }
            }
        }
        source.to_mut().resolved = true;
    }

    pub(crate) fn resolve_import(parser: &ASTParser, _source: &Source, import: &mut ASTImport) {
        let from_source = parser.sources.iter().find(|(_source_id, source)| {
            &import.path == &source.path
        }).unwrap().1;
        import.from_id = Some(from_source.id);
        for (item_id, top) in from_source.tops.iter() {
            if top.is_model() {
                let model = top.as_model().unwrap();
                for identifier in import.identifiers.iter() {
                    if identifier.name == model.identifier.name {
                        import.references.insert(identifier.name.clone(), Reference::ModelReference((from_source.id, *item_id, identifier.name.clone())));
                    }
                }
            } else if top.is_constant() {
                let constant = top.as_constant().unwrap();
                for identifier in import.identifiers.iter() {
                    if identifier.name == constant.identifier.name {
                        import.references.insert(identifier.name.clone(), Reference::ConstantReference((from_source.id, *item_id)));
                    }
                }
            } else if top.is_data_set() {
                let dataset = top.as_data_set().unwrap();
                for identifier in import.identifiers.iter() {
                    if identifier.name == dataset.identifier.name {
                        import.references.insert(identifier.name.clone(), Reference::DataSetReference((from_source.id, *item_id)));
                    }
                }
            }
        }
        import.resolved = true;
    }

    pub(crate) fn resolve_constant(parser: &ASTParser, source: &Source, constant: &mut Constant) {
        Self::resolve_expression(parser, source, &mut constant.expression);
        constant.resolved = true;
    }

    pub(crate) fn resolve_enum(parser: &ASTParser, source: &Source, r#enum: &mut ASTEnum) {
        for choice in r#enum.choices.iter_mut() {
            Self::resolve_enum_choice(parser, source, choice);
        }
        r#enum.resolved = true;
    }

    pub(crate) fn resolve_enum_choice(_parser: &ASTParser, _source: &Source, choice: &mut EnumChoice) {
        choice.resolved = true;
    }

    pub(crate) fn resolve_model(parser: &ASTParser, source: &Source, model: &mut ASTModel) {
        // decorators
        for decorator in model.decorators.iter_mut() {
            Self::resolve_model_decorator(parser, source, decorator);
        }
        // fields
        for field in model.fields.iter_mut() {
            Self::resolve_field(parser, source, field);
        }
        // cached enums
        //
        model.resolved = true;
    }

    fn resolve_model_decorator(parser: &ASTParser, source: &Source, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = parser.global_model_decorators();
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = parser.global_model_decorators();
                let mut accessible = d.get(&identifier.name);
                let mut arg_list: Option<ArgumentList> = None;
                for (index, expression) in unit.expressions.iter().enumerate() {
                    if index == 0 { continue }
                    match expression {
                        ExpressionKind::ArgumentList(argument_list) => {
                            arg_list = Some(argument_list.clone());
                        }
                        ExpressionKind::Subscript(_subscript) => {
                            panic!("Cannot access decorator object with subscript.")
                        }
                        ExpressionKind::Identifier(identifier) => {
                            accessible = accessible.access_property(&identifier.name).as_accessible().unwrap()
                        }
                        _ => panic!()
                    }
                }
                decorator.accessible = Some(accessible.clone());
                for argument in arg_list.as_mut().unwrap().arguments.iter_mut() {
                    let when_option = identifier.name.as_str() == "disable";
                    let result = Self::resolve_expression_kind(parser, source, &argument.value, when_option);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_field_decorator(parser: &ASTParser, source: &Source, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = parser.global_field_decorators();
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = parser.global_field_decorators();
                let mut accessible = d.get(&identifier.name);
                let mut arg_list: Option<ArgumentList> = None;
                for (index, expression) in unit.expressions.iter().enumerate() {
                    if index == 0 { continue }
                    match expression {
                        ExpressionKind::ArgumentList(argument_list) => {
                            arg_list = Some(argument_list.clone());
                        }
                        ExpressionKind::Subscript(_subscript) => {
                            panic!("Cannot access decorator object with subscript.")
                        }
                        ExpressionKind::Identifier(identifier) => {
                            accessible = accessible.access_property(&identifier.name).as_accessible().unwrap()
                        }
                        _ => panic!()
                    }
                }
                decorator.accessible = Some(accessible.clone());
                for argument in arg_list.as_mut().unwrap().arguments.iter_mut() {
                    let result = Self::resolve_expression_kind(parser, source, &argument.value, false);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_property_decorator(parser: &ASTParser, source: &Source, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = parser.global_property_decorators();
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = parser.global_property_decorators();
                let mut accessible = d.get(&identifier.name);
                let mut arg_list: Option<ArgumentList> = None;
                for (index, expression) in unit.expressions.iter().enumerate() {
                    if index == 0 { continue }
                    match expression {
                        ExpressionKind::ArgumentList(argument_list) => {
                            arg_list = Some(argument_list.clone());
                        }
                        ExpressionKind::Subscript(_subscript) => {
                            panic!("Cannot access decorator object with subscript.")
                        }
                        ExpressionKind::Identifier(identifier) => {
                            accessible = accessible.access_property(&identifier.name).as_accessible().unwrap()
                        }
                        _ => panic!()
                    }
                }
                decorator.accessible = Some(accessible.clone());
                for argument in arg_list.as_mut().unwrap().arguments.iter_mut() {
                    let result = Self::resolve_expression_kind(parser, source, &argument.value, false);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_relation_decorator(parser: &ASTParser, source: &Source, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = parser.global_relation_decorators();
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = parser.global_relation_decorators();
                let mut accessible = d.get(&identifier.name);
                let mut arg_list: Option<ArgumentList> = None;
                for (index, expression) in unit.expressions.iter().enumerate() {
                    if index == 0 { continue }
                    match expression {
                        ExpressionKind::ArgumentList(argument_list) => {
                            arg_list = Some(argument_list.clone());
                        }
                        ExpressionKind::Subscript(_subscript) => {
                            panic!("Cannot access decorator object with subscript.")
                        }
                        ExpressionKind::Identifier(identifier) => {
                            accessible = accessible.access_property(&identifier.name).as_accessible().unwrap()
                        }
                        _ => panic!()
                    }
                }
                decorator.accessible = Some(accessible.clone());
                for argument in arg_list.as_mut().unwrap().arguments.iter_mut() {
                    let result = Self::resolve_expression_kind(parser, source, &argument.value, false);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_pipeline(parser: &ASTParser, source: &Source, pipeline: &ASTPipeline) -> Entity {
        let mut items: Vec<ASTPipelineItem> = vec![];
        match pipeline.expression.as_ref() {
            ExpressionKind::Identifier(identifier) => {
                let installer = parser.global_pipeline_installers().get(&identifier.name);
                if let Some(installer) = installer {
                    items.push(ASTPipelineItem {
                        installer: Some(installer.clone()),
                        function_installer: None,
                        lookup_table: parser.callback_lookup_table,
                        args: vec![]
                    })
                } else {
                    let installer = parser.global_function_installers().get(&identifier.name);
                    if let Some(installer) = installer {
                        items.push(ASTPipelineItem {
                            installer: None,
                            function_installer: Some(installer.clone()),
                            lookup_table: parser.callback_lookup_table,
                            args: vec![]
                        })
                    } else {
                        panic!("Cannot find pipeline item named '{}'.", identifier.name);
                    }
                }
            }
            ExpressionKind::Unit(unit) => {
                let mut previous_identifier: Option<&ASTIdentifier> = None;
                for expression in &unit.expressions {
                    match expression {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(previous_identifier) = previous_identifier {
                                let installer = parser.global_pipeline_installers().get(&previous_identifier.name);
                                if let Some(installer) = installer {
                                    items.push(ASTPipelineItem { installer: Some(installer.clone()), function_installer: None, lookup_table: parser.callback_lookup_table, args: vec![]});
                                } else {
                                    panic!("Cannot find pipeline item named '{}'.", identifier.name);
                                }
                            }
                            previous_identifier = Some(&identifier);
                        }
                        ExpressionKind::ArgumentList(argument_list) => {
                            let args = argument_list.to_mut();
                            for (index, arg) in &mut args.arguments.iter_mut().enumerate() {
                                let value = if ((&previous_identifier.unwrap().name == "when") || (&previous_identifier.unwrap().name == "redirect")) && index == 0 {
                                    Self::resolve_expression_kind_force_value(parser, source, &arg.value, true)
                                } else {
                                    Self::resolve_expression_kind_force_value(parser, source, &arg.value, false)
                                };
                                arg.resolved = Some(Entity::Value(value));
                            }
                            let installer = parser.global_pipeline_installers().get(&previous_identifier.unwrap().name);
                            if let Some(installer) = installer {
                                items.push(ASTPipelineItem { installer: Some(installer.clone()), function_installer: None, lookup_table: parser.callback_lookup_table, args: argument_list.arguments().clone()});
                            } else {
                                let installer = parser.global_function_installers().get(&previous_identifier.unwrap().name);
                                if let Some(installer) = installer {
                                    items.push(ASTPipelineItem { installer: None, function_installer: Some(installer.clone()), lookup_table: parser.callback_lookup_table, args: argument_list.arguments().clone()});
                                } else {
                                    panic!("Cannot find pipeline item named '{}'.", previous_identifier.unwrap().name);
                                }
                            }
                            previous_identifier = None;
                        }
                        _ => panic!()
                    }
                }
                if let Some(previous_identifier) = previous_identifier {
                    let installer = parser.global_pipeline_installers().get(&previous_identifier.name);
                    if let Some(installer) = installer {
                        items.push(ASTPipelineItem { installer: Some(installer.clone()), function_installer: None, lookup_table: parser.callback_lookup_table, args: vec![]});
                    } else {
                        panic!("Cannot find pipeline item named '{}'.", previous_identifier.name);
                    }
                }
            }
            _ => panic!()
        }
        let ast_pipeline = ASTResolvedPipeline { items };
        let value_pipeline = ast_pipeline.to_value_pipeline();
        Entity::Value(Value::Pipeline(value_pipeline))
    }

    fn resolve_field(parser: &ASTParser, source: &Source, field: &mut ASTField) {
        field.figure_out_class();
        match &field.field_class {
            ASTFieldClass::Field => {
                for decorator in field.decorators.iter_mut() {
                    Self::resolve_field_decorator(parser, source, decorator);
                }
            }
            ASTFieldClass::Relation => {
                for decorator in field.decorators.iter_mut() {
                    Self::resolve_relation_decorator(parser, source, decorator);
                }
            }
            ASTFieldClass::Property => {
                for decorator in field.decorators.iter_mut() {
                    Self::resolve_property_decorator(parser, source, decorator);
                }
            }
            _ => {}
        }
        field.resolved = true;
    }

    pub(crate) fn resolve_connector(parser: &ASTParser) -> DatabaseName {
        if parser.connector.is_none() {
            panic!("Connector is not defined.");
        }
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let top = source.to_mut().tops.get_mut(&connector_ref.1).unwrap();
        let connector = top.as_connector_mut().unwrap();
        for item in connector.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let provider_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let provider_str = provider_value.as_raw_enum_choice().unwrap();
                    match provider_str {
                        #[cfg(feature = "data-source-sqlite")]
                        "sqlite" => connector.provider = Some(DatabaseName::SQLite),
                        "mongo" => connector.provider = Some(DatabaseName::MongoDB),
                        "mysql" => connector.provider = Some(DatabaseName::MySQL),
                        "postgres" => connector.provider = Some(DatabaseName::PostgreSQL),
                        _ => panic!("Unrecognized provider. {}", provider_str)
                    }
                },
                "url" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let url_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let url_str = url_value.as_str().unwrap();
                    connector.url = Some(url_str.to_owned());
                },
                _ => { panic!("Undefined name '{}' in connector block.", item.identifier.name.as_str())}
            }
        }
        connector.provider.unwrap()
    }

    pub(crate) fn resolve_client_generator(parser: &ASTParser, source: &Source, client: &mut ASTClient) {
        for item in client.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let provider_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let provider_str = provider_value.as_raw_enum_choice().unwrap();
                    match provider_str {
                        "javaScript" | "typeScript" => client.provider = Some(Kind::TypeScript),
                        "swift" => client.provider = Some(Kind::Swift),
                        "kotlin" => client.provider = Some(Kind::Kotlin),
                        "cSharp" => client.provider = Some(Kind::CSharp),
                        "dart" => client.provider = Some(Kind::Dart),
                        _ => panic!("Unrecognized client generator provider. {}", provider_str)
                    }
                },
                "dest" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let dest_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let dest_str = dest_value.as_str().unwrap();
                    let mut dest_path = source.path.clone();
                    dest_path.pop();
                    let dest = dest_path.join(PathBuf::from(dest_str));
                    let absolute = dest.absolutize().unwrap();
                    client.dest = Some(absolute.as_ref().to_owned());
                },
                "package" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let package_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let package_bool = package_value.as_bool().unwrap();
                    client.package = Some(package_bool);
                },
                "host" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let host_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let host_str = host_value.as_str().unwrap();
                    client.host = Some(host_str.to_owned());
                },
                "objectName" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let object_name_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let object_name_str = object_name_value.as_str().unwrap();
                    client.object_name = object_name_str.to_owned();
                },
                "gitCommit" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let git_commit_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let git_commit_bool = git_commit_value.as_bool().unwrap();
                    client.git_commit = git_commit_bool;
                }
                _ => { panic!("Undefined name '{}' in client generator block.", item.identifier.name.as_str())}
            }
        }
    }

    pub(crate) fn resolve_model_entity_generator(parser: &ASTParser, source: &Source, generator: &mut ASTEntity) {
        for item in generator.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let provider_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let provider_str = provider_value.as_raw_enum_choice().unwrap();
                    match provider_str {
                        "rust" => generator.provider = Some(ProgramLang::Rust),
                        "node" => generator.provider = Some(ProgramLang::NodeJS),
                        "python" => generator.provider = Some(ProgramLang::Python),
                        "go" => generator.provider = Some(ProgramLang::Go),
                        "java" => generator.provider = Some(ProgramLang::Java),
                        _ => panic!("Unrecognized entity generator provider. {}", provider_str)
                    }
                },
                "dest" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let dest_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let mut dest = source.path.clone();
                    dest.pop();
                    dest.push(PathBuf::from(dest_value.as_str().unwrap()));
                    let absolute = dest.absolutize().unwrap();
                    generator.dest = Some(absolute.as_ref().to_owned());
                },
                _ => { panic!("Undefined name '{}' in entity generator block.", item.identifier.name.as_str())}
            }
        }
    }

    pub(crate) fn resolve_data_set(parser: &ASTParser, source: &Source, data_set: &mut DataSet) {
        for group in data_set.groups.iter_mut() {
            for record in group.records.iter_mut() {
                record.resolved = Some(Self::resolve_dictionary_literal(parser, source, &record.dictionary).as_value().unwrap().clone());
            }
        }
    }

    pub(crate) fn resolve_debug_conf(parser: &ASTParser, source: &Source, config: &mut ASTDebugConf) {
        for item in config.items.iter_mut() {
            match item.identifier.name.as_str() {
                "logQueries" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match value {
                        Value::Bool(_b) => config.log_queries = true,
                        Value::Null => (),
                        _ => panic!(),
                    }
                },
                "logMigrations" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match value {
                        Value::Bool(_b) => config.log_migrations = true,
                        Value::Null => (),
                        _ => panic!(),
                    }
                },
                "logSeedRecords" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match value {
                        Value::Bool(_b) => config.log_seed_records = true,
                        Value::Null => (),
                        _ => panic!(),
                    }
                },
                _ => panic!()
            }
        }
    }

    pub(crate) fn resolve_test_conf(parser: &ASTParser, source: &Source, config: &mut ASTTestConf) {
        for item in config.items.iter_mut() {
            match item.identifier.name.as_str() {
                "resetAfterFind" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    config.reset_after_find = value;
                },
                _ => panic!()
            }
        }
    }

    pub(crate) fn resolve_action_group(parser: &ASTParser, source: &Source, action_group_declaration: &mut ActionGroupDeclaration) {
        for action in &mut action_group_declaration.actions {
            Self::resolve_custom_action_declaration(parser, source, action)
        }
    }

    pub(crate) fn resolve_custom_action_declaration(parser: &ASTParser, source: &Source, action: &mut ActionDeclaration) {
        let input_interface_name = action.input_type.name.name.as_str();
        let binding = parser.interfaces();
        let interface = match binding.iter().find(|i| i.name.name.name.as_str() == input_interface_name) {
            Some(i) => i,
            None => panic!("Interface with name '{}' is not found.", input_interface_name)
        };
        action.resolved_input_interface = Some((interface.source_id, interface.id));
        action.resolved_input_shape = Some(Self::resolve_action_input_shape(parser, source, *interface, &action.input_type, false));
    }

    pub(crate) fn resolve_action_input_shape(parser: &ASTParser, source: &Source, interface: &InterfaceDeclaration, input_type: &InterfaceType, optional: bool) -> ResolvedInterfaceField {
        let map: HashMap<String, InterfaceType> = interface.args().iter().enumerate().map(|(i, a)| {
            (a.name.name.clone(), input_type.args.get(i).unwrap().clone())
        }).collect();
        let mut shape: HashMap<String, ResolvedInterfaceField> = hashmap!{};
        for extend in &interface.extends {
            let interface = Self::search_interface_by_name(parser, source, extend.name.name.as_str());
            Self::install_interface_items_to_shape(parser, source, &map, &interface.items, &mut shape);
        }
        Self::install_interface_items_to_shape(parser, source, &map, &interface.items, &mut shape);
        ResolvedInterfaceFieldType::Shape(shape).optional(optional)
    }

    pub(crate) fn install_interface_items_to_shape(parser: &ASTParser, source: &Source, map: &HashMap<String, InterfaceType>, items: &Vec<InterfaceItemDeclaration>, shape: &mut HashMap<String, ResolvedInterfaceField>) {
        for item in items {
            if Self::need_to_alter_generics_with_map(parser, source, map, &item.kind) {
                let replaced_type = item.kind.alter_generics_with(map);
                Self::install_interface_items_with_generics_filled_to_shape(parser, source, &item.name, &replaced_type, shape);
            } else {
                Self::install_interface_items_with_generics_filled_to_shape(parser, source, &item.name, &item.kind, shape);
            }
        }
    }

    pub(crate) fn install_interface_items_with_generics_filled_to_shape(parser: &ASTParser, source: &Source, name: &ASTIdentifier, kind: &InterfaceType, shape: &mut HashMap<String, ResolvedInterfaceField>) {
        shape.insert(name.name.clone(), Self::resolve_type_with_filled_generics(parser, source, kind));
    }

    pub(crate) fn resolve_predefined_interface_type(parser: &ASTParser, source: &Source, a: &InterfaceType) -> Option<ResolvedInterfaceFieldType> {
        Some(match a.name.name.as_str() {
            "Data" => ResolvedInterfaceFieldType::Shape(hashmap!{"data".to_owned() => Self::resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap())}),
            "DataMeta" => ResolvedInterfaceFieldType::Shape(hashmap!{
                "data".to_owned() => Self::resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap()),
                "meta".to_owned() => Self::resolve_type_with_filled_generics(parser, source, a.args.get(1).unwrap()),
            }),
            _ => None?,
        })
    }

    // we're not handle arrays, maps, enums yet
    pub(crate) fn resolve_type_with_filled_generics(parser: &ASTParser, source: &Source, a: &InterfaceType) -> ResolvedInterfaceField {
        let result_without_arity = match a.name.name.as_str() {
            "String" => ResolvedInterfaceFieldType::String.optional(a.optional),
            "ObjectId" => ResolvedInterfaceFieldType::ObjectId.optional(a.optional),
            "Bool" => ResolvedInterfaceFieldType::Bool.optional(a.optional),
            "Int32" | "Int" => ResolvedInterfaceFieldType::I32.optional(a.optional),
            "Int64" => ResolvedInterfaceFieldType::I64.optional(a.optional),
            "Float" | "Float64" => ResolvedInterfaceFieldType::F64.optional(a.optional),
            "Float32" => ResolvedInterfaceFieldType::F32.optional(a.optional),
            "Decimal" => ResolvedInterfaceFieldType::Decimal.optional(a.optional),
            "Date" => ResolvedInterfaceFieldType::Date.optional(a.optional),
            "DateTime" => ResolvedInterfaceFieldType::DateTime.optional(a.optional),
            "Array" => ResolvedInterfaceFieldType::Vec(Box::new(Self::resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap()))).optional(a.collection_optional),
            "Dict" => ResolvedInterfaceFieldType::HashMap(Box::new(Self::resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap()))).optional(a.collection_optional),
            "Any" => ResolvedInterfaceFieldType::Any.optional(a.optional),
            // other user defined interfaces
            _ => {
                if let Some(result) = Self::resolve_predefined_interface_type(parser, source, a) {
                    result.optional(a.optional)
                } else {
                    let interface_name = a.name.name.as_str();
                    let interface = Self::search_interface_by_name(parser, source, interface_name);
                    Self::resolve_action_input_shape(parser, source, interface, a, a.optional)
                }
            }
        };
        match a.arity {
            Arity::Scalar => result_without_arity,
            Arity::Array => ResolvedInterfaceFieldType::Vec(Box::new(result_without_arity)).optional(a.collection_optional),
            Arity::Dictionary => ResolvedInterfaceFieldType::HashMap(Box::new(result_without_arity)).optional(a.collection_optional),
        }
    }

    pub(crate) fn search_interface_by_name<'a>(parser: &'a ASTParser, _source: &'a Source, name: &str) -> &'a InterfaceDeclaration {
        let binding = parser.interfaces();
        let interface = match binding.iter().find(|i| i.name.name.name.as_str() == name) {
            Some(i) => i,
            None => panic!("Interface with name '{}' is not found.", name)
        };
        *interface
    }

    pub(crate) fn need_to_alter_generics_with_map(parser: &ASTParser, source: &Source, map: &HashMap<String, InterfaceType>, def: &InterfaceType) -> bool {
        // if def.arity != Arity::Scalar {
        //     return true;
        // }
        if map.contains_key(&def.name.name) {
            return true;
        }
        for arg in &def.args {
            if Self::need_to_alter_generics_with_map(parser, source, map, arg) {
                return true;
            }
        }
        return false;
    }

    pub(crate) fn resolve_server_config_block(parser: &ASTParser, source: &Source, config: &mut ASTServer) {
        for item in config.items.iter_mut() {
            match item.identifier.name.as_str() {
                "bind" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let bind_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match bind_value.as_tuple() {
                        Some(tuple_vec) => {
                            let arg1 = tuple_vec.get(0).unwrap();
                            let arg2 = tuple_vec.get(1).unwrap();
                            let str = arg1.as_str().unwrap().to_owned();
                            let int = arg2.as_i32().unwrap().to_owned();
                            config.bind = Some((str, int as u16));
                        }
                        None => panic!("Argument to 'bind' should be a tuple.")
                    }
                }
                "jwtSecret" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let jwt_secret_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match jwt_secret_value {
                        Value::Null => (),
                        Value::String(s) => config.jwt_secret = Some(s.clone()),
                        _ => panic!("Value of 'jwtSecret' should be string.")
                    }
                }
                "pathPrefix" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let path_prefix_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match path_prefix_value {
                        Value::Null => (),
                        Value::String(s) => config.path_prefix = Some(s.clone()),
                        _ => panic!("Value of 'pathPrefix' should be string.")
                    }
                }
                _ => { panic!("Undefined name '{}' in config block.", item.identifier.name.as_str())}
            }
        }
    }

    // Expression

    pub(crate) fn resolve_expression<'a>(parser: &ASTParser, source: &Source, expression: &mut Expression) {
        expression.resolved = Some(Self::resolve_expression_kind(parser, source, &mut expression.kind, false));
    }

    pub(crate) fn resolve_expression_kind(parser: &ASTParser, source: &Source, expression_kind: &ExpressionKind, when_option: bool) -> Entity {
        match expression_kind {
            ExpressionKind::Group(group) => {
                Self::resolve_group(parser, source, group, when_option)
            }
            ExpressionKind::NullishCoalescing(nullish_coalescing) => {
                Self::resolve_nullish_coalescing(parser, source, nullish_coalescing)
            }
            ExpressionKind::Negation(negation) => {
                Self::resolve_negation(parser, source, negation)
            }
            ExpressionKind::BitwiseNegation(negation) => {
                Self::resolve_bitwise_negation(parser, source, negation, when_option)
            }
            ExpressionKind::ArithExpr(arith) => {
                Self::resolve_arith_expr(parser, source, arith, when_option)
            }
            ExpressionKind::NumericLiteral(n) => {
                Self::resolve_numeric_literal(n)
            }
            ExpressionKind::StringLiteral(s) => {
                Self::resolve_string_literal(s)
            }
            ExpressionKind::RegExpLiteral(r) => {
                Self::resolve_regexp_literal(r)
            }
            ExpressionKind::BoolLiteral(b) => {
                Self::resolve_bool_literal(b)
            }
            ExpressionKind::NullLiteral(n) => {
                Self::resolve_null_literal(n)
            }
            ExpressionKind::EnumChoiceLiteral(e) => {
                Self::resolve_enum_choice_literal(parser, source, e)
            }
            ExpressionKind::RangeLiteral(range_literal) => {
                Self::resolve_range_literal(parser, source, range_literal)
            }
            ExpressionKind::TupleLiteral(tuple_literal) => {
                Self::resolve_tuple_literal(parser, source, tuple_literal)
            }
            ExpressionKind::ArrayLiteral(array_literal) => {
                Self::resolve_array_literal(parser, source, array_literal, when_option)
            }
            ExpressionKind::DictionaryLiteral(dictionary_literal) => {
                Self::resolve_dictionary_literal(parser, source, dictionary_literal)
            }
            ExpressionKind::Identifier(identifier) => {
                Self::resolve_identifier(parser, source, identifier, None)
            }
            ExpressionKind::ArgumentList(_a) => {
                panic!("Argument list cannot appear alone.")
            }
            ExpressionKind::Subscript(_s) => {
                panic!("Subscript cannot appear alone.")
            }
            ExpressionKind::Unit(unit) => {
                Self::resolve_unit(parser, source, unit)
            }
            ExpressionKind::Pipeline(pipeline) => {
                Self::resolve_pipeline(parser, source, pipeline)
            }
        }
    }

    fn resolve_expression_kind_force_value(parser: &ASTParser, source: &Source, expression_kind: &ExpressionKind, when_option: bool) -> Value {
        let entity = Self::resolve_expression_kind(parser, source, expression_kind, when_option);
        Self::unwrap_into_value_if_needed(parser, source, &entity)
    }

    // identifier

    fn resolve_group(parser: &ASTParser, source: &Source, group: &Group, when_option: bool) -> Entity {
        Self::resolve_expression_kind(parser, source, &group.expression, when_option)
    }

    fn resolve_identifier(parser: &ASTParser, source: &Source, identifier: &ASTIdentifier, parent: Option<&Entity>) -> Entity {
        match parent {
            Some(parent) => {
                if parent.is_accessible() {
                    let parent = parent.as_accessible().unwrap();
                    if parent.is_container() {
                        let container = parent.as_container().unwrap();
                        let result = container.objects.get(&identifier.name);
                        match result {
                            Some(entity) => entity.clone(),
                            None => panic!("Cannot access {}", identifier.name),
                        }
                    } else {
                        panic!("Cannot access {}", identifier.name);
                    }
                } else {
                    panic!("Cannot access {}", identifier.name);
                }
            }
            None => {
                match Self::find_identifier_origin_in_source(parser, source, identifier) {
                    Some(reference) => Entity::Reference(reference),
                    None => Container::std_global_constants().access_property(&identifier.name).clone()
                }
            }
        }
    }

    fn resolve_unit(parser: &ASTParser, source: &Source, unit: &Unit) -> Entity {
        let first_expression = unit.expressions.get(0).unwrap();
        let mut entity = Self::resolve_expression_kind(parser, source, first_expression, false);
        for (index, expression) in unit.expressions.iter().enumerate() {
            if index == 0 { continue }
            entity = Self::resolve_accessor(parser, source, expression, &entity);
        }
        return entity
    }

    fn resolve_accessor(parser: &ASTParser, source: &Source, expression_kind: &ExpressionKind, entity: &Entity) -> Entity {
        match expression_kind {
            ExpressionKind::Subscript(subscript) => {
                Self::resolve_subscript(parser, source, subscript, entity)
            }
            ExpressionKind::ArgumentList(argument_list) => {
                let mut args = argument_list.clone();
                for arg in &mut args.arguments.iter_mut() {
                    let value = Self::resolve_expression_kind_force_value(parser, source, &arg.value, false);
                    arg.resolved = Some(Entity::Value(value));
                }
                match entity.as_accessible().unwrap() {
                    Accessible::Callable(callable) => Entity::Value(callable(args.arguments())),
                    _ => unreachable!(),
                }
            }
            ExpressionKind::Identifier(identifier) => {
                Self::resolve_identifier(parser, source, identifier, Some(entity))
            }
            _ => panic!()
        }
    }

    fn resolve_subscript(parser: &ASTParser, source: &Source, subscript: &Subscript, entity: &Entity) -> Entity {
        let index_entity = Self::resolve_expression_kind(parser, source, &subscript.expression, false);
        let index_value = Self::unwrap_into_value_if_needed(parser, source, &index_entity);
        if entity.is_accessible() {
            let accessible = entity.as_accessible().unwrap();
            match accessible {
                Accessible::Env(env) => {
                    match index_value.as_str() {
                        Some(s) => Entity::Value(env.get_value(s)),
                        None => panic!("ENV can only be subscripted with string.")
                    }
                }
                _ => panic!("Cannot access subscript"),
            }
        } else {
            let entity_value = Self::unwrap_into_value_if_needed(parser, source, entity);
            match entity_value {
                Value::String(s) => {
                    match index_value.as_i64() {
                        Some(i) => Entity::Value(Value::String(s.chars().nth(i as usize).unwrap().to_string())),
                        None => panic!("String can only be subscripted with integer.")
                    }
                }
                Value::Vec(v) => {
                    match index_value.as_i64() {
                        Some(i) => Entity::Value(v.get(i as usize).unwrap().clone()),
                        None => panic!("Array can only be subscripted with integer.")
                    }
                }
                Value::HashMap(m) => {
                    match index_value.as_str() {
                        Some(s) => Entity::Value(m.get(s).unwrap().clone()),
                        None => panic!("Map can only be subscripted with string.")
                    }
                }
                Value::BTreeMap(m) => {
                    match index_value.as_str() {
                        Some(s) => Entity::Value(m.get(s).unwrap().clone()),
                        None => panic!("Map can only be subscripted with string.")
                    }
                }
                Value::IndexMap(m) => {
                    match index_value.as_str() {
                        Some(s) => Entity::Value(m.get(s).unwrap().clone()),
                        None => panic!("Map can only be subscripted with string.")
                    }
                }
                _ => {
                    panic!("")
                }
            }
        }
    }

    // literals and operators

    fn resolve_numeric_literal(n: &NumericLiteral) -> Entity {
        let i = i32::from_str(&n.value);
        if i.is_ok() {
            return Entity::Value(Value::I32(i.unwrap()));
        }
        let i = i64::from_str(&n.value);
        if i.is_ok() {
            return Entity::Value(Value::I64(i.unwrap()));
        }
        let i = f64::from_str(&n.value);
        if i.is_ok() {
            return Entity::Value(Value::F64(i.unwrap()));
        }
        panic!("Cannot resolve numeric value: {}.", n.value.as_str())
    }

    fn resolve_string_literal(s: &StringLiteral) -> Entity {
        return Entity::Value(Value::String(unescape(s.value.as_str()).unwrap()));
    }

    fn resolve_regexp_literal(r: &RegExpLiteral) -> Entity {
        return Entity::Value(Value::RegExp(Regex::new(r.value.as_str()).unwrap()));
    }

    fn resolve_bool_literal(b: &BoolLiteral) -> Entity {
        match b.value.as_str() {
            "true" => Entity::Value(Value::Bool(true)),
            "false" => Entity::Value(Value::Bool(false)),
            _ => panic!("Cannot resolve bool value: {}", b.value.as_str())
        }
    }

    fn resolve_null_literal(_: &NullLiteral) -> Entity {
        Entity::Value(Value::Null)
    }

    fn resolve_enum_choice_literal(parser: &ASTParser, source: &Source, e: &EnumChoiceLiteral) -> Entity {
        if e.argument_list.is_some() {
            Entity::Value(Value::RawEnumChoice(e.value.clone(), Some(Self::resolve_argument_list_as_tuple_vec(parser, source, e.argument_list.as_ref().unwrap()))))
        } else {
            Entity::Value(Value::RawEnumChoice(e.value.clone(), None))
        }
    }

    fn resolve_argument_list_as_tuple_vec(parser: &ASTParser, source: &Source, arg_list: &ArgumentList) -> Vec<(Option<String>, Value)> {
        let mut result = vec![];
        for arg in arg_list.arguments.iter() {
            let name = arg.name.as_ref().map(|i| i.name.clone());
            let resolve_result = Self::resolve_expression_kind(parser, source, &arg.value, false);
            let value = Self::unwrap_into_value_if_needed(parser, source, &resolve_result);
            result.push((name, value));
        }
        result
    }

    fn resolve_range_literal(parser: &ASTParser, source: &Source, range_literal: &RangeLiteral) -> Entity {
        let a = Self::resolve_expression_kind(parser, source, range_literal.expressions.get(0).unwrap(), false);
        let a_v = Self::unwrap_into_value_if_needed(parser, source, &a);
        let start = Box::new(a_v);
        let b = Self::resolve_expression_kind(parser, source, range_literal.expressions.get(1).unwrap(), false);
        let b_v = Self::unwrap_into_value_if_needed(parser, source, &b);
        let end = Box::new(b_v);
        Entity::Value(Value::Range(Range { closed: range_literal.closed.clone(), start, end }))
    }

    fn resolve_tuple_literal(parser: &ASTParser, source: &Source, tuple_literal: &TupleLiteral) -> Entity {
        let mut resolved = vec![];
        for expression in tuple_literal.expressions.iter() {
            let e = Self::resolve_expression_kind(parser, source, expression, false);
            let v = Self::unwrap_into_value_if_needed(parser, source, &e);
            resolved.push(v);
        }
        Entity::Value(Value::Tuple(resolved))
    }

    fn resolve_array_literal(parser: &ASTParser, source: &Source, array_literal: &ArrayLiteral, when_option: bool) -> Entity {
        let mut resolved = vec![];
        for expression in array_literal.expressions.iter() {
            let e = Self::resolve_expression_kind(parser, source, expression, when_option);
            let v = Self::unwrap_into_value_if_needed(parser, source, &e);
            resolved.push(v);
        }
        Entity::Value(Value::Vec(resolved))
    }

    fn resolve_dictionary_literal(parser: &ASTParser, source: &Source, dic: &DictionaryLiteral) -> Entity {
        let mut resolved: HashMap<String, Value> = HashMap::new();
        for (key, value) in dic.expressions.iter() {
            let k = Self::resolve_expression_kind(parser, source, key, false);
            let k = Self::unwrap_into_value_if_needed(parser, source, &k);
            let v = Self::resolve_expression_kind(parser, source, value, false);
            let v = Self::unwrap_into_value_if_needed(parser, source, &v);
            resolved.insert(k.as_str().unwrap().to_string(), v);
        }
        Entity::Value(Value::HashMap(resolved))
    }

    fn resolve_nullish_coalescing(parser: &ASTParser, source: &Source, nullish_coalescing: &NullishCoalescing) -> Entity {
        let mut resolved = Entity::Value(Value::Null);
        for e in nullish_coalescing.expressions.iter() {
            resolved = Self::resolve_expression_kind(parser, source, e, false);
            if !resolved.is_null() {
                return resolved;
            }
        }
        return resolved
    }

    fn resolve_negation(parser: &ASTParser, source: &Source, negation: &Negation) -> Entity {
        let value = Self::resolve_expression_kind_force_value(parser, source, &negation.expression, false);
        Entity::Value(match value {
            Value::I32(v) => Value::I32(-v),
            Value::I64(v) => Value::I64(-v),
            Value::F32(v) => Value::F32(-v),
            Value::F64(v) => Value::F64(-v),
            _ => panic!("Cannot negate value {:?}", value)
        })
    }

    fn resolve_bitwise_negation(parser: &ASTParser, source: &Source, negation: &BitwiseNegation, when_option: bool) -> Entity {
        let value = Self::resolve_expression_kind_force_value(parser, source, &negation.expression, when_option);
        Entity::Value(match value {
            Value::I32(v) => Value::I32(!v),
            Value::I64(v) => Value::I64(!v),
            Value::RawEnumChoice(e, _) => if when_option {
                Value::RawOptionChoice(Action::from_name(&e).neg().to_u32())
            } else {
                panic!("Unhandled option bitwise operation")
            }
            Value::RawOptionChoice(o) => if when_option {
                Value::RawOptionChoice(Action::from_u32(o).neg().to_u32())
            } else {
                panic!("Unhandled option bitwise operation")
            },
            _ => panic!("Cannot negate value {:?}", value)
        })
    }

    fn resolve_arith_expr(parser: &ASTParser, source: &Source, arith_expr: &ArithExpr, when_option: bool) -> Entity {
        match arith_expr {
            ArithExpr::Expression(expression) => return Self::resolve_expression_kind(parser, source, &expression, when_option),
            ArithExpr::UnaryNeg(expression) => {
                let origin = Self::resolve_expression_kind_force_value(parser, source, &expression, when_option);
                return Entity::Value((-origin).unwrap());
            }
            ArithExpr::UnaryBitNeg(expression) => {
                let origin = Self::resolve_expression_kind_force_value(parser, source, &expression, when_option);
                return Entity::Value(match origin {
                    Value::I32(v) => Value::I32(!v),
                    Value::I64(v) => Value::I64(!v),
                    Value::RawEnumChoice(e, _) => if when_option {
                        Value::RawOptionChoice(Action::from_name(&e).neg().to_u32())
                    } else {
                        panic!("Unhandled option bitwise operation")
                    }
                    Value::RawOptionChoice(o) => if when_option {
                        Value::RawOptionChoice(Action::from_u32(o).neg().to_u32())
                    } else {
                        panic!("Unhandled option bitwise operation")
                    },
                    _ => panic!("Cannot negate value {:?}", origin)
                });
            }
            ArithExpr::BinaryOp { lhs, op, rhs } => {
                let lhs_value = Self::resolve_arith_expr(parser, source, &lhs, when_option).as_value().unwrap().clone();
                let rhs_value = Self::resolve_arith_expr(parser, source, &rhs, when_option).as_value().unwrap().clone();
                match op {
                    Op::Add => {
                        Entity::Value((lhs_value.clone() + rhs_value.clone()).unwrap())
                    }
                    Op::Sub => {
                        Entity::Value((lhs_value.clone() - rhs_value.clone()).unwrap())
                    }
                    Op::Mul => {
                        Entity::Value((lhs_value.clone() * rhs_value.clone()).unwrap())
                    }
                    Op::Div => {
                        Entity::Value((lhs_value.clone() / rhs_value.clone()).unwrap())
                    }
                    Op::Mod => {
                        Entity::Value((lhs_value.clone() % rhs_value.clone()).unwrap())
                    }
                    Op::BitAnd => {
                        if when_option {
                            let lhs_action = Self::value_to_action_option(&lhs_value);
                            let rhs_action = Self::value_to_action_option(&rhs_value);
                            Entity::Value(Value::RawOptionChoice(lhs_action.and(rhs_action).to_u32()))
                        } else {
                            Entity::Value((lhs_value.clone() & rhs_value.clone()).unwrap())
                        }
                    }
                    Op::BitXor => {
                        if when_option {
                            let lhs_action = Self::value_to_action_option(&lhs_value);
                            let rhs_action = Self::value_to_action_option(&rhs_value);
                            Entity::Value(Value::RawOptionChoice(lhs_action.xor(rhs_action).to_u32()))
                        } else {
                            Entity::Value((lhs_value.clone() ^ rhs_value.clone()).unwrap())
                        }
                    }
                    Op::BitOr => {
                        if when_option {
                            let lhs_action = Self::value_to_action_option(&lhs_value);
                            let rhs_action = Self::value_to_action_option(&rhs_value);
                            Entity::Value(Value::RawOptionChoice(lhs_action.or(rhs_action).to_u32()))
                        } else {
                            Entity::Value((lhs_value.clone() | rhs_value.clone()).unwrap())
                        }
                    }
                    _ => unreachable!()
                }
            }
        }
    }

    fn value_to_action_option(v: &Value) -> Action {
        match v {
            Value::RawEnumChoice(e, _) => Action::from_name(&e),
            Value::RawOptionChoice(u) => Action::from_u32(*u),
            _ => unreachable!()
        }
    }

    // Unwrap references

    fn find_identifier_origin_in_source(parser: &ASTParser, source: &Source, identifier: &ASTIdentifier) -> Option<Reference> {
        // test for constant
        for id in source.constants.iter() {
            let c = source.get_constant(*id);
            if &identifier.name == &c.identifier.name {
                return Some(Reference::ConstantReference((source.id, c.id)));
            }
        }
        // test for model
        for id in source.models.iter() {
            let m = source.get_model(*id);
            if &identifier.name == &m.identifier.name {
                return Some(Reference::ModelReference((source.id, m.id, identifier.name.clone())));
            }
        }
        // test for import
        for id in source.imports.iter() {
            let i = source.get_import(*id);
            let found = i.identifiers.iter().find(|i| &i.name == &identifier.name);
            if found.is_some() {
                let source_id = i.from_id.unwrap();
                let origin_source = parser.get_source(source_id);
                return Self::find_identifier_origin_in_source(parser, origin_source, identifier);
            }
        }
        None
    }

    fn constant_with_reference(parser: &ASTParser, _source: &Source, reference: (usize, usize)) -> Value {
        let source = parser.get_source(reference.0);
        let c = source.get_constant(reference.1);
        let entity = c.expression.resolved.as_ref().unwrap();
        Self::unwrap_into_value_if_needed(parser, source, entity)
    }

    fn unwrap_into_value_if_needed(parser: &ASTParser, source: &Source, entity: &Entity) -> Value {
        if entity.is_value() {
            return entity.as_value().unwrap().clone()
        } else if entity.is_reference() {
            let r = entity.as_reference().unwrap();
            return if r.is_constant_ref() {
                Self::constant_with_reference(parser, source, r.as_constant_ref().unwrap())
            } else {
                Value::RawEnumChoice(r.as_model_ref().unwrap().2.clone(), None)
            }
        } else {
            panic!("Cannot unwrap accessible into value.")
        }
    }
}
