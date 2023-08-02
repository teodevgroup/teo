use std::borrow::Cow;
use std::sync::Arc;
use crate::core::field::field::Field;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::gen::internal::server::outline::class::Class;
use crate::gen::internal::server::outline::class_field::{ClassField, ClassFieldAggregate};
use crate::gen::internal::server::outline::enum_variant::EnumVariant;
use crate::gen::internal::server::outline::field_kind::FieldKind;
use crate::gen::internal::server::outline::r#enum::Enum;
use crate::gen::internal::server_type_lookup::ServerTypeLookup;
use crate::prelude::{Graph};

pub(in crate::gen) struct EntityOutline<'a> {
    pub(in crate::gen) classes: Vec<Class<'a>>,
    pub(in crate::gen) enums: Vec<Enum<'a>>,
}

impl<'a> EntityOutline<'a> {
    pub(in crate::gen) fn new<L>(graph: &'a Graph, lookup: L) -> Self where L: ServerTypeLookup {
        EntityOutline {
            classes: graph.models.values().filter_map(|m| {
                if m.is_teo_internal() {
                    None
                } else {
                    Some(Class {
                        name: m.name(),
                        fields: {
                            let mut fields = vec![];
                            fields.extend(m.fields().iter().map(|f| {
                                ClassField {
                                    name: f.name(),
                                    kind: FieldKind::Field,
                                    input_field_type: lookup.input_type(f.field_type(), f.is_optional()),
                                    input_optional: f.is_optional(),
                                    output_field_type: lookup.output_type(f.field_type(), f.is_optional()),
                                    output_optional: f.is_optional(),
                                    localized_name: Cow::Owned(f.localized_name()),
                                    desc: f.description_with_default(),
                                    getter: true,
                                    setter: true,
                                    is_enum: f.field_type().is_enum(),
                                    aggregate: Some(Self::aggregate_for_field(f, &lookup)),
                                }
                            }));
                            fields.extend(m.relations().iter().map(|r| {
                                ClassField {
                                    name: r.name(),
                                    kind: FieldKind::Relation,
                                    input_field_type: Cow::Borrowed(r.model()),
                                    input_optional: r.is_optional(),
                                    output_field_type: Cow::Borrowed(r.model()),
                                    output_optional: r.is_optional(),
                                    localized_name: Cow::Owned(r.localized_name()),
                                    desc: r.description_with_default(),
                                    getter: true,
                                    setter: true,
                                    is_enum: false,
                                    aggregate: None,
                                }
                            }));
                            fields.extend(m.properties().iter().map(|p| {
                                ClassField {
                                    name: p.name(),
                                    kind: FieldKind::Property,
                                    input_field_type: lookup.input_type(p.field_type(), p.is_optional()),
                                    input_optional: p.is_optional(),
                                    output_field_type: lookup.output_type(p.field_type(), p.is_optional()),
                                    output_optional: p.is_optional(),
                                    localized_name: Cow::Owned(p.localized_name()),
                                    desc: p.description_with_default(),
                                    getter: p.has_getter(),
                                    setter: p.has_setter(),
                                    is_enum: p.field_type().is_enum(),
                                    aggregate: None,
                                }
                            }));
                            fields
                        },
                        localized_name: m.localized_name(),
                        desc: m.description(),
                    })
                }
            }).collect(),
            enums: graph.enums.values().map(|e| {
                Enum {
                    name: e.name(),
                    variants: e.variants().iter().map(|v| {
                        EnumVariant {
                            name: v.name(),
                            localized_name: v.localized_name(),
                            desc: v.description_with_default(),
                        }
                    }).collect(),
                    localized_name: e.localized_name(),
                    desc: e.description_with_default(),
                }
            }).collect(),
        }
    }

    fn aggregate_for_field<L>(field: &'a Arc<Field>, lookup: &L) -> ClassFieldAggregate<'a> where L: ServerTypeLookup {
        ClassFieldAggregate {
            can_count: true,
            can_max: field.field_type().is_scalar(),
            max_type: lookup.output_type(field.field_type(), false),
            can_min: field.field_type().is_scalar(),
            min_type: lookup.output_type(field.field_type(), false),
            can_avg: field.field_type().is_number(),
            can_sum: field.field_type().is_number(),
            sum_type: if field.field_type().is_number() {
                if field.field_type().is_int() {
                    lookup.output_type(&FieldType::I64, false)
                } else {
                    lookup.output_type(&FieldType::F64, false)
                }
            } else {
                Cow::Borrowed("")
            },
            can_group_by: field.field_type().is_scalar(),
            group_by_type: if field.field_type().is_scalar() {
                Some(lookup.output_type(field.field_type(), false))
            } else {
                None
            },
        }
    }
}