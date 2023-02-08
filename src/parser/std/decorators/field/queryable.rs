use crate::core::field::Field;
use crate::core::field::{QueryAbility};
use crate::parser::ast::argument::Argument;

pub(crate) fn queryable_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.query_ability = QueryAbility::Queryable;
}
