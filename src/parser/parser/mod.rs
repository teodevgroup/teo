pub(crate) mod resolver;

use std::borrow::Borrow;
use snailquote::unescape;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};
use maplit::{btreemap, btreeset};
use pest::Parser as PestParser;
use to_mut::ToMut;
use to_mut_proc_macro::ToMut;
use crate::core::app::builder::CallbackLookupTable;
use crate::parser::ast::argument::{Argument, ArgumentList};
use crate::parser::ast::client::Client;
use crate::parser::ast::config::Config;
use crate::parser::ast::connector::Connector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::expression::{Expression, ExpressionKind, ArrayLiteral, BoolLiteral, DictionaryLiteral, EnumChoiceLiteral, NullLiteral, NumericLiteral, RangeLiteral, StringLiteral, TupleLiteral, RegExpLiteral, NullishCoalescing};
use crate::parser::ast::field::Field;
use crate::parser::ast::generator::Generator;
use crate::parser::ast::group::Group;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::import::Import;
use crate::parser::ast::item::Item;
use crate::parser::ast::model::Model;
use crate::parser::ast::pipeline::Pipeline;
use crate::parser::ast::r#enum::{Enum, EnumChoice};
use crate::parser::ast::r#type::{Arity, Type};
use crate::parser::ast::source::Source;
use crate::parser::ast::span::Span;
use crate::parser::ast::subscript::Subscript;
use crate::parser::ast::top::Top;
use crate::parser::ast::unit::Unit;
use crate::parser::parser::resolver::Resolver;
use crate::parser::std::decorators::field::GlobalFieldDecorators;
use crate::parser::std::decorators::model::GlobalModelDecorators;
use crate::parser::std::decorators::property::GlobalPropertyDecorators;
use crate::parser::std::decorators::relation::GlobalRelationDecorators;
use crate::parser::std::pipeline::global::{GlobalFunctionInstallers, GlobalPipelineInstallers};

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(Debug, ToMut)]
pub(crate) struct Parser {
    pub(crate) sources: BTreeMap<usize, Source>,
    pub(crate) enums: Vec<(usize, usize)>,
    pub(crate) models: Vec<(usize, usize)>,
    pub(crate) connector: Option<(usize, usize)>,
    pub(crate) config: Option<(usize, usize)>,
    pub(crate) generators: Vec<(usize, usize)>,
    pub(crate) clients: Vec<(usize, usize)>,
    pub(crate) next_id: usize,
    pub(crate) resolved: bool,
    pub(crate) global_model_decorators: Option<GlobalModelDecorators>,
    pub(crate) global_field_decorators: Option<GlobalFieldDecorators>,
    pub(crate) global_relation_decorators: Option<GlobalRelationDecorators>,
    pub(crate) global_property_decorators: Option<GlobalPropertyDecorators>,
    pub(crate) global_pipeline_installers: Option<GlobalPipelineInstallers>,
    pub(crate) global_function_installers: Option<GlobalFunctionInstallers>,
    pub(crate) callback_lookup_table: Arc<Mutex<CallbackLookupTable>>,
}

impl Parser {

    pub(crate) fn new(callback_lookup_table: Arc<Mutex<CallbackLookupTable>>) -> Self {
        Self {
            sources: btreemap!{},
            enums: vec![],
            models: vec![],
            connector: None,
            config: None,
            generators: vec![],
            clients: vec![],
            next_id: 0,
            resolved: false,
            global_model_decorators: None,
            global_field_decorators: None,
            global_relation_decorators: None,
            global_property_decorators: None,
            global_pipeline_installers: None,
            global_function_installers: None,
            callback_lookup_table,
        }
    }

    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    pub(crate) fn parse(&mut self, main: Option<&str>) -> () {
        let main = if main.is_some() { main.unwrap() } else {
            let mut result: Option<&str> = None;
            for name in ["schema.teo", "src/schema.teo"] {
                let relative = PathBuf::from(name);
                let absolute = match fs::canonicalize(&relative) {
                    Ok(_path) => Some(name),
                    Err(_) => None,
                };
                if absolute.is_some() {
                    result = absolute;
                    break
                }
            }
            if result.is_some() {
                result.unwrap()
            } else {
                panic!("Cannot find a schema file.")
            }
        };
        let relative = PathBuf::from(main);
        let absolute = match fs::canonicalize(&relative) {
            Ok(path) => path,
            Err(_) => panic!("Schema file '{}' is not found.", relative.to_str().unwrap()),
        };
        self.parse_source(&absolute);
        Resolver::resolve_parser(self);
    }

