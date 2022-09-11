use inflector::Inflector;
use crate::core::action::r#type::{ActionResultData, ActionResultMeta, ActionType};
use crate::app::app::ClientConfiguration;
use crate::client::shared::code::Code;
use crate::client::typescript::pkg::src::index_ts::docs::{action_doc, action_group_doc, create_or_update_doc, credentials_doc, cursor_doc, field_doc, include_doc, main_object_doc, nested_connect_doc, nested_create_doc, nested_create_or_connect_doc, nested_delete_doc, nested_disconnect_doc, nested_set_doc, nested_update_doc, nested_upsert_doc, order_by_doc, page_number_doc, page_size_doc, relation_doc, select_doc, skip_doc, take_doc, unique_connect_create_doc, unique_connect_doc, unique_where_doc, where_doc, where_doc_first};
use crate::client::typescript::r#type::ToTypeScriptType;
use crate::core::field::Optionality;
use crate::core::graph::Graph;
use crate::core::model::{Model, ModelIndexType};

mod docs;

fn generate_model_create_nested_input(_graph: &Graph, model: &Model, without: Option<&str>, many: bool) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let many_title = if many { "Many" } else { "One" };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}CreateNested{many_title}{without_title}Input = {{"), |b| {
            if many {
                b.doc(nested_create_doc(model, many));
                b.line(format!("create?: Enumerable<{model_name}Create{without_title}Input>"));
                b.doc(nested_create_or_connect_doc(model, many));
                b.line(format!("connectOrCreate?: Enumerable<{model_name}CreateOrConnect{without_title}Input>"));
                b.doc(nested_connect_doc(model, many));
                b.line(format!("connect?: Enumerable<{model_name}WhereUniqueInput>"));
            } else {
                b.doc(nested_create_doc(model, many));
                b.line(format!("create?: {model_name}Create{without_title}Input"));
                b.doc(nested_create_or_connect_doc(model, many));
                b.line(format!("connectOrCreate?: {model_name}CreateOrConnect{without_title}Input"));
                b.doc(nested_connect_doc(model, many));
                b.line(format!("connect?: {model_name}WhereUniqueInput"));
            }
        }, "}")
    }).to_string()
}

fn generate_model_create_or_connect_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}CreateOrConnect{without_title}Input = {{"), |b| {
            b.doc(unique_connect_doc(model));
            b.line(format!("where: {model_name}WhereUniqueInput"));
            b.doc(unique_connect_create_doc(model));
            b.line(format!("create: {model_name}Create{without_title}Input"));
        }, "}")
    }).to_string()
}

fn generate_model_create_input(graph: &Graph, model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let without_relation = if let Some(title) = without {
        Some(model.relation(title).unwrap())
    } else {
        None
    };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}Create{without_title}Input = {{"), |b| {
            model.input_keys().iter().for_each(|k| {
                if let Some(field) = model.field(k) {
                    let field_name = &field.name;
                    let field_ts_type = field.field_type.to_typescript_create_input_type(field.optionality == Optionality::Optional);
                    if let Some(without_relation) = without_relation {
                        if !without_relation.fields.contains(k) {
                            b.doc(field_doc(field));
                            b.line(format!("{field_name}?: {field_ts_type}"));
                        }
                    } else {
                        b.doc(field_doc(field));
                        b.line(format!("{field_name}?: {field_ts_type}"));
                    }
                } else if let Some(relation) = model.relation(k) {
                    let relation_name = &relation.name;
                    let relation_model_name = &relation.model;
                    let relation_model = graph.model(relation_model_name).unwrap();
                    let num = if relation.is_vec { "Many" } else { "One" };
                    if let Some(without_relation) = without_relation {
                        if &without_relation.name != k {
                            if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                                r.fields == relation.references && r.references == relation.fields
                            }) {
                                let opposite_relation_name = opposite_relation.name.to_pascal_case();
                                b.doc(relation_doc(relation));
                                b.line(format!("{relation_name}?: {relation_model_name}CreateNested{num}Without{opposite_relation_name}Input"))
                            } else {
                                b.doc(relation_doc(relation));
                                b.line(format!("{relation_name}?: {relation_model_name}CreateNested{num}Input"))
                            }
                        }
                    } else {
                        if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                            r.fields == relation.references && r.references == relation.fields
                        }) {
                            let opposite_relation_name = opposite_relation.name.to_pascal_case();
                            b.doc(relation_doc(relation));
                            b.line(format!("{relation_name}?: {relation_model_name}CreateNested{num}Without{opposite_relation_name}Input"))
                        } else {
                            b.doc(relation_doc(relation));
                            b.line(format!("{relation_name}?: {relation_model_name}CreateNested{num}Input"))
                        }
                    }
                }
            });
        }, "}");
    }).to_string()
}

