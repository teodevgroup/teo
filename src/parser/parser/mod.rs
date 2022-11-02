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
use crate::parser::ast::source::Source;
use crate::parser::ast::span::Span;
use crate::parser::ast::top::Top;

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

const N: usize = 0;

pub(crate) struct Parser {
    main: Option<Arc<Source>>,
    sources: HashMap<usize, Arc<Source>>,
    tops: HashMap<usize, Arc<Top>>,
}

impl Parser {

    fn next_id() -> usize {
        N += 1;
        N
    }

    pub(crate) fn parse(&mut self, main: Option<&str>) -> () {
        let main = main.unwrap_or("schema.teo");
        let relative = PathBuf::from(main);
        let absolute = match fs::canonicalize(&relative) {
            Ok(path) => path,
            Err(_) => panic!("Schema file '{}' is not found.", relative.to_str().unwrap()),
        };
        let source = self.parse_source(absolute, Self::next_id());
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
        let mut tops: Vec<Top> = vec![];
        let mut pairs = pairs.into_inner().peekable();
        while let Some(current) = pairs.next() {
            match current.as_rule() {
                Rule::model_declaration => tops.push(Top::Model(Self::parse_model(current))),
                Rule::enum_declaration => tops.push(Top::Enum(Self::parse_enum(current))),
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

    fn parse_model(pair: Pair<'_>) -> Model {
        let mut identifier: Option<Identifier> = None;
        let mut fields: Vec<Field> = vec![];
        let mut decorators: Vec<Decorator> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::MODEL_KEYWORD | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => {}
                Rule::identifier => identifier = Some(current.into()),
                Rule::field_declaration => fields.push(Self::parse_field(current)),
                _ => panic!("error."),
            }
        }
        Model {
            identifier: identifier.unwrap(),
            fields,
            decorators,
            span: Self::parse_span(pair.as_span()),
        }
    }

    fn parse_field(pair: Pair<'_>) -> Field {

    }

    fn parse_enum(pair: Pair<'_>) -> Enum {
        let mut identifier: Option<Identifier> = None;
        let mut choices: Vec<EnumChoice> = vec![];
        Enum {
            identifier: identifier.unwrap(),
            choices,
            span: Self::parse_span(pair.as_span()),
        }
    }

    fn parse_identifier(pair: Pair<'_>) -> Identifier {
        Identifier {
            name: pair.as_str().to_owned(),
            span: pair.as_span().into(),
        }
    }

    fn parse_span(span: pest::Span<'_>) -> Span {
        Span {
            start: span.start(),
            end: span.end(),
        }
    }
}
