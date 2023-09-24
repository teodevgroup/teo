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
use crate::parser::ast::expression::{ArrayLiteral, BitwiseNegation, BoolLiteral, DictionaryLiteral, EnumVariantLiteral, Expression, ExpressionKind, Negation, NullishCoalescing, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::parser::ast::field::{ASTField, ASTFieldClass};
use crate::parser::ast::group::Group;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::import::ASTImport;
use crate::parser::ast::model::ASTModel;
use crate::parser::ast::pipeline::ASTPipeline;
use crate::parser::ast::r#enum::{ASTEnum, EnumChoice};
use crate::parser::ast::reference::{Reference};
use crate::parser::ast::source::ASTSource;
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
use crate::parser::ast::data_set::ASTDataSet;
use crate::parser::ast::debug_conf::ASTDebugConf;
use crate::parser::ast::generator::ASTEntity;
use crate::parser::ast::interface::{InterfaceDeclaration, InterfaceItemDeclaration};
use crate::parser::ast::test_conf::ASTTestConf;
use crate::parser::ast::interface_type::InterfaceType;
use crate::parser::ast::namespace::ASTNamespace;
use crate::parser::ast::r#type::{Arity, ASTFieldType, TypeClass};
use crate::parser::ast::span::Span;
use crate::parser::ast::static_files::StaticFiles;
use crate::parser::diagnostics::diagnostics::{Diagnostics, DiagnosticsError};
use crate::parser::std::pipeline::global::{GlobalPipelineInstallers};

pub(crate) struct Resolver {
    pub(crate) global_model_decorators: GlobalModelDecorators,
    pub(crate) global_field_decorators: GlobalFieldDecorators,
    pub(crate) global_relation_decorators: GlobalRelationDecorators,
    pub(crate) global_property_decorators: GlobalPropertyDecorators,
    pub(crate) global_pipeline_installers: GlobalPipelineInstallers,
}

impl Resolver {

    pub(crate) fn new_for_connector() -> Self {
        Self {
            global_model_decorators: GlobalModelDecorators::new(),
            global_field_decorators: GlobalFieldDecorators::new(DatabaseName::SQLite),
            global_relation_decorators: GlobalRelationDecorators::new(),
            global_property_decorators: GlobalPropertyDecorators::new(),
            global_pipeline_installers: GlobalPipelineInstallers::new(),
        }
    }

    pub(crate) fn new(parser: &ASTParser) -> Self {
        let database_name = Self::new_for_connector().resolve_connector(parser);
        Self {
            global_model_decorators: GlobalModelDecorators::new(),
            global_field_decorators: GlobalFieldDecorators::new(database_name),
            global_relation_decorators: GlobalRelationDecorators::new(),
            global_property_decorators: GlobalPropertyDecorators::new(),
            global_pipeline_installers: GlobalPipelineInstallers::new(),
        }
    }

    pub(crate) fn resolve_parser(&self, parser: &ASTParser, diagnostics: &mut Diagnostics) {
        let main = parser.get_source(1);
        self.resolve_source_first_time(parser, main, diagnostics);
        for (index, source) in parser.sources.iter() {
            if *index == 1 { continue }
            self.resolve_source_first_time(parser, source, diagnostics);
        }
        for (index, source) in parser.sources.iter() {
            self.resolve_source_second_time(parser, source, diagnostics);
        }
        parser.to_mut().resolved = true;
    }

    pub(crate) fn resolve_source_first_time(&self, parser: &ASTParser, source: &ASTSource, diagnostics: &mut Diagnostics) {
        if source.resolved_first { return }
        for (_item_id, top) in source.to_mut().tops.iter_mut() {
            match top {
                Top::Import(import) => {
                    self.resolve_import(parser, source, import);
                }
                Top::Constant(constant) => {
                    self.resolve_constant(parser, source, constant);
                }
                Top::Enum(r#enum) => {
                    self.resolve_enum(parser, source, r#enum);
                }
                Top::Model(model) => {
                    self.resolve_model(parser, source, model, diagnostics);
                }
                Top::Connector(_connector) => {
                    continue;
                }
                Top::Generator(generator) => {
                    self.resolve_model_entity_generator(parser, source, generator);
                }
                Top::Client(client) => {
                    self.resolve_client_generator(parser, source, client);
                }
                Top::ServerConfig(config) => {
                    self.resolve_server_config_block(parser, source, config);
                }
                Top::DataSet(data_set) => {
                    // do not resolve yet
                }
                Top::DebugConf(debug_conf) => {
                    self.resolve_debug_conf(parser, source, debug_conf);
                }
                Top::TestConf(test_conf) => {
                    self.resolve_test_conf(parser, source, test_conf);
                }
                Top::MiddlewareDeclaration(middleware_declaration) => {
                    continue;
                }
                Top::ActionGroupDeclaration(action_group_declaration) => {
                    self.resolve_action_group(parser, source, action_group_declaration);
                }
                Top::InterfaceDeclaration(interface_declaration) => {
                    continue;
                }
                Top::StaticFiles(static_files) => {
                    self.resolve_static_files(parser, source, static_files);
                }
                Top::ASTNamespace(ast_namespace) => {
                    self.resolve_namespace_first_time(parser, source, ast_namespace, diagnostics);
                }
            }
        }
        source.to_mut().resolved_first = true;
    }

    pub(crate) fn resolve_source_second_time(&self, parser: &ASTParser, source: &ASTSource, diagnostics: &mut Diagnostics) {
        for (_item_id, top) in source.to_mut().tops.iter_mut() {
            match top {
                Top::DataSet(data_set) => {
                    self.resolve_data_set(parser, source, data_set, diagnostics);
                },
                Top::ASTNamespace(ast_namespace) => {
                    self.resolve_namespace_second_time(parser, source, ast_namespace, diagnostics);
                }
                _ => (),
            }
        }
        source.to_mut().resolved_second = true;
    }

    pub(crate) fn resolve_namespace_second_time(&self, parser: &ASTParser, source: &ASTSource, ast_namespace: &mut ASTNamespace, diagnostics: &mut Diagnostics) {
        for (_item_id, top) in ast_namespace.tops.iter_mut() {
            match top {
                Top::DataSet(data_set) => {
                    self.resolve_data_set(parser, source, data_set, diagnostics);
                },
                Top::ASTNamespace(ast_namespace) => {
                    self.resolve_namespace_second_time(parser, source, ast_namespace, diagnostics);
                }
                _ => ()
            }
        }
        ast_namespace.resolved = true;
    }


    pub(crate) fn resolve_namespace_first_time(&self, parser: &ASTParser, source: &ASTSource, ast_namespace: &mut ASTNamespace, diagnostics: &mut Diagnostics) {
        for (_item_id, top) in ast_namespace.tops.iter_mut() {
            match top {
                Top::Import(import) => {
                    self.resolve_import(parser, source, import);
                }
                Top::Constant(constant) => {
                    self.resolve_constant(parser, source, constant);
                }
                Top::Enum(r#enum) => {
                    self.resolve_enum(parser, source, r#enum);
                }
                Top::Model(model) => {
                    self.resolve_model(parser, source, model, diagnostics);
                }
                Top::Connector(_connector) => {
                    continue;
                }
                Top::Generator(generator) => {
                    self.resolve_model_entity_generator(parser, source, generator);
                }
                Top::Client(client) => {
                    self.resolve_client_generator(parser, source, client);
                }
                Top::ServerConfig(config) => {
                    self.resolve_server_config_block(parser, source, config);
                }
                Top::DataSet(_data_set) => {
                    // resolve later
                    // self.resolve_data_set(parser, source, data_set, diagnostics);
                }
                Top::DebugConf(debug_conf) => {
                    self.resolve_debug_conf(parser, source, debug_conf);
                }
                Top::TestConf(test_conf) => {
                    self.resolve_test_conf(parser, source, test_conf);
                }
                Top::MiddlewareDeclaration(middleware_declaration) => {
                    continue;
                }
                Top::ActionGroupDeclaration(action_group_declaration) => {
                    self.resolve_action_group(parser, source, action_group_declaration);
                }
                Top::InterfaceDeclaration(interface_declaration) => {
                    continue;
                }
                Top::StaticFiles(static_files) => {
                    self.resolve_static_files(parser, source, static_files);
                }
                Top::ASTNamespace(ast_namespace) => {
                    self.resolve_namespace_first_time(parser, source, ast_namespace, diagnostics);
                }
            }
        }
    }

    pub(crate) fn resolve_import(&self, parser: &ASTParser, _source: &ASTSource, import: &mut ASTImport) {
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

    pub(crate) fn resolve_constant(&self, parser: &ASTParser, source: &ASTSource, constant: &mut Constant) {
        self.resolve_expression(parser, source, &mut constant.expression);
        constant.resolved = true;
    }

    pub(crate) fn resolve_enum(&self, parser: &ASTParser, source: &ASTSource, r#enum: &mut ASTEnum) {
        for choice in r#enum.choices.iter_mut() {
            self.resolve_enum_choice(parser, source, choice);
        }
        r#enum.resolved = true;
    }

    pub(crate) fn resolve_enum_choice(&self, _parser: &ASTParser, _source: &ASTSource, choice: &mut EnumChoice) {
        choice.resolved = true;
    }

    pub(crate) fn resolve_model(&self, parser: &ASTParser, source: &ASTSource, model: &mut ASTModel, diagnostics: &mut Diagnostics) {
        // decorators
        for decorator in model.decorators.iter_mut() {
            self.resolve_model_decorator(parser, source, decorator);
        }
        // fields
        let ns_path = model.ns_path.clone();
        for field in model.fields.iter_mut() {
            self.resolve_field(parser, source, field, &ns_path, diagnostics);
        }
        model.resolved = true;
    }

    fn resolve_model_decorator(&self, parser: &ASTParser, source: &ASTSource, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = &self.global_model_decorators;
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = &self.global_model_decorators;
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
                    let result = self.resolve_expression_kind(parser, source, &argument.value, when_option);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_field_decorator(&self, parser: &ASTParser, source: &ASTSource, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = &self.global_field_decorators;
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = &self.global_field_decorators;
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
                    let result = self.resolve_expression_kind(parser, source, &argument.value, false);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_property_decorator(&self, parser: &ASTParser, source: &ASTSource, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = &self.global_property_decorators;
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = &self.global_property_decorators;
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
                    let result = self.resolve_expression_kind(parser, source, &argument.value, false);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_relation_decorator(&self, parser: &ASTParser, source: &ASTSource, decorator: &mut ASTDecorator) {
        match &decorator.expression {
            ExpressionKind::Identifier(identifier) => {
                let d = &self.global_relation_decorators;
                let accessible = d.get(&identifier.name);
                decorator.accessible = Some(accessible.clone());
                decorator.arguments = None;
            }
            ExpressionKind::Unit(unit) => {
                let identifier = unit.expressions.get(0).unwrap().as_identifier().unwrap();
                let d = &self.global_relation_decorators;
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
                    let result = self.resolve_expression_kind(parser, source, &argument.value, false);
                    let value = Self::unwrap_into_value_if_needed(parser, source, &result);
                    argument.resolved = Some(Entity::Value(value));
                }
                decorator.arguments = arg_list;
            }
            _ => panic!()
        }
        decorator.resolved = true;
    }

    fn resolve_pipeline(&self, parser: &ASTParser, source: &ASTSource, pipeline: &ASTPipeline) -> Entity {
        let mut items: Vec<ASTPipelineItem> = vec![];
        match pipeline.expression.as_ref() {
            ExpressionKind::Identifier(identifier) => {
                let installer = (&self.global_pipeline_installers).get(&identifier.name);
                if let Some(installer) = installer {
                    items.push(ASTPipelineItem {
                        installer: Some(installer.clone()),
                        args: vec![]
                    })
                } else {
                    panic!("Cannot find pipeline item named '{}'.", identifier.name);
                }
            }
            ExpressionKind::Unit(unit) => {
                let mut previous_identifier: Option<&ASTIdentifier> = None;
                for expression in &unit.expressions {
                    match expression {
                        ExpressionKind::Identifier(identifier) => {
                            if let Some(previous_identifier) = previous_identifier {
                                let installer = (&self.global_pipeline_installers).get(&previous_identifier.name);
                                if let Some(installer) = installer {
                                    items.push(ASTPipelineItem { installer: Some(installer.clone()), args: vec![]});
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
                                    self.resolve_expression_kind_force_value(parser, source, &arg.value, true)
                                } else {
                                    self.resolve_expression_kind_force_value(parser, source, &arg.value, false)
                                };
                                arg.resolved = Some(Entity::Value(value));
                            }
                            let installer = (&self.global_pipeline_installers).get(&previous_identifier.unwrap().name);
                            if let Some(installer) = installer {
                                items.push(ASTPipelineItem { installer: Some(installer.clone()), args: argument_list.arguments().clone()});
                            } else {
                                panic!("Cannot find pipeline item named '{}'.", previous_identifier.unwrap().name);
                            }
                            previous_identifier = None;
                        }
                        _ => panic!()
                    }
                }
                if let Some(previous_identifier) = previous_identifier {
                    let installer = (&self.global_pipeline_installers).get(&previous_identifier.name);
                    if let Some(installer) = installer {
                        items.push(ASTPipelineItem { installer: Some(installer.clone()), args: vec![]});
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

    fn resolve_field(&self, parser: &ASTParser, source: &ASTSource, field: &mut ASTField, ns_path: &Vec<String>, diagnostics: &mut Diagnostics) {
        field.figure_out_class();
        match &field.field_class {
            ASTFieldClass::Field => {
                for decorator in field.decorators.iter_mut() {
                    self.resolve_field_decorator(parser, source, decorator);
                }
                let span = field.r#type.identifiers.span.clone();
                self.resolve_field_primitive_type(parser, source, &mut field.r#type, ns_path, diagnostics, span);
            }
            ASTFieldClass::Relation => {
                for decorator in field.decorators.iter_mut() {
                    self.resolve_relation_decorator(parser, source, decorator);
                }
                let span = field.r#type.identifiers.span.clone();
                self.resolve_field_relation_type(parser, source, &mut field.r#type, ns_path, diagnostics, span);
            }
            ASTFieldClass::Property => {
                for decorator in field.decorators.iter_mut() {
                    self.resolve_property_decorator(parser, source, decorator);
                }
                let span = field.r#type.identifiers.span.clone();
                self.resolve_field_primitive_type(parser, source, &mut field.r#type, ns_path, diagnostics, span);
            }
            _ => {}
        }
        field.resolved = true;
    }

    pub(crate) fn resolve_connector(&self, parser: &ASTParser) -> DatabaseName {
        if parser.connector.is_none() {
            panic!("Connector is not defined.");
        }
        let connector_ref = parser.connector.as_ref().unwrap();
        let source = parser.get_source(*connector_ref.get(0).unwrap());
        let top = source.to_mut().tops.get_mut(&connector_ref.get(1).unwrap()).unwrap();
        let connector = top.as_connector_mut().unwrap();
        for item in connector.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    self.resolve_expression(parser, source, &mut item.expression);
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
                    self.resolve_expression(parser, source, &mut item.expression);
                    let url_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let url_str = url_value.as_str().unwrap();
                    connector.url = Some(url_str.to_owned());
                },
                _ => { panic!("Undefined name '{}' in connector block.", item.identifier.name.as_str())}
            }
        }
        connector.provider.unwrap()
    }

    pub(crate) fn resolve_client_generator(&self, parser: &ASTParser, source: &ASTSource, client: &mut ASTClient) {
        for item in client.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    self.resolve_expression(parser, source, &mut item.expression);
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
                    self.resolve_expression(parser, source, &mut item.expression);
                    let dest_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let dest_str = dest_value.as_str().unwrap();
                    let mut dest_path = source.path.clone();
                    dest_path.pop();
                    let dest = dest_path.join(PathBuf::from(dest_str));
                    let absolute = dest.absolutize().unwrap();
                    client.dest = Some(absolute.as_ref().to_owned());
                },
                "package" => {
                    self.resolve_expression(parser, source, &mut item.expression);
                    let package_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let package_bool = package_value.as_bool().unwrap();
                    client.package = Some(package_bool);
                },
                "host" => {
                    self.resolve_expression(parser, source, &mut item.expression);
                    let host_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let host_str = host_value.as_str().unwrap();
                    client.host = Some(host_str.to_owned());
                },
                "objectName" => {
                    self.resolve_expression(parser, source, &mut item.expression);
                    let object_name_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let object_name_str = object_name_value.as_str().unwrap();
                    client.object_name = object_name_str.to_owned();
                },
                "gitCommit" => {
                    self.resolve_expression(parser, source, &mut item.expression);
                    let git_commit_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    let git_commit_bool = git_commit_value.as_bool().unwrap();
                    client.git_commit = git_commit_bool;
                }
                _ => { panic!("Undefined name '{}' in client generator block.", item.identifier.name.as_str())}
            }
        }
    }

    pub(crate) fn resolve_model_entity_generator(&self, parser: &ASTParser, source: &ASTSource, generator: &mut ASTEntity) {
        for item in generator.items.iter_mut() {
            match item.identifier.name.as_str() {
                "provider" => {
                    self.resolve_expression(parser, source, &mut item.expression);
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
                    self.resolve_expression(parser, source, &mut item.expression);
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

    pub(crate) fn resolve_data_set(&self, parser: &ASTParser, source: &ASTSource, data_set: &mut ASTDataSet, diagnostics: &mut Diagnostics) {
        let ns_path = data_set.ns_path.clone();
        let mut dataset_path: Vec<String> = ns_path.clone();
        dataset_path.push(data_set.identifier.name.clone());
        for group in data_set.groups.iter_mut() {
            let span = group.identifiers.span.clone();
            let Some(model_id_path) = self.resolve_model_from_ns_with_path(
                parser,
                source,
                &group.identifiers.path(),
                &ns_path,
                diagnostics,
                span,
                source,
                vec![]
            ) else {
                self.insert_unresolved_model(source, diagnostics, span);
                return
            };
            group.resolve(model_id_path.clone());
            let model = parser.model_by_id(&model_id_path);
            for record in group.records.iter_mut() {
                record.resolved = Some(self.resolve_dataset_record_dictionary_literal(parser, source, &record.dictionary, model, diagnostics, &dataset_path).as_value().unwrap().clone());
            }
        }
    }

    pub(crate) fn resolve_debug_conf(&self, parser: &ASTParser, source: &ASTSource, config: &mut ASTDebugConf) {
        for item in config.items.iter_mut() {
            match item.identifier.name.as_str() {
                "logQueries" => {
                    self.resolve_expression(parser, source, &mut item.expression);
                    let value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match value {
                        Value::Bool(_b) => config.log_queries = true,
                        Value::Null => (),
                        _ => panic!(),
                    }
                },
                "logMigrations" => {
                    self.resolve_expression(parser, source, &mut item.expression);
                    let value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match value {
                        Value::Bool(_b) => config.log_migrations = true,
                        Value::Null => (),
                        _ => panic!(),
                    }
                },
                "logSeedRecords" => {
                    self.resolve_expression(parser, source, &mut item.expression);
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

    pub(crate) fn resolve_test_conf(&self, parser: &ASTParser, source: &ASTSource, config: &mut ASTTestConf) {
        for item in config.items.iter_mut() {
            match item.identifier.name.as_str() {
                "resetAfterFind" => {
                    self.resolve_expression(parser, source, &mut item.expression);
                    let value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    config.reset_after_find = value;
                },
                _ => panic!()
            }
        }
    }

    pub(crate) fn resolve_static_files(&self, parser: &ASTParser, source: &ASTSource, static_files: &mut StaticFiles) {
        self.resolve_expression(parser, source, &mut static_files.path);
        self.resolve_expression(parser, source, &mut static_files.map);
        static_files.resolved_path = Some(static_files.path.resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap().to_owned());
        static_files.resolved_map = Some(static_files.map.resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap().to_owned());
    }

    pub(crate) fn resolve_action_group(&self, parser: &ASTParser, source: &ASTSource, action_group_declaration: &mut ActionGroupDeclaration) {
        for action in &mut action_group_declaration.actions {
            self.resolve_custom_action_declaration(parser, source, action)
        }
    }

    pub(crate) fn resolve_custom_action_declaration(&self, parser: &ASTParser, source: &ASTSource, action: &mut ActionDeclaration) {
        let input_interface_name = action.input_type.name.name.as_str();
        let binding = parser.interfaces();
        let interface = match binding.iter().find(|i| i.name.name.name.as_str() == input_interface_name) {
            Some(i) => i,
            None => panic!("Interface with name '{}' is not found.", input_interface_name)
        };
        action.resolved_input_interface = Some((interface.source_id, interface.id));
        action.resolved_input_shape = Some(self.resolve_action_input_shape(parser, source, *interface, &action.input_type, false));
    }

    pub(crate) fn resolve_action_input_shape(&self, parser: &ASTParser, source: &ASTSource, interface: &InterfaceDeclaration, input_type: &InterfaceType, optional: bool) -> ResolvedInterfaceField {
        let map: HashMap<String, InterfaceType> = interface.args().iter().enumerate().map(|(i, a)| {
            (a.name.name.clone(), input_type.args.get(i).unwrap().clone())
        }).collect();
        let mut shape: HashMap<String, ResolvedInterfaceField> = hashmap!{};
        for extend in &interface.extends {
            let interface = Self::search_interface_by_name(parser, source, extend.name.name.as_str());
            self.install_interface_items_to_shape(parser, source, &map, &interface.items, &mut shape);
        }
        self.install_interface_items_to_shape(parser, source, &map, &interface.items, &mut shape);
        ResolvedInterfaceFieldType::Shape(shape).optional(optional)
    }

    pub(crate) fn install_interface_items_to_shape(&self, parser: &ASTParser, source: &ASTSource, map: &HashMap<String, InterfaceType>, items: &Vec<InterfaceItemDeclaration>, shape: &mut HashMap<String, ResolvedInterfaceField>) {
        for item in items {
            if Self::need_to_alter_generics_with_map(parser, source, map, &item.kind) {
                let replaced_type = item.kind.alter_generics_with(map);
                self.install_interface_items_with_generics_filled_to_shape(parser, source, &item.name, &replaced_type, shape);
            } else {
                self.install_interface_items_with_generics_filled_to_shape(parser, source, &item.name, &item.kind, shape);
            }
        }
    }

    pub(crate) fn install_interface_items_with_generics_filled_to_shape(&self, parser: &ASTParser, source: &ASTSource, name: &ASTIdentifier, kind: &InterfaceType, shape: &mut HashMap<String, ResolvedInterfaceField>) {
        shape.insert(name.name.clone(), self.resolve_type_with_filled_generics(parser, source, kind));
    }

    pub(crate) fn resolve_predefined_interface_type(&self, parser: &ASTParser, source: &ASTSource, a: &InterfaceType) -> Option<ResolvedInterfaceFieldType> {
        Some(match a.name.name.as_str() {
            "Data" => ResolvedInterfaceFieldType::Shape(hashmap!{"data".to_owned() => self.resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap())}),
            "DataMeta" => ResolvedInterfaceFieldType::Shape(hashmap!{
                "data".to_owned() => self.resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap()),
                "meta".to_owned() => self.resolve_type_with_filled_generics(parser, source, a.args.get(1).unwrap()),
            }),
            _ => None?,
        })
    }

    // we're not handle arrays, maps, enums yet
    pub(crate) fn resolve_type_with_filled_generics(&self, parser: &ASTParser, source: &ASTSource, a: &InterfaceType) -> ResolvedInterfaceField {
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
            "Array" => ResolvedInterfaceFieldType::Vec(Box::new(self.resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap()))).optional(a.collection_optional),
            "Dict" => ResolvedInterfaceFieldType::HashMap(Box::new(self.resolve_type_with_filled_generics(parser, source, a.args.get(0).unwrap()))).optional(a.collection_optional),
            "Any" => ResolvedInterfaceFieldType::Any.optional(a.optional),
            "File" => ResolvedInterfaceFieldType::File.optional(a.optional),
            // other user defined interfaces
            _ => {
                if let Some(result) = self.resolve_predefined_interface_type(parser, source, a) {
                    result.optional(a.optional)
                } else {
                    let interface_name = a.name.name.as_str();
                    let interface = Self::search_interface_by_name(parser, source, interface_name);
                    self.resolve_action_input_shape(parser, source, interface, a, a.optional)
                }
            }
        };
        match a.arity {
            Arity::Scalar => result_without_arity,
            Arity::Array => ResolvedInterfaceFieldType::Vec(Box::new(result_without_arity)).optional(a.collection_optional),
            Arity::Dictionary => ResolvedInterfaceFieldType::HashMap(Box::new(result_without_arity)).optional(a.collection_optional),
        }
    }

    pub(crate) fn search_interface_by_name<'a>(parser: &'a ASTParser, _source: &'a ASTSource, name: &str) -> &'a InterfaceDeclaration {
        let binding = parser.interfaces();
        let interface = match binding.iter().find(|i| i.name.name.name.as_str() == name) {
            Some(i) => i,
            None => panic!("Interface with name '{}' is not found.", name)
        };
        *interface
    }

    pub(crate) fn need_to_alter_generics_with_map(parser: &ASTParser, source: &ASTSource, map: &HashMap<String, InterfaceType>, def: &InterfaceType) -> bool {
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

    pub(crate) fn resolve_server_config_block(&self, parser: &ASTParser, source: &ASTSource, config: &mut ASTServer) {
        for item in config.items.iter_mut() {
            match item.identifier.name.as_str() {
                "bind" => {
                    self.resolve_expression(parser, source, &mut item.expression);
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
                    self.resolve_expression(parser, source, &mut item.expression);
                    let jwt_secret_value = Self::unwrap_into_value_if_needed(parser, source, item.expression.resolved.as_ref().unwrap());
                    match jwt_secret_value {
                        Value::Null => (),
                        Value::String(s) => config.jwt_secret = Some(s.clone()),
                        _ => panic!("Value of 'jwtSecret' should be string.")
                    }
                }
                "pathPrefix" => {
                    self.resolve_expression(parser, source, &mut item.expression);
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

    pub(crate) fn resolve_expression<'a>(&self, parser: &ASTParser, source: &ASTSource, expression: &mut Expression) {
        expression.resolved = Some(self.resolve_expression_kind(parser, source, &mut expression.kind, false));
    }

    pub(crate) fn resolve_expression_kind(&self, parser: &ASTParser, source: &ASTSource, expression_kind: &ExpressionKind, when_option: bool) -> Entity {
        match expression_kind {
            ExpressionKind::Group(group) => {
                self.resolve_group(parser, source, group, when_option)
            }
            ExpressionKind::NullishCoalescing(nullish_coalescing) => {
                self.resolve_nullish_coalescing(parser, source, nullish_coalescing)
            }
            ExpressionKind::Negation(negation) => {
                self.resolve_negation(parser, source, negation)
            }
            ExpressionKind::BitwiseNegation(negation) => {
                self.resolve_bitwise_negation(parser, source, negation, when_option)
            }
            ExpressionKind::ArithExpr(arith) => {
                self.resolve_arith_expr(parser, source, arith, when_option)
            }
            ExpressionKind::NumericLiteral(n) => {
                self.resolve_numeric_literal(n)
            }
            ExpressionKind::StringLiteral(s) => {
                self.resolve_string_literal(s)
            }
            ExpressionKind::RegExpLiteral(r) => {
                self.resolve_regexp_literal(r)
            }
            ExpressionKind::BoolLiteral(b) => {
                self.resolve_bool_literal(b)
            }
            ExpressionKind::NullLiteral(n) => {
                self.resolve_null_literal(n)
            }
            ExpressionKind::EnumVariantLiteral(e) => {
                self.resolve_enum_choice_literal(parser, source, e)
            }
            ExpressionKind::RangeLiteral(range_literal) => {
                self.resolve_range_literal(parser, source, range_literal)
            }
            ExpressionKind::TupleLiteral(tuple_literal) => {
                self.resolve_tuple_literal(parser, source, tuple_literal)
            }
            ExpressionKind::ArrayLiteral(array_literal) => {
                self.resolve_array_literal(parser, source, array_literal, when_option)
            }
            ExpressionKind::DictionaryLiteral(dictionary_literal) => {
                self.resolve_dictionary_literal(parser, source, dictionary_literal)
            }
            ExpressionKind::Identifier(identifier) => {
                self.resolve_identifier(parser, source, identifier, None)
            }
            ExpressionKind::ArgumentList(_a) => {
                panic!("Argument list cannot appear alone.")
            }
            ExpressionKind::Subscript(_s) => {
                panic!("Subscript cannot appear alone.")
            }
            ExpressionKind::Unit(unit) => {
                self.resolve_unit(parser, source, unit)
            }
            ExpressionKind::Pipeline(pipeline) => {
                self.resolve_pipeline(parser, source, pipeline)
            }
        }
    }

    fn resolve_expression_kind_force_value(&self, parser: &ASTParser, source: &ASTSource, expression_kind: &ExpressionKind, when_option: bool) -> Value {
        let entity = self.resolve_expression_kind(parser, source, expression_kind, when_option);
        Self::unwrap_into_value_if_needed(parser, source, &entity)
    }

    // identifier

    fn resolve_group(&self, parser: &ASTParser, source: &ASTSource, group: &Group, when_option: bool) -> Entity {
        self.resolve_expression_kind(parser, source, &group.expression, when_option)
    }

    fn resolve_identifier(&self, parser: &ASTParser, source: &ASTSource, identifier: &ASTIdentifier, parent: Option<&Entity>) -> Entity {
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

    fn resolve_unit(&self, parser: &ASTParser, source: &ASTSource, unit: &Unit) -> Entity {
        let first_expression = unit.expressions.get(0).unwrap();
        let mut entity = self.resolve_expression_kind(parser, source, first_expression, false);
        for (index, expression) in unit.expressions.iter().enumerate() {
            if index == 0 { continue }
            entity = self.resolve_accessor(parser, source, expression, &entity);
        }
        return entity
    }

    fn resolve_accessor(&self, parser: &ASTParser, source: &ASTSource, expression_kind: &ExpressionKind, entity: &Entity) -> Entity {
        match expression_kind {
            ExpressionKind::Subscript(subscript) => {
                self.resolve_subscript(parser, source, subscript, entity)
            }
            ExpressionKind::ArgumentList(argument_list) => {
                let mut args = argument_list.clone();
                for arg in &mut args.arguments.iter_mut() {
                    let value = self.resolve_expression_kind_force_value(parser, source, &arg.value, false);
                    arg.resolved = Some(Entity::Value(value));
                }
                match entity.as_accessible().unwrap() {
                    Accessible::Callable(callable) => Entity::Value(callable(args.arguments())),
                    _ => unreachable!(),
                }
            }
            ExpressionKind::Identifier(identifier) => {
                self.resolve_identifier(parser, source, identifier, Some(entity))
            }
            _ => panic!()
        }
    }

    fn resolve_subscript(&self, parser: &ASTParser, source: &ASTSource, subscript: &Subscript, entity: &Entity) -> Entity {
        let index_entity = self.resolve_expression_kind(parser, source, &subscript.expression, false);
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

    fn resolve_numeric_literal(&self, n: &NumericLiteral) -> Entity {
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

    fn resolve_string_literal(&self, s: &StringLiteral) -> Entity {
        return Entity::Value(Value::String(unescape(s.value.as_str()).unwrap()));
    }

    fn resolve_regexp_literal(&self, r: &RegExpLiteral) -> Entity {
        return Entity::Value(Value::RegExp(Regex::new(r.value.as_str()).unwrap()));
    }

    fn resolve_bool_literal(&self, b: &BoolLiteral) -> Entity {
        match b.value.as_str() {
            "true" => Entity::Value(Value::Bool(true)),
            "false" => Entity::Value(Value::Bool(false)),
            _ => panic!("Cannot resolve bool value: {}", b.value.as_str())
        }
    }

    fn resolve_null_literal(&self, _: &NullLiteral) -> Entity {
        Entity::Value(Value::Null)
    }

    fn resolve_enum_choice_literal(&self, parser: &ASTParser, source: &ASTSource, e: &EnumVariantLiteral) -> Entity {
        if e.argument_list.is_some() {
            Entity::Value(Value::RawEnumChoice(e.value.clone(), Some(self.resolve_argument_list_as_tuple_vec(parser, source, e.argument_list.as_ref().unwrap()))))
        } else {
            Entity::Value(Value::RawEnumChoice(e.value.clone(), None))
        }
    }

    fn resolve_argument_list_as_tuple_vec(&self, parser: &ASTParser, source: &ASTSource, arg_list: &ArgumentList) -> Vec<(Option<String>, Value)> {
        let mut result = vec![];
        for arg in arg_list.arguments.iter() {
            let name = arg.name.as_ref().map(|i| i.name.clone());
            let resolve_result = self.resolve_expression_kind(parser, source, &arg.value, false);
            let value = Self::unwrap_into_value_if_needed(parser, source, &resolve_result);
            result.push((name, value));
        }
        result
    }

    fn resolve_range_literal(&self, parser: &ASTParser, source: &ASTSource, range_literal: &RangeLiteral) -> Entity {
        let a = self.resolve_expression_kind(parser, source, range_literal.expressions.get(0).unwrap(), false);
        let a_v = Self::unwrap_into_value_if_needed(parser, source, &a);
        let start = Box::new(a_v);
        let b = self.resolve_expression_kind(parser, source, range_literal.expressions.get(1).unwrap(), false);
        let b_v = Self::unwrap_into_value_if_needed(parser, source, &b);
        let end = Box::new(b_v);
        Entity::Value(Value::Range(Range { closed: range_literal.closed.clone(), start, end }))
    }

    fn resolve_tuple_literal(&self, parser: &ASTParser, source: &ASTSource, tuple_literal: &TupleLiteral) -> Entity {
        let mut resolved = vec![];
        for expression in tuple_literal.expressions.iter() {
            let e = self.resolve_expression_kind(parser, source, expression, false);
            let v = Self::unwrap_into_value_if_needed(parser, source, &e);
            resolved.push(v);
        }
        Entity::Value(Value::Tuple(resolved))
    }

    fn resolve_array_literal(&self, parser: &ASTParser, source: &ASTSource, array_literal: &ArrayLiteral, when_option: bool) -> Entity {
        let mut resolved = vec![];
        for expression in array_literal.expressions.iter() {
            let e = self.resolve_expression_kind(parser, source, expression, when_option);
            let v = Self::unwrap_into_value_if_needed(parser, source, &e);
            resolved.push(v);
        }
        Entity::Value(Value::Vec(resolved))
    }

    fn resolve_array_literal_for_dataset_record_relation(&self, parser: &ASTParser, source: &ASTSource, array_literal: &ArrayLiteral, ref_model: &ASTModel, ds_path: &Vec<String>, diagnostics: &mut Diagnostics) -> Entity {
        let mut resolved = vec![];
        for expression in array_literal.expressions.iter() {
            let v_span = expression.span();
            let e = self.resolve_expression_kind(parser, source, expression, false);
            let v = Self::unwrap_into_value_if_needed(parser, source, &e);
            if v.is_raw_enum_choice() {
                let record_name = v.as_raw_enum_choice().unwrap();
                if parser.data_set_record_counts(ref_model, ds_path, record_name) < 1 {
                    self.insert_data_set_record_relation_value_is_not_enum_variant(source, diagnostics, v_span.clone(), ref_model.path().join(".").as_str(), ds_path.join(".").as_str());
                }
            } else {
                self.insert_data_set_record_relation_value_is_not_enum_variant(source, diagnostics, v_span.clone(), ref_model.path().join(".").as_str(), ds_path.join(".").as_str());
            }
            resolved.push(v);
        }
        Entity::Value(Value::Vec(resolved))
    }

    fn resolve_dictionary_literal(&self, parser: &ASTParser, source: &ASTSource, dic: &DictionaryLiteral) -> Entity {
        let mut resolved: HashMap<String, Value> = HashMap::new();
        for (key, value) in dic.expressions.iter() {
            let k = self.resolve_expression_kind(parser, source, key, false);
            let k = Self::unwrap_into_value_if_needed(parser, source, &k);
            let v = self.resolve_expression_kind(parser, source, value, false);
            let v = Self::unwrap_into_value_if_needed(parser, source, &v);
            resolved.insert(k.as_str().unwrap().to_string(), v);
        }
        Entity::Value(Value::HashMap(resolved))
    }

    fn resolve_dataset_record_dictionary_literal(&self, parser: &ASTParser, source: &ASTSource, dic: &DictionaryLiteral, model: &ASTModel, diagnostics: &mut Diagnostics, dataset_path: &Vec<String>) -> Entity {
        let mut resolved: HashMap<String, Value> = HashMap::new();
        let mut used_keys = vec![];
        for (key, value) in dic.expressions.iter() {
            let k_span = key.span();
            let k = self.resolve_expression_kind(parser, source, key, false);
            let k = Self::unwrap_into_value_if_needed(parser, source, &k);
            if !k.is_string() {
                self.insert_data_set_record_key_type_is_not_string(source, diagnostics, k_span.clone());
            }
            if used_keys.contains(&k.as_str().unwrap().to_string()) {
                self.insert_data_set_record_key_is_duplicated(source, diagnostics, k_span.clone());
            }
            used_keys.push(k.as_str().unwrap().to_string());
            if let Some(field) = model.field_for_key(k.as_str().unwrap()) {
                if field.field_class.is_relation() {
                    let referenced_model = parser.model_by_id(&field.r#type.type_id);
                    let v_span = value.span();
                    if field.r#type.arity.is_array() { // to many relation
                        if value.is_array_literal() {
                            let v = self.resolve_array_literal_for_dataset_record_relation(parser, source, value.as_array_literal().unwrap(), referenced_model, dataset_path, diagnostics);
                            let v = Self::unwrap_into_value_if_needed(parser, source, &v);
                            resolved.insert(if k.is_string() { k.as_str().unwrap().to_string() } else { "".to_owned() }, v);
                        } else {
                            let v = self.resolve_expression_kind(parser, source, value, false);
                            let v = Self::unwrap_into_value_if_needed(parser, source, &v);
                            if v.is_vec() {
                                for vec_item in v.as_vec().unwrap() {
                                    if vec_item.is_raw_enum_choice() {
                                        let record_name = vec_item.as_raw_enum_choice().unwrap();
                                        if parser.data_set_record_counts(referenced_model, dataset_path, record_name) < 1 {
                                            self.insert_data_set_record_relation_value_is_not_records_array(source, diagnostics, v_span.clone(), referenced_model.path().join(".").as_str(), dataset_path.join(".").as_str());
                                        }
                                    } else {
                                        self.insert_data_set_record_relation_value_is_not_records_array(source, diagnostics, v_span.clone(), referenced_model.path().join(".").as_str(), dataset_path.join(".").as_str());
                                    }
                                }
                            } else {
                                self.insert_data_set_record_relation_value_is_not_array(source, diagnostics, v_span.clone());
                            }
                            resolved.insert(if k.is_string() { k.as_str().unwrap().to_string() } else { "".to_owned() }, v);
                        }
                    } else { // to one relation
                        if value.is_null_literal() {
                            // do nothing yet
                        } else if value.is_enum_variant_literal() {
                            let v = self.resolve_expression_kind(parser, source, value, false);
                            let v = Self::unwrap_into_value_if_needed(parser, source, &v);
                            let record_name = v.as_raw_enum_choice().unwrap();
                            if parser.data_set_record_counts(referenced_model, dataset_path, record_name) < 1 {
                                self.insert_data_set_record_relation_value_is_not_enum_variant(source, diagnostics, v_span.clone(), referenced_model.path().join(".").as_str(), dataset_path.join(".").as_str());
                            }
                            resolved.insert(if k.is_string() { k.as_str().unwrap().to_string() } else { "".to_owned() }, v);
                        } else {
                            let v = self.resolve_expression_kind(parser, source, value, false);
                            let v = Self::unwrap_into_value_if_needed(parser, source, &v);
                            resolved.insert(if k.is_string() { k.as_str().unwrap().to_string() } else { "".to_owned() }, v);
                        }
                    }
                    // let v = self.resolve_expression_kind_for_data_set_record(parser, source, value, false);
                } else if field.field_class.is_primitive_field() {
                    let v_span = value.span();
                    let v = self.resolve_expression_kind(parser, source, value, false);
                    let v = Self::unwrap_into_value_if_needed(parser, source, &v);
                    // validate primitive input
                    if let Some(message) = field.validate_primitive_value(&v) {
                        self.insert_data_set_record_primitive_value_type_error(source, diagnostics, v_span.clone(), message);
                    }
                    resolved.insert(if k.is_string() { k.as_str().unwrap().to_string() } else { "".to_owned() }, v);
                } else if field.field_class.is_dropped() {
                    self.insert_data_set_record_key_is_dropped(source, diagnostics, k_span.clone(), k.as_str().unwrap(), &model.path().join("."));
                } else { // property
                    self.insert_data_set_record_key_is_property(source, diagnostics, k_span.clone());
                }
            } else {
                self.insert_data_set_record_key_is_undefined(source, diagnostics, k_span.clone(), k.as_str().unwrap(), &model.path().join("."));
            }
        }
        Entity::Value(Value::HashMap(resolved))
    }

    fn resolve_nullish_coalescing(&self, parser: &ASTParser, source: &ASTSource, nullish_coalescing: &NullishCoalescing) -> Entity {
        let mut resolved = Entity::Value(Value::Null);
        for e in nullish_coalescing.expressions.iter() {
            resolved = self.resolve_expression_kind(parser, source, e, false);
            if !resolved.is_null() {
                return resolved;
            }
        }
        return resolved
    }

    fn resolve_negation(&self, parser: &ASTParser, source: &ASTSource, negation: &Negation) -> Entity {
        let value = self.resolve_expression_kind_force_value(parser, source, &negation.expression, false);
        Entity::Value(match value {
            Value::I32(v) => Value::I32(-v),
            Value::I64(v) => Value::I64(-v),
            Value::F32(v) => Value::F32(-v),
            Value::F64(v) => Value::F64(-v),
            _ => panic!("Cannot negate value {:?}", value)
        })
    }

    fn resolve_bitwise_negation(&self, parser: &ASTParser, source: &ASTSource, negation: &BitwiseNegation, when_option: bool) -> Entity {
        let value = self.resolve_expression_kind_force_value(parser, source, &negation.expression, when_option);
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

    fn is_primitive_type_builtin(&self, parser: &ASTParser, name: &str) -> bool {
        match name {
            "String" | "Bool" | "Int" | "Int32" | "Int64" | "Float" | "Float32" | "Float64" |
            "Date" | "DateTime" | "Decimal" => true,
            "ObjectId" => parser.connector().unwrap().provider.unwrap().is_mongo(),
            _ => false,
        }
    }

    fn resolve_field_primitive_type(&self, parser: &ASTParser, source: &ASTSource, field_type: &mut ASTFieldType, ns_path: &Vec<String>, diagnostics: &mut Diagnostics, span: Span) {
        let type_path = field_type.identifiers.path();
        if type_path.len() == 1 {
            let name = type_path.get(0).unwrap().as_str();
            if self.is_primitive_type_builtin(parser, name) {
                field_type.resolve(vec![], TypeClass::Builtin);
                return
            }
        }
        let id_path = self.resolve_enum_from_ns_with_path(parser, source, &field_type.identifiers.path(), ns_path, diagnostics, span, source, vec![]);
        if let Some(id_path) = id_path {
            field_type.resolve(id_path, TypeClass::Enum);
        } else {
            self.insert_unresolved_enum(source, diagnostics, span);
            unreachable!()
        }
    }

    fn resolve_field_relation_type(&self, parser: &ASTParser, source: &ASTSource, field_type: &mut ASTFieldType, ns_path: &Vec<String>, diagnostics: &mut Diagnostics, span: Span) {
        let id_path = self.resolve_model_from_ns_with_path(parser, source, &field_type.identifiers.path(), ns_path, diagnostics, span, source, vec![]);
        if let Some(id_path) = id_path {
            field_type.resolve(id_path, TypeClass::Model);
        } else {
            self.insert_unresolved_model(source, diagnostics, span);
            unreachable!()
        }
    }

    fn resolve_enum_from_ns_with_path(&self, parser: &ASTParser, source: &ASTSource, ref_path: &Vec<String>, ns_path: &Vec<String>, diagnostics: &mut Diagnostics, span: Span, original_source: &ASTSource, used_sources: Vec<&ASTSource>) -> Option<Vec<usize>> {
        if used_sources.contains(&source) {
            return None
        }
        let mut ns_path_mut: Option<Vec<&str>> = Some(ns_path.iter().map(|s| s.as_str()).collect());
        loop {
            if let Some(ns_path_ref) = ns_path_mut.as_ref() {
                if ns_path_ref.is_empty() {
                    if let Some(model) = source.get_enum_by_path(ref_path.iter().map(|s| s.as_str()).collect()) {
                        return Some(model.id_path.clone())
                    }
                } else {
                    if let Some(ns) = source.get_namespace_by_path(ns_path_ref.clone()) {
                        if let Some(model) = ns.get_enum_by_path(ref_path.iter().map(|s| s.as_str()).collect()) {
                            return Some(model.id_path.clone())
                        }
                    }
                }
                ns_path_mut = Self::next_ns_path(ns_path_ref);
            } else {
                break
            }
        }
        for import in source.imports() {
            // find with imports
            ns_path_mut = Some(ns_path.iter().map(|s| s.as_str()).collect());
            let from_source = parser.sources.iter().find(|(_source_id, source)| {
                &import.path == &source.path
            }).unwrap().1;
            if let Some(result) = self.resolve_enum_from_ns_with_path(parser, from_source, ref_path, ns_path, diagnostics, span, original_source, {
                let mut new_sources = used_sources.clone();
                new_sources.push(source);
                new_sources
            }) {
                return Some(result)
            }
        }
        None
    }

    fn resolve_model_from_ns_with_path(&self, parser: &ASTParser, source: &ASTSource, ref_path: &Vec<String>, ns_path: &Vec<String>, diagnostics: &mut Diagnostics, span: Span, original_source: &ASTSource, used_sources: Vec<&ASTSource>) -> Option<Vec<usize>> {
        if used_sources.contains(&source) {
            return None
        }
        let mut ns_path_mut: Option<Vec<&str>> = Some(ns_path.iter().map(|s| s.as_str()).collect());
        loop {
            if let Some(ns_path_ref) = ns_path_mut.as_ref() {
                if ns_path_ref.is_empty() {
                    if let Some(model) = source.get_model_by_path(ref_path.iter().map(|s| s.as_str()).collect()) {
                        return Some(model.id_path.clone())
                    }
                } else {
                    if let Some(ns) = source.get_namespace_by_path(ns_path_ref.clone()) {
                        if let Some(model) = ns.get_model_by_path(ref_path.iter().map(|s| s.as_str()).collect()) {
                            return Some(model.id_path.clone())
                        }
                    }
                }
                ns_path_mut = Self::next_ns_path(ns_path_ref);
            } else {
                break
            }
        }
        for import in source.imports() {
            // find with imports
            ns_path_mut = Some(ns_path.iter().map(|s| s.as_str()).collect());
            let from_source = parser.sources.iter().find(|(_source_id, source)| {
                &import.path == &source.path
            }).unwrap().1;
            if let Some(result) = self.resolve_model_from_ns_with_path(parser, from_source, ref_path, ns_path, diagnostics, span, original_source, {
                let mut new_sources = used_sources.clone();
                new_sources.push(source);
                new_sources
            }) {
                return Some(result)
            }
        }
        None
    }

    fn next_ns_path<'a>(prev: &Vec<&'a str>) -> Option<Vec<&'a str>> {
        if prev.len() == 0 { return None }
        let mut retval = prev.clone();
        retval.remove(prev.len() - 1);
        Some(retval)
    }

    fn resolve_arith_expr(&self, parser: &ASTParser, source: &ASTSource, arith_expr: &ArithExpr, when_option: bool) -> Entity {
        match arith_expr {
            ArithExpr::Expression(expression) => return self.resolve_expression_kind(parser, source, &expression, when_option),
            ArithExpr::UnaryNeg(expression) => {
                let origin = self.resolve_expression_kind_force_value(parser, source, &expression, when_option);
                return Entity::Value((-origin).unwrap());
            }
            ArithExpr::UnaryBitNeg(expression) => {
                let origin = self.resolve_expression_kind_force_value(parser, source, &expression, when_option);
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
            ArithExpr::BinaryOp { span, lhs, op, rhs } => {
                let lhs_value = self.resolve_arith_expr(parser, source, &lhs, when_option).as_value().unwrap().clone();
                let rhs_value = self.resolve_arith_expr(parser, source, &rhs, when_option).as_value().unwrap().clone();
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

    fn find_identifier_origin_in_source(parser: &ASTParser, source: &ASTSource, identifier: &ASTIdentifier) -> Option<Reference> {
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

    fn constant_with_reference(parser: &ASTParser, _source: &ASTSource, reference: (usize, usize)) -> Value {
        let source = parser.get_source(reference.0);
        let c = source.get_constant(reference.1);
        let entity = c.expression.resolved.as_ref().unwrap();
        Self::unwrap_into_value_if_needed(parser, source, entity)
    }

    fn unwrap_into_value_if_needed(parser: &ASTParser, source: &ASTSource, entity: &Entity) -> Value {
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

    fn insert_unresolved_model(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span) {
        let source_path = source.path.clone();
        diagnostics.insert_unresolved_model(span, source_path);
    }

    fn insert_unresolved_enum(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span) {
        let source_path = source.path.clone();
        diagnostics.insert_unresolved_enum(span, source_path);
    }

    fn insert_data_set_record_key_type_is_not_string(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span) {
        let source_path = source.path.clone();
        diagnostics.insert(DiagnosticsError::new(span, "Data set record key is not string", source_path));
    }

    fn insert_data_set_record_key_is_duplicated(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span) {
        let source_path = source.path.clone();
        diagnostics.insert(DiagnosticsError::new(span, "Data set record key is duplicated", source_path));
    }

    fn insert_data_set_record_key_is_undefined(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span, key: &str, model: &str) {
        diagnostics.insert(DiagnosticsError::new(span, format!("Field with name '{key}' is undefined on model `{model}'"), source.path.clone()));
    }

    fn insert_data_set_record_key_is_property(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span) {
        diagnostics.insert(DiagnosticsError::new(span, format!("Property is not allowed in data set record"), source.path.clone()));
    }

    fn insert_data_set_record_key_is_dropped(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span, key: &str, model: &str) {
        diagnostics.insert(DiagnosticsError::new(span, format!("Field with name '{key}' is dropped on model `{model}'"), source.path.clone()));
    }

    fn insert_data_set_record_primitive_value_type_error(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span, message: String) {
        diagnostics.insert(DiagnosticsError::new(span, message, source.path.clone()));
    }

    fn insert_data_set_record_relation_value_is_not_array(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span) {
        diagnostics.insert(DiagnosticsError::new(span, "Relation value is not array", source.path.clone()));
    }

    fn insert_data_set_record_relation_value_is_not_records_array(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span, model_name: &str, dataset_path: &str) {
        diagnostics.insert(DiagnosticsError::new(span, format!("Relation value is not array of `{model_name}` records in dataset `{dataset_path}`"), source.path.clone()));
    }

    fn insert_data_set_record_relation_value_is_not_enum_variant(&self, source: &ASTSource, diagnostics: &mut Diagnostics, span: Span, model_name: &str, dataset_path: &str) {
        diagnostics.insert(DiagnosticsError::new(span, format!("Relation value is not enum variant of `{model_name}` records in dataset `{dataset_path}`"), source.path.clone()));
    }
}
