use inflector::Inflector;
use crate::core::action::{Action, CREATE_HANDLER, FIND_FIRST_HANDLER, ResData, ResMeta, UPDATE_HANDLER, UPDATE_MANY_HANDLER, UPSERT_HANDLER};
use crate::core::app::conf::ClientGeneratorConf;
use crate::core::field::r#type::FieldTypeOwner;
use crate::generator::client::csharp::pkg::index::doc::{action_doc, action_group_doc, create_or_update_doc, credentials_doc, cursor_doc, field_doc, include_doc, nested_connect_doc, nested_create_doc, nested_create_or_connect_doc, nested_delete_doc, nested_disconnect_doc, nested_set_doc, nested_update_doc, nested_upsert_doc, order_by_doc, page_number_doc, page_size_doc, relation_doc, select_doc, skip_doc, take_doc, unique_connect_create_doc, unique_connect_doc, unique_where_doc, where_doc, where_doc_first};
use crate::generator::client::csharp::r#type::ToCSharpType;

use crate::core::graph::Graph;
use crate::core::model::{Model};
use crate::generator::lib::code::Code;


mod doc;

fn get_set() -> &'static str {
    "{ get; set; }"
}

static ESCAPE_LIST: [&str; 2] = ["is", "where"];

fn escape(before: impl AsRef<str>) -> String {
    let before = before.as_ref();
    if ESCAPE_LIST.contains(&before) {
        format!("@{before}")
    } else {
        before.to_string()
    }
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
        let total = self.fields.len();
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
                    let get_set = get_set();
                    let field_name = &f.n;
                    let question_mark = if f.o { "?" } else { "" };
                    let field_type = &f.t;
                    if let Some(json_key) = &f.j {
                        b.line(format!(r#"[JsonPropertyName("{json_key}")]"#));
                    }
                    b.line(format!("public {field_type}{question_mark} {field_name} {get_set}"))
                }
                b.empty_line();
                b.block(format!("public {class_name}("), |b| {
                    let mut used = 0;
                    for f in &required_fields {
                        let camelized = escape(f.n.to_camel_case());
                        let field_type = &f.t;
                        let has_comma = used != total - 1;
                        let comma = if has_comma { "," } else { "" };
                        b.line(format!("{field_type} {camelized}{comma}"));
                        used += 1;
                    }
                    for f in &optional_fields {
                        let camelized = escape(f.n.to_camel_case());
                        let field_type = &f.t;
                        let has_comma = used != total - 1;
                        let comma = if has_comma { "," } else { "" };
                        b.line(format!("{field_type}? {camelized} = null{comma}"));
                        used += 1;
                    }
                }, ") {");
                b.block("", |b| {
                    for f in &self.fields {
                        let name = &f.n;
                        let camelized = escape(name.to_camel_case());
                        b.line(format!("{name} = {camelized};"));
                    }
                }, "}");
            }, "}");
        }).to_string()
    }
}

