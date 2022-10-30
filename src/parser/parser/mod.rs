use pest::Parser as PestParser;
use crate::parser::ast::top::Top;

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

pub(crate) struct Parser { }

impl Parser {
    pub(crate) fn parse_entry_file(file: &str) -> () {
        let result = SchemaParser::parse(Rule::schema, "model User {\n  @id\n  id: Int\n}");
        if let Err(_err) = result {
            panic!("Schema parsing error.")
        }
        let result = result.unwrap();
        let result = result.next().unwrap();
        let mut tops: Vec<Top> = vec![];
        let mut pairs = datamodel.into_inner().peekable();
        while let Some(current) = pairs.next() {
            match current.as_rule() {
                Rule::model_declaration => {
                    tops.push(Top::Model(parse_model(current, pending_block_comment.take(), diagnostics)))
                },
                Rule::enum_declaration => top_level_definitions.push(Top::Enum(parse_enum(current,pending_block_comment.take(),  diagnostics))),
                Rule::config_block => {
                    top_level_definitions.push(parse_config_block(current, diagnostics));
                },
                Rule::type_alias => {
                    let error = DatamodelError::new_validation_error(
                        "Invalid type definition. Please check the documentation in https://pris.ly/d/composite-types",
                        current.as_span().into()
                    );

                    diagnostics.push_error(error);
                }
                Rule::comment_block => {
                    match pairs.peek().map(|b| b.as_rule()) {
                        Some(Rule::empty_lines) => {
                            // free floating
                        }
                        Some(Rule::model_declaration) | Some(Rule::enum_declaration) | Some(Rule::config_block) => {
                            pending_block_comment = Some(current);
                        }
                        _ => (),
                    }
                },
                Rule::EOI => {}
                Rule::CATCH_ALL => diagnostics.push_error(DatamodelError::new_validation_error(
                    "This line is invalid. It does not start with any known Prisma schema keyword.",
                    current.as_span().into(),
                )),
                Rule::arbitrary_block => diagnostics.push_error(DatamodelError::new_validation_error(
                    "This block is invalid. It does not start with any known Prisma schema keyword. Valid keywords include \'model\', \'enum\', \'datasource\' and \'generator\'.",
                    current.as_span().into(),
                )),
                Rule::empty_lines => (),
                _ => unreachable!(),
            }
        }
        println!("{:?}", result)
    }

    pub(crate) fn parse_model() -> () {

    }
}
