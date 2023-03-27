use std::borrow::Cow;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::prelude::Graph;

pub(crate) struct ModelOutputField<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) field_type: Cow<'a, str>,
    pub(crate) optional: bool,
    pub(crate) localized_name: Cow<'a, str>,
    pub(crate) desc: Option<Cow<'a, str>>,
}

pub(crate) struct ModelOutput<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) fields: Vec<ModelOutputField<'a>>,
}

pub(crate) fn model_outputs_without_relations<F>(graph: &Graph, type_to_string: F) -> Vec<ModelOutput> where F: Fn(&FieldType) -> Cow<str> {
    model_outputs(graph, false, type_to_string, |v| Cow::Borrowed(v))
}

pub(crate) fn model_outputs_with_relations<F, G>(graph: &Graph, type_to_string: F, type_to_vec: G) -> Vec<ModelOutput> where F: Fn(&FieldType) -> Cow<str>, G: Fn(&str) -> Cow<str> {
    model_outputs(graph, false, type_to_string, type_to_vec)
}

fn model_outputs<F, G>(graph: &Graph, include_relations: bool, type_to_string: F, type_to_vec: G) -> Vec<ModelOutput> where F: Fn(&FieldType) -> Cow<str>, G: Fn(&str) -> Cow<str> {
    graph.models().iter().map(|m| {
        let mut fields = vec![];
        for key in m.output_keys() {
            if let Some(field) = m.field(key) {
                fields.push(ModelOutputField {
                    name: Cow::Borrowed(field.name()),
                    field_type: type_to_string(field.field_type()),
                    optional: field.is_optional(),
                    localized_name: Cow::Owned(field.localized_name()),
                    desc: field.description().map(|d| Cow::Borrowed(d)),
                });
            } else if let Some(property) = m.property(key) {
                fields.push(ModelOutputField {
                    name: Cow::Borrowed(property.name()),
                    field_type: type_to_string(property.field_type()),
                    optional: property.is_optional(),
                    localized_name: Cow::Owned(property.localized_name()),
                    desc: property.description.as_ref().map(|s| Cow::Borrowed(s.as_str())),
                })
            }
        }
        if include_relations {
            for relation in m.relations() {
                fields.push(ModelOutputField {
                    name: Cow::Borrowed(relation.name()),
                    field_type: if relation.is_vec() {
                        type_to_vec(relation.model())
                    } else {
                        Cow::Borrowed(relation.name())
                    },
                    optional: relation.is_optional(),
                    localized_name: Cow::Owned(relation.localized_name()),
                    desc: relation.description().map(|d| Cow::Borrowed(d.as_str())),
                })
            }
        }
        ModelOutput {
            name: Cow::Borrowed(m.name()),
            fields,
        }
    }).collect()
}
