use array_tool::vec::Join;
use inflector::Inflector;
use askama::Template;
use crate::core::app::conf::ClientGeneratorConf;
use crate::core::r#enum::Enum;
use crate::generator::client::swift::types::{field_type_to_swift_output_type, field_type_to_swift_vec, swift_action_result};
use crate::generator::lib::shared::delegate::{Delegate, delegates};
use crate::generator::lib::shared::model_input::{model_inputs, ModelInput};
use crate::generator::lib::shared::model_output::{model_outputs_with_relations, ModelOutput};
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

#[derive(Template)]
#[template(path = "client/swift/model_outputs.swift.jinja")]
pub(crate) struct ModelOutputsTemplate<'a> {
    pub(crate) models: Vec<ModelOutput<'a>>,
}
fn generate_model_output_types(graph: &Graph) -> String {
    ModelOutputsTemplate {
        models: model_outputs_with_relations(graph, field_type_to_swift_output_type, field_type_to_swift_vec)
    }.render().unwrap()
}

#[derive(Template)]
#[template(path = "client/swift/model_inputs.swift.jinja")]
pub(crate) struct ModelInputsTemplate<'a> {
    pub(crate) models: Vec<ModelInput<'a>>,
}
fn generate_input_types(graph: &Graph) -> String {
    ModelInputsTemplate {
        models: model_inputs(graph),
    }.render().unwrap()
}

#[derive(Template)]
#[template(path = "client/swift/delegates.swift.jinja")]
pub(crate) struct DelegatesTemplate<'a> {
    pub(crate) delegates: Vec<Delegate<'a>>,
}
fn generate_delegate_classes(graph: &Graph) -> String {
    DelegatesTemplate {
        delegates: delegates(graph, swift_action_result),
    }.render().unwrap()
}

pub(crate) fn generate_teo_swift(graph: &Graph, client: &ClientGeneratorConf) -> String {
    let header = include_str!("header.swift");
    let enums = generate_enums(graph);
    let output_types = generate_model_output_types(graph);
    let input_types = generate_input_types(graph);
    let delegate_classes = generate_delegate_classes(graph);
    let footer = generate_footer(graph, client);
    format!("{header}\n{enums}\n{output_types}\n{input_types}\n{delegate_classes}\n{footer}")
}
