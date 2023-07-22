use std::borrow::Cow;
use crate::gen::internal::server::outline::field_kind::FieldKind;

pub(in crate::gen) struct ClassField<'a> {
    pub(in crate::gen) name: &'a str,
    pub(in crate::gen) kind: FieldKind,
    pub(in crate::gen) input_field_type: Cow<'a, str>,
    pub(in crate::gen) input_optional: bool,
    pub(in crate::gen) output_field_type: Cow<'a, str>,
    pub(in crate::gen) output_optional: bool,
    pub(in crate::gen) localized_name: Cow<'a, str>,
    pub(in crate::gen) desc: &'a str,
    pub(in crate::gen) getter: bool,
    pub(in crate::gen) setter: bool,
    pub(in crate::gen) is_enum: bool,
    pub(in crate::gen) aggregate: Option<ClassFieldAggregate<'a>>,
}

impl<'a> ClassField<'a> {
    pub(in crate::gen) fn aggregate(&self) -> &ClassFieldAggregate {
        self.aggregate.as_ref().unwrap()
    }
}

pub(in crate::gen) struct ClassFieldAggregate<'a> {
    pub(in crate::gen) can_count: bool,
    pub(in crate::gen) can_max: bool,
    pub(in crate::gen) max_type: Cow<'a, str>,
    pub(in crate::gen) can_min: bool,
    pub(in crate::gen) min_type: Cow<'a, str>,
    pub(in crate::gen) can_avg: bool,
    pub(in crate::gen) can_sum: bool,
    pub(in crate::gen) sum_type: Cow<'a, str>,
    pub(in crate::gen) can_group_by: bool,
    pub(in crate::gen) group_by_type: Option<Cow<'a, str>>,
}