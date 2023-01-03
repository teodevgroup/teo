use crate::core::field::builder::FieldBuilder;
use crate::core::field::{Field, QueryAbility};
use crate::parser::ast::argument::Argument;

pub(crate) fn queryable_decorator(_args: Vec<Argument>, field: &mut FieldBuilder) {
    field.query_ability = QueryAbility::Queryable;
}