    fn parse_source(&mut self, path: &PathBuf) {
        let source_id = self.next_id();
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => panic!("{}", err)
        };
        let mut pairs = match SchemaParser::parse(Rule::schema, &content) {
            Ok(pairs) => pairs,
            Err(err) => panic!("{}", err)
        };
        let pairs = pairs.next().unwrap();
        let mut tops: BTreeMap<usize, Top> = btreemap![];
        let mut imports: BTreeSet<usize> = btreeset!{};
        let mut constants: BTreeSet<usize> = btreeset!{};
        let mut enums: BTreeSet<usize> = btreeset!{};
        let mut models: BTreeSet<usize> = btreeset!{};
        let mut pairs = pairs.into_inner().peekable();

        while let Some(current) = pairs.next() {
            let item_id = self.next_id();
            match current.as_rule() {
                Rule::import_statement => {
                    let import = self.parse_import(current, source_id, item_id, path.clone());
                    tops.insert(source_id, import);
                    imports.insert(item_id);
                },
                Rule::let_declaration => {
                    let constant = self.parse_let_declaration(current, source_id, item_id);
                    tops.insert(item_id, constant);
                    constants.insert(item_id);
                },
                Rule::model_declaration => {
                    let model = self.parse_model(current, source_id, item_id);
                    tops.insert(item_id, model);
                    models.insert(item_id);
                    self.models.push((source_id, item_id));
                },
                Rule::enum_declaration => {
                    let r#enum = self.parse_enum(current, source_id, item_id);
                    tops.insert(item_id, r#enum);
                    enums.insert(item_id);
                    self.enums.push((source_id, item_id));
                },
                Rule::config_declaration => {
                    let config_block = self.parse_config_block(current, source_id, item_id);
                    tops.insert(item_id, config_block);
                },
                Rule::EOI | Rule::EMPTY_LINES => {},
                Rule::CATCH_ALL => panic!("Found catch all."),
                Rule::comment_block => (),
                _ => panic!("Parsing panic! {}", current),
            }
        }
        let result = Source::new(source_id, path.clone(), tops, imports, constants, enums, models);
        for import in result.borrow().imports() {
            let found = self.sources.values().find(|v| {
                (*v).borrow().path == import.path
            });
            if found.is_none() {
                self.parse_source(&import.path);
            }
        }
        self.sources.insert(source_id, result);
    }

