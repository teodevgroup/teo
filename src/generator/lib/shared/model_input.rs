use std::borrow::Cow;
use itertools::Itertools;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::prelude::Graph;

pub(crate) struct ModelInclude<'a> {
    pub(crate) relation_name: Cow<'a, str>,
    pub(crate) model_name: Cow<'a, str>,
    pub(crate) many: bool,
}

pub(crate) struct ModelWhereField<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) filter_type: Cow<'a, str>,
}

pub(crate) struct ModelWhereUniqueField<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) create_type: Cow<'a, str>,
}

pub(crate) struct ModelInput<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) select: Vec<Cow<'a, str>>,
    pub(crate) includes: Vec<ModelInclude<'a>>,
    pub(crate) where_fields: Vec<ModelWhereField<'a>>,
    pub(crate) where_unique_fields: Vec<ModelWhereUniqueField<'a>>,
    pub(crate) order_by_fields: Vec<Cow<'a, str>>,
}

pub(crate) fn model_inputs<F, G>(graph: &Graph, field_type_to_filter_type: F, field_type_to_create_type: G) -> Vec<ModelInput> where F: Fn(&FieldType, bool) -> Cow<str>, G: Fn(&FieldType, bool) -> Cow<str> {
    graph.models().iter().map(|m| {
        ModelInput {
            name: Cow::Borrowed(m.name()),
            select: m.output_keys().iter().filter(|k| m.field(k).is_some()).map(|k| Cow::Borrowed(k.as_str())).collect(),
            includes: m.relations().iter().map(|r| ModelInclude { relation_name: Cow::Borrowed(r.name()), model_name: Cow::Borrowed(r.model()), many: r.is_vec() }).collect(),
            where_fields: m.query_keys().iter().map(|k| if let Some(field) = m.field(k) {
                ModelWhereField {
                    name: Cow::Borrowed(field.name()),
                    filter_type: field_type_to_filter_type(field.field_type(), field.is_optional()),
                }
            } else if let Some(relation) = m.relation(k) {
                ModelWhereField {
                    name: Cow::Borrowed(relation.name()),
                    filter_type: if relation.is_vec() { Cow::Owned(relation.model().to_owned() + "ListRelationFilter") } else { Cow::Owned(relation.model().to_owned() + "RelationFilter") }
                }
            } else { unreachable!()}).collect(),
            where_unique_fields: m.indices().iter().filter(|i| i.r#type().is_unique()).map(|i| i.keys().iter().map(|k| m.field(k).unwrap()).map(|f| ModelWhereUniqueField {
                name: Cow::Borrowed(f.name()),
                create_type: field_type_to_create_type(f.field_type(), f.is_optional()),
            })).flatten().dedup_by(|f1, f2| f1.name == f2.name).collect(),
            order_by_fields: m.sort_keys().iter().map(|k| Cow::Borrowed(k.as_str())).collect(),
        }
    }).collect()
}
