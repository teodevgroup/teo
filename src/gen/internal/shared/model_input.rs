use std::borrow::Cow;
use std::cmp::Ordering;
use inflector::Inflector;
use itertools::Itertools;
use crate::core::action::{Action, IDENTITY_HANDLER, SIGN_IN_HANDLER};
use crate::core::field::r#type::FieldTypeOwner;
use crate::gen::internal::type_lookup::TypeLookup;
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

pub(crate) struct ActionArg<'a> {
    pub(crate) name: &'static str,
    pub(crate) docs: Option<Cow<'a, str>>,
    pub(crate) fields: Vec<ActionArgField<'a>>,
}

impl<'a> ActionArg<'a> {
    pub(crate) fn sorted_fields(&'a self) -> Vec<&'a ActionArgField<'a>> {
        self.fields.iter().sorted_by(|a, b| if a.optional { Ordering::Greater } else { Ordering::Less }).collect()
    }
}

pub(crate) struct ActionArgField<'a> {
    pub(crate) name: &'static str,
    pub(crate) docs: Cow<'a, str>,
    pub(crate) field_type: Cow<'a, str>,
    pub(crate) optional: bool,
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
    pub(crate) action_args: Vec<ActionArg<'a>>,
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
            },
            action_args: {
                let mut args = vec![
                    ActionArg {
                        name: "Args",
                        docs: None,
                        fields: vec![
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                        ]
                    },
                    ActionArg {
                        name: "FindUniqueArgs",
                        docs: None,
                        fields: vec![
                            args_where_unique_field(m.name(), false),
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                        ]
                    },
                    ActionArg {
                        name: "FindFirstArgs",
                        docs: None,
                        fields: vec![
                            args_where_field(m.name(), true, true),
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                            args_order_by_field(m.name(), &lookup, true),
                            args_cursor_field(m.name(), true),
                            args_take_field(m.name(), lookup.number_type(), true),
                            args_skip_field(m.name(), lookup.number_type(), true),
                            args_page_size_field(m.name(), lookup.number_type(), true),
                            args_page_number_field(m.name(), lookup.number_type(), true),
                        ]
                    },
                    ActionArg {
                        name: "FindManyArgs",
                        docs: None,
                        fields: vec![
                            args_where_field(m.name(), false, true),
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                            args_order_by_field(m.name(), &lookup, true),
                            args_cursor_field(m.name(), true),
                            args_take_field(m.name(), lookup.number_type(), true),
                            args_skip_field(m.name(), lookup.number_type(), true),
                            args_page_size_field(m.name(), lookup.number_type(), true),
                            args_page_number_field(m.name(), lookup.number_type(), true),
                        ]
                    },
                    ActionArg {
                        name: "CreateArgs",
                        docs: None,
                        fields: vec![
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                            args_create_input(m.name(), false),
                        ]
                    },
                    ActionArg {
                        name: "UpdateArgs",
                        docs: None,
                        fields: vec![
                            args_where_unique_field(m.name(), false),
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                            args_update_input(m.name(), false),
                        ]
                    },
                    ActionArg {
                        name: "UpsertArgs",
                        docs: None,
                        fields: vec![
                            args_where_unique_field(m.name(), false),
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                            args_create_input(m.name(), false),
                            args_update_input(m.name(), false),
                        ]
                    },
                    ActionArg {
                        name: "DeleteArgs",
                        docs: None,
                        fields: vec![
                            args_where_unique_field(m.name(), false),
                            args_select_field(m.name(), true),
                        ]
                    },
                    ActionArg {
                        name: "CreateManyArgs",
                        docs: None,
                        fields: vec![
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                            args_create_many_input(m.name(), &lookup, false),
                        ]
                    },
                    ActionArg {
                        name: "UpdateManyArgs",
                        docs: None,
                        fields: vec![
                            args_where_field(m.name(), false, true),
                            args_select_field(m.name(), true),
                            args_include_field(m.name(), true),
                            args_update_input(m.name(), false),
                        ]
                    },
                    ActionArg {
                        name: "DeleteManyArgs",
                        docs: None,
                        fields: vec![
                            args_where_field(m.name(), false, true),
                            args_select_field(m.name(), true),
                        ]
                    },
                    ActionArg {
                        name: "CountArgs",
                        docs: None,
                        fields: vec![
                            args_where_field(m.name(), false, true),
                            args_cursor_field(m.name(), true),
                            args_skip_field(m.name(), lookup.number_type(), true),
                            args_take_field(m.name(), lookup.number_type(), true),
                            args_order_by_field(m.name(), &lookup, true),
                            args_count_select_field(m.name(), true),
                        ]
                    },
                    ActionArg {
                        name: "AggregateArgs",
                        docs: None,
                        fields: vec![
                            args_where_field(m.name(), false, true),
                            args_cursor_field(m.name(), true),
                            args_skip_field(m.name(), lookup.number_type(), true),
                            args_take_field(m.name(), lookup.number_type(), true),
                            args_page_size_field(m.name(), lookup.number_type(), true),
                            args_page_number_field(m.name(), lookup.number_type(), true),
                            args_order_by_field(m.name(), &lookup, true),
                            args_distinct_field(m.name(), &lookup, true),
                            args__count_field(m.name(), true),
                            args__avg_field(m.name(), true),
                            args__sum_field(m.name(), true),
                            args__min_field(m.name(), true),
                            args__max_field(m.name(), true),
                        ]
                    },
                    ActionArg {
                        name: "GroupByArgs",
                        docs: None,
                        fields: vec![
                            args_where_field(m.name(), false, true),
                            args_by_field(m.name(), false, &lookup),
                            args_having_field(m.name(), &lookup, true),
                            args_cursor_field(m.name(), true),
                            args_skip_field(m.name(), lookup.number_type(), true),
                            args_take_field(m.name(), lookup.number_type(), true),
                            args_page_size_field(m.name(), lookup.number_type(), true),
                            args_page_number_field(m.name(), lookup.number_type(), true),
                            args_order_by_field(m.name(), &lookup, true),
                            args_distinct_field(m.name(), &lookup, true),
                            args__count_field(m.name(), true),
                            args__avg_field(m.name(), true),
                            args__sum_field(m.name(), true),
                            args__min_field(m.name(), true),
                            args__max_field(m.name(), true),
                        ]
                    }
                ];
                if m.has_action(Action::from_u32(SIGN_IN_HANDLER)) {
                    args.push(ActionArg {
                        name: "SignInArgs",
                        docs: None,
                        fields: vec![
                            args_credentials_field(m.name(), false),
                            args_select_field(m.name(), false),
                            args_include_field(m.name(), false),
                        ]
                    })
                }
                if m.has_action(Action::from_u32(IDENTITY_HANDLER)) {
                    args.push(ActionArg {
                        name: "IdentityArgs",
                        docs: None,
                        fields: vec![
                            args_select_field(m.name(), false),
                            args_include_field(m.name(), false),
                        ]
                    })
                }
                args
            }
        }
    }).collect()
}