fn generate_model_create_nested_input(_graph: &Graph, model: &Model, without: Option<&str>, many: bool) -> String {
    let _get_set = get_set();
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
        t: format!("{enumerable_prefix}{model_name}ConnectOrCreate{without_title}Input{enumerable_suffix}"),
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
        indent_level: 0
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
        name: format!("{model_name}ConnectOrCreate{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 0
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
            let field_cs_type = field.field_type().to_csharp_type(false);
            let ignore_this_field = if let Some(without_relation) = without_relation {
                without_relation.fields().contains(k)
            } else {
                false
            };
            if !ignore_this_field {
                class_fields.push(CSharpClassField {
                    n: field_name.to_pascal_case(),
                    t: field_cs_type,
                    o: field.optionality.is_optional(),
                    d: Some(field_doc(field)),
                    j: None
                });
            }
        } else if let Some(relation) = model.relation(k) {
            let relation_name = relation.name();
            let relation_model_name = relation.model();
            let relation_model = graph.model(relation_model_name).unwrap();
            let num = if relation.is_vec() { "Many" } else { "One" };
            let ignore_this_field = if let Some(without_relation) = without_relation {
                without_relation.name() == k
            } else {
                false
            };
            if !ignore_this_field {
                let without = if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                    r.fields() == relation.references() && r.references() == relation.fields()
                }) {
                    let opposite_relation_name = opposite_relation.name().to_pascal_case();
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
        indent_level: 0
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
        d: Some(create_or_update_doc(model, Action::from_u32(UPDATE_HANDLER))),
        j: None
    });
    class_fields.push(CSharpClassField {
        n: "Create".to_string(),
        t: format!("{model_name}Create{without_title}Input"),
        o: false,
        d: Some(create_or_update_doc(model, Action::from_u32(CREATE_HANDLER))),
        j: None
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}UpsertWithWhereUnique{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 0
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
        d: Some(create_or_update_doc(model, Action::from_u32(UPDATE_HANDLER))),
        j: None
    });
    let builder = CSharpClassBuilder {
        name: format!("{model_name}UpdateWithWhereUnique{without_title}Input"),
        fields: class_fields,
        indent_spaces: 4,
        indent_level: 0
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
                d: Some(create_or_update_doc(model, Action::from_u32(UPDATE_MANY_HANDLER))),
                j: None
            }
        ],
        indent_spaces: 4,
        indent_level: 0
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
        t: format!("{enumerable_prefix}{model_name}ConnectOrCreate{without_title}Input{enumerable_suffix}"),
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
        indent_level: 0
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
            let field_cs_type = field.field_type().to_csharp_update_input_type(field.optionality.is_optional(), true);
            let ignore_this_field = if let Some(without_relation) = without_relation {
                without_relation.fields().contains(k)
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
            let relation_name = relation.name();
            let relation_model_name = relation.model();
            let relation_model = graph.model(relation_model_name).unwrap();
            let num = if relation.is_vec() { "Many" } else { "One" };
            let ignore_this_field = if let Some(without_relation) = without_relation {
                &without_relation.name() == k
            } else {
                false
            };
            if !ignore_this_field {
                let without = if let Some(opposite_relation) = relation_model.relations().iter().find(|r| {
                    r.fields() == relation.references() && r.references() == relation.fields()
                }) {
                    let opposite_relation_name = opposite_relation.name().to_pascal_case();
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
        indent_level: 0
    };
    builder.build()
}

fn generate_model_credentials_input(model: &Model) -> String {
    let model_name = model.name();
    let _class_name = format!("{model_name}CredentialsInput");
    let mut class_fields = Vec::<CSharpClassField>::new();
    let auth_identity_keys = model.auth_identity_keys();
    let auth_by_keys = model.auth_by_keys();
    let auth_identity_optional = auth_identity_keys.len() != 1;
    let auth_by_keys_optional = auth_by_keys.len() != 1;
    for key in auth_identity_keys {
        let field = model.field(key).unwrap();
        let field_name = &field.name;
        let field_type = field.field_type().to_csharp_type(auth_identity_optional);
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
        let field_type = field.field_type().to_csharp_type(auth_by_keys_optional);
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
        indent_level: 0
    };
    builder.build()
}

pub(crate) async fn generate_index_cs(graph: &Graph, _client: &ClientGeneratorConf) -> String {
    Code::new(0, 4, |c| {
        c.line("using System;");
        c.line("using System.Text.Json.Serialization;");
        c.line("using System.Threading.Tasks;");
        c.empty_line();
        c.line("#nullable enable");
        c.block("namespace Teo {", |c| {
            c.empty_line();
            // enum definitions
            graph.enums().iter().for_each(|e| {
                let name = e.0;
                let choices = &e.1.values();
                c.block(format!("public enum {name} {{"), |b| {
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
                        let field_type = field.field_type().to_csharp_type(false);
                        model_fields.push(CSharpClassField {
                            n: field_name,
                            t: field_type,
                            o: true,
                            d: Some(field_doc(field)),
                            j: None
                        });
                    } else if let Some(relation) = m.relation(k) {
                        let relation_name = relation.name().to_pascal_case();
                        let relation_type = relation.model();
                        let array = if relation.is_vec() { "[]" } else { "" };
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
                    indent_level: 0
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
                    indent_level: 0
                };
                c.indented(builder.build());
                // include
                let mut include_fields = Vec::<CSharpClassField>::new();
                for relation in m.relations() {
                    let name = relation.name();
                    let is_vec = relation.is_vec();
                    let find_many = if is_vec { "FindMany" } else { "" };
                    let r_model = relation.model();
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
                    indent_level: 0
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
                        let field_filter = field.field_type().to_csharp_filter_type(field.optionality.is_optional());
                        where_fields.push(CSharpClassField {
                            n: field_name.to_pascal_case(),
                            t: field_filter,
                            o: true,
                            d: Some(field_doc(field)),
                            j: None,
                        });
                    } else if let Some(relation) = m.relation(k) {
                        let list = if relation.is_vec() { "List" } else { "" };
                        let relation_name = relation.name();
                        let relation_model = relation.model();
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
                    indent_level: 0
                };
                c.indented(builder.build());
                // where unique
                let mut where_unique_fields = Vec::<CSharpClassField>::new();
                let mut used_where_unique_field_names: Vec<&str> = Vec::new();
                m.indices().iter().for_each(|index| {
                    if index.r#type().is_unique() {
                        index.items().iter().for_each(|item| {
                            if !used_where_unique_field_names.contains(&&***&&item.field_name()) {
                                if let Some(field) = m.field(&item.field_name()) {
                                    let cs_type = field.field_type().to_csharp_type(false);
                                    let field_name = &item.field_name();
                                    where_unique_fields.push(CSharpClassField {
                                        n: field_name.to_pascal_case(),
                                        t: cs_type,
                                        o: true,
                                        d: Some(field_doc(field)),
                                        j: None
                                    });
                                }
                                used_where_unique_field_names.push(item.field_name());
                            }
                        });
                    }
                });
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}WhereUniqueInput"),
                    fields: where_unique_fields,
                    indent_spaces: 4,
                    indent_level: 0
                };
                c.indented(builder.build());
                // relation filter
                let relation_filter_fields = vec![
                    CSharpClassField {
                        n: "Is".to_string(),
                        t: format!("{model_name}WhereInput"),
                        o: true,
                        d: None,
                        j: None
                    },
                    CSharpClassField {
                        n: "IsNot".to_string(),
                        t: format!("{model_name}WhereInput"),
                        o: true,
                        d: None,
                        j: None
                    }
                ];
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}RelationFilter"),
                    fields: relation_filter_fields,
                    indent_spaces: 4,
                    indent_level: 0
                };
                c.indented(builder.build());
                // list relation filter
                let list_relation_filter_fields = vec![
                    CSharpClassField {
                        n: "Every".to_string(),
                        t: format!("{model_name}WhereInput"),
                        o: true,
                        d: None,
                        j: None
                    },
                    CSharpClassField {
                        n: "Some".to_string(),
                        t: format!("{model_name}WhereInput"),
                        o: true,
                        d: None,
                        j: None
                    },
                    CSharpClassField {
                        n: "None".to_string(),
                        t: format!("{model_name}WhereInput"),
                        o: true,
                        d: None,
                        j: None
                    },
                ];
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}ListRelationFilter"),
                    fields: list_relation_filter_fields,
                    indent_spaces: 4,
                    indent_level: 0
                };
                c.indented(builder.build());
                // order by
                let mut order_by_fields = Vec::<CSharpClassField>::new();
                m.query_keys().iter().for_each(|k| {
                    if let Some(field) = m.field(k) {
                        let field_name = &field.name;
                        order_by_fields.push(CSharpClassField {
                            n: field_name.to_pascal_case(),
                            t: "SortOrder".to_owned(),
                            o: true,
                            d: Some(field_doc(field)),
                            j: None
                        });
                    }
                });
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}OrderByInput"),
                    fields: order_by_fields,
                    indent_spaces: 4,
                    indent_level: 0
                };
                c.indented(builder.build());
                // create and update inputs without anything
                c.indented(generate_model_create_input(graph, m, None));
                c.indented(generate_model_create_nested_input(graph, m, None, true));
                c.indented(generate_model_create_nested_input(graph, m, None, false));
                c.indented(generate_model_create_or_connect_input(m, None));
                m.relations().iter().for_each(|r| {
                    c.indented(generate_model_create_input(graph, m, Some(r.name())));
                    c.indented(generate_model_create_nested_input(graph, m, Some(r.name()), true));
                    c.indented(generate_model_create_nested_input(graph, m, Some(r.name()), false));
                    c.indented(generate_model_create_or_connect_input(m, Some(r.name())));
                });
                c.indented(generate_model_update_input(graph, m, None));
                c.indented(generate_model_update_nested_input(graph, m, None, true));
                c.indented(generate_model_update_nested_input(graph, m, None, false));
                c.indented(generate_model_upsert_with_where_unique_input(m, None));
                c.indented(generate_model_update_with_where_unique_input(m, None));
                c.indented(generate_model_update_many_with_where_input(m, None));
                m.relations().iter().for_each(|r| {
                    c.indented(generate_model_update_input(graph, m, Some(r.name())));
                    c.indented(generate_model_update_nested_input(graph, m, Some(r.name()), true));
                    c.indented(generate_model_update_nested_input(graph, m, Some(r.name()), false));
                    c.indented(generate_model_upsert_with_where_unique_input(m, Some(r.name())));
                    c.indented(generate_model_update_with_where_unique_input(m, Some(r.name())));
                    c.indented(generate_model_update_many_with_where_input(m, Some(r.name())));
                });
                if m.identity() {
                    c.indented(generate_model_credentials_input(m));
                }
                // action args
                let builder = CSharpClassBuilder {
                    name: format!("{model_name}Args"),
                    fields: vec![
                        CSharpClassField {
                            n: "Select".to_owned(),
                            t: format!("{model_name}Select"),
                            o: true,
                            d: Some(select_doc(m)),
                            j: None
                        },
                        CSharpClassField {
                            n: "Include".to_owned(),
                            t: format!("{model_name}Include"),
                            o: true,
                            d: Some(include_doc(m)),
                            j: None
                        }
                    ],
                    indent_spaces: 4,
                    indent_level: 0
                };
                c.indented(builder.build());
                Action::handlers_iter().for_each(|a| {
                    if !m.has_action(*a) { return }
                    let action_name = a.as_handler_str();
                    let mut fields = Vec::<CSharpClassField>::new();
                    if a.handler_requires_where() {
                        fields.push(CSharpClassField {
                            n: "Where".to_owned(),
                            t: format!("{model_name}WhereInput"),
                            o: true,
                            d: Some(if *a == Action::from_u32(FIND_FIRST_HANDLER) { where_doc_first(m) } else { where_doc(m) }),
                            j: None,
                        });
                    }
                    if a.handler_requires_where_unique() {
                        fields.push(CSharpClassField {
                            n: "Where".to_owned(),
                            t: format!("{model_name}WhereUniqueInput"),
                            o: true,
                            d: Some(unique_where_doc(m)),
                            j: None,
                        });
                    }
                    fields.push(CSharpClassField {
                        n: "Select".to_owned(),
                        t: format!("{model_name}Select"),
                        o: true,
                        d: Some(select_doc(m)),
                        j: None,
                    });
                    fields.push(CSharpClassField {
                        n: "Include".to_owned(),
                        t: format!("{model_name}Include"),
                        o: true,
                        d: Some(include_doc(m)),
                        j: None,
                    });
                    if a.handler_requires_where() {
                        fields.push(CSharpClassField {
                            n: "OrderBy".to_owned(),
                            t: format!("Enumerable<{model_name}OrderByInput>"),
                            o: true,
                            d: Some(order_by_doc(m)),
                            j: None,
                        });
                        fields.push(CSharpClassField {
                            n: "Cursor".to_owned(),
                            t: format!("{model_name}WhereUniqueInput"),
                            o: true,
                            d: Some(cursor_doc(m)),
                            j: None,
                        });
                        fields.push(CSharpClassField {
                            n: "Take".to_owned(),
                            t: "uint".to_owned(),
                            o: true,
                            d: Some(take_doc(m)),
                            j: None,
                        });
                        fields.push(CSharpClassField {
                            n: "Skip".to_owned(),
                            t: "uint".to_owned(),
                            o: true,
                            d: Some(skip_doc(m)),
                            j: None,
                        });
                        fields.push(CSharpClassField {
                            n: "PageSize".to_owned(),
                            t: "uint".to_owned(),
                            o: true,
                            d: Some(page_size_doc(m)),
                            j: None,
                        });
                        fields.push(CSharpClassField {
                            n: "PageNumber".to_owned(),
                            t: "uint".to_owned(),
                            o: true,
                            d: Some(page_number_doc(m)),
                            j: None,
                        });
                    }
                    if a.handler_requires_create() {
                        fields.push(CSharpClassField {
                            n: "Create".to_owned(),
                            t: format!("{model_name}CreateInput"),
                            o: true,
                            d: Some(create_or_update_doc(m, if a == &Action::from_u32(UPSERT_HANDLER) { Action::from_u32(CREATE_HANDLER) } else { a.clone() })),
                            j: None,
                        });
                    }
                    if a.handler_requires_update() {
                        fields.push(CSharpClassField {
                            n: "Update".to_owned(),
                            t: format!("{model_name}UpdateInput"),
                            o: true,
                            d: Some(create_or_update_doc(m, if a == &Action::from_u32(UPSERT_HANDLER) { Action::from_u32(UPDATE_HANDLER) } else { a.clone() })),
                            j: None,
                        });
                    }
                    if a.handler_requires_credentials() {
                        fields.push(CSharpClassField {
                            n: "Credentials".to_owned(),
                            t: format!("{model_name}CredentialsInput"),
                            o: true,
                            d: Some(credentials_doc(m, *a)),
                            j: None,
                        });
                    }
                    let builder = CSharpClassBuilder {
                        name: format!("{model_name}{action_name}Args"),
                        fields,
                        indent_spaces: 4,
                        indent_level: 0
                    };
                    c.indented(builder.build());
                });
            });
            // delegates
            let object_name = "teo";
            c.empty_line();
            graph.models().iter().for_each(|m| {
                if m.actions().len() > 0 {
                    let model_name = m.name();
                    let model_var_name = model_name.to_camel_case();
                    let model_class_name = model_var_name.to_pascal_case();
                    let model_url_segment_name = m.name();
                    c.block(format!("public class {model_class_name}Delegate : Delegate {{"), |b| {
                        b.empty_line();
                        b.line("readonly string? _Token;");
                        b.empty_line();
                        b.block(format!("protected internal {model_class_name}Delegate(string? token = null) {{"), |b| {
                            b.line("_Token = token;");
                        }, "}");
                        Action::handlers_iter().for_each(|a| {
                            if m.has_action(*a) {
                                let action_name = a.as_handler_str();
                                let action_var_name = a.as_handler_str().to_pascal_case();
                                let action_url_name = a.as_handler_str();
                                let res_meta = match a.handler_res_meta() {
                                    ResMeta::PagingInfo => "PagingInfo, ",
                                    ResMeta::TokenInfo => "TokenInfo, ",
                                    ResMeta::NoMeta => "",
                                    ResMeta::Other => "",
                                };
                                let res_data = match a.handler_res_data() {
                                    ResData::Single => model_name.to_string(),
                                    ResData::Vec => model_name.to_string() + "[]",
                                    ResData::Other => "short".to_string(),
                                    ResData::Number => "uint".to_string(),
                                };
                                b.empty_line();
                                b.doc(action_doc(object_name, a.clone(), m));
                                b.block(format!("public async Task<Response<{res_meta}{res_data}>> {action_var_name}({model_name}{action_name}Args? args = null, string? token = null) {{"), |b| {
                                    b.line(format!(r#"return await Request<Response<{res_meta}{res_data}>>("{model_url_segment_name}", "{action_url_name}", args ?? new(), token ?? _Token);"#));
                                }, "}");
                            }
                        });
                    }, "}");
                    c.empty_line();
                }
            });
            // main class
            c.block(format!("public class Teo {{"), |b| {
                b.empty_line();
                graph.models().iter().for_each(|m| {
                    if m.actions().len() > 0 {
                        let model_name = m.name();
                        let model_class_name = model_name.to_pascal_case();
                        b.doc(action_group_doc(object_name, m));
                        b.line(format!("public {model_class_name}Delegate {model_class_name} {{ get; }}"));
                    }
                });
                b.empty_line();
                b.block("public Teo(string? token = null) {", |b| {
                    graph.models().iter().for_each(|m| {
                        if m.actions().len() > 0 {
                            let model_name = m.name();
                            let model_class_name = model_name.to_pascal_case();
                            b.line(format!("{model_class_name} = new(token);"));
                        }
                    })
                }, "}");
            }, "}");
        }, "}");
    }).to_string()
}
