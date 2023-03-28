use std::borrow::Cow;
use itertools::Itertools;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::generator::lib::shared::type_lookup::TypeLookup;
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

pub(crate) struct ModelCreateField<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) create_type: Cow<'a, str>,
}

pub(crate) struct ModelUpdateField<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) update_type: Cow<'a, str>,
}

pub(crate) struct ModelInput<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) select: Vec<Cow<'a, str>>,
    pub(crate) includes: Vec<ModelInclude<'a>>,
    pub(crate) where_fields: Vec<ModelWhereField<'a>>,
    pub(crate) where_unique_fields: Vec<ModelWhereUniqueField<'a>>,
    pub(crate) order_by_fields: Vec<Cow<'a, str>>,
    pub(crate) create_fields: Vec<ModelCreateField<'a>>,
    pub(crate) update_fields: Vec<ModelUpdateField<'a>>,
    pub(crate) without: Vec<Cow<'a, str>>,

}

impl<'a> ModelInput<'a> {
    pub(crate) fn create_fields_without(&'a self, without: &'a str) -> Vec<&'a ModelCreateField<'a>> {
        self.create_fields.iter().filter(|f| f.name.as_ref() != without).collect()
    }

    pub(crate) fn update_fields_without(&'a self, without: &'a str) -> Vec<&'a ModelUpdateField<'a>> {
        self.update_fields.iter().filter(|f| f.name.as_ref() != without).collect()
    }
}

pub(crate) fn model_inputs<'a, T>(graph: &'a Graph, lookup: T) -> Vec<ModelInput> where T: TypeLookup + 'a {
    graph.models().iter().map(|m| {
        ModelInput {
            name: Cow::Borrowed(m.name()),
            select: m.output_keys().iter().filter(|k| m.field(k).is_some()).map(|k| Cow::Borrowed(k.as_str())).collect(),
            includes: m.relations().iter().map(|r| ModelInclude { relation_name: Cow::Borrowed(r.name()), model_name: Cow::Borrowed(r.model()), many: r.is_vec() }).collect(),
            where_fields: m.query_keys().iter().map(|k| if let Some(field) = m.field(k) {
                ModelWhereField {
                    name: Cow::Borrowed(field.name()),
                    filter_type: lookup.field_type_to_filter_type(field.field_type(), field.is_optional()),
                }
            } else if let Some(relation) = m.relation(k) {
                ModelWhereField {
                    name: Cow::Borrowed(relation.name()),
                    filter_type: if relation.is_vec() { Cow::Owned(relation.model().to_owned() + "ListRelationFilter") } else { Cow::Owned(relation.model().to_owned() + "RelationFilter") }
                }
            } else { unreachable!() }).collect(),
            where_unique_fields: m.indices().iter().filter(|i| i.r#type().is_unique()).map(|i| i.keys().iter().map(|k| m.field(k).unwrap()).map(|f| ModelWhereUniqueField {
                name: Cow::Borrowed(f.name()),
                create_type: lookup.field_type_to_create_type(f.field_type(), f.is_optional()),
            })).flatten().dedup_by(|f1, f2| f1.name == f2.name).collect(),
            order_by_fields: m.sort_keys().iter().map(|k| Cow::Borrowed(k.as_str())).collect(),
            create_fields: m.input_keys().iter().map(|k| if let Some(field) = m.field(k) {
                ModelCreateField {
                    name: Cow::Borrowed(field.name()),
                    create_type: lookup.field_type_to_create_type(field.field_type(), field.is_optional()),
                }
            } else if let Some(property) = m.property(k) {
                ModelCreateField {
                    name: Cow::Borrowed(property.name()),
                    create_type: lookup.field_type_to_create_type(property.field_type(), property.is_optional()),
                }
            } else if let Some(relation) = m.relation(k) {
                ModelCreateField {
                    name: Cow::Borrowed(relation.name()),
                    create_type: Cow::Owned(relation.model().to_owned() + "CreateNested" + if relation.is_vec() { "Many" } else { "One" } + "Without" + m.name() + "Input"),
                }
            } else { unreachable!() }).collect(),
            update_fields: m.input_keys().iter().map(|k| if let Some(field) = m.field(k) {
                ModelUpdateField {
                    name: Cow::Borrowed(field.name()),
                    update_type: lookup.field_type_to_update_type(field.field_type(), field.is_optional()),
                }
            } else if let Some(property) = m.property(k) {
                ModelUpdateField {
                    name: Cow::Borrowed(property.name()),
                    update_type: lookup.field_type_to_update_type(property.field_type(), property.is_optional()),
                }
            } else if let Some(relation) = m.relation(k) {
                ModelUpdateField {
                    name: Cow::Borrowed(relation.name()),
                    update_type: Cow::Owned(relation.model().to_owned() + "UpdateNested" + if relation.is_vec() { "Many" } else { "One" } + "Without" + m.name() + "Input"),
                }
            } else { unreachable!() }).collect(),
            without: {
                let mut without = vec![Cow::Borrowed("")];
                without.append(&mut m.relations().iter().map(|r| Cow::Borrowed(r.name())).collect());
                without
            }
        }
    }).collect()
}
