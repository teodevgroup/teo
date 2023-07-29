use crate::core::field::field::FieldIndex;

pub(crate) trait FieldIndexable {
    fn index(&self) -> Option<&FieldIndex>;
    fn set_index(&mut self, index: Option<FieldIndex>);
    fn set_primary(&mut self, primary: bool);
}