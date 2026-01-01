use crate::partial::Partial;

pub trait Model: Partial {

    fn table_name(&self) -> &'static str;

    fn table_name_quoted(&self) -> &'static str;
}
