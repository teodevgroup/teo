pub(crate) mod resolver;

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};
use pest::Parser as PestParser;
use crate::parser::ast::argument::Argument;
use crate::parser::ast::call::Call;
use crate::parser::ast::client::Client;
use crate::parser::ast::config::Config;
use crate::parser::ast::connector::Connector;
use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::expression::{ArrayExpression, BoolExpression, DictionaryExpression, EnumChoiceExpression, Expression, NullExpression, NumericExpression, RangeExpression, StringExpression, TupleExpression};
use crate::parser::ast::field::Field;
use crate::parser::ast::generator::Generator;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::import::Import;
use crate::parser::ast::item::Item;
use crate::parser::ast::model::Model;
use crate::parser::ast::path::Path;
use crate::parser::ast::pipeline::Pipeline;
use crate::parser::ast::r#enum::{Enum, EnumChoice};
use crate::parser::ast::r#type::{Arity, Type};
use crate::parser::ast::source::Source;
use crate::parser::ast::span::Span;
use crate::parser::ast::top::Top;
use crate::parser::parser::resolver::Resolver;

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

#[derive(Debug)]
pub(crate) struct Parser {
    pub(crate) main: Option<Arc<Mutex<Source>>>,
    pub(crate) sources: HashMap<usize, Arc<Mutex<Source>>>,
    pub(crate) tops: HashMap<usize, Arc<Mutex<Top>>>,
    pub(crate) connector: Option<Arc<Mutex<Top>>>,
    pub(crate) next_id: usize,
}

impl Parser {

    pub(crate) fn new() -> Self {
        Self {
            main: None,
            sources: HashMap::new(),
            tops: HashMap::new(),
            connector: None,
            next_id: 0,
        }
    }

    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    pub(crate) fn parse(&mut self, main: Option<&str>) -> () {
        let main = main.unwrap_or("schema.teo");
        let relative = PathBuf::from(main);
        let absolute = match fs::canonicalize(&relative) {
            Ok(path) => path,
            Err(_) => panic!("Schema file '{}' is not found.", relative.to_str().unwrap()),
        };
        let id = self.next_id();
        let source = self.parse_source(absolute, id);
        self.main = Some(source.clone());
        // println!("{:?}", self);
        Resolver::resolve_parser(self);
    }

