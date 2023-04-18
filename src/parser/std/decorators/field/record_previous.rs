use crate::core::field::field::{Field, PreviousValueRule};
use crate::parser::ast::argument::Argument;

pub(crate) fn record_previous_decorator(_args: Vec<Argument>, field: &mut Field) {
    field.previous_value_rule = PreviousValueRule::Keep;
}
