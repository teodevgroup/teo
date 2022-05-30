use inflector::Inflector;
use crate::action::action::{ActionResultData, ActionResultMeta, ActionType};
use crate::client::shared::code::Code;
use crate::client::typescript::r#type::ToTypeScriptType;
use crate::core::field::Availability;
use crate::core::graph::Graph;


pub(crate) async fn generate_index_ts(graph: &'static Graph) -> String {
    Code::new(0, 4, |c| {
        c.line(r#"import { request, Response, PagingInfo } from "./runtime""#);
        c.empty_line();
        // enum definitions
        graph.enums().iter().for_each(|e| {
            let name = e.0;
            let choices = e.1.iter().map(|i| {String::from("\"") + i + "\""}).collect::<Vec<String>>().join(" | ");
            c.line(format!("export type {name} = {choices}"));
            c.empty_line();
        });
        // model definitions
        graph.models().iter().for_each(|m| {
            if m.actions().len() > 0 {
                let model_name = m.name();
                c.block(format!("export type {model_name} = {{"), |b| {
                    m.output_keys().iter().for_each(|k| {
                        let field = m.field(k);
                        let field_name = field.name;
                        let field_type = field.r#type.to_typescript_type(field.availability == Availability::Optional);
                        b.line(format!("{field_name}: {field_type}"));
                    });
                }, "}");
                c.empty_line();
            }
        });
        // model input arguments
        // TODO: here add model input args

        // delegates
        graph.models().iter().for_each(|m| {
            if m.actions().len() > 0 {
                let model_name = m.name();
                let model_var_name = model_name.to_camel_case();
                let model_url_segment_name = m.url_segment_name();
                c.block(format!("const {model_var_name}Delegate = {{"), |b| {
                    ActionType::iter().for_each(|a| {
                        if m.actions().contains(a) {
                            let action_name = a.as_str();
                            let action_var_name = a.as_str().to_camel_case();
                            let result_meta = match a.result_meta() {
                                ActionResultMeta::PagingInfo => "PagingInfo",
                                ActionResultMeta::TokenInfo => "TokenInfo",
                                ActionResultMeta::NoMeta => "undefined",
                                ActionResultMeta::Other => "undefined",
                            };
                            let result_data = match a.result_data() {
                                ActionResultData::Single => model_name.to_string(),
                                ActionResultData::Vec => model_name.to_string() + "[]",
                                ActionResultData::Other => "never".to_string(),
                            };
                            b.empty_line();
                            b.block(format!("async {action_var_name}(args: {model_name}{action_name}Args): Promise<Response<{result_meta}, {result_data}>> {{"), |b| {
                                b.line(format!(r#"return await request("{model_url_segment_name}", "{action_name}", args)"#));
                            }, "},")
                        }
                    });
                }, "}");
                c.empty_line();
            }
        });
        // main interface
        c.block("const teo = {", |b| {
            graph.models().iter().for_each(|m| {
                if m.actions().len() > 0 {
                    let model_name = m.name();
                    let model_var_name = model_name.to_camel_case();
                    b.line(format!("{model_var_name}: {model_var_name}Delegate,"));
                }
            });
        }, "}");
        c.empty_line();
        c.line("export default teo");
    }).to_string()
}
