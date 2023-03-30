use std::borrow::Cow;
use inflector::Inflector;
use itertools::Itertools;
use crate::core::field::r#type::FieldTypeOwner;
use crate::gen::internal::client::outline::class::Class;
use crate::gen::internal::client::outline::class_kind::ClassKind;
use crate::gen::internal::client::outline::field::Field;
use crate::gen::internal::client::outline::field_kind::FieldKind;
use crate::gen::internal::type_lookup::TypeLookup;
use crate::prelude::Graph;

pub(in crate::gen) struct Outline<'a> {
    pub(in crate::gen) classes: Vec<Class<'a>>,
}

impl Outline<'_> {
    pub(in crate::gen) fn new<L>(graph: &Graph, lookup: L) -> Self where L: TypeLookup {
        Self {
            classes: {
                let mut results = graph.enums().iter().map(|(name, enum_def)| {
                    Class {
                        model_name: enum_def.name(),
                        localized_name: Cow::Borrowed(enum_def.localized_name()),
                        name_suffix: Cow::Borrowed(""),
                        docs: Cow::Borrowed(enum_def.description().unwrap_or("")),
                        kind: ClassKind::Enum,
                        fields: enum_def.variants.iter().map(|v| Field {
                            name: v.name(),
                            localized_name: Cow::Borrowed(v.localized_name()),
                            docs: Cow::Borrowed(v.description().unwrap_or("")),
                            field_type: Cow::Borrowed(""),
                            optional: false,
                            kind: FieldKind::EnumVariant,
                        }).collect(),
                    }
                }).collect::<Vec<Class>>();
                results.extend(graph.models().iter().map(|m| {
                    let mut classes = vec![
                        // data output
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Owned(m.localized_name()),
                            name_suffix: Cow::Borrowed(""),
                            docs: Cow::Borrowed(m.description()),
                            kind: ClassKind::DataOutput,
                            fields: {
                                let mut fields = vec![];
                                for key in m.output_keys() {
                                    if let Some(field) = m.field(key) {
                                        fields.push(Field {
                                            name: field.name(),
                                            field_type: lookup.field_type_to_result_type(field.field_type(), false),
                                            optional: field.is_optional(),
                                            localized_name: Cow::Owned(field.localized_name()),
                                            docs: field.description().map(|d| Cow::Borrowed(d)).unwrap_or(Cow::Borrowed("")),
                                            kind: FieldKind::Field,
                                        });
                                    } else if let Some(property) = m.property(key) {
                                        fields.push(Field {
                                            name: property.name(),
                                            field_type: lookup.field_type_to_result_type(property.field_type(), property.is_optional()),
                                            optional: property.is_optional(),
                                            localized_name: Cow::Owned(property.localized_name()),
                                            docs: property.description.as_ref().map(|s| Cow::Borrowed(s.as_str())).unwrap_or(Cow::Borrowed("")),
                                            kind: FieldKind::Property,
                                        })
                                    }
                                }
                                for relation in m.relations() {
                                    fields.push(Field {
                                        name: relation.name(),
                                        field_type: if relation.is_vec() {
                                            lookup.generated_type_to_vec(Cow::Borrowed(relation.model()))
                                        } else {
                                            Cow::Borrowed(relation.name())
                                        },
                                        optional: relation.is_optional(),
                                        localized_name: Cow::Owned(relation.localized_name()),
                                        docs: relation.description().map(|d| Cow::Borrowed(d)).unwrap_or(Cow::Borrowed("")),
                                        kind: FieldKind::Relation,
                                    })
                                }
                                fields
                            },
                        }),
                        // select input
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("Select"),
                            docs: Cow::Owned(format!("Select fields from the {} model.", m.name().to_word_case())),
                            fields: m.output_keys().iter().filter_map(|k| m.field(k)).map(|f| Field {
                                name: f.name(),
                                field_type: lookup.field_type_to_result_type(f.field_type(), false),
                                optional: f.is_optional(),
                                localized_name: Cow::Owned(f.localized_name()),
                                docs: f.description().map(|d| Cow::Borrowed(d)).unwrap_or(Cow::Borrowed("")),
                                kind: FieldKind::Field,
                            }).collect(),
                            kind: ClassKind::SelectInput,
                        }),
                        // include input
                        if m.relations().is_empty() {
                            None
                        } else {
                            Some(Class {
                                model_name: m.name(),
                                localized_name: Cow::Borrowed(""),
                                name_suffix: Cow::Borrowed("Include"),
                                docs: Cow::Owned(format!("Include relations of the {} model.", m.name().to_word_case())),
                                fields: m.relations().iter().map(|r| Field {
                                    name: r.name(),
                                    field_type: Cow::Owned(format!("{}{}Args", r.model(), if r.is_vec() { "FindMany" } else { "" })),
                                    optional: true,
                                    localized_name: Cow::Owned(r.localized_name()),
                                    docs: r.description().map(|d| Cow::Borrowed(d)).unwrap_or(Cow::Borrowed("")),
                                    kind: FieldKind::Relation,
                                }).collect(),
                                kind: ClassKind::IncludeInput,
                            })
                        },
                        // where input
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("WhereInput"),
                            docs: Cow::Owned(format!("{} filter.", m.name())),
                            fields: m.query_keys().iter().map(|k| if let Some(field) = m.field(k) {
                                Field {
                                    name: field.name(),
                                    field_type: lookup.field_type_to_filter_type(field.field_type(), field.is_optional()),
                                    optional: true,
                                    localized_name: Cow::Owned(field.localized_name()),
                                    docs: Cow::Borrowed(field.description().unwrap_or("")),
                                    kind: FieldKind::Field,
                                }
                            } else if let Some(relation) = m.relation(k) {
                                Field {
                                    name: relation.name(),
                                    field_type: if relation.is_vec() { Cow::Owned(relation.model().to_owned() + "ListRelationFilter") } else { Cow::Owned(relation.model().to_owned() + "RelationFilter") },
                                    optional: true,
                                    localized_name: Cow::Owned(relation.localized_name()),
                                    docs: Cow::Borrowed(relation.description().unwrap_or("")),
                                    kind: FieldKind::Relation,
                                }
                            } else { unreachable!() }).collect(),
                            kind: ClassKind::WhereInput,
                        }),
                        // where unique input
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("WhereUniqueInput"),
                            docs: Cow::Owned(format!("{} unique filter.", m.name())),
                            fields: m.indices().iter().filter(|i| i.r#type().is_unique()).map(|i| i.keys().iter().map(|k| m.field(k).unwrap()).map(|f| Field {
                                name: f.name(),
                                localized_name: Cow::Owned(f.localized_name()),
                                docs: Cow::Borrowed(f.description().unwrap_or("")),
                                field_type: lookup.field_type_to_create_type(f.field_type(), false),
                                optional: true,
                                kind: FieldKind::Field,
                            })).flatten().dedup_by(|f1, f2| f1.name == f2.name).collect(),
                            kind: ClassKind::WhereUniqueInput,
                        }),
                        // order by input
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("OrderByInput"),
                            docs: Cow::Owned(format!("{} order by input.", m.name())),
                            fields: m.sort_keys().iter().map(|k| {
                                let f = m.field(k).unwrap();
                                Field {
                                    name: f.name(),
                                    localized_name: Cow::Owned(f.localized_name()),
                                    docs: Cow::Borrowed(f.description().unwrap_or("")),
                                    field_type: Cow::Borrowed("SortOrder"),
                                    optional: true,
                                    kind: FieldKind::Field,
                                }
                            }).collect(),
                            kind: ClassKind::OrderByInput,
                        }),
                    ];
                    let without = {
                        let mut without = vec![""];
                        without.append(&mut m.relations().iter().map(|r| r.name()).collect());
                        without
                    };
                    // create input
                    classes.extend(without.iter().map(|w| vec![
                        // create input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("Create", w, "Input"),
                            docs: Cow::Owned(format!("{} create input.", m.name())),
                            kind: ClassKind::CreateInput,
                            fields: m.input_keys().iter().filter_map(|k| if let Some(field) = m.field(k) {
                                Some(Field {
                                    name: field.name(),
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(field.description().unwrap_or("")),
                                    field_type: lookup.field_type_to_create_type(field.field_type(), false),
                                    optional: field.input_omissible,
                                    kind: FieldKind::Field,
                                })
                            } else if let Some(property) = m.property(k) {
                                Some(Field {
                                    name: property.name(),
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(property.description.as_ref().map(|v| v.as_str()).unwrap_or("")),
                                    field_type: lookup.field_type_to_create_type(property.field_type(), false),
                                    optional: property.input_omissible,
                                    kind: FieldKind::Property,
                                })
                            } else if let Some(relation) = m.relation(k) {
                                if relation.name() == *w {
                                    None
                                } else {
                                    Some(Field {
                                        name: relation.name(),
                                        localized_name: Cow::Borrowed(""),
                                        docs: Cow::Borrowed(relation.description().unwrap_or("")),
                                        field_type: {
                                            if let Some(opposite) = graph.opposite_relation(relation).1 {
                                                helper::without_infix(relation.model(), "Create", opposite.name(), "Input")
                                            } else {
                                                Cow::Owned(format!("{}CreateInput", relation.model()))
                                            }
                                        },
                                        optional: relation.is_optional(),
                                        kind: FieldKind::Relation,
                                    })
                                }
                            } else { unreachable!() }).collect(),
                        },
                        // create nested many input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("CreateNestedMany", w, "Input"),
                            docs: Cow::Owned(format!("{} create nested many input.", m.name())),
                            kind: ClassKind::CreateNestedManyInput,
                            fields: vec![
                                Field {
                                    name: "create",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::create_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate((helper::without_infix(m.name(), "Create", w, "Input"))),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "connectOrCreate",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::connect_or_create_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate((helper::without_infix(m.name(), "ConnectOrCreate", w, "Input"))),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "connect",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::connect_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}WhereUniqueInput", m.name()))),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                            ]
                        },
                        // create nested one input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("CreateNestedOne", w, "Input"),
                            docs: Cow::Owned(format!("{} create nested one input.", m.name())),
                            kind: ClassKind::CreateNestedOneInput,
                            fields: vec![
                                Field {
                                    name: "create",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::create_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "Create", w, "Input"),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "connectOrCreate",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::connect_or_create_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "ConnectOrCreate", w, "Input"),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "connect",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::connect_doc(m.name()),
                                    field_type: Cow::Owned(format!("{}WhereUniqueInput", m.name())),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                            ],
                        },
                        // connect or create input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("ConnectOrCreate", w, "Input"),
                            docs: Cow::Owned(format!("{} connect or create input.", m.name())),
                            kind: ClassKind::ConnectOrCreateInput,
                            fields: vec![
                                Field {
                                    name: "where",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::where_unique_doc(m.name()),
                                    field_type: Cow::Owned(format!("{}WhereUniqueInput", m.name())),
                                    optional: false,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "create",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::create_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "Create", w, "Input"),
                                    optional: false,
                                    kind: FieldKind::Predefined,
                                },
                            ],
                        },
                    ]).flatten().collect());
                    // update input
                    classes.extend(without.iter().map(|w| vec![

                    ]).flatten().collect());
                    classes
                }).flatten().collect());
                results
            }
        }
    }
}

mod helper {
    use std::borrow::Cow;
    use inflector::Inflector;

    pub(super) fn without_infix<'a>(model_name: &'a str, before: &'a str, without: &'a str, after: &'a str) -> Cow<'a, str> {
        if without.is_empty() {
            Cow::Owned(model_name.to_owned() + before + after)
        } else {
            Cow::Owned(model_name.to_owned() + before + "Without" + without.to_pascal_case().as_str() + after)
        }
    }

    pub(super) fn without_infix_no_model_name<'a>(before: &'a str, without: &'a str, after: &'a str) -> Cow<'a, str> {
        if without.is_empty() {
            Cow::Owned(before.to_owned() + after)
        } else {
            Cow::Owned(before.to_owned() + "Without" + without.to_pascal_case().as_str() + after)
        }
    }

    pub(super) fn create_doc<'a>(model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn connect_doc<'a>(model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn connect_or_create_doc<'a>(model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn where_unique_doc<'a>(model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }
}
