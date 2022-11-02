use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use pest::Parser as PestParser;
use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::field::Field;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::{Enum, EnumChoice};
use crate::parser::ast::r#type::{Arity, Type};
use crate::parser::ast::source::Source;
use crate::parser::ast::span::Span;
use crate::parser::ast::top::Top;

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub(crate) struct Parser {
    main: Option<Arc<Source>>,
    sources: HashMap<usize, Arc<Source>>,
    tops: HashMap<usize, Arc<Top>>,
    next_id: usize,
}

impl Parser {

    pub(crate) fn new() -> Self {
        Self {
            main: None,
            sources: HashMap::new(),
            tops: HashMap::new(),
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
        let source = self.parse_source(absolute, self.next_id());
        self.main = Some(source.clone());
    }

    fn parse_source(&mut self, path: PathBuf, id: usize) -> Arc<Source> {
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => panic!("{}", err)
        };
        let pairs = match SchemaParser::parse(Rule::schema, &content) {
            Ok(pairs) => pairs,
            Err(err) => panic!("{}", err)
        };
        let pairs = pairs.next().unwrap();
        let mut tops: Vec<Arc<Top>> = vec![];
        let mut pairs = pairs.into_inner().peekable();
        while let Some(current) = pairs.next() {
            match current.as_rule() {
                Rule::model_declaration => tops.push(self.parse_model(current, id)),
                Rule::enum_declaration => tops.push(self.parse_enum(current, id)),
                Rule::EOI => {},
                Rule::CATCH_ALL => panic!("Found catch all."),
                Rule::empty_lines => (),
                _ => panic!("Parsing panic!"),
            }
        }
        let result = Arc::new(Source { id, path, tops });
        self.sources.insert(id, result.clone());
        result
    }

    fn parse_model(&mut self, pair: Pair<'_>, source_id: usize) -> Arc<Top> {
        let mut identifier: Option<Identifier> = None;
        let mut fields: Vec<Field> = vec![];
        let mut decorators: Vec<Decorator> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::MODEL_KEYWORD | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => {}
                Rule::identifier => identifier = Some(Self::parse_identifier(current)),
                Rule::field_declaration => fields.push(Self::parse_field(current)),
                _ => panic!("error."),
            }
        }
        let result = Arc::new(Top::Model(Model {
            id: self.next_id(),
            source_id,
            identifier: identifier.unwrap(),
            fields,
            decorators,
            span: Self::parse_span(pair),
        }));
        self.tops.insert(result.id(), result.clone());
        result
    }

    fn parse_field(pair: Pair<'_>) -> Field {
        let mut identifier: Option<Identifier> = None;
        let mut r#type: Option<Type> = None;
        let mut decorators: Vec<Decorator> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(current)),
                Rule::field_type => r#type = Some(Self::parse_type(current)),
                Rule::item_decorator => (),
                _ => panic!("error."),
            }
        }
        Field {
            identifier: identifier.unwrap(),
            r#type: r#type.unwrap(),
            decorators,
            span: Self::parse_span(pair),
        }
    }

    fn parse_enum(&mut self, pair: Pair<'_>, source_id: usize) -> Arc<Top> {
        let mut identifier: Option<Identifier> = None;
        let mut choices: Vec<EnumChoice> = vec![];
        let result = Arc::new(Top::Enum(Enum {
            id: self.next_id(),
            source_id,
            identifier: identifier.unwrap(),
            choices,
            span: Self::parse_span(pair),
        }));
        self.tops.insert(result.id(), result.clone());
        result
    }
    
    fn parse_type(pair: Pair<'_>) -> Type {
        let mut identifier = None;
        let mut arity = Arity::Scalar;
        let mut required = true;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(current)),
                Rule::arity => if current.as_str() == "[]" { arity = Arity::Array; } else { arity = Arity::Dictionary; },
                Rule::optionality => required = false,
                _ => panic!(),
            }
        }
        Type {
            identifier: identifier.unwrap(),
            arity: Arity::Scalar,
            required: false
        }
    }

    fn parse_identifier(pair: Pair<'_>) -> Identifier {
        Identifier {
            name: pair.as_str().to_owned(),
            span: Self::parse_span(pair),
        }
    }

    fn parse_span(pair: Pair<'_>) -> Span {
        let span = pair.as_span();
        Span {
            start: span.start(),
            end: span.end(),
        }
    }
}
