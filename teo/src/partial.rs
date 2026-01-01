use crate::{model::Model, row::Row};

pub trait Partial {

    type Model: Model;

    fn fields() -> &'static str;

    fn fields_sql_len() -> usize;

    fn from_row(row: &Row) -> Self;
}
