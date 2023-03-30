use std::borrow::Cow;
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

impl Outline {
    pub(in crate::gen) fn new<L>(graph: &Graph, lookup: L) -> Self where L: TypeLookup {
        Self {
            classes: {
                let mut results = graph.enums().iter().map(|(name, enum_def)| {
                    Class {
                        model_name: enum_def.name(),
                        localized_name: Cow::Borrowed(enum_def.localized_name()),
                        name_suffix: "",
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
                        Class {
                            model_name: m.name(),
                            localized_name: Cow::Owned(m.localized_name()),
                            name_suffix: "",
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
                                if include_relations {
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
                                            docs: relation.description().map(|d| Cow::Borrowed(d.as_str())).unwrap_or(Cow::Borrowed("")),
                                            kind: FieldKind::Relation,
                                        })
                                    }
                                }
                                fields
                            },
                        },
                        //
                    ];
                    classes
                }).flatten().collect());
                results
            }
        }
    }
}
