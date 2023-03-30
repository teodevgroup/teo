use askama::Template;
use crate::core::app::conf::ClientGeneratorConf;
use crate::core::r#enum::Enum;
use crate::gen::generators::client::csharp::types::CSharpTypes;
use crate::gen::lib::shared::delegate::{Delegate, delegates};
use crate::gen::lib::shared::model_input::{model_inputs, ModelInput};
use crate::gen::lib::shared::model_output::{model_outputs_with_relations, ModelOutput};
use crate::prelude::Graph;
use crate::gen::lib::shared::filters;

#[derive(Template)]
#[template(path = "client/csharp/enums.cs.jinja", escape = "none")]
pub(crate) struct EnumsTemplate {
    pub(crate) enums: Vec<Enum>,
}
fn generate_enums(graph: &Graph) -> String {
    EnumsTemplate {
        enums: graph.enums().iter().map(|(_, e)| e.clone()).collect(),
    }.render().unwrap()
}

#[derive(Template)]
#[template(path = "client/csharp/model_outputs.cs.jinja", escape = "none")]
pub(crate) struct ModelOutputsTemplate<'a> {
    pub(crate) models: Vec<ModelOutput<'a>>,
}
fn generate_model_output_types(graph: &Graph) -> String {
    ModelOutputsTemplate {
        models: model_outputs_with_relations(graph, CSharpTypes::new()),
    }.render().unwrap()
}

#[derive(Template)]
#[template(path = "client/csharp/model_inputs.cs.jinja", escape = "none")]
pub(crate) struct ModelInputsTemplate<'a> {
    pub(crate) models: Vec<ModelInput<'a>>,
}
fn generate_input_types(graph: &Graph) -> String {
    ModelInputsTemplate {
        models: model_inputs(graph, CSharpTypes::new()),
    }.render().unwrap()
}

#[derive(Template)]
#[template(path = "client/csharp/delegates.cs.jinja", escape = "none")]
pub(crate) struct DelegatesTemplate<'a> {
    pub(crate) delegates: Vec<Delegate<'a>>,
}
fn generate_delegate_classes(graph: &Graph) -> String {
    DelegatesTemplate {
        delegates: delegates(graph, CSharpTypes::new()),
    }.render().unwrap()
}

pub(crate) fn generate_teo_cs(graph: &Graph, client: &ClientGeneratorConf) -> String {
    let header = include_str!("header.cs");
    let enums = generate_enums(graph);
    let output_types = generate_model_output_types(graph);
    let input_types = generate_input_types(graph);
    let delegate_classes = generate_delegate_classes(graph);
    let footer = "";// generate_footer(graph, client);
    format!("{header}\n{enums}\n{output_types}\n{input_types}\n{delegate_classes}\n{footer}")
}
