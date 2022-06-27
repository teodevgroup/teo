use inflector::Inflector;
use crate::action::action::{ActionResultData, ActionResultMeta, ActionType};
use crate::client::shared::code::Code;
use crate::client::typescript::r#type::ToTypeScriptType;
use crate::core::field::Optionality;
use crate::core::graph::Graph;
use crate::core::model::ModelIndexType;


pub(crate) async fn generate_index_ts(graph: &'static Graph) -> String {
    Code::new(0, 4, |c| {
        c.line(r#"import { request, Response, PagingInfo, Order } from "./runtime""#);
        c.empty_line();
        // enum definitions
        graph.enums().iter().for_each(|e| {
            let name = e.0;
            let choices = e.1.values.iter().map(|i| {String::from("\"") + i + "\""}).collect::<Vec<String>>().join(" | ");
            c.line(format!("export type {name} = {choices}"));
            c.empty_line();
        });
        // model definitions
        graph.models().iter().for_each(|m| {
            if m.actions().len() > 0 {
                let model_name = m.name();
                c.block(format!("export type {model_name} = {{"), |b| {
                    m.output_keys().iter().for_each(|k| {
                        if let Some(field) = m.field(k) {
                            let field = m.field(k).unwrap();
                            let field_name = &field.name;
                            let field_type = field.field_type.to_typescript_type(field.optionality == Optionality::Optional);
                            b.line(format!("{field_name}: {field_type}"));
                        } else if let Some(relation) = m.relation(k) {

                        }
                    });
                }, "}");
                c.empty_line();
            }
        });
        // model input arguments
        graph.models().iter().for_each(|m| {
            if m.actions().len() == 0 { return }
            let model_name = m.name();
            let model_var_name = model_name.to_camel_case();
            c.block(format!("export type {model_name}Select = {{"), |b| {
                m.output_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field = m.field(k).unwrap();
                        let field_name = &field.name;
                        b.line("{field_name}?: boolean");
                    }
                })
            }, "}");
            c.block(format!("export type {model_name}Include = {{"), |b| {
                b.empty_line();
            }, "}");
            c.block(format!("export type {model_name}WhereInput = {{"), |b| {
                for op in ["AND", "OR", "NOT"] {
                    b.line(format!("{op}?: Enumerable<{model_name}WhereInput>"));
                }
                m.query_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        let field_filter = field.field_type.to_typescript_filter_type(field.optionality == Optionality::Optional);
                        b.line(format!("{field_name}?: {field_filter}"));
                    } else if let Some(relation) = m.relation(k) {
                        let list = if relation.is_vec { "List" } else { "" };
                        let relation_name = &relation.name;
                        let relation_model = &relation.model;
                        b.line(format!("{relation_name}?: {relation_model}{list}RelationFilter"));
                    }
                })
            }, "}");
            c.block(format!("export type {model_name}WhereUniqueInput = {{"), |b| {
                use ModelIndexType::*;
                let mut used_field_names: Vec<&str> = Vec::new();
                m.indices.iter().for_each(|index| {
                    if index.index_type == Primary || index.index_type == Unique {
                        index.items.iter().for_each(|item| {
                            if !used_field_names.contains(&&***&&item.field_name) {
                                if let Some(field) = m.field(&item.field_name) {
                                    let ts_type = field.field_type.to_typescript_type(false);
                                    let field_name = &item.field_name;
                                    b.line(format!("{field_name}?: ts_type"));
                                }
                                used_field_names.push(&item.field_name);
                            }
                        });
                    }
                });
            }, "}");
            c.block(format!("export type {model_name}OrderByWithRelationInput = {{"), |b| {
                m.query_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        b.line(format!("{field_name}?: SortOrder"));
                    } else if let Some(relation) = m.relation(k) {
                        let relation_model = &relation.model;
                        let relation_name = &relation.name;
                        //b.line(format!("{relation_name}?: {relation_model}OrderByRelationAggregateInput"));
                    }
                })
            }, "}");
            c.block(format!("export type {model_name}CreateInput = {{"), |b| {
                m.input_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        let field_ts_type = field.field_type.to_typescript_create_input_type(field.optionality == Optionality::Optional);
                        b.line(format!("{field_name}?: {field_ts_type}"));
                    } else if let Some(relation) = m.relation(k) {
                        let relation_name = &relation.name;
                        let relation_model_name = &relation.model;
                        let relation_model = graph.model(relation_model_name);
                        let num = if relation.is_vec { "Many" } else { "One" };
                        if let Some(opposite_relation) = relation_model.relations_vec.iter().find(|r| {
                            r.fields == relation.references && r.references == relation.fields
                        }) {
                            let opposite_relation_name = opposite_relation.name.to_pascal_case();
                            b.line(format!("{relation_name}?: {relation_model_name}CreateNested{num}Without{opposite_relation_name}Input"))
                        } else {
                            b.line(format!("{relation_name}?: {relation_model_name}CreateNested{num}Input"))
                        }
                    }
                });
            }, "}");
            c.block(format!("export type {model_name}UpdateInput = {{"), |b| {
                m.input_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        let field_ts_type = field.field_type.to_typescript_update_input_type(field.optionality == Optionality::Optional);
                        b.line("{field_name}?: {field_ts_type}");
                    } else if let Some(relation) = m.relation(k) {

                    }
                });
            }, "}");
            // args
            ActionType::iter().for_each(|a| {
                if !m.actions().contains(a) { return }
                let action_name = a.as_str();
                let action_var_name = a.as_str().to_camel_case();
                c.block(format!(r#"export type {model_name}{action_name}Args = {{"#), |b| {
                    b.line(format!(r#"select?: {model_name}Select"#));
                    b.line(format!(r#"include?: {model_name}Include"#));
                    if a.requires_where() {
                        b.line(format!(r#"where?: {model_name}WhereInput"#));
                    }
                    if a.requires_where_unique() {
                        b.line(format!(r#"where?: {model_name}WhereUniqueInput"#));
                    }
                    if a.requires_where() {
                        b.line(format!(r#"orderBy?: Enumerable<{model_name}OrderByInput>"#));
                        b.line(format!(r#"cursor?: {model_name}WhereUniqueInput"#));
                        b.line(format!(r#"take?: number"#));
                        b.line(format!(r#"skip?: number"#));
                        b.line(format!(r#"pageSize?: number"#));
                        b.line(format!(r#"pageNumber?: number"#));
                        //b.line(format!{r#"distinct? {model_name}ScalarFieldEnum"#})
                    }
                    if a.requires_create() {
                        b.line(format!(r#"create?: {model_name}CreateInput"#));
                    }
                    if a.requires_update() {
                        b.line(format!(r#"update?: {model_name}UpdateInput"#));
                    }
                }, "}");
            })
        });
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
