use std::borrow::Cow;
use inflector::Inflector;
use itertools::Itertools;
use crate::core::action::{Action, IDENTITY_HANDLER, SIGN_IN_HANDLER};
use crate::core::field::r#type::FieldTypeOwner;
use crate::gen::internal::client::outline::class::Class;
use crate::gen::internal::client::outline::class_kind::ClassKind;
use crate::gen::internal::client::outline::delegate::{Delegate, DelegateAction};
use crate::gen::internal::client::outline::field::Field;
use crate::gen::internal::client::outline::field_kind::FieldKind;
use crate::gen::internal::type_lookup::TypeLookup;
use crate::prelude::Graph;

pub(in crate::gen) struct Outline<'a> {
    pub(in crate::gen) classes: Vec<Class<'a>>,
    pub(in crate::gen) delegates: Vec<Delegate<'a>>,
}

impl<'a> Outline<'a> {
    pub(in crate::gen) fn new<L>(graph: &'a Graph, lookup: L) -> Self where L: TypeLookup {
        Self {
            classes: {
                let mut results = graph.enums().iter().map(|(_name, enum_def)| {
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
                results.extend(graph.models_without_teo_internal().iter().map(|m| {
                    let mut classes: Vec<Class> = vec![
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
                                            field_type: lookup.field_type_to_result_type(field.field_type()),
                                            optional: field.is_optional(),
                                            localized_name: Cow::Owned(field.localized_name()),
                                            docs: field.description().map(|d| Cow::Borrowed(d)).unwrap_or(Cow::Borrowed("")),
                                            kind: FieldKind::Field,
                                        });
                                    } else if let Some(property) = m.property(key) {
                                        fields.push(Field {
                                            name: property.name(),
                                            field_type: lookup.field_type_to_result_type(property.field_type()),
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
                                            Cow::Borrowed(relation.model())
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
                                field_type: lookup.field_type_to_result_type(f.field_type()),
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
                            fields: {
                                let mut fields: Vec<Field> = helper::where_the_three_ops(lookup.generated_type_to_enumerate(Cow::Owned(format!("{}WhereInput", m.name()))));
                                fields.extend(m.query_keys().iter().map(|k| if let Some(field) = m.field(k) {
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
                                } else { unreachable!() }).collect::<Vec<Field>>());
                                fields
                            },
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
                        // scalar where with aggregates input
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("ScalarWhereWithAggregatesInput"),
                            docs: Cow::Owned(format!("{} scalar where with aggregates input.", m.name())),
                            kind: ClassKind::ScalarWhereWithAggregatesInput,
                            fields: {
                                let mut fields: Vec<Field> = helper::where_the_three_ops(lookup.generated_type_to_enumerate(Cow::Owned(format!("{}ScalarWhereWithAggregatesInput", m.name()))));
                                fields.extend(m.query_keys().iter().filter_map(|k| if let Some(field) = m.field(k) {
                                    if field.field_type().is_scalar() {
                                        Some(Field {
                                            name: field.name(),
                                            field_type: lookup.field_type_to_filter_with_aggregates_type(field.field_type(), field.is_optional()),
                                            optional: true,
                                            localized_name: Cow::Owned(field.localized_name()),
                                            docs: Cow::Borrowed(field.description().unwrap_or("")),
                                            kind: FieldKind::Field,
                                        })
                                    } else {
                                        None
                                    }
                                } else { None }));
                                fields
                            },
                        }),
                        // relation filter
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("RelationFilter"),
                            docs: Cow::Owned(format!("{} relation filter.", m.name())),
                            kind: ClassKind::RelationFilter,
                            fields: vec![
                                Field {
                                    name: "is",
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(""),
                                    field_type: Cow::Owned(format!("{}WhereInput", m.name())),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "isNot",
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(""),
                                    field_type: Cow::Owned(format!("{}WhereInput", m.name())),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                            ],
                        }),
                        // list relation filter
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("ListRelationFilter"),
                            docs: Cow::Owned(format!("{} list relation filter.", m.name())),
                            kind: ClassKind::ListRelationFilter,
                            fields: vec![
                                Field {
                                    name: "every",
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(""),
                                    field_type: Cow::Owned(format!("{}WhereInput", m.name())),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "some",
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(""),
                                    field_type: Cow::Owned(format!("{}WhereInput", m.name())),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "none",
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(""),
                                    field_type: Cow::Owned(format!("{}WhereInput", m.name())),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                            ],
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
                        // scalar field enum
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("ScalarFieldEnum"),
                            docs: Cow::Owned(format!("{} scalar field enum.", m.name())),
                            kind: ClassKind::ScalarFieldEnum,
                            fields: m.fields().iter().filter_map(|f| if f.field_type().is_scalar() {
                                Some(Field {
                                    name: f.name(),
                                    localized_name: Cow::Owned(f.localized_name()),
                                    docs: Cow::Borrowed(f.description().unwrap_or("")),
                                    field_type: Cow::Borrowed(""),
                                    optional: false,
                                    kind: FieldKind::Field,
                                })
                            } else {
                                None
                            }).collect(),
                        }),
                        // aggregate: count
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("CountAggregateInputType"),
                            docs: Cow::Owned(format!("{} count aggregate input type.", m.name())),
                            kind: ClassKind::AggregateInputType,
                            fields: {
                                let mut fields: Vec<Field> = m.fields().iter().map(|f| Field {
                                    name: f.name(),
                                    localized_name: Cow::Owned(f.localized_name()),
                                    docs: Cow::Borrowed(f.description().unwrap_or("")),
                                    field_type: Cow::Borrowed(lookup.bool_type()),
                                    optional: true,
                                    kind: FieldKind::Field,
                                }).collect();
                                fields.push(Field {
                                    name: "_all",
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed("Count all records."),
                                    field_type: Cow::Borrowed(lookup.bool_type()),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                });
                                fields
                            },
                        }),
                        // aggregate: avg
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("AvgAggregateInputType"),
                            docs: Cow::Owned(format!("{} average aggregate input type.", m.name())),
                            kind: ClassKind::AggregateInputType,
                            fields: m.fields().iter().filter_map(|f| if f.field_type().is_number() {
                                Some(Field {
                                    name: f.name(),
                                    localized_name: Cow::Owned(f.localized_name()),
                                    docs: Cow::Borrowed(f.description().unwrap_or("")),
                                    field_type: Cow::Borrowed(lookup.bool_type()),
                                    optional: true,
                                    kind: FieldKind::Field,
                                })
                            } else {
                                None
                            }).collect(),
                        }),
                        // aggregate: sum
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("SumAggregateInputType"),
                            docs: Cow::Owned(format!("{} sum aggregate input type.", m.name())),
                            kind: ClassKind::AggregateInputType,
                            fields: m.fields().iter().filter_map(|f| if f.field_type().is_number() {
                                Some(Field {
                                    name: f.name(),
                                    localized_name: Cow::Owned(f.localized_name()),
                                    docs: Cow::Borrowed(f.description().unwrap_or("")),
                                    field_type: Cow::Borrowed(lookup.bool_type()),
                                    optional: true,
                                    kind: FieldKind::Field,
                                })
                            } else {
                                None
                            }).collect(),
                        }),
                        // aggregate: min
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("MinAggregateInputType"),
                            docs: Cow::Owned(format!("{} min aggregate input type.", m.name())),
                            kind: ClassKind::AggregateInputType,
                            fields: m.fields().iter().map(|f| Field {
                                name: f.name(),
                                localized_name: Cow::Owned(f.localized_name()),
                                docs: Cow::Borrowed(f.description().unwrap_or("")),
                                field_type: Cow::Borrowed(lookup.bool_type()),
                                optional: true,
                                kind: FieldKind::Field,
                            }).collect(),
                        }),
                        // aggregate: max
                        Some(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("MaxAggregateInputType"),
                            docs: Cow::Owned(format!("{} max aggregate input type.", m.name())),
                            kind: ClassKind::AggregateInputType,
                            fields: m.fields().iter().map(|f| Field {
                                name: f.name(),
                                localized_name: Cow::Owned(f.localized_name()),
                                docs: Cow::Borrowed(f.description().unwrap_or("")),
                                field_type: Cow::Borrowed(lookup.bool_type()),
                                optional: true,
                                kind: FieldKind::Field,
                            }).collect(),
                        }),
                        // credentials
                        if m.identity() {
                            Some(Class {
                                model_name: m.name(),
                                localized_name: Cow::Borrowed(""),
                                name_suffix: Cow::Borrowed("CredentialsInput"),
                                docs: Cow::Owned(format!("{} credentials for signing in.", m.name())),
                                kind: ClassKind::CredentialsInput,
                                fields: {
                                    let mut fields = vec![];
                                    fields.extend(m.auth_identity_keys().iter().map(|k| {
                                        let f = m.field(k).unwrap();
                                        Field {
                                            name: f.name(),
                                            localized_name: Cow::Owned(f.localized_name()),
                                            docs: Cow::Borrowed(f.description().unwrap_or("")),
                                            field_type: lookup.field_type_to_create_type(f.field_type(), false),
                                            optional: if m.auth_identity_keys().len() == 1 { false } else { true },
                                            kind: FieldKind::Field,
                                        }
                                    }));
                                    fields.extend(m.auth_by_keys().iter().map(|k| {
                                        let f = m.field(k).unwrap();
                                        Field {
                                            name: f.name(),
                                            localized_name: Cow::Owned(f.localized_name()),
                                            docs: Cow::Borrowed(f.description().unwrap_or("")),
                                            field_type: lookup.field_type_to_create_type(f.field_type(), false),
                                            optional: if m.auth_by_keys().len() == 1 { false } else { true },
                                            kind: FieldKind::Field,
                                        }
                                    }));
                                    fields
                                }
                            })
                        } else {
                            None
                        }
                    ].into_iter().flatten().collect();
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
                                                let before = "CreateNested".to_owned() + if relation.is_vec() { "Many" } else { "One" };
                                                helper::without_infix(relation.model(), before.as_str(), opposite.name(), "Input")
                                            } else {
                                                Cow::Owned(format!("{}CreateNested{}Input", relation.model(), if relation.is_vec() { "Many" } else { "One" }))
                                            }
                                        },
                                        optional: if relation.has_foreign_key() { true } else { relation.is_optional() },
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
                                    field_type: lookup.generated_type_to_enumerate(helper::without_infix(m.name(), "Create", w, "Input")),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "connectOrCreate",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::connect_or_create_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(helper::without_infix(m.name(), "ConnectOrCreate", w, "Input")),
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
                    ]).flatten().collect::<Vec<Class>>());
                    // update input
                    classes.extend(without.iter().map(|w| vec![
                        // update input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("Update", w, "Input"),
                            docs: Cow::Owned(format!("{} update input.", m.name())),
                            kind: ClassKind::UpdateInput,
                            fields: m.input_keys().iter().filter_map(|k| if let Some(field) = m.field(k) {
                                Some(Field {
                                    name: field.name(),
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(field.description().unwrap_or("")),
                                    field_type: lookup.field_type_to_update_type(field.field_type(), field.is_optional()),
                                    optional: true,
                                    kind: FieldKind::Field,
                                })
                            } else if let Some(property) = m.property(k) {
                                Some(Field {
                                    name: property.name(),
                                    localized_name: Cow::Borrowed(""),
                                    docs: Cow::Borrowed(property.description.as_ref().map(|v| v.as_str()).unwrap_or("")),
                                    field_type: lookup.field_type_to_update_type(property.field_type(), property.is_optional()),
                                    optional: true,
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
                                                helper::without_infix(relation.model(), &("UpdateNested".to_owned() + if relation.is_vec() { "Many" } else { "One" }), opposite.name(), "Input")
                                            } else {
                                                Cow::Owned(format!("{}UpdateNested{}Input", relation.model(), if relation.is_vec() { "Many" } else { "One" }))
                                            }
                                        },
                                        optional: true,
                                        kind: FieldKind::Relation,
                                    })
                                }
                            } else { unreachable!() }).collect()
                        },
                        // update nested many input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("UpdateNestedMany", w, "Input"),
                            docs: Cow::Owned(format!("{} update nested many input.", m.name())),
                            kind: ClassKind::UpdateNestedManyInput,
                            fields: vec![
                                Field {
                                    name: "create",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::create_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(helper::without_infix(m.name(), "Create", w, "Input")),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "connectOrCreate",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::connect_or_create_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(helper::without_infix(m.name(), "ConnectOrCreate", w, "Input")),
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
                                Field {
                                    name: "set",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::set_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}WhereUniqueInput", m.name()))),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "update",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::update_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(helper::without_infix(m.name(), "UpdateWithWhereUnique", w, "Input")),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "upsert",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::upsert_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(helper::without_infix(m.name(), "UpsertWithWhereUnique", w, "Input")),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "disconnect",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::disconnect_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}WhereUniqueInput", m.name()))),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "delete",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::delete_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}WhereUniqueInput", m.name()))),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "updateMany",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::update_many_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(helper::without_infix(m.name(), "UpdateManyWithWhere", w, "Input")),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "deleteMany",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::delete_many_doc(m.name()),
                                    field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}WhereInput", m.name()))),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                }
                            ]
                        },
                        // update nested one input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("UpdateNestedOne", w, "Input"),
                            docs: Cow::Owned(format!("{} update nested one input.", m.name())),
                            kind: ClassKind::UpdateNestedOneInput,
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
                                Field {
                                    name: "set",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::set_doc(m.name()),
                                    field_type: Cow::Owned(format!("{}WhereUniqueInput", m.name())),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "update",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::update_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "UpdateWithWhereUnique", w, "Input"),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "upsert",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::upsert_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "UpsertWithWhereUnique", w, "Input"),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "disconnect",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::disconnect_doc(m.name()),
                                    field_type: Cow::Borrowed(lookup.bool_type()),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "delete",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::delete_doc(m.name()),
                                    field_type: Cow::Borrowed(lookup.bool_type()),
                                    optional: true,
                                    kind: FieldKind::Predefined,
                                },
                            ],
                        },
                        // update with where unique input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("UpdateWithWhereUnique", w, "Input"),
                            docs: Cow::Owned(format!("{} update with where unique input.", m.name())),
                            kind: ClassKind::UpdateWithWhereUniqueInput,
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
                                    name: "update",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::update_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "Update", w, "Input"),
                                    optional: false,
                                    kind: FieldKind::Predefined,
                                },
                            ]
                        },
                        // update many with where input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("UpdateManyWithWhere", w, "Input"),
                            docs: Cow::Owned(format!("{} update many with where input.", m.name())),
                            kind: ClassKind::UpdateWithWhereUniqueInput,
                            fields: vec![
                                Field {
                                    name: "where",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::where_doc(m.name()),
                                    field_type: Cow::Owned(format!("{}WhereInput", m.name())),
                                    optional: false,
                                    kind: FieldKind::Predefined,
                                },
                                Field {
                                    name: "update",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::update_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "Update", w, "Input"),
                                    optional: false,
                                    kind: FieldKind::Predefined,
                                },
                            ]
                        },
                        // upsert with where unique input
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: helper::without_infix_no_model_name("UpsertWithWhereUnique", w, "Input"),
                            docs: Cow::Owned(format!("{} upsert with where unique input.", m.name())),
                            kind: ClassKind::UpdateWithWhereUniqueInput,
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
                                    name: "update",
                                    localized_name: Cow::Borrowed(""),
                                    docs: helper::update_doc(m.name()),
                                    field_type: helper::without_infix(m.name(), "Update", w, "Input"),
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
                            ]
                        },
                    ]).flatten().collect::<Vec<Class>>());
                    classes.extend(vec![
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("Args"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                }
                            ].into_iter().flatten().collect::<Vec<_>>(),
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("FindUniqueArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_where_unique_field(m.name(), false)),
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                }
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("FindFirstArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_where_field(m.name(), true, true)),
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                                Some(helper::args_order_by_field(m.name(), &lookup, true)),
                                Some(helper::args_cursor_field(m.name(), true)),
                                Some(helper::args_take_field(m.name(), lookup.number_type(), true)),
                                Some(helper::args_skip_field(m.name(), lookup.number_type(), true)),
                                Some(helper::args_page_size_field(m.name(), lookup.number_type(), true)),
                                Some(helper::args_page_number_field(m.name(), lookup.number_type(), true)),
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("FindManyArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_where_field(m.name(), false, true)),
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                                Some(helper::args_order_by_field(m.name(), &lookup, true)),
                                Some(helper::args_cursor_field(m.name(), true)),
                                Some(helper::args_take_field(m.name(), lookup.number_type(), true)),
                                Some(helper::args_skip_field(m.name(), lookup.number_type(), true)),
                                Some(helper::args_page_size_field(m.name(), lookup.number_type(), true)),
                                Some(helper::args_page_number_field(m.name(), lookup.number_type(), true)),
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("CreateArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                                Some(helper::args_create_input(m.name(), false)),
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("UpdateArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_where_unique_field(m.name(), false)),
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                                Some(helper::args_update_input(m.name(), false)),
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("UpsertArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_where_unique_field(m.name(), false)),
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                                Some(helper::args_create_input(m.name(), false)),
                                Some(helper::args_update_input(m.name(), false)),
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("DeleteArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                helper::args_where_unique_field(m.name(), false),
                                helper::args_select_field(m.name(), true),
                            ]
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("CreateManyArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                                Some(helper::args_create_many_input(m.name(), &lookup, false)),
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("UpdateManyArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_where_field(m.name(), false, true)),
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                                Some(helper::args_update_input(m.name(), false)),
                            ].into_iter().flatten().collect::<Vec<_>>()
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("DeleteManyArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                helper::args_where_field(m.name(), false, true),
                                helper::args_select_field(m.name(), true),
                            ]
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("CountArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                helper::args_where_field(m.name(), false, true),
                                helper::args_cursor_field(m.name(), true),
                                helper::args_skip_field(m.name(), lookup.number_type(), true),
                                helper::args_take_field(m.name(), lookup.number_type(), true),
                                helper::args_order_by_field(m.name(), &lookup, true),
                                helper::args_count_select_field(m.name(), true),
                            ]
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("AggregateArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                helper::args_where_field(m.name(), false, true),
                                helper::args_cursor_field(m.name(), true),
                                helper::args_skip_field(m.name(), lookup.number_type(), true),
                                helper::args_take_field(m.name(), lookup.number_type(), true),
                                helper::args_page_size_field(m.name(), lookup.number_type(), true),
                                helper::args_page_number_field(m.name(), lookup.number_type(), true),
                                helper::args_order_by_field(m.name(), &lookup, true),
                                helper::args_distinct_field(m.name(), &lookup, true),
                                helper::args__count_field(m.name(), true),
                                helper::args__avg_field(m.name(), true),
                                helper::args__sum_field(m.name(), true),
                                helper::args__min_field(m.name(), true),
                                helper::args__max_field(m.name(), true),
                            ]
                        },
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("GroupByArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                helper::args_where_field(m.name(), false, true),
                                helper::args_by_field(m.name(), false, &lookup),
                                helper::args_having_field(m.name(), &lookup, true),
                                helper::args_cursor_field(m.name(), true),
                                helper::args_skip_field(m.name(), lookup.number_type(), true),
                                helper::args_take_field(m.name(), lookup.number_type(), true),
                                helper::args_page_size_field(m.name(), lookup.number_type(), true),
                                helper::args_page_number_field(m.name(), lookup.number_type(), true),
                                helper::args_order_by_field(m.name(), &lookup, true),
                                helper::args_distinct_field(m.name(), &lookup, true),
                                helper::args__count_field(m.name(), true),
                                helper::args__avg_field(m.name(), true),
                                helper::args__sum_field(m.name(), true),
                                helper::args__min_field(m.name(), true),
                                helper::args__max_field(m.name(), true),
                            ]
                        }
                    ]);
                    if m.has_action(Action::from_u32(SIGN_IN_HANDLER)) {
                        classes.push(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("SignInArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_credentials_field(m.name(), false)),
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                            ].into_iter().flatten().collect::<Vec<_>>(),
                        });
                    }
                    if m.has_action(Action::from_u32(IDENTITY_HANDLER)) {
                        classes.push(Class {
                            model_name: m.name(),
                            localized_name: Cow::Borrowed(""),
                            name_suffix: Cow::Borrowed("IdentityArgs"),
                            docs: Cow::Borrowed(""),
                            kind: ClassKind::ActionArgs,
                            fields: vec![
                                Some(helper::args_select_field(m.name(), true)),
                                if m.relations().is_empty() {
                                    None
                                } else {
                                    Some(helper::args_include_field(m.name(), true))
                                },
                            ].into_iter().flatten().collect::<Vec<_>>(),
                        });
                    }
                    classes
                }).flatten().collect::<Vec<Class>>());
                results
            },
            delegates: graph.models().iter().map(|m| {
                Delegate {
                    model_name: Cow::Borrowed(m.name()),
                    actions: m.actions().iter().map(|a| DelegateAction {
                        name: a.as_handler_str(),
                        response: lookup.action_result_type(*a, m.name()),
                        docs: None,
                    }).collect(),
                }
            }).collect()
        }
    }
}

