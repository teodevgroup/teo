use inflector::Inflector;
use askama::Template;
use crate::core::app::conf::ClientGeneratorConf;
use crate::core::r#enum::Enum;
use crate::prelude::Graph;

mod filters {
    use inflector::Inflector;

    pub fn camelcase<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let s = s.to_string();
        Ok(s.to_camel_case())
    }
}

#[derive(Template)]
#[template(path = "client/swift/footer.swift.jinja")]
pub(crate) struct FooterTemplate<'a> {
    pub(crate) object_name: &'a str,
    pub(crate) model_names: Vec<String>
}

fn generate_footer(graph: &Graph, client: &ClientGeneratorConf) -> String {
    FooterTemplate {
        object_name: client.object_name.as_ref().map_or("teo", |n| n.as_str()),
        model_names: graph.models().iter().map(|m| m.name().to_camel_case()).collect(),
    }.render().unwrap()
}

#[derive(Template)]
#[template(path = "client/swift/enums.swift.jinja")]
pub(crate) struct EnumsTemplate {
    pub(crate) enums: Vec<Enum>,
}

fn generate_enums(graph: &Graph) -> String {
    EnumsTemplate {
        enums: graph.enums().iter().map(|(_, e)| e.clone()).collect(),
    }.render().unwrap()
}

pub(crate) fn generate_teo_swift(graph: &Graph, client: &ClientGeneratorConf) -> String {
    let header = include_str!("header.swift");
    let body = "";
    let footer = generate_footer(graph, client);
    format!("{header}\n{body}\n{footer}")
}