    fn parse_import(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, path: PathBuf) -> Top {
        let mut identifiers = vec![];
        let span = Self::parse_span(&pair);
        let mut source: Option<StringLiteral> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::string_literal => source = Some(StringLiteral { value: current.as_str().to_string(), span }),
                Rule::import_identifier_list => identifiers = Self::parse_import_identifier_list(current),
                _ => panic!("error."),
            }
        }
        let unescaped = unescape(source.as_ref().unwrap().value.as_str()).unwrap();
        let relative = PathBuf::from(unescaped);
        let mut dir = path.clone();
        dir.pop();
        let new = dir.join(&relative);
        let absolute = match fs::canonicalize(&new) {
            Ok(path) => path,
            Err(_) => panic!("Schema file '{}' is not found.", relative.to_str().unwrap()),
        };
        Top::Import(Import::new(item_id, source_id, identifiers, source.unwrap(), absolute, span))
    }

    fn parse_model(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize) -> Top {
        let mut identifier: Option<Identifier> = None;
        let mut fields: Vec<Field> = vec![];
        let mut decorators: Vec<Decorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::MODEL_KEYWORD | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE | Rule::EMPTY_LINES => {}
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::field_declaration => fields.push(Self::parse_field(current)),
                Rule::block_decorator => decorators.push(Self::parse_decorator(current)),
                Rule::item_decorator => decorators.push(Self::parse_decorator(current)),
                Rule::comment_block => (),
                _ => panic!("error. {:?}", current),
            }
        }
        Top::Model(Model::new(
            item_id,
            source_id,
            identifier.unwrap(),
            fields,
            decorators,
            span,
        ))
    }

    fn parse_field(pair: Pair<'_>) -> Field {
        let mut identifier: Option<Identifier> = None;
        let mut r#type: Option<Type> = None;
        let mut decorators: Vec<Decorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::field_type => r#type = Some(Self::parse_type(current)),
                Rule::item_decorator => decorators.push(Self::parse_decorator(current)),
                _ => panic!("error."),
            }
        }
        Field::new(
            identifier.unwrap(),
            r#type.unwrap(),
            decorators,
            span,
        )
    }

    fn parse_enum(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize) -> Top {
        let mut identifier: Option<Identifier> = None;
        let mut choices: Vec<EnumChoice> = vec![];
        let mut decorators: Vec<Decorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::ENUM_KEYWORD | Rule::COLON | Rule::EMPTY_LINES | Rule::comment_block | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::enum_value_declaration => choices.push(self.parse_enum_value(current)),
                Rule::block_decorator => decorators.push(Self::parse_decorator(current)),
                _ => panic!("error. {}", current),
            }
        }
        Top::Enum(Enum::new(
            item_id,
            source_id,
            identifier.unwrap(),
            decorators,
            choices,
            span,
        ))
    }

    fn parse_enum_value(&mut self, pair: Pair<'_>) -> EnumChoice {
        let mut identifier: Option<Identifier> = None;
        let mut decorators: Vec<Decorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON | Rule::EMPTY_LINES | Rule::comment_block => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::item_decorator => decorators.push(Self::parse_decorator(current)),
                _ => panic!("error."),
            }
        }
        EnumChoice::new(identifier.unwrap(), decorators, span)
    }

    fn parse_let_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize) -> Top {
        let span = Self::parse_span(&pair);
        let mut identifier: Option<Identifier> = None;
        let mut expression: Option<Expression> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::expression => expression = Some(Self::parse_expression(current)),
                _ => panic!("error."),
            }
        }
        Top::Constant(Constant::new(item_id, source_id, identifier.unwrap(), expression.unwrap(), span))
    }

    fn parse_config_block(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize) -> Top {
        let mut identifier: Option<Identifier> = None;
        let mut items: Vec<Item> = vec![];
        let mut keyword = "";
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE | Rule::EMPTY_LINES => (),
                Rule::config_keywords => keyword = current.as_str(),
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::config_item => items.push(Self::parse_config_item(current)),
                Rule::comment_block => (),
                _ => {
                    panic!("see current {} error.", current)
                },
            }
        }
        match keyword {
            "config" => {
                if self.config.is_some() {
                    panic!("Duplicated config found.");
                }
                self.config = Some((source_id, item_id));
                Top::Config(Config::new(item_id, source_id, items, span))
            },
            "connector" => {
                if self.connector.is_some() {
                    panic!("Duplicated connector found.");
                }
                self.connector = Some((source_id, item_id));
                Top::Connector(Connector::new(items, span, source_id, item_id))
            },
            "entity" => {
                self.generators.push((source_id, item_id));
                Top::Generator(Generator::new(item_id, source_id, identifier.unwrap(), items, span))
            },
            "client" => {
                self.clients.push((source_id, item_id));
                Top::Client(Client::new(item_id, source_id, identifier.unwrap(), items, span))
            },
            _ => panic!(),
        }
    }

    fn parse_config_item(pair: Pair<'_>) -> Item {
        let span = Self::parse_span(&pair);
        let mut identifier: Option<Identifier> = None;
        let mut expression: Option<Expression> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::expression => expression = Some(Self::parse_expression(current)),
                _ => panic!("error."),
            }
        }
        Item { identifier: identifier.unwrap(), expression: expression.unwrap(), span }
    }

    fn parse_decorator(pair: Pair<'_>) -> Decorator {
        let span = Self::parse_span(&pair);
        let mut unit: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier_unit => unit = Some(Self::parse_unit(current)),
                _ => panic!(),
            }
        }
        Decorator::new(unit.unwrap(), span)
    }

    fn parse_pipeline(pair: Pair<'_>) -> Pipeline {
        let span = Self::parse_span(&pair);
        let mut unit: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier_unit => unit = Some(Self::parse_unit(current)),
                _ => panic!(),
            }
        }
        Pipeline {
            expression: Box::new(unit.unwrap()),
            span,
        }
    }

    fn parse_argument(pair: Pair<'_>) -> Argument {
        let span = Self::parse_span(&pair);
        let name: Option<Identifier> = None;
        let mut value: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::named_argument => {
                    return Self::parse_named_argument(current);
                },
                Rule::expression => value = Some(Self::parse_expression(current).kind),
                Rule::empty_argument => panic!("Empty argument found."),
                _ => panic!(),
            }
        }
        Argument { name, value: value.unwrap(), span, resolved: None }
    }

    fn parse_named_argument(pair: Pair<'_>) -> Argument {
        let span = Self::parse_span(&pair);
        let mut name: Option<Identifier> = None;
        let mut value: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => name = Some(Self::parse_identifier(&current)),
                Rule::expression => value = Some(Self::parse_expression(current).kind),
                Rule::empty_argument => panic!("Empty argument found."),
                _ => panic!(),
            }
        }
        Argument { name, value: value.unwrap(), span, resolved: None }
    }

    fn parse_expression(pair: Pair<'_>) -> Expression {
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::nullish_coalescing => return Expression::new(ExpressionKind::NullishCoalescing(Self::parse_nullish_coalescing(current))),
                Rule::unit => return Expression::new(Self::parse_unit(current)),
                Rule::pipeline => return Expression::new(ExpressionKind::Pipeline(Self::parse_pipeline(current))),
                _ => panic!(),
            }
        }
        panic!();
    }

    fn parse_unit(pair: Pair<'_>) -> ExpressionKind {
        let span = Self::parse_span(&pair);
        let mut unit = Unit { expressions: vec![], span };
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::group => unit.expressions.push(ExpressionKind::Group(Self::parse_group(current))),
                Rule::null_literal => unit.expressions.push(ExpressionKind::NullLiteral(NullLiteral { value: current.as_str().to_string(), span })),
                Rule::bool_literal => unit.expressions.push(ExpressionKind::BoolLiteral(BoolLiteral { value: current.as_str().to_string(), span })),
                Rule::numeric_literal => unit.expressions.push(ExpressionKind::NumericLiteral(NumericLiteral { value: current.as_str().to_string(), span })),
                Rule::string_literal => unit.expressions.push(ExpressionKind::StringLiteral(StringLiteral { value: current.as_str().to_string(), span })),
                Rule::regexp_literal => unit.expressions.push(ExpressionKind::RegExpLiteral(RegExpLiteral { value: current.as_str().to_string(), span })),
                Rule::enum_choice_literal => unit.expressions.push(ExpressionKind::EnumChoiceLiteral(EnumChoiceLiteral { value: current.as_str().to_string(), span })),
                Rule::tuple_literal => unit.expressions.push(ExpressionKind::TupleLiteral(Self::parse_tuple_literal(current))),
                Rule::array_literal => unit.expressions.push(ExpressionKind::ArrayLiteral(Self::parse_array_literal(current))),
                Rule::dictionary_literal => unit.expressions.push(ExpressionKind::DictionaryLiteral(Self::parse_dictionary_literal(current))),
                Rule::range_literal => unit.expressions.push(ExpressionKind::RangeLiteral(Self::parse_range_literal(current))),
                Rule::identifier => unit.expressions.push(ExpressionKind::Identifier(Self::parse_identifier(&current))),
                Rule::subscript => unit.expressions.push(ExpressionKind::Subscript(Self::parse_subscript(current))),
                Rule::argument_list => unit.expressions.push(ExpressionKind::ArgumentList(Self::parse_argument_list(current))),
                _ => panic!(),
            }
        }
        if unit.expressions.len() == 1 {
            return unit.expressions.get(0).unwrap().clone()
        } else {
            return ExpressionKind::Unit(unit);
        }
    }

    fn parse_nullish_coalescing(pair: Pair<'_>) -> NullishCoalescing {
        let span = Self::parse_span(&pair);
        let mut expressions = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::unit => expressions.push(Self::parse_unit(current)),
                _ => panic!()
            }
        }
        NullishCoalescing { expressions, span }
    }

    fn parse_subscript(pair: Pair<'_>) -> Subscript {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => return Subscript { expression: Box::new(Self::parse_expression(current).kind), span },
                _ => panic!(),
            }
        }
        panic!()
    }

    fn parse_argument_list(pair: Pair<'_>) -> ArgumentList {
        let span = Self::parse_span(&pair);
        let mut arguments: Vec<Argument> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::argument => arguments.push(Self::parse_argument(current)),
                _ => panic!("{}", current),
            }
        }
        ArgumentList { arguments, span, resolved: false }
    }

    fn parse_group(pair: Pair<'_>) -> Group {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => return Group { expression: Box::new(Self::parse_expression(current).kind), span },
                _ => panic!(),
            }
        }
        panic!()
    }

    fn parse_range_literal(pair: Pair<'_>) -> RangeLiteral {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<ExpressionKind> = vec![];
        let mut closed = false;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::range_end => expressions.push(Self::parse_range_end(current)),
                Rule::RANGE_OPEN => closed = false,
                Rule::RANGE_CLOSE => closed = true,
                _ => panic!("{:?} {:?}", current.as_rule(), current.as_span()),
            }
        }
        RangeLiteral { closed, expressions, span }
    }

    fn parse_range_end(pair: Pair<'_>) -> ExpressionKind {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::numeric_literal => return ExpressionKind::NumericLiteral(NumericLiteral { value: current.as_str().to_string(), span }),
                Rule::unit_without_range_literal => return Self::parse_unit(current),
                _ => panic!(),
            }
        }
        panic!()
    }

    fn parse_tuple_literal(pair: Pair<'_>) -> TupleLiteral {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<ExpressionKind> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => expressions.push(Self::parse_expression(current).kind),
                _ => panic!(),
            }
        }
        TupleLiteral { expressions, span }
    }

    fn parse_array_literal(pair: Pair<'_>) -> ArrayLiteral {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<ExpressionKind> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => expressions.push(Self::parse_expression(current).kind),
                _ => panic!(),
            }
        }
        ArrayLiteral { expressions, span }
    }

    fn parse_dictionary_literal(_pair: Pair<'_>) -> DictionaryLiteral {
        panic!()
    }
    
    fn parse_type(pair: Pair<'_>) -> Type {
        let mut identifier = None;
        let mut arity = Arity::Scalar;
        let mut item_required = true;
        let mut collection_required = true;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::arity => if current.as_str() == "[]" { arity = Arity::Array; } else { arity = Arity::Dictionary; },
                Rule::optionality => {
                    if arity == Arity::Scalar {
                        item_required = false;
                    } else {
                        collection_required = false;
                    }
                },
                _ => panic!(),
            }
        }
        Type::new(
            identifier.unwrap(),
            arity,
            item_required,
            collection_required,
        )
    }

    fn parse_import_identifier_list(pair: Pair<'_>) -> Vec<Identifier> {
        let mut identifiers = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifiers.push(Self::parse_identifier(&current)),
                _ => panic!(),
            }
        }
        identifiers
    }

    fn parse_identifier(pair: &Pair<'_>) -> Identifier {
        Identifier {
            name: pair.as_str().to_owned(),
            span: Self::parse_span(pair),
        }
    }

    fn parse_span(pair: &Pair<'_>) -> Span {
        let span = pair.as_span();
        Span {
            start: span.start(),
            end: span.end(),
        }
    }

    pub(crate) fn get_source(&self, id: usize) -> &Source {
        self.sources.get(&id).unwrap()
    }

    pub(crate) fn set_global_model_decorators(&self, deco: GlobalModelDecorators) {
        self.to_mut().global_model_decorators = Some(deco);
    }

    pub(crate) fn set_global_field_decorators(&self, deco: GlobalFieldDecorators) {
        self.to_mut().global_field_decorators = Some(deco);
    }

    pub(crate) fn set_global_relation_decorators(&self, deco: GlobalRelationDecorators) {
        self.to_mut().global_relation_decorators = Some(deco);
    }

    pub(crate) fn set_global_property_decorators(&self, deco: GlobalPropertyDecorators) {
        self.to_mut().global_property_decorators = Some(deco);
    }

    pub(crate) fn set_global_pipeline_installers(&self, installer: GlobalPipelineInstallers) {
        self.to_mut().global_pipeline_installers = Some(installer);
    }

    pub(crate) fn set_global_function_installers(&self, installer: GlobalFunctionInstallers) {
        self.to_mut().global_function_installers = Some(installer);
    }

    pub(crate) fn global_model_decorators(&self) -> &GlobalModelDecorators {
        self.global_model_decorators.as_ref().unwrap()
    }

    pub(crate) fn global_field_decorators(&self) -> &GlobalFieldDecorators {
        self.global_field_decorators.as_ref().unwrap()
    }

    pub(crate) fn global_relation_decorators(&self) -> &GlobalRelationDecorators {
        self.global_relation_decorators.as_ref().unwrap()
    }

    pub(crate) fn global_property_decorators(&self) -> &GlobalPropertyDecorators {
        self.global_property_decorators.as_ref().unwrap()
    }

    pub(crate) fn global_pipeline_installers(&self) -> &GlobalPipelineInstallers {
        self.global_pipeline_installers.as_ref().unwrap()
    }

    pub(crate) fn global_function_installers(&self) -> &GlobalFunctionInstallers {
        self.global_function_installers.as_ref().unwrap()
    }
}