fn generate_model_upsert_with_where_unique_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}UpsertWithWhereUnique{without_title}Input = {{"), |b| {
            b.doc(unique_where_doc(model));
            b.line(format!("where: {model_name}WhereUniqueInput"));
            b.doc(create_or_update_doc(model, ActionType::Update));
            b.line(format!("update: {model_name}Update{without_title}Input"));
            b.doc(create_or_update_doc(model, ActionType::Create));
            b.line(format!("create: {model_name}Create{without_title}Input"));
        }, "}")
    }).to_string()
}

fn generate_model_update_with_where_unique_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}UpdateWithWhereUnique{without_title}Input = {{"), |b| {
            b.doc(unique_where_doc(model));
            b.line(format!("where: {model_name}WhereUniqueInput"));
            b.doc(create_or_update_doc(model, ActionType::Update));
            b.line(format!("update: {model_name}Update{without_title}Input"));
        }, "}")
    }).to_string()
}

fn generate_model_update_many_with_where_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}UpdateManyWithWhere{without_title}Input = {{"), |b| {
            b.doc(where_doc(model));
            b.line(format!("where: {model_name}WhereInput"));
            b.doc(create_or_update_doc(model, ActionType::UpdateMany));
            b.line(format!("update: {model_name}Update{without_title}Input"));
        }, "}")
    }).to_string()
}

fn generate_model_update_nested_input(_graph: &Graph, model: &Model, without: Option<&str>, many: bool) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let many_title = if many { "Many" } else { "One" };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}UpdateNested{many_title}{without_title}Input = {{"), |b| {
            if many {
                b.doc(nested_create_doc(model, many));
                b.line(format!("create?: Enumerable<{model_name}Create{without_title}Input>"));
                b.doc(nested_create_or_connect_doc(model, many));
                b.line(format!("connectOrCreate?: Enumerable<{model_name}CreateOrConnect{without_title}Input>"));
                b.doc(nested_connect_doc(model, many));
                b.line(format!("connect?: Enumerable<{model_name}WhereUniqueInput>"));
                b.doc(nested_set_doc(model, many));
                b.line(format!("set?: Enumerable<{model_name}WhereUniqueInput>"));
                b.doc(nested_update_doc(model, many));
                b.line(format!("update?: Enumerable<{model_name}UpdateWithWhereUnique{without_title}Input>"));
                b.doc(nested_upsert_doc(model, many));
                b.line(format!("upsert?: Enumerable<{model_name}UpsertWithWhereUnique{without_title}Input>"));
                b.doc(nested_disconnect_doc(model, many));
                b.line(format!("disconnect?: Enumerable<{model_name}WhereUniqueInput>"));
                b.doc(nested_delete_doc(model, many));
                b.line(format!("delete?: Enumerable<{model_name}WhereUniqueInput>"));
                b.doc(nested_update_doc(model, true));
                b.line(format!("updateMany?: Enumerable<{model_name}UpdateManyWithWhere{without_title}Input>"));
                b.doc(nested_delete_doc(model, true));
                b.line(format!("deleteMany?: Enumerable<{model_name}WhereInput>"));
            } else {
                b.doc(nested_create_doc(model, many));
                b.line(format!("create?: {model_name}Create{without_title}Input"));
                b.doc(nested_create_or_connect_doc(model, many));
                b.line(format!("connectOrCreate?: {model_name}CreateOrConnect{without_title}Input"));
                b.doc(nested_connect_doc(model, many));
                b.line(format!("connect?: {model_name}WhereUniqueInput"));
                b.doc(nested_set_doc(model, many));
                b.line(format!("set?: {model_name}WhereUniqueInput"));
                b.doc(nested_update_doc(model, many));
                b.line(format!("update?: {model_name}UpdateWithWhereUnique{without_title}Input"));
                b.doc(nested_upsert_doc(model, many));
                b.line(format!("upsert?: {model_name}UpsertWithWhereUnique{without_title}Input"));
                b.doc(nested_disconnect_doc(model, many));
                b.line(format!("disconnect?: {model_name}WhereUniqueInput"));
                b.doc(nested_delete_doc(model, many));
                b.line(format!("delete?: {model_name}WhereUniqueInput"));
            }
        }, "}")
    }).to_string()
}

