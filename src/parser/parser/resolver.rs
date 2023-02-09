use std::str::FromStr;
use indexmap::map::IndexMap;
use regex::Regex;
use snailquote::unescape;
use crate::core::database::name::DatabaseName;
use crate::core::teon::range::Range;
use crate::parser::ast::accessible::{Accessible, ASTPipeline, ASTPipelineItem, Container};
use crate::parser::ast::argument::ArgumentList;
use crate::parser::ast::config::ServerConfig;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::entity::Entity;
use crate::parser::ast::expression::{AddSub, ArrayLiteral, BitwiseAnd, BitwiseNegation, BitwiseOr, BitwiseXor, BoolLiteral, DictionaryLiteral, EnumChoiceLiteral, Expression, ExpressionKind, MulDivMod, Negation, NullishCoalescing, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::parser::ast::field::{Field, FieldClass};
use crate::parser::ast::group::Group;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::import::Import;
use crate::parser::ast::model::Model;
use crate::parser::ast::pipeline::Pipeline;
use crate::parser::ast::r#enum::{Enum, EnumChoice};
use crate::parser::ast::reference::{Reference};
use crate::parser::ast::source::Source;
use crate::parser::ast::subscript::Subscript;
use crate::parser::ast::top::Top;
use crate::parser::ast::unit::Unit;
use crate::parser::parser::Parser;
use crate::parser::std::decorators::field::GlobalFieldDecorators;
use crate::parser::std::decorators::model::GlobalModelDecorators;
use crate::parser::std::decorators::property::GlobalPropertyDecorators;
use crate::parser::std::decorators::relation::GlobalRelationDecorators;
use crate::prelude::Value;
use to_mut::ToMut;
use crate::core::action::Action;
use crate::core::app::environment::Environment;
use crate::parser::ast::client::{Client, ClientLanguage};
use crate::parser::ast::generator::Generator;
use crate::parser::std::pipeline::global::{GlobalFunctionInstallers, GlobalPipelineInstallers};

pub(crate) struct Resolver { }

impl Resolver {

    pub(crate) fn resolve_parser(parser: &Parser) {
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

    pub(crate) fn resolve_source(parser: &Parser, source: &Source) {
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
                    Self::resolve_config(parser, source, config);
                }
            }
        }
        source.to_mut().resolved = true;
    }

    pub(crate) fn resolve_import(parser: &Parser, _source: &Source, import: &mut Import) {
        let from_source = parser.sources.iter().find(|(_source_id, source)| {
            &source.path == &import.path
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
            }
        }
        import.resolved = true;
    }

    pub(crate) fn resolve_constant(parser: &Parser, source: &Source, constant: &mut Constant) {
        Self::resolve_expression(parser, source, &mut constant.expression);
        constant.resolved = true;
    }

    pub(crate) fn resolve_enum(parser: &Parser, source: &Source, r#enum: &mut Enum) {
        for choice in r#enum.choices.iter_mut() {
            Self::resolve_enum_choice(parser, source, choice);
        }
        r#enum.resolved = true;
    }

    pub(crate) fn resolve_enum_choice(_parser: &Parser, _source: &Source, choice: &mut EnumChoice) {
        choice.resolved = true;
    }

    pub(crate) fn resolve_model(parser: &Parser, source: &Source, model: &mut Model) {
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

    fn resolve_model_decorator(parser: &Parser, source: &Source, decorator: &mut Decorator) {
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

    fn resolve_field_decorator(parser: &Parser, source: &Source, decorator: &mut Decorator) {
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

    fn resolve_property_decorator(parser: &Parser, source: &Source, decorator: &mut Decorator) {
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

    fn resolve_relation_decorator(parser: &Parser, source: &Source, decorator: &mut Decorator) {
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

    fn resolve_pipeline(parser: &Parser, source: &Source, pipeline: &Pipeline) -> Entity {
        let mut items: Vec<ASTPipelineItem> = vec![];
        match pipeline.expression.as_ref() {
            ExpressionKind::Identifier(identifier) => {
                let installer = parser.global_pipeline_installers().get(&identifier.name);
                if let Some(installer) = installer {
                    items.push(ASTPipelineItem {
                        installer: Some(installer.clone()),
                        function_installer: None,
                        lookup_table: None,
                        args: vec![]
                    })
                } else {
                    let installer = parser.global_function_installers().get(&identifier.name);
                    if let Some(installer) = installer {
                        items.push(ASTPipelineItem {
                            installer: None,
                            function_installer: Some(installer.clone()),
                            lookup_table: Some(parser.callback_lookup_table.clone()),
                            args: vec![]
                        })
                    } else {
                        panic!("Cannot find pipeline item named '{}'.", identifier.name);
                    }
                }
            }
            ExpressionKind::Unit(unit) => {
                let mut previous_identifier: Option<&Identifier> = None;
                for expression in &unit.expressions {
                    match expression {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(previous_identifier) = previous_identifier {
                                let installer = parser.global_pipeline_installers().get(&previous_identifier.name);
                                if let Some(installer) = installer {
                                    items.push(ASTPipelineItem { installer: Some(installer.clone()), function_installer: None, lookup_table: None, args: vec![]});
                                } else {
                                    panic!("Cannot find pipeline item named '{}'.", identifier.name);
                                }
                            }
                            previous_identifier = Some(&identifier);
                        }
                        ExpressionKind::ArgumentList(argument_list) => {
                            let mut args = argument_list.clone();
                            for (index, arg) in &mut args.arguments.iter_mut().enumerate() {
                                let value = if &previous_identifier.unwrap().name == "when" && index == 0 {
                                    Self::resolve_expression_kind_force_value(parser, source, &arg.value, true)
                                } else {
                                    Self::resolve_expression_kind_force_value(parser, source, &arg.value, false)
                                };
                                arg.resolved = Some(Entity::Value(value));
                            }
                            let installer = parser.global_pipeline_installers().get(&previous_identifier.unwrap().name);
                            if let Some(installer) = installer {
                                items.push(ASTPipelineItem { installer: Some(installer.clone()), function_installer: None, lookup_table: None, args: args.arguments});
                            } else {
                                let installer = parser.global_function_installers().get(&previous_identifier.unwrap().name);
                                if let Some(installer) = installer {
                                    items.push(ASTPipelineItem { installer: None, function_installer: Some(installer.clone()), lookup_table: Some(parser.callback_lookup_table.clone()), args: args.arguments});
                                } else {
                                    panic!("Cannot find pipeline item named '{}'.", previous_identifier.unwrap().name);
                                }
                            }
                            previous_identifier = None;
                        }
                        _ => panic!()
                    }
                }
            }
            _ => panic!()
        }
        let ast_pipeline = ASTPipeline { items };
        let value_pipeline = ast_pipeline.to_value_pipeline();
        Entity::Value(Value::Pipeline(value_pipeline))
    }

    fn resolve_field(parser: &Parser, source: &Source, field: &mut Field) {
        field.figure_out_class();
        match &field.field_class {
            FieldClass::Field => {
                for decorator in field.decorators.iter_mut() {
                    Self::resolve_field_decorator(parser, source, decorator);
                }
            }
            FieldClass::Relation => {
                for decorator in field.decorators.iter_mut() {
                    Self::resolve_relation_decorator(parser, source, decorator);
                }
            }
            FieldClass::Property => {
                for decorator in field.decorators.iter_mut() {
                    Self::resolve_property_decorator(parser, source, decorator);
                }
            }
            _ => {}
        }
        field.resolved = true;
    }

    pub(crate) fn resolve_connector(parser: &Parser) -> DatabaseName {
        if parser.connector.is_none() {
            panic!("Connector is not defined.");
        }
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let top = source.to_mut().tops.get_mut(&connector_ref.1).unwrap();
        let mut connector = top.as_connector_mut().unwrap();
        for item in connector.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let provider_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let provider_str = provider_value.as_raw_enum_choice().unwrap();
                    match provider_str {
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

    pub(crate) fn resolve_client_generator(parser: &Parser, source: &Source, client: &mut Client) {
        for item in client.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let provider_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let provider_str = provider_value.as_raw_enum_choice().unwrap();
                    match provider_str {
                        "javaScript" | "typeScript" => client.provider = Some(ClientLanguage::TypeScript),
                        "swift" => client.provider = Some(ClientLanguage::Swift),
                        "kotlin" => client.provider = Some(ClientLanguage::Kotlin),
                        "cSharp" => client.provider = Some(ClientLanguage::CSharp),
                        "dart" => client.provider = Some(ClientLanguage::Dart),
                        _ => panic!("Unrecognized client generator provider. {}", provider_str)
                    }
                },
                "dest" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let dest_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let dest_str = dest_value.as_str().unwrap();
                    client.dest = Some(dest_str.to_owned());
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
                    client.object_name = Some(object_name_str.to_owned());
                },
                _ => { panic!("Undefined name '{}' in entity generator block.", item.identifier.name.as_str())}
            }
        }
    }

    pub(crate) fn resolve_model_entity_generator(parser: &Parser, source: &Source, generator: &mut Generator) {
        for item in generator.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let provider_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let provider_str = provider_value.as_raw_enum_choice().unwrap();
                    match provider_str {
                        "rust" => generator.provider = Some(Environment::Rust),
                        "node" => generator.provider = Some(Environment::NodeJS),
                        "python" => generator.provider = Some(Environment::Python),
                        "go" => generator.provider = Some(Environment::Go),
                        "java" => generator.provider = Some(Environment::Java),
                        _ => panic!("Unrecognized entity generator provider. {}", provider_str)
                    }
                },
                "dest" => {
                    Self::resolve_expression(parser, source, &mut item.expression);
                    let dest_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let dest_str = dest_value.as_str().unwrap();
                    generator.dest = Some(dest_str.to_owned());
                },
                _ => { panic!("Undefined name '{}' in entity generator block.", item.identifier.name.as_str())}
            }
        }
    }

    pub(crate) fn resolve_config(parser: &Parser, source: &Source, config: &mut ServerConfig) {
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

    pub(crate) fn resolve_expression<'a>(parser: &Parser, source: &Source, expression: &mut Expression) {
        expression.resolved = Some(Self::resolve_expression_kind(parser, source, &mut expression.kind, false));
    }

    pub(crate) fn resolve_expression_kind(parser: &Parser, source: &Source, expression_kind: &ExpressionKind, when_option: bool) -> Entity {
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
            ExpressionKind::AddSub(add_sub) => {
                Self::resolve_add_sub(parser, source, add_sub)
            }
            ExpressionKind::MulDivMod(mul_div_mod) => {
                Self::resolve_mul_div_mod(parser, source, mul_div_mod)
            }
            ExpressionKind::BitwiseAnd(and) => {
                Self::resolve_bitwise_and(parser, source, and, when_option)
            }
            ExpressionKind::BitwiseXor(xor) => {
                Self::resolve_bitwise_xor(parser, source, xor, when_option)
            }
            ExpressionKind::BitwiseOr(or) => {
                Self::resolve_bitwise_or(parser, source, or, when_option)
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
                Self::resolve_enum_choice_literal(e)
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

    fn resolve_expression_kind_force_value(parser: &Parser, source: &Source, expression_kind: &ExpressionKind, when_option: bool) -> Value {
        let entity = Self::resolve_expression_kind(parser, source, expression_kind, when_option);
        Self::unwrap_into_value_if_needed(parser, source, &entity)
    }

    // identifier

    fn resolve_group(parser: &Parser, source: &Source, group: &Group, when_option: bool) -> Entity {
        Self::resolve_expression_kind(parser, source, &group.expression, when_option)
    }

    fn resolve_identifier(parser: &Parser, source: &Source, identifier: &Identifier, parent: Option<&Entity>) -> Entity {
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

    fn resolve_unit(parser: &Parser, source: &Source, unit: &Unit) -> Entity {
        let first_expression = unit.expressions.get(0).unwrap();
        let mut entity = Self::resolve_expression_kind(parser, source, first_expression, false);
        for (index, expression) in unit.expressions.iter().enumerate() {
            if index == 0 { continue }
            entity = Self::resolve_accessor(parser, source, expression, &entity);
        }
        return entity
    }

    fn resolve_accessor(parser: &Parser, source: &Source, expression_kind: &ExpressionKind, entity: &Entity) -> Entity {
        match expression_kind {
            ExpressionKind::Subscript(subscript) => {
                Self::resolve_subscript(parser, source, subscript, entity)
            }
            ExpressionKind::ArgumentList(_argument_list) => {
                // currently don't handle argument list yet
                panic!()
            }
            ExpressionKind::Identifier(identifier) => {
                Self::resolve_identifier(parser, source, identifier, Some(entity))
            }
            _ => panic!()
        }
    }

    fn resolve_subscript(parser: &Parser, source: &Source, subscript: &Subscript, entity: &Entity) -> Entity {
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

    fn resolve_enum_choice_literal(e: &EnumChoiceLiteral) -> Entity {
        Entity::Value(Value::RawEnumChoice(e.value.chars().skip(1).take(e.value.len() - 1).collect()))
    }

    fn resolve_range_literal(parser: &Parser, source: &Source, range_literal: &RangeLiteral) -> Entity {
        let a = Self::resolve_expression_kind(parser, source, range_literal.expressions.get(0).unwrap(), false);
        let a_v = Self::unwrap_into_value_if_needed(parser, source, &a);
        let start = Box::new(a_v);
        let b = Self::resolve_expression_kind(parser, source, range_literal.expressions.get(1).unwrap(), false);
        let b_v = Self::unwrap_into_value_if_needed(parser, source, &b);
        let end = Box::new(b_v);
        Entity::Value(Value::Range(Range { closed: range_literal.closed.clone(), start, end }))
    }

    fn resolve_tuple_literal(parser: &Parser, source: &Source, tuple_literal: &TupleLiteral) -> Entity {
        let mut resolved = vec![];
        for expression in tuple_literal.expressions.iter() {
            let e = Self::resolve_expression_kind(parser, source, expression, false);
            let v = Self::unwrap_into_value_if_needed(parser, source, &e);
            resolved.push(v);
        }
        Entity::Value(Value::Tuple(resolved))
    }

    fn resolve_array_literal(parser: &Parser, source: &Source, array_literal: &ArrayLiteral, when_option: bool) -> Entity {
        let mut resolved = vec![];
        for expression in array_literal.expressions.iter() {
            let e = Self::resolve_expression_kind(parser, source, expression, when_option);
            let v = Self::unwrap_into_value_if_needed(parser, source, &e);
            resolved.push(v);
        }
        Entity::Value(Value::Vec(resolved))
    }

    fn resolve_dictionary_literal(parser: &Parser, source: &Source, dic: &DictionaryLiteral) -> Entity {
        let mut resolved: IndexMap<String, Value> = IndexMap::new();
        for (key, value) in dic.expressions.iter() {
            let k = Self::resolve_expression_kind(parser, source, key, false);
            let k = Self::unwrap_into_value_if_needed(parser, source, &k);
            let v = Self::resolve_expression_kind(parser, source, value, false);
            let v = Self::unwrap_into_value_if_needed(parser, source, &v);
            resolved.insert(k.as_str().unwrap().to_string(), v);
        }
        Entity::Value(Value::IndexMap(resolved))
    }

    fn resolve_nullish_coalescing(parser: &Parser, source: &Source, nullish_coalescing: &NullishCoalescing) -> Entity {
        let mut resolved = Entity::Value(Value::Null);
        for e in nullish_coalescing.expressions.iter() {
            resolved = Self::resolve_expression_kind(parser, source, e, false);
            if !resolved.is_null() {
                return resolved;
            }
        }
        return resolved
    }

    fn resolve_negation(parser: &Parser, source: &Source, negation: &Negation) -> Entity {
        let value = Self::resolve_expression_kind_force_value(parser, source, &negation.expression, false);
        Entity::Value(match value {
            Value::I32(v) => Value::I32(-v),
            Value::I64(v) => Value::I64(-v),
            Value::F32(v) => Value::F32(-v),
            Value::F64(v) => Value::F64(-v),
            _ => panic!("Cannot negate value {:?}", value)
        })
    }

    fn resolve_add_sub(parser: &Parser, source: &Source, add_sub: &AddSub) -> Entity {
        let mut lhs = Self::resolve_expression_kind_force_value(parser, source, add_sub.expressions.get(0).unwrap(), false);
        for (index, expression) in add_sub.expressions.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let rhs = Self::resolve_expression_kind_force_value(parser, source, expression, false);
            match *add_sub.operators.get(index - 1).unwrap() {
                "+" => {
                    lhs = lhs + rhs;
                }
                "-" => {
                    lhs = lhs - rhs;
                }
                _ => unreachable!()
            }
        }
        Entity::Value(lhs)
    }

    fn resolve_mul_div_mod(parser: &Parser, source: &Source, mul_div_mod: &MulDivMod) -> Entity {
        let mut lhs = Self::resolve_expression_kind_force_value(parser, source, mul_div_mod.expressions.get(0).unwrap(), false);
        for (index, expression) in mul_div_mod.expressions.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let rhs = Self::resolve_expression_kind_force_value(parser, source, expression, false);
            match *mul_div_mod.operators.get(index - 1).unwrap() {
                "*" => {
                    lhs = lhs * rhs;
                }
                "/" => {
                    lhs = lhs / rhs;
                }
                "%" => {
                    lhs = lhs % rhs;
                }
                _ => unreachable!()
            }
        }
        Entity::Value(lhs)
    }

    fn resolve_bitwise_negation(parser: &Parser, source: &Source, negation: &BitwiseNegation, when_option: bool) -> Entity {
        let value = Self::resolve_expression_kind_force_value(parser, source, &negation.expression, when_option);
        Entity::Value(match value {
            Value::I32(v) => Value::I32(!v),
            Value::I64(v) => Value::I64(!v),
            Value::RawEnumChoice(e) => if when_option {
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

    fn value_to_action_option(v: &Value) -> Action {
        match v {
            Value::RawEnumChoice(e) => Action::from_name(&e),
            Value::RawOptionChoice(u) => Action::from_u32(*u),
            _ => unreachable!()
        }
    }

    fn resolve_bitwise_and(parser: &Parser, source: &Source, bitwise_and: &BitwiseAnd, when_mode: bool) -> Entity {
        let mut lhs = Self::resolve_expression_kind_force_value(parser, source, bitwise_and.expressions.get(0).unwrap(), false);
        for (index, expression) in bitwise_and.expressions.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let rhs = Self::resolve_expression_kind_force_value(parser, source, expression, false);
            if when_mode {
                let lhs_action = Self::value_to_action_option(&lhs);
                let rhs_action = Self::value_to_action_option(&rhs);
                lhs = Value::RawOptionChoice(lhs_action.and(rhs_action).to_u32());
            } else {
                lhs = lhs & rhs;
            }
        }
        Entity::Value(lhs)
    }

    fn resolve_bitwise_xor(parser: &Parser, source: &Source, bitwise_xor: &BitwiseXor, when_mode: bool) -> Entity {
        let mut lhs = Self::resolve_expression_kind_force_value(parser, source, bitwise_xor.expressions.get(0).unwrap(), false);
        for (index, expression) in bitwise_xor.expressions.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let rhs = Self::resolve_expression_kind_force_value(parser, source, expression, false);
            if when_mode {
                let lhs_action = Self::value_to_action_option(&lhs);
                let rhs_action = Self::value_to_action_option(&rhs);
                lhs = Value::RawOptionChoice(lhs_action.xor(rhs_action).to_u32());
            } else {
                lhs = lhs ^ rhs;
            }
        }
        Entity::Value(lhs)
    }

    fn resolve_bitwise_or(parser: &Parser, source: &Source, bitwise_or: &BitwiseOr, when_mode: bool) -> Entity {
        let mut lhs = Self::resolve_expression_kind_force_value(parser, source, bitwise_or.expressions.get(0).unwrap(), false);
        for (index, expression) in bitwise_or.expressions.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let rhs = Self::resolve_expression_kind_force_value(parser, source, expression, false);
            if when_mode {
                let lhs_action = Self::value_to_action_option(&lhs);
                let rhs_action = Self::value_to_action_option(&rhs);
                lhs = Value::RawOptionChoice(lhs_action.or(rhs_action).to_u32());
            } else {
                lhs = lhs | rhs;
            }
        }
        Entity::Value(lhs)
    }

    // Unwrap references

    fn find_identifier_origin_in_source(parser: &Parser, source: &Source, identifier: &Identifier) -> Option<Reference> {
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

    fn constant_with_reference(parser: &Parser, _source: &Source, reference: (usize, usize)) -> Value {
        let source = parser.get_source(reference.0);
        let c = source.get_constant(reference.1);
        let entity = c.expression.resolved.as_ref().unwrap();
        Self::unwrap_into_value_if_needed(parser, source, entity)
    }

    fn unwrap_into_value_if_needed(parser: &Parser, source: &Source, entity: &Entity) -> Value {
        if entity.is_value() {
            return entity.as_value().unwrap().clone()
        } else if entity.is_reference() {
            let r = entity.as_reference().unwrap();
            return if r.is_constant_ref() {
                Self::constant_with_reference(parser, source, r.as_constant_ref().unwrap())
            } else {
                Value::RawEnumChoice(r.as_model_ref().unwrap().2.clone())
            }
        } else {
            panic!("Cannot unwrap accessible into value.")
        }
    }
}
