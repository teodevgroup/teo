use pest::Parser as PestParser;
use crate::parser::ast::model::Model;
use crate::parser::ast::r#enum::Enum;
use crate::parser::ast::top::Top;

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

pub(crate) struct Parser { }

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

impl Parser {
    pub(crate) fn parse_entry_file(file: &str) -> () {
        let result = SchemaParser::parse(Rule::schema, "model User {\n  @id\n  id: Int\n}");
        if let Err(_err) = result {
            panic!("Schema parsing error.")
        }
        let result = result.unwrap();
        let result = result.next().unwrap();
        let mut tops: Vec<Top> = vec![];
        let mut pairs = result.into_inner().peekable();
        while let Some(current) = pairs.next() {
            match current.as_rule() {
                Rule::model_declaration => {
                    tops.push(Top::Model(Self::parse_model(current)));
                },
                Rule::enum_declaration => {
                    tops.push(Top::Enum(Self::parse_enum(current)));
                }
                Rule::EOI => {}
                Rule::CATCH_ALL => panic!("Found catch all."),
                Rule::empty_lines => (),
                _ => panic!("Parsing panic!"),
            }
        }
        println!("{:?}", result)
    }

    pub(crate) fn parse_model(current: Pair<'_>) -> Model {
        Model {}
    }

    pub(crate) fn parse_enum(current: Pair<'_>) -> Enum {

    }
}