mod helper {
    use std::borrow::Cow;
    use inflector::Inflector;
    use crate::gen::internal::client::outline::field::Field;
    use crate::gen::internal::client::outline::field_kind::FieldKind;
    use crate::gen::internal::type_lookup::TypeLookup;

    pub(super) fn without_infix<'a>(model_name: &str, before: &str, without: &str, after: &str) -> Cow<'a, str> {
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

    pub(super) fn create_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn update_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn upsert_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn delete_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn delete_many_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn update_many_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn disconnect_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn connect_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn connect_or_create_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn where_unique_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn where_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    pub(super) fn set_doc<'a>(_model: &str) -> Cow<'a, str> {
        Cow::Borrowed("")
    }

    // fields

    pub(super) fn args_where_field(model: &str, doc_singular: bool, optional: bool) -> Field {
        Field {
            name: "where",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("The filter to find {}.", if doc_singular { model.to_word_case().articlize() } else { model.to_word_case().to_plural() })),
            field_type: Cow::Owned(format!("{}WhereInput", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_by_field<'a, T>(model: &str, optional: bool, lookup: &T) -> Field<'a> where T: TypeLookup {
        Field {
            name: "by",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Select which fields to group by."),
            field_type: lookup.generated_type_to_vec(Cow::Owned(format!("{}ScalarFieldEnum", model))),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_having_field<'a, T>(model: &str, lookup: &T, optional: bool) -> Field<'a> where T: TypeLookup {
        Field {
            name: "having",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Filter after aggregation."),
            field_type: lookup.generated_type_to_vec(Cow::Owned(format!("{}ScalarWhereWithAggregatesInput", model))),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_where_unique_field(model: &str, optional: bool) -> Field {
        Field {
            name: "where",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("The unique filter to find the {}.", model)),
            field_type: Cow::Owned(format!("{}WhereUniqueInput", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_select_field(model: &str, optional: bool) -> Field {
        Field {
            name: "select",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Select scalar fields to fetch from the {} model.", model.to_word_case())),
            field_type: Cow::Owned(format!("{}Select", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_count_select_field(model: &str, optional: bool) -> Field {
        Field {
            name: "select",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Select countable scalar fields to count from the {} model.", model.to_word_case())),
            field_type: Cow::Owned(format!("{}CountAggregateInputType", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_include_field(model: &str, optional: bool) -> Field {
        Field {
            name: "include",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Include relations to fetch from the {} model.", model.to_word_case())),
            field_type: Cow::Owned(format!("{}Include", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_order_by_field<'a, T>(model: &str, lookup: &T, optional: bool) -> Field<'a> where T: TypeLookup {
        Field {
            name: "orderBy",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Determine the order of {} to fetch.", model.to_word_case().to_plural())),
            field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}OrderByInput", model))),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_distinct_field<'a, T>(model: &str, lookup: &T, optional: bool) -> Field<'a> where T: TypeLookup {
        Field {
            name: "distinct",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Select distinct records by fields."),
            field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}ScalarFieldEnum", model))),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_cursor_field(model: &str, optional: bool) -> Field {
        Field {
            name: "cursor",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Sets the position for searching for {}.", model.to_word_case().to_plural())),
            field_type: Cow::Owned(format!("{}WhereUniqueInput", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_take_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> Field<'a> {
        Field {
            name: "take",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("How many {} to take. If cursor is set and this value is negative, take from the other direction.", model.to_word_case().to_plural())),
            field_type: Cow::Borrowed(number_type),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_skip_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> Field<'a> {
        Field {
            name: "skip",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Skip the first `n` {}.", model.to_word_case().to_plural())),
            field_type: Cow::Borrowed(number_type),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_page_size_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> Field<'a> {
        Field {
            name: "pageSize",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Sets the page size for the returned {} data.", model.to_word_case().to_plural())),
            field_type: Cow::Borrowed(number_type),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_page_number_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> Field<'a> {
        Field {
            name: "pageNumber",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Sets the page number of {} data.", model.to_word_case().to_plural())),
            field_type: Cow::Borrowed(number_type),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_create_input(model: &str, optional: bool) -> Field {
        Field {
            name: "create",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Data needed to create {}.", model.to_word_case().articlize())),
            field_type: Cow::Owned(format!("{}CreateInput", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_create_many_input<'a, T>(model: &str, lookup: &T, optional: bool) -> Field<'a> where T: TypeLookup {
        Field {
            name: "createMany",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Data needed to create {}.", model.to_word_case().to_plural())),
            field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}CreateInput", model))),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_update_input(model: &str, optional: bool) -> Field {
        Field {
            name: "update",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Data needed to update {}.", model.to_word_case().articlize())),
            field_type: Cow::Owned(format!("{}UpdateInput", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args__count_field(model: &str, optional: bool) -> Field {
        Field {
            name: "_count",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Select which field to count."),
            field_type: Cow::Owned(format!("{}CountAggregateInputType", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }
    pub(super) fn args__avg_field(model: &str, optional: bool) -> Field {
        Field {
            name: "_avg",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Select which field to calculate average with."),
            field_type: Cow::Owned(format!("{}AvgAggregateInputType", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }
    pub(super) fn args__sum_field(model: &str, optional: bool) -> Field {
        Field {
            name: "_sum",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Select which field to calculate sum with."),
            field_type: Cow::Owned(format!("{}SumAggregateInputType", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }
    pub(super) fn args__min_field(model: &str, optional: bool) -> Field {
        Field {
            name: "_min",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Select which field to calculate min with."),
            field_type: Cow::Owned(format!("{}MinAggregateInputType", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }
    pub(super) fn args__max_field(model: &str, optional: bool) -> Field {
        Field {
            name: "_max",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Borrowed("Select which field to calculate max with."),
            field_type: Cow::Owned(format!("{}MaxAggregateInputType", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn args_credentials_field(model: &str, optional: bool) -> Field {
        Field {
            name: "credentials",
            localized_name: Cow::Borrowed(""),
            docs: Cow::Owned(format!("Credential data needed to sign in {}.", model.to_word_case().articlize())),
            field_type: Cow::Owned(format!("{}CredentialsInput", model)),
            optional,
            kind: FieldKind::Predefined,
        }
    }

    pub(super) fn where_the_three_ops(field_type: Cow<str>) -> Vec<Field> {
        vec![
            Field {
                name: "AND",
                localized_name: Cow::Borrowed(""),
                docs: Cow::Borrowed(""),
                field_type: field_type.clone(),
                optional: true,
                kind: FieldKind::Predefined,
            },
            Field {
                name: "OR",
                localized_name: Cow::Borrowed(""),
                docs: Cow::Borrowed(""),
                field_type: field_type.clone(),
                optional: true,
                kind: FieldKind::Predefined,
            },
            Field {
                name: "NOT",
                localized_name: Cow::Borrowed(""),
                docs: Cow::Borrowed(""),
                field_type,
                optional: true,
                kind: FieldKind::Predefined,
            }
        ]
    }
}