fn generate_model_update_input(graph: &Graph, model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let without_relation = if let Some(title) = without {
        Some(model.relation(title).unwrap())
    } else {
        None
    };
    Code::new(0, 4, |c| {
        c.block(format!("export type {model_name}Update{without_title}Input = {{"), |b| {
            model.input_keys().iter().for_each(|k| {
                if let Some(field) = model.field(k) {
                    let field_name = &field.name;
                    let field_ts_type = field.field_type.to_typescript_update_input_type(field.optionality == Optionality::Optional);
                    b.doc(field_doc(field));
                    b.line(format!("{field_name}?: {field_ts_type}"));
                } else if let Some(relation) = model.relation(k) {
                    let relation_name = &relation.name;
                    let relation_model_name = &relation.model;
                    let relation_model = graph.model(relation_model_name).unwrap();
                    let num = if relation.is_vec { "Many" } else { "One" };
                    if let Some(without_relation) = without_relation {
                        if &without_relation.name != k {
                            if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                                r.fields == relation.references && r.references == relation.fields
                            }) {
                                let opposite_relation_name = opposite_relation.name.to_pascal_case();
                                b.doc(relation_doc(relation));
                                b.line(format!("{relation_name}?: {relation_model_name}UpdateNested{num}Without{opposite_relation_name}Input"))
                            } else {
                                b.doc(relation_doc(relation));
                                b.line(format!("{relation_name}?: {relation_model_name}UpdateNested{num}Input"))
                            }
                        }
                    } else {
                        if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                            r.fields == relation.references && r.references == relation.fields
                        }) {
                            let opposite_relation_name = opposite_relation.name.to_pascal_case();
                            b.doc(relation_doc(relation));
                            b.line(format!("{relation_name}?: {relation_model_name}UpdateNested{num}Without{opposite_relation_name}Input"))
                        } else {
                            b.doc(relation_doc(relation));
                            b.line(format!("{relation_name}?: {relation_model_name}UpdateNested{num}Input"))
                        }
                    }
                }
            });
        }, "}")
    }).to_string()
}

fn generate_model_credentials_input(model: &Model) -> String {
    let model_name = model.name();
    Code::new(0, 4, |c| {
        c.block(format!(r#"export type {model_name}CredentialsInput = {{"#), |b| {
            let auth_identity_keys = model.auth_identity_keys();
            let auth_by_keys = model.auth_by_keys();
            let auth_identity_optional = auth_identity_keys.len() != 1;
            let auth_by_keys_optional = auth_by_keys.len() != 1;
            for key in auth_identity_keys {
                let field = model.field(key).unwrap();
                let field_name = &field.name;
                let field_type = field.field_type.to_typescript_type(auth_identity_optional);
                b.doc(field_doc(field));
                b.line(format!("{field_name}: {field_type}"));
            }
            for key in auth_by_keys {
                let field = model.field(key).unwrap();
                let field_name = &field.name;
                let field_type = field.field_type.to_typescript_type(auth_by_keys_optional);
                b.doc(field_doc(field));
                b.line(format!("{field_name}: {field_type}"));
            }
        }, "}");
    }).to_string()
}

