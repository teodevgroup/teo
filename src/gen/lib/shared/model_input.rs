use std::borrow::Cow;
use std::cmp::Ordering;
use inflector::Inflector;
use itertools::Itertools;
use crate::core::action::{Action, IDENTITY_HANDLER, SIGN_IN_HANDLER};
use crate::core::field::r#type::FieldTypeOwner;
use crate::gen::lib::shared::type_lookup::TypeLookup;
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

fn args_where_field(model: &str, doc_singular: bool, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "where",
        docs: Cow::Owned(format!("The filter to find {}.", if doc_singular { model.to_word_case().articlize() } else { model.to_word_case().to_plural() })),
        field_type: Cow::Owned(format!("{}WhereInput", model)),
        optional,
    }
}

fn args_by_field<'a, T>(model: &str, optional: bool, lookup: &T) -> ActionArgField<'a> where T: TypeLookup {
    ActionArgField {
        name: "by",
        docs: Cow::Borrowed("Select which fields to group by."),
        field_type: lookup.generated_type_to_vec(Cow::Owned(format!("{}ScalarFieldEnum", model))),
        optional,
    }
}

fn args_having_field<'a, T>(model: &str, lookup: &T, optional: bool) -> ActionArgField<'a> where T: TypeLookup {
    ActionArgField {
        name: "having",
        docs: Cow::Borrowed("Filter after aggregation."),
        field_type: lookup.generated_type_to_vec(Cow::Owned(format!("{}ScalarWhereWithAggregatesInput", model))),
        optional,
    }
}

fn args_where_unique_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "where",
        docs: Cow::Owned(format!("The unique filter to find the {}.", model)),
        field_type: Cow::Owned(format!("{}WhereUniqueInput", model)),
        optional,
    }
}

fn args_select_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "select",
        docs: Cow::Owned(format!("Select scalar fields to fetch from the {} model.", model.to_word_case())),
        field_type: Cow::Owned(format!("{}Select", model)),
        optional,
    }
}

fn args_count_select_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "select",
        docs: Cow::Owned(format!("Select countable scalar fields to count from the {} model.", model.to_word_case())),
        field_type: Cow::Owned(format!("{}CountAggregateInputType", model)),
        optional,
    }
}

fn args_include_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "include",
        docs: Cow::Owned(format!("Include relations to fetch from the {} model.", model.to_word_case())),
        field_type: Cow::Owned(format!("{}Include", model)),
        optional,
    }
}

fn args_order_by_field<'a, T>(model: &str, lookup: &T, optional: bool) -> ActionArgField<'a> where T: TypeLookup {
    ActionArgField {
        name: "orderBy",
        docs: Cow::Owned(format!("Determine the order of {} to fetch.", model.to_word_case().to_plural())),
        field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}OrderByInput", model))),
        optional,
    }
}

fn args_distinct_field<'a, T>(model: &str, lookup: &T, optional: bool) -> ActionArgField<'a> where T: TypeLookup {
    ActionArgField {
        name: "distinct",
        docs: Cow::Borrowed("Select distinct records by fields."),
        field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}DistinctFieldEnum", model))),
        optional,
    }
}

fn args_cursor_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "cursor",
        docs: Cow::Owned(format!("Sets the position for searching for {}.", model.to_word_case().to_plural())),
        field_type: Cow::Owned(format!("{}WhereUniqueInput", model)),
        optional,
    }
}

fn args_take_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> ActionArgField<'a> {
    ActionArgField {
        name: "take",
        docs: Cow::Owned(format!("How many {} to take. If cursor is set and this value is negative, take from the other direction.", model.to_word_case().to_plural())),
        field_type: Cow::Borrowed(number_type),
        optional,
    }
}

fn args_skip_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> ActionArgField<'a> {
    ActionArgField {
        name: "skip",
        docs: Cow::Owned(format!("Skip the first `n` {}.", model.to_word_case().to_plural())),
        field_type: Cow::Borrowed(number_type),
        optional,
    }
}

fn args_page_size_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> ActionArgField<'a> {
    ActionArgField {
        name: "pageSize",
        docs: Cow::Owned(format!("Sets the page size for the returned {} data.", model.to_word_case().to_plural())),
        field_type: Cow::Borrowed(number_type),
        optional,
    }
}

fn args_page_number_field<'a>(model: &'a str, number_type: &'static str, optional: bool) -> ActionArgField<'a> {
    ActionArgField {
        name: "pageNumber",
        docs: Cow::Owned(format!("Sets the page number of {} data.", model.to_word_case().to_plural())),
        field_type: Cow::Borrowed(number_type),
        optional,
    }
}

fn args_create_input(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "create",
        docs: Cow::Owned(format!("Data needed to create {}.", model.to_word_case().articlize())),
        field_type: Cow::Owned(format!("{}CreateInput", model)),
        optional,
    }
}

fn args_create_many_input<'a, T>(model: &str, lookup: &T, optional: bool) -> ActionArgField<'a> where T: TypeLookup {
    ActionArgField {
        name: "createMany",
        docs: Cow::Owned(format!("Data needed to create {}.", model.to_word_case().to_plural())),
        field_type: lookup.generated_type_to_enumerate(Cow::Owned(format!("{}CreateInput", model))),
        optional,
    }
}

fn args_update_input(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "update",
        docs: Cow::Owned(format!("Data needed to update {}.", model.to_word_case().articlize())),
        field_type: Cow::Owned(format!("{}UpdateInput", model)),
        optional,
    }
}

fn args__count_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "_count",
        docs: Cow::Borrowed("Select which field to count."),
        field_type: Cow::Owned(format!("{}CountAggregateInputType", model)),
        optional,
    }
}
fn args__avg_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "_count",
        docs: Cow::Borrowed("Select which field to calculate average with."),
        field_type: Cow::Owned(format!("{}AvgAggregateInputType", model)),
        optional,
    }
}
fn args__sum_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "_sum",
        docs: Cow::Borrowed("Select which field to calculate sum with."),
        field_type: Cow::Owned(format!("{}SumAggregateInputType", model)),
        optional,
    }
}
fn args__min_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "_min",
        docs: Cow::Borrowed("Select which field to calculate min with."),
        field_type: Cow::Owned(format!("{}MinAggregateInputType", model)),
        optional,
    }
}
fn args__max_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "_max",
        docs: Cow::Borrowed("Select which field to calculate max with."),
        field_type: Cow::Owned(format!("{}MaxAggregateInputType", model)),
        optional,
    }
}

fn args_credentials_field(model: &str, optional: bool) -> ActionArgField {
    ActionArgField {
        name: "credentials",
        docs: Cow::Owned(format!("Credential data needed to sign in {}.", model.to_word_case().articlize())),
        field_type: Cow::Owned(format!("{}CredentialsInput", model)),
        optional,
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
