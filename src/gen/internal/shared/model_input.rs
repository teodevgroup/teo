use std::borrow::Cow;
use std::cmp::Ordering;
use inflector::Inflector;
use itertools::Itertools;
use crate::core::action::{Action, IDENTITY_HANDLER, SIGN_IN_HANDLER};
use crate::core::field::r#type::FieldTypeOwner;
use crate::gen::internal::type_lookup::TypeLookup;
use crate::prelude::Graph;

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

pub(crate) fn model_inputs<'a, T>(graph: &'a Graph, lookup: T) -> Vec<ModelInput> where T: TypeLookup + 'a {
    graph.models().iter().map(|m| {
        ModelInput {
            name: Cow::Borrowed(m.name()),
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