pub(crate) async fn generate_index_ts(graph: &Graph, conf: &ClientConfiguration) -> String {
    Code::new(0, 4, |c| {
        c.line(r#"import { request, Response, PagingInfo, TokenInfo, SortOrder, Enumerable, CheckSelectInclude, SelectSubset, ExistKeys } from "./runtime""#);
        c.block("import {", |b| {
            b.line("ObjectIdFilter, ObjectIdNullableFilter, StringFilter, StringNullableFilter, NumberFilter,");
            b.line("NumberNullableFilter, BoolFilter, BoolNullableFilter, DateFilter, DateNullableFilter,");
            b.line("DateTimeFilter, DateTimeNullableFilter, EnumFilter, EnumNullableFilter,");
            b.line("ArrayFilter, ArrayNullableFilter,");
        }, "} from \"./filter\"");
        c.block("import {", |b| {
            b.line("ObjectIdFieldUpdateOperationsInput, NullableObjectIdFieldUpdateOperationsInput, StringFieldUpdateOperationsInput,");
            b.line("NullableStringFieldUpdateOperationsInput, NumberFieldUpdateOperationsInput, NullableNumberFieldUpdateOperationsInput,");
            b.line("BoolFieldUpdateOperationsInput, NullableBoolFieldUpdateOperationsInput, DateFieldUpdateOperationsInput,");
            b.line("NullableDateFieldUpdateOperationsInput, DateTimeFieldUpdateOperationsInput, NullableDateTimeFieldUpdateOperationsInput,");
            b.line("EnumFieldUpdateOperationsInput, NullableEnumFieldUpdateOperationsInput,");
            b.line("ArrayFieldUpdateOperationsInput, NullableArrayFieldUpdateOperationsInput,");
        }, "} from \"./operation\"");

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
                        if let Some(_field) = m.field(k) {
                            let field = m.field(k).unwrap();
                            let field_name = &field.name;
                            let field_type = field.field_type.to_typescript_type(field.optionality == Optionality::Optional);
                            b.line(format!("{field_name}: {field_type}"));
                        } else if let Some(_relation) = m.relation(k) {

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
            let _model_var_name = model_name.to_camel_case();
            c.block(format!("export type {model_name}Select = {{"), |b| {
                m.output_keys().iter().for_each(|k| {
                    if let Some(_field) = m.field(k) {
                        let field = m.field(k).unwrap();
                        let field_name = &field.name;
                        b.line(format!("{field_name}?: boolean"));
                    }
                })
            }, "}");
            c.block(format!("export type {model_name}Include = {{"), |b| {
                for relation in m.relations() {
                    let name = &relation.name;
                    let is_vec = relation.is_vec;
                    let find_many = if is_vec { "FindMany" } else { "" };
                    let r_model = &relation.model;
                    b.line(format!("{name}?: boolean | null | {r_model}{find_many}Args"));
                }
            }, "}");
            c.block(format!("export type {model_name}WhereInput = {{"), |b| {
                for op in ["AND", "OR", "NOT"] {
                    b.line(format!("{op}?: Enumerable<{model_name}WhereInput>"));
                }
                m.query_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        let field_filter = field.field_type.to_typescript_filter_type(field.optionality == Optionality::Optional);
                        b.doc(field_doc(field));
                        b.line(format!("{field_name}?: {field_filter}"));
                    } else if let Some(relation) = m.relation(k) {
                        let list = if relation.is_vec { "List" } else { "" };
                        let relation_name = &relation.name;
                        let relation_model = &relation.model;
                        b.doc(relation_doc(relation));
                        b.line(format!("{relation_name}?: {relation_model}{list}RelationFilter"));
                    }
                })
            }, "}");
            c.block(format!("export type {model_name}WhereUniqueInput = {{"), |b| {
                use ModelIndexType::*;
                let mut used_field_names: Vec<&str> = Vec::new();
                m.indices().iter().for_each(|index| {
                    if index.index_type == Primary || index.index_type == Unique {
                        index.items.iter().for_each(|item| {
                            if !used_field_names.contains(&&***&&item.field_name) {
                                if let Some(field) = m.field(&item.field_name) {
                                    let ts_type = field.field_type.to_typescript_type(false);
                                    let field_name = &item.field_name;
                                    b.doc(field_doc(field));
                                    b.line(format!("{field_name}?: {ts_type}"));
                                }
                                used_field_names.push(&item.field_name);
                            }
                        });
                    }
                });
            }, "}");
            c.block(format!("export type {model_name}RelationFilter = {{"), |b| {
                b.line(format!("is?: {model_name}WhereInput"));
                b.line(format!("isNot?: {model_name}WhereInput"));
            }, "}");
            c.block(format!("export type {model_name}ListRelationFilter = {{"), |b| {
                b.line(format!("every?: {model_name}WhereInput"));
                b.line(format!("some?: {model_name}WhereInput"));
                b.line(format!("none?: {model_name}WhereInput"));
            }, "}");
            c.block(format!("export type {model_name}OrderByInput = {{"), |b| {
                m.query_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        b.line(format!("{field_name}?: SortOrder"));
                    } else if let Some(relation) = m.relation(k) {
                        let _relation_model = &relation.model;
                        let _relation_name = &relation.name;
                        //b.line(format!("{relation_name}?: {relation_model}OrderByRelationAggregateInput"));
                    }
                })
            }, "}");
            c.line(generate_model_create_input(graph, m, None));
            c.line(generate_model_create_nested_input(graph, m, None, true));
            c.line(generate_model_create_nested_input(graph, m, None, false));
            c.line(generate_model_create_or_connect_input(m, None));
            m.relations().iter().for_each(|r| {
                c.line(generate_model_create_input(graph, m, Some(&r.name)));
                c.line(generate_model_create_nested_input(graph, m, Some(&r.name), true));
                c.line(generate_model_create_nested_input(graph, m, Some(&r.name), false));
                c.line(generate_model_create_or_connect_input(m, Some(&r.name)));
            });
            c.line(generate_model_update_input(graph, m, None));
            c.line(generate_model_update_nested_input(graph, m, None, true));
            c.line(generate_model_update_nested_input(graph, m, None, false));
            c.line(generate_model_upsert_with_where_unique_input(m, None));
            c.line(generate_model_update_with_where_unique_input(m, None));
            c.line(generate_model_update_many_with_where_input(m, None));
            m.relations().iter().for_each(|r| {
                c.line(generate_model_update_input(graph, m, Some(&r.name)));
                c.line(generate_model_update_nested_input(graph, m, Some(&r.name), true));
                c.line(generate_model_update_nested_input(graph, m, Some(&r.name), false));
                c.line(generate_model_upsert_with_where_unique_input(m, Some(&r.name)));
                c.line(generate_model_update_with_where_unique_input(m, Some(&r.name)));
                c.line(generate_model_update_many_with_where_input(m, Some(&r.name)));
            });
            if m.identity() {
                c.line(generate_model_credentials_input(m));
            }
            // args
            c.block(format!(r#"export type {model_name}Args = {{"#), |b| {
                b.doc(select_doc(m));
                b.line(format!(r#"select?: {model_name}Select"#));
                b.doc(include_doc(m));
                b.line(format!(r#"include?: {model_name}Include"#));
            }, "}");
            ActionType::iter().for_each(|a| {
                if !m.actions().contains(a) { return }
                let action_name = a.as_str();
                let _action_var_name = a.as_str().to_camel_case();
                c.block(format!(r#"export type {model_name}{action_name}Args = {{"#), |b| {
                    if a.requires_where() {
                        if a == &ActionType::FindFirst {
                            b.doc(where_doc_first(m));
                        } else {
                            b.doc(where_doc(m));
                        }
                        b.line(format!(r#"where?: {model_name}WhereInput"#));
                    }
                    if a.requires_where_unique() {
                        b.doc(unique_where_doc(m));
                        b.line(format!(r#"where?: {model_name}WhereUniqueInput"#));
                    }
                    b.doc(select_doc(m));
                    b.line(format!(r#"select?: {model_name}Select"#));
                    b.doc(include_doc(m));
                    b.line(format!(r#"include?: {model_name}Include"#));
                    if a.requires_where() {
                        b.doc(order_by_doc(m));
                        b.line(format!(r#"orderBy?: Enumerable<{model_name}OrderByInput>"#));
                        b.doc(cursor_doc(m));
                        b.line(format!(r#"cursor?: {model_name}WhereUniqueInput"#));
                        b.doc(take_doc(m));
                        b.line(format!(r#"take?: number"#));
                        b.doc(skip_doc(m));
                        b.line(format!(r#"skip?: number"#));
                        b.doc(page_size_doc(m));
                        b.line(format!(r#"pageSize?: number"#));
                        b.doc(page_number_doc(m));
                        b.line(format!(r#"pageNumber?: number"#));
                        //b.line(format!{r#"distinct? {model_name}ScalarFieldEnum"#})
                    }
                    if a.requires_create() {
                        b.doc(create_or_update_doc(m, if a == &ActionType::Upsert { ActionType::Create } else { a.clone() }));
                        b.line(format!(r#"create: {model_name}CreateInput"#));
                    }
                    if a.requires_update() {
                        b.doc(create_or_update_doc(m, if a == &ActionType::Upsert { ActionType::Update } else { a.clone() }));
                        b.line(format!(r#"update: {model_name}UpdateInput"#));
                    }
                    if a.requires_credentials() {
                        b.doc(credentials_doc(m, *a));
                        b.line(format!(r#"credentials: {model_name}CredentialsInput"#))
                    }
                }, "}");
            });
            c.block(format!("export type {model_name}GetPayload<S extends boolean | null | undefined | {model_name}Args, U = keyof S> = S extends true"), |b| {
                b.line(format!("? {model_name}"));
                b.block(": S extends undefined", |b| {
                    b.line("? never");
                    b.block(format!(": S extends {model_name}Args | {model_name}FindManyArgs"), |b| {
                        b.block("? 'include' extends U", |b| {
                            b.block(format!("? SelectSubset<{model_name}, S> & {{"), |b| {
                                b.block(format!("[P in ExistKeys<S['include']>]:"), |b| {
                                    for relation in m.relations() {
                                        let name = &relation.name;
                                        let is_array = relation.is_vec;
                                        let required = relation.optionality == Optionality::Required;
                                        let required_mark = if required { "" } else { "?" };
                                        let r_model = &relation.model;
                                        let array_prefix = if is_array { "Array<" } else { "" };
                                        let array_suffix = if is_array { ">" } else { "" };
                                        b.line(format!("P extends '{name}' ? {array_prefix}{r_model}GetPayload<S['include'][P]>{array_suffix}{required_mark} :"));
                                    }
                                }, "never");
                            }, "}");
                            b.line(format!(": SelectSubset<{model_name}, S>"));
                        }, format!(": {model_name}"));
                    }, "");
                }, "");
            }, "")
        });
        // delegates
        let object_name = &conf.type_script.as_ref().unwrap().object_name;
        let object_class_name = object_name.to_pascal_case();
        graph.models().iter().for_each(|m| {
            if m.actions().len() > 0 {
                let model_name = m.name();
                let model_var_name = model_name.to_camel_case();
                let model_class_name = model_var_name.to_pascal_case();
                let model_url_segment_name = m.url_segment_name();
                c.block(format!("class {model_class_name}Delegate {{"), |b| {
                    b.line("_token?: string");
                    b.block("constructor(token?: string) {", |b| {
                        b.line("this._token = token");
                    }, "}");
                    ActionType::iter().for_each(|a| {
                        if m.actions().contains(a) {
                            let action_name = a.as_str();
                            let action_var_name = a.as_str().to_camel_case();
                            let action_url_name = a.as_url_segment();
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
                                ActionResultData::Number => "number".to_string(),
                            };
                            let payload_array = match a.result_data() {
                                ActionResultData::Vec => "[]",
                                _ => ""
                            };
                            b.empty_line();
                            b.doc(action_doc(object_name, a.clone(), m));
                            b.block(format!("async {action_var_name}<T extends {model_name}{action_name}Args>(args?: T): Promise<Response<{result_meta}, CheckSelectInclude<T, {result_data}, {model_name}GetPayload<T>{payload_array}>>> {{"), |b| {
                                b.line(format!(r#"return await request("{model_url_segment_name}", "{action_url_name}", args ?? {{}}, this._token)"#));
                            }, "}");
                        }
                    });
                }, "}");
                c.empty_line();
            }
        });
        // main interface
        c.block(format!("class {object_class_name} {{"), |b| {
            b.line("_token?: string");
            graph.models().iter().for_each(|m| {
                if m.actions().len() > 0 {
                    let model_name = m.name();
                    let model_var_name = model_name.to_camel_case();
                    let model_class_name = model_var_name.to_pascal_case();
                    b.doc(action_group_doc(object_name, m));
                    b.line(format!("{model_var_name}: {model_class_name}Delegate"));
                }
            });
            b.block("constructor(token?: string) {", |b| {
                b.line("this._token = token");
                graph.models().iter().for_each(|m| {
                    if m.actions().len() > 0 {
                        let model_name = m.name();
                        let model_var_name = model_name.to_camel_case();
                        let model_class_name = model_var_name.to_pascal_case();
                        b.line(format!("this.{model_var_name} = new {model_class_name}Delegate(token)"));
                    }
                })
            }, "}");
            b.block("$withToken(token?: string) {", |b| {
                b.line(format!("return new {object_class_name}(token)"));
            }, "}")
        }, "}");
        c.empty_line();
        c.line(main_object_doc(object_name, graph));
        c.line(format!("const {object_name} = new {object_class_name}()"));
        c.empty_line();
        c.line(format!("export default {object_name}"));
    }).to_string()
}
