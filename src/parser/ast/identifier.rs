use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::parser::ast::interface_type::InterfaceType;
use crate::parser::ast::r#type::Arity;
use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ASTIdentifier {
    pub(crate) name: String,
    pub(crate) span: Span,
}

impl ASTIdentifier {
    pub(crate) fn alter_generics_with(&self, map: &HashMap<String, InterfaceType>) -> InterfaceType {
        if map.contains_key(&self.name) {
            map.get(&self.name).unwrap().clone()
        } else {
            InterfaceType {
                name: self.clone(),
                args: vec![],
                span: self.span.clone(),
                collection_optional: false,
                optional: false,
                arity: Arity::Scalar,
            }
        }
    }
}

impl Display for ASTIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
