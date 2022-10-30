use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

pub(crate) fn parse(file: &str) -> () {
    let result = SchemaParser::parse(Rule::schema, "model User {\n  @id\n  id: Int\n}");
    println!("{:?}", result)
}