    fn parse_source(&mut self, path: PathBuf, id: usize) -> Arc<Mutex<Source>> {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => panic!("{}", err)
        };
        let mut pairs = match SchemaParser::parse(Rule::schema, &content) {
            Ok(pairs) => pairs,
            Err(err) => panic!("{}", err)
        };
        let pairs = pairs.next().unwrap();
        let mut tops: Vec<Arc<Mutex<Top>>> = vec![];
        let mut pairs = pairs.into_inner().peekable();
        while let Some(current) = pairs.next() {
            match current.as_rule() {
                Rule::import_statement => tops.push(self.parse_import(current, id)),
                Rule::model_declaration => tops.push(self.parse_model(current, id)),
                Rule::enum_declaration => tops.push(self.parse_enum(current, id)),
                Rule::config_declaration => tops.push(self.parse_config_block(current, id)),
                Rule::EOI | Rule::EMPTY_LINES => {},
                Rule::CATCH_ALL => panic!("Found catch all."),
                _ => panic!("Parsing panic!"),
            }
        }
        let result = Arc::new(Mutex::new(Source { id, path, tops }));
        self.sources.insert(id, result.clone());
        result
    }

    fn parse_import(&mut self, pair: Pair<'_>, source_id: usize) -> Arc<Mutex<Top>> {
        let mut identifiers = vec![];
        let span = Self::parse_span(&pair);
        let mut source: Option<StringExpression> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::string_literal => source = Some(StringExpression::new(current.as_str().to_string(), span)),
                Rule::import_identifier_list => identifiers = Self::parse_import_identifier_list(current),
                _ => panic!("error."),
            }
        }
        Arc::new(Mutex::new(Top::Import(Import { identifiers, source: source.unwrap(), span })))
    }

    fn parse_model(&mut self, pair: Pair<'_>, source_id: usize) -> Arc<Mutex<Top>> {
        let mut identifier: Option<Identifier> = None;
        let mut fields: Vec<Field> = vec![];
        let mut decorators: Vec<Decorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::MODEL_KEYWORD | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => {}
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::field_declaration => fields.push(Self::parse_field(current)),
                _ => panic!("error."),
            }
        }
        let result = Arc::new(Mutex::new(Top::Model(Model::new(
            self.next_id(),
            source_id,
            identifier.unwrap(),
            fields,
            decorators,
            span,
        ))));
        self.tops.insert(result.lock().unwrap().id(), result.clone());
        result
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

    fn parse_enum(&mut self, pair: Pair<'_>, source_id: usize) -> Arc<Mutex<Top>> {
        let mut identifier: Option<Identifier> = None;
        let mut choices: Vec<EnumChoice> = vec![];
        let result = Arc::new(Mutex::new(Top::Enum(Enum {
            id: self.next_id(),
            source_id,
            identifier: identifier.unwrap(),
            choices,
            span: Self::parse_span(&pair),
        })));
        self.tops.insert(result.lock().unwrap().id(), result.clone());
        result
    }

    fn parse_config_block(&mut self, pair: Pair<'_>, source_id: usize) -> Arc<Mutex<Top>> {
        let mut identifier: Option<Identifier> = None;
        let mut items: Vec<Item> = vec![];
        let mut keyword = "";
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => (),
                Rule::config_keywords => keyword = current.as_str(),
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::config_item => items.push(Self::parse_config_item(current)),
                _ => panic!("error."),
            }
        }
        let result = Arc::new(Mutex::new(match keyword {
            "config" => Top::Config(Config { items, span, source_id }),
            "connector" => Top::Connector(Connector::new(items, span, source_id)),
            "generator" => Top::Generator(Generator { identifier: identifier.unwrap(), items, span, source_id }),
            "client" => Top::Client(Client { identifier: identifier.unwrap(), items, span, source_id }),
            _ => panic!(),
        }));
        self.tops.insert(result.lock().unwrap().id(), result.clone());
        if keyword == "connector" {
            self.connector = Some(result.clone());
        }
        result
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
        let call = Self::parse_call(pair);
        Decorator { call, span }
    }

    fn parse_call(pair: Pair<'_>) -> Call {
        let span = Self::parse_span(&pair);
        let mut path: Option<Path> = None;
        let mut arguments: Vec<Argument> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::path => path = Some(Self::parse_path(current)),
                Rule::arguments_list => arguments = Self::parse_arguments(current),
                _ => panic!(),
            }
        }
        Call { path: path.unwrap(), arguments, span }
    }

    fn parse_pipeline(pair: Pair<'_>) -> Pipeline {
        let span = Self::parse_span(&pair);
        let mut path: Option<Path> = None;
        let mut arguments: Vec<Argument> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::path => path = Some(Self::parse_path(current)),
                Rule::arguments_list => arguments = Self::parse_arguments(current),
                _ => panic!(),
            }
        }
        Pipeline {
            call: Call { path: path.unwrap(), arguments, span: span.clone() },
            span,
        }
    }

    fn parse_arguments(pair: Pair<'_>) -> Vec<Argument> {
        let mut arguments: Vec<Argument> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::argument => arguments.push(Self::parse_argument(current)),
                _ => panic!(),
            }
        }
        arguments
    }

    fn parse_argument(pair: Pair<'_>) -> Argument {
        let span = Self::parse_span(&pair);
        let mut name: Option<Identifier> = None;
        let mut value: Option<Expression> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::named_argument => {
                    return Self::parse_named_argument(current);
                },
                Rule::expression => value = Some(Self::parse_expression(current)),
                Rule::empty_argument => panic!("Empty argument found."),
                _ => panic!(),
            }
        }
        Argument { name, value: value.unwrap(), span, resolved: None }
    }

    fn parse_named_argument(pair: Pair<'_>) -> Argument {
        let span = Self::parse_span(&pair);
        let mut name: Option<Identifier> = None;
        let mut value: Option<Expression> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => name = Some(Self::parse_identifier(&current)),
                Rule::expression => value = Some(Self::parse_expression(current)),
                Rule::empty_argument => panic!("Empty argument found."),
                _ => panic!(),
            }
        }
        Argument { name, value: value.unwrap(), span, resolved: None }
    }

    fn parse_expression(pair: Pair<'_>) -> Expression {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::bool_literal => return Expression::Bool(BoolExpression::new(current.as_str().to_string(), span)),
                Rule::null_literal => return Expression::Null(NullExpression::new(current.as_str().to_string(), span)),
                Rule::numeric_literal => return Expression::Numeric(NumericExpression::new(current.as_str().to_string(), span)),
                Rule::string_literal => return Expression::String(StringExpression::new(current.as_str().to_string(), span)),
                Rule::call => return Expression::Call(Self::parse_call(current)),
                Rule::pipeline => return Expression::Pipeline(Self::parse_pipeline(current)),
                Rule::range_literal => return Expression::Range(Self::parse_range_literal(current)),
                Rule::tuple_literal => return Expression::Tuple(Self::parse_tuple_literal(current)),
                Rule::array_literal => return Expression::Array(Self::parse_array_literal(current)),
                Rule::dictionary_literal => return Expression::Dictionary(Self::parse_dictionary_literal(current)),
                Rule::path => return Expression::Path(Self::parse_path(current)),
                Rule::enum_choice_literal => return Expression::EnumChoice(EnumChoiceExpression::new(current.as_str().to_string(), span)),
                _ => panic!(),
            }
        }
        panic!();
    }

    fn parse_range_literal(pair: Pair<'_>) -> RangeExpression {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<Expression> = vec![];
        let mut closed = false;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => expressions.push(Self::parse_expression(current)),
                Rule::RANGE_OPEN => closed = false,
                Rule::RANGE_CLOSE => closed = true,
                _ => panic!(),
            }
        }
        RangeExpression::new(closed, expressions, span)
    }

    fn parse_tuple_literal(pair: Pair<'_>) -> TupleExpression {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<Expression> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => expressions.push(Self::parse_expression(current)),
                _ => panic!(),
            }
        }
        TupleExpression::new(expressions, span)
    }

    fn parse_array_literal(pair: Pair<'_>) -> ArrayExpression {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<Expression> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => expressions.push(Self::parse_expression(current)),
                _ => panic!(),
            }
        }
        ArrayExpression::new(expressions, span)
    }

    fn parse_dictionary_literal(pair: Pair<'_>) -> DictionaryExpression {
        panic!()
    }

    fn parse_path(pair: Pair<'_>) -> Path {
        let span = Self::parse_span(&pair);
        let mut identifiers: Vec<Identifier> = vec![];
        println!("see this path: {:?}", pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifiers.push(Self::parse_identifier(&current)),
                Rule::path => identifiers.extend(Self::parse_path(current).identifiers),
                _ => panic!(),
            }
        }
        Path { identifiers, span }
    }
    
    fn parse_type(pair: Pair<'_>) -> Type {
        let mut identifier = None;
        let mut arity = Arity::Scalar;
        let mut required = true;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::arity => if current.as_str() == "[]" { arity = Arity::Array; } else { arity = Arity::Dictionary; },
                Rule::optionality => required = false,
                _ => panic!(),
            }
        }
        Type::new(
            identifier.unwrap(),
            arity,
            required,
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

    pub(crate) fn get_source_by_id(&self, id: usize) -> Option<Arc<Mutex<Source>>> {
        self.sources.get(&id).cloned()
    }
}
