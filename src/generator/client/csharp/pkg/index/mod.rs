// use inflector::Inflector;
// use crate::core::action::{Action, CREATE_HANDLER, FIND_FIRST_HANDLER, ResData, ResMeta, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
// use crate::core::app::conf::ClientGeneratorConf;
// use crate::core::field::r#type::FieldTypeOwner;
// use crate::generator::client::csharp::pkg::index::doc::{action_doc, action_group_doc, create_or_update_doc, credentials_doc, cursor_doc, field_doc, include_doc, nested_connect_doc, nested_create_doc, nested_create_or_connect_doc, nested_delete_doc, nested_disconnect_doc, nested_set_doc, nested_update_doc, nested_upsert_doc, order_by_doc, page_number_doc, page_size_doc, relation_doc, select_doc, skip_doc, take_doc, unique_connect_create_doc, unique_connect_doc, unique_where_doc, where_doc, where_doc_first};
// use crate::generator::client::csharp::r#type::ToCSharpType;
// use crate::core::graph::Graph;
// use crate::core::model::{Model};
// use crate::generator::lib::code::Code;
//
// mod doc;
//
//             // delegates
//             let object_name = "teo";
//             c.empty_line();
//             graph.models().iter().for_each(|m| {
//                 if m.actions().len() > 0 {
//                     let model_name = m.name();
//                     let model_var_name = model_name.to_camel_case();
//                     let model_class_name = model_var_name.to_pascal_case();
//                     let model_url_segment_name = m.name();
//                     c.block(format!("public class {model_class_name}Delegate : Delegate {{"), |b| {
//                         b.empty_line();
//                         b.line("readonly string? _Token;");
//                         b.empty_line();
//                         b.block(format!("protected internal {model_class_name}Delegate(string? token = null) {{"), |b| {
//                             b.line("_Token = token;");
//                         }, "}");
//                         Action::handlers_iter().for_each(|a| {
//                             if m.has_action(*a) {
//                                 let action_name = a.as_handler_str();
//                                 let action_var_name = a.as_handler_str().to_pascal_case();
//                                 let action_url_name = a.as_handler_str();
//                                 let res_meta = match a.handler_res_meta() {
//                                     ResMeta::PagingInfo => "PagingInfo, ",
//                                     ResMeta::TokenInfo => "TokenInfo, ",
//                                     ResMeta::NoMeta => "",
//                                     ResMeta::Other => "",
//                                 };
//                                 let res_data = match a.handler_res_data() {
//                                     ResData::Single => model_name.to_string(),
//                                     ResData::Vec => model_name.to_string() + "[]",
//                                     ResData::Other => "short".to_string(),
//                                     ResData::Number => "uint".to_string(),
//                                 };
//                                 b.empty_line();
//                                 b.doc(action_doc(object_name, a.clone(), m));
//                                 b.block(format!("public async Task<Response<{res_meta}{res_data}>> {action_var_name}({model_name}{action_name}Args? args = null, string? token = null) {{"), |b| {
//                                     b.line(format!(r#"return await Request<Response<{res_meta}{res_data}>>("{model_url_segment_name}", "{action_url_name}", args ?? new(), token ?? _Token);"#));
//                                 }, "}");
//                             }
//                         });
//                     }, "}");
//                     c.empty_line();
//                 }
//             });
//             // main class
//             c.block(format!("public class Teo {{"), |b| {
//                 b.empty_line();
//                 graph.models().iter().for_each(|m| {
//                     if m.actions().len() > 0 {
//                         let model_name = m.name();
//                         let model_class_name = model_name.to_pascal_case();
//                         b.doc(action_group_doc(object_name, m));
//                         b.line(format!("public {model_class_name}Delegate {model_class_name} {{ get; }}"));
//                     }
//                 });
//                 b.empty_line();
//                 b.block("public Teo(string? token = null) {", |b| {
//                     graph.models().iter().for_each(|m| {
//                         if m.actions().len() > 0 {
//                             let model_name = m.name();
//                             let model_class_name = model_name.to_pascal_case();
//                             b.line(format!("{model_class_name} = new(token);"));
//                         }
//                     })
//                 }, "}");
//             }, "}");
//         }, "}");
//     }).to_string()
// }
