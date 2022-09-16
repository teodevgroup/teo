use inflector::Inflector;
use crate::core::action::r#type::{ActionResultData, ActionResultMeta, ActionType};
use crate::app::app::ClientConfiguration;
use crate::client::csharp::pkg::index::doc::{create_or_update_doc, field_doc, nested_connect_doc, nested_create_doc, nested_create_or_connect_doc, nested_delete_doc, nested_disconnect_doc, nested_set_doc, nested_update_doc, nested_upsert_doc, relation_doc, unique_connect_create_doc, unique_connect_doc, unique_where_doc, where_doc};
use crate::client::csharp::r#type::ToCSharpType;
use crate::client::shared::code::Code;
use crate::core::field::Optionality;
use crate::core::graph::Graph;
use crate::core::model::{Model, ModelIndexType};

mod doc;

fn get_set() -> &str {
    "{ get; set; }"
}

struct CSharpClassField {
    n: String,
    t: String,
    o: bool,
    d: Option<String>,
    j: Option<String>,
}

struct CSharpClassBuilder {
    name: String,
    fields: Vec<CSharpClassField>,
    indent_spaces: u8,
    indent_level: u8,
}

impl CSharpClassBuilder {
    fn build(&self) -> String {
        let required_fields = self.fields.iter().filter(|f| {
            f.o == false
        }).collect::<Vec<&CSharpClassField>>();
        let optional_fields = self.fields.iter().filter(|f| {
            f.o == true
        }).collect::<Vec<&CSharpClassField>>();
        Code::new(self.indent_level, self.indent_spaces, |c| {
            let class_name = &self.name;
            c.block(format!("public class {class_name} {{"), |b| {
                for f in &self.fields {
                    if let Some(doc) = &f.d {
                        b.doc(doc);
                    }
                    let field_name = &f.n;
                    let question_mark = if f.o { "?" } else { "" };
                    let field_type = &f.t;
                    if let Some(json_key) = &f.j {
                        b.line(format!(r#"[JsonPropertyName("{json_key}")]"#));
                    }
                    b.line(format!("public {field_type}{question_mark} {field_name} {get_set}"))
                }
                b.block(format!("public {class_name}("), |b| {
                    for f in required_fields {
                        let camelized = f.n.to_camel_case();
                        let field_type = &f.t;
                        b.line(format!("{field_type} {camelized},"));
                    }
                    for f in optional_fields {
                        let camelized = f.n.to_camel_case();
                        let field_type = &f.t;
                        b.line(format!("{field_type}? {camelized} = null,"));
                    }
                }, ") {");
                b.block("", |b| {
                    for f in &self.fields {
                        let name = &f.n;
                        let camelized = name.to_camel_case();
                        b.line(format!("{name} = {camelized};"));
                    }
                }, "}");
            }, "}");
        }).to_string()
    }
}

fn generate_model_create_nested_input(_graph: &Graph, model: &Model, without: Option<&str>, many: bool) -> String {
    let get_set = get_set();
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let many_title = if many { "Many" } else { "One" };
    let mut class_fields: Vec<CSharpClassField> = Vec::new();
    let enumerable_prefix = if many { "Enumerable<" } else { "" };
    let enumerable_suffix = if many { ">" } else { "" };
    class_fields.push(CSharpClassField {
        n: "Create".to_owned(),
        t: format!("{enumerable_prefix}{model_name}Create{without_title}Input{enumerable_suffix}"),
        o: true,
        d: Some(nested_create_doc(model, many)),
        j: None,
    });
    class_fields.push(CSharpClassField {
        n: "ConnectOrCreate".to_owned(),
        t: format!("{enumerable_prefix}{model_name}CreateOrConnect{without_title}Input{enumerable_suffix}"),
        o: true,
        d: Some(nested_create_or_connect_doc(model, many)),
        j: None,
    });
    class_fields.push(CSharpClassField {
        n: "Connect".to_owned(),
        t: format!("{enumerable_prefix}{model_name}WhereUniqueInput{enumerable_suffix}"),
        o: true,
        d: Some(nested_connect_doc(model, many)),
        j: None,
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}CreateNested{many_title}{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
}

fn generate_model_create_or_connect_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let mut class_fields: Vec<CSharpClassField> = Vec::new();
    class_fields.push(CSharpClassField {
        n: "Where".to_owned(),
        t: format!("{model_name}WhereUniqueInput"),
        o: false,
        d: Some(unique_connect_doc(model)),
        j: None,
    });
    class_fields.push(CSharpClassField {
        n: "Create".to_owned(),
        t: format!("{model_name}Create{without_title}Input"),
        o: false,
        d: Some(unique_connect_create_doc(model)),
        j: None,
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}CreateOrConnect{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
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
    let mut class_fields = Vec::<CSharpClassField>::new();
    model.input_keys().iter().for_each(|k| {
        if let Some(field) = model.field(k) {
            let field_name = &field.name;
            let field_cs_type = field.field_type.to_csharp_type(field.optionality == Optionality::Optional);
            let ignore_this_field = if let Some(without_relation) = without_relation {
                without_relation.fields.contains(k)
            } else {
                false
            };
            if !ignore_this_field {
                class_fields.push(CSharpClassField {
                    n: field_name.to_pascal_case(),
                    t: field_cs_type,
                    o: field.optionality == Optionality::Optional,
                    d: Some(field_doc(field)),
                    j: None
                });
            }
        } else if let Some(relation) = model.relation(k) {
            let relation_name = &relation.name;
            let relation_model_name = &relation.model;
            let relation_model = graph.model(relation_model_name).unwrap();
            let num = if relation.is_vec { "Many" } else { "One" };
            let ignore_this_field = if let Some(without_relation) = without_relation {
                &without_relation.name == k
            } else {
                false
            };
            if !ignore_this_field {
                let without = if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                    r.fields == relation.references && r.references == relation.fields
                }) {
                    let opposite_relation_name = opposite_relation.name.to_pascal_case();
                    format!("Without{opposite_relation_name}")
                } else {
                    "".to_owned()
                };
                class_fields.push(CSharpClassField {
                    n: relation_name.to_pascal_case(),
                    t: format!("{relation_model_name}CreateNested{num}{without}Input"),
                    o: true,
                    d: Some(relation_doc(relation)),
                    j: None
                });
            }
        }
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}Create{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
}

fn generate_model_upsert_with_where_unique_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let mut class_fields = Vec::<CSharpClassField>::new();
    class_fields.push(CSharpClassField {
        n: "Where".to_string(),
        t: format!("{model_name}WhereUniqueInput"),
        o: false,
        d: Some(unique_where_doc(model)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Update".to_string(),
        t: format!("{model_name}Update{without_title}Input"),
        o: false,
        d: Some(create_or_update_doc(model, ActionType::Update)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Create".to_string(),
        t: format!("{model_name}Create{without_title}Input"),
        o: false,
        d: Some(create_or_update_doc(model, ActionType::Create)),
        j: None
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}UpsertWithWhereUnique{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
}

fn generate_model_update_with_where_unique_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let mut class_fields = Vec::<CSharpClassField>::new();
    class_fields.push(CSharpClassField {
        n: "Where".to_string(),
        t: format!("{model_name}WhereUniqueInput"),
        o: false,
        d: Some(unique_where_doc(model)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Update".to_string(),
        t: format!("{model_name}Update{without_title}Input"),
        o: false,
        d: Some(create_or_update_doc(model, ActionType::Update)),
        j: None
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}UpdateWithWhereUnique{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
}

fn generate_model_update_many_with_where_input(model: &Model, without: Option<&str>) -> String {
    let model_name = model.name();
    let without_title = if let Some(title) = without {
        let title = title.to_pascal_case();
        format!("Without{title}")
    } else {
        "".to_owned()
    };
    let builder = CSharpClassBuilder {
        name: format!("{model_name}UpdateManyWithWhere{without_title}Input"),
        fields: vec![
            CSharpClassField {
                n: "Where".to_string(),
                t: format!("{model_name}WhereInput"),
                o: false,
                d: Some(where_doc(model)),
                j: None
            },
            CSharpClassField {
                n: "Update".to_string(),
                t: format!("{model_name}Update{without_title}Input"),
                o: false,
                d: Some(create_or_update_doc(model, ActionType::UpdateMany)),
                j: None
            }
        ],
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
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
    let mut class_fields: Vec<CSharpClassField> = Vec::new();
    let enumerable_prefix = if many { "Enumerable<" } else { "" };
    let enumerable_suffix = if many { ">" } else { "" };
    class_fields.push(CSharpClassField {
        n: "Create".to_string(),
        t: format!("{enumerable_prefix}{model_name}Create{without_title}Input{enumerable_suffix}"),
        o: true,
        d: Some(nested_create_doc(model, many)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "ConnectOrCreate".to_string(),
        t: format!("{enumerable_prefix}{model_name}CreateOrConnect{without_title}Input{enumerable_suffix}"),
        o: true,
        d: Some(nested_create_or_connect_doc(model, many)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Connect".to_string(),
        t: format!("{enumerable_prefix}{model_name}WhereUniqueInput{enumerable_suffix}"),
        o: true,
        d: Some(nested_connect_doc(model, many)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Set".to_string(),
        t: format!("{enumerable_prefix}{model_name}WhereUniqueInput{enumerable_suffix}"),
        o: true,
        d: Some(nested_set_doc(model, many)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Update".to_string(),
        t: format!("{enumerable_prefix}{model_name}UpdateWithWhereUnique{without_title}Input{enumerable_suffix}"),
        o: true,
        d: Some(nested_update_doc(model, many)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Upsert".to_string(),
        t: format!("{enumerable_prefix}{model_name}UpsertWithWhereUnique{without_title}Input{enumerable_suffix}"),
        o: true,
        d: Some(nested_upsert_doc(model, many)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Disconnect".to_string(),
        t: format!("{enumerable_prefix}{model_name}WhereUniqueInput{enumerable_suffix}"),
        o: true,
        d: Some(nested_disconnect_doc(model, many)),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Delete".to_string(),
        t: format!("{enumerable_prefix}{model_name}WhereUniqueInput{enumerable_suffix}"),
        o: true,
        d: Some(nested_delete_doc(model, many)),
        j: None
    });
    if many {
        class_fields.push(CSharpClassField {
            n: "UpdateMany".to_string(),
            t: format!("{enumerable_prefix}{model_name}UpdateManyWithWhere{without_title}Input{enumerable_suffix}"),
            o: true,
            d: Some(nested_update_doc(model, many)),
            j: None
        });
        class_fields.push(CSharpClassField {
            n: "DeleteMany".to_string(),
            t: format!("{enumerable_prefix}{model_name}WhereInput{enumerable_suffix}"),
            o: true,
            d: Some(nested_delete_doc(model, many)),
            j: None
        });
    }
    let builder = CSharpClassBuilder {
        name: format!("{model_name}UpdateNested{many_title}{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
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
    let mut class_fields = Vec::<CSharpClassField>::new();
    model.input_keys().iter().for_each(|k| {
        if let Some(field) = model.field(k) {
            let field_name = &field.name;
            let field_cs_type = field.field_type.to_csharp_update_input_type(field.optionality == Optionality::Optional);
            let ignore_this_field = if let Some(without_relation) = without_relation {
                without_relation.fields.contains(k)
            } else {
                false
            };
            if !ignore_this_field {
                class_fields.push(CSharpClassField {
                    n: field_name.to_pascal_case(),
                    t: field_cs_type,
                    o: true,
                    d: Some(field_doc(field)),
                    j: None
                });
            }
        } else if let Some(relation) = model.relation(k) {
            let relation_name = &relation.name;
            let relation_model_name = &relation.model;
            let relation_model = graph.model(relation_model_name).unwrap();
            let num = if relation.is_vec { "Many" } else { "One" };
            let ignore_this_field = if let Some(without_relation) = without_relation {
                &without_relation.name == k
            } else {
                false
            };
            if !ignore_this_field {
                let without = if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                    r.fields == relation.references && r.references == relation.fields
                }) {
                    let opposite_relation_name = opposite_relation.name.to_pascal_case();
                    format!("Without{opposite_relation_name}")
                } else {
                    "".to_owned()
                };
                class_fields.push(CSharpClassField {
                    n: relation_name.to_pascal_case(),
                    t: format!("{relation_model_name}UpdateNested{num}{without}Input"),
                    o: true,
                    d: Some(relation_doc(relation)),
                    j: None
                });
            }
        }
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}Update{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
}

fn generate_model_credentials_input(model: &Model) -> String {
    let model_name = model.name();
    let class_name = format!("{model_name}CredentialsInput");
    let mut class_fields = Vec::<CSharpClassField>::new();
    let auth_identity_keys = model.auth_identity_keys();
    let auth_by_keys = model.auth_by_keys();
    let auth_identity_optional = auth_identity_keys.len() != 1;
    let auth_by_keys_optional = auth_by_keys.len() != 1;
    for key in auth_identity_keys {
        let field = model.field(key).unwrap();
        let field_name = &field.name;
        let field_type = field.field_type.to_csharp_type(auth_identity_optional);
        class_fields.push(CSharpClassField {
            n: field_name.to_pascal_case(),
            t: field_type,
            o: auth_identity_optional,
            d: Some(field_doc(field)),
            j: None
        });
    }
    for key in auth_by_keys {
        let field = model.field(key).unwrap();
        let field_name = &field.name;
        let field_type = field.field_type.to_csharp_type(auth_by_keys_optional);
        class_fields.push(CSharpClassField {
            n: field_name.to_pascal_case(),
            t: field_type,
            o: auth_by_keys_optional,
            d: Some(field_doc(field)),
            j: None
        });
    }
    let builder = CSharpClassBuilder {
        name: format!("{model_name}CredentialsInput"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 1
    };
    builder.build()
}

pub(crate) async fn generate_index_cs(graph: &Graph, conf: &ClientConfiguration) -> String {
    Code::new(0, 4, |c| {
        c.line("using System;");
        c.empty_line();
        c.line("#nullable enable");
        c.block("namespace Teo {", |c| {
            // enum definitions
            graph.enums().iter().for_each(|e| {
                let name = e.0;
                let choices = &e.1.values;
                c.block(format!("public enum {name}"), |b| {
                    for (index, choice) in choices.iter().enumerate() {
                        let pascalized = choice.to_pascal_case();
                        let val = index + 1;
                        b.line(format!("{pascalized} = {val},"));
                    }
                }, "}");
                c.empty_line();
            });
            // model definitions
            graph.models().iter().for_each(|m| {
                let mut model_fields = Vec::<CSharpClassField>::new();
                m.output_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = field.name.to_pascal_case();
                        let field_type = field.field_type.to_csharp_type(false);
                        model_fields.push(CSharpClassField {
                            n: field_name,
                            t: field_type,
                            o: true,
                            d: Some(field_doc(field)),
                            j: None
                        });
                    } else if let Some(relation) = m.relation(k) {
                        let relation_name = relation.name.to_pascal_case();
                        let relation_type = &relation.model;
                        let array = if relation.is_vec { "[]" } else { "" };
                        model_fields.push(CSharpClassField {
                            n: relation_name,
                            t: format!("{relation_type}{array}"),
                            o: true,
                            d: Some(relation_doc(relation)),
                            j: None
                        });
                    }
                });
                let builder = CSharpClassBuilder {
                    name: m.name().to_owned(),
                    fields: model_fields,
                    indent_spaces: 4,
                    indent_level: 1
                };
                c.indented(builder.build());
                c.empty_line();
            });
            // model input arguments
            graph.models().iter().for_each(|m| {
                let model_name = m.name();
                // select
                let mut select_fields = Vec::<CSharpClassField>::new();
                m.output_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        select_fields.push(CSharpClassField {
                            n: field_name.to_pascal_case(),
                            t: "bool".to_string(),
                            o: true,
                            d: Some(field_doc(field)),
                            j: None
                        });
                    }
                });
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}Select"),
                    fields: select_fields,
                    indent_spaces: 4,
                    indent_level: 1
                };
                c.indented(builder.build());
                // include
                let mut include_fields = Vec::<CSharpClassField>::new();
                for relation in m.relations() {
                    let name = &relation.name;
                    let is_vec = relation.is_vec;
                    let find_many = if is_vec { "FindMany" } else { "" };
                    let r_model = &relation.model;
                    include_fields.push(CSharpClassField {
                        n: name.to_pascal_case(),
                        t: format!("OneOf<bool, {r_model}{find_many}Args>"),
                        o: true,
                        d: Some(relation_doc(relation)),
                        j: None
                    });
                }
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}Include"),
                    fields: include_fields,
                    indent_spaces: 4,
                    indent_level: 1
                };
                c.indented(builder.build());
                // where
                let mut where_fields = vec![
                    CSharpClassField {
                        n: "AND".to_owned(),
                        t: format!("Enumerable<{model_name}WhereInput>"),
                        o: true,
                        d: None,
                        j: Some("AND".to_owned())
                    },
                    CSharpClassField {
                        n: "OR".to_owned(),
                        t: format!("Enumerable<{model_name}WhereInput>"),
                        o: true,
                        d: None,
                        j: Some("OR".to_owned())
                    },
                    CSharpClassField {
                        n: "NOT".to_owned(),
                        t: format!("Enumerable<{model_name}WhereInput>"),
                        o: true,
                        d: None,
                        j: Some("NOT".to_owned())
                    },
                ];
                m.query_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        let field_filter = field.field_type.to_csharp_filter_type(field.optionality == Optionality::Optional);
                        where_fields.push(CSharpClassField {
                            n: field_name.to_pascal_case(),
                            t: field_filter,
                            o: true,
                            d: Some(field_doc(field)),
                            j: None,
                        });
                    } else if let Some(relation) = m.relation(k) {
                        let list = if relation.is_vec { "List" } else { "" };
                        let relation_name = &relation.name;
                        let relation_model = &relation.model;
                        where_fields.push(CSharpClassField {
                            n: relation_name.to_pascal_case(),
                            t: format!("{relation_model}{list}RelationFilter"),
                            o: true,
                            d: Some(relation_doc(relation)),
                            j: None,
                        });
                    }
                });
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}WhereInput"),
                    fields: where_fields,
                    indent_spaces: 4,
                    indent_level: 1
                };
                c.indented(builder.build());

            });
        }, "}");
    }).to_string()
}