use std::collections::HashMap;
use crate::parser::ast::accessible::Accessible;

pub(crate) struct GlobalFieldDecorators {
    objects: HashMap<String, Accessible>
}
