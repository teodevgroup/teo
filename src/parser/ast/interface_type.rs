use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::r#type::Arity;
use crate::parser::ast::span::Span;

#[derive(Debug, Clone)]
pub(crate) struct InterfaceType {
    pub(crate) name: ASTIdentifier,
    pub(crate) args: Vec<InterfaceType>,
    pub(crate) arity: Arity,
    pub(crate) collection_optional: bool,
    pub(crate) optional: bool,
    pub(crate) span: Span,
}

impl InterfaceType {

    pub(crate) fn alter_generics_with(&self, map: &HashMap<String, InterfaceType>) -> Self {
        InterfaceType {
            name: self.name.alter_generics_with(map).name,
            args: self.args.iter().map(|arg| arg.alter_generics_with(map)).collect(),
            span: self.span.clone(),
            collection_optional: self.collection_optional,
            optional: self.optional,
            arity: self.arity,
        }
    }
}

impl Display for InterfaceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name.name)?;
        if self.args.len() > 0 {
            f.write_str("<")?;
        }
        for (index, arg) in self.args.iter().enumerate() {
            Display::fmt(arg, f)?;
            if index != self.args.len() - 1 {
                f.write_str(", ")?;
            }
        }
        if self.args.len() > 0 {
            f.write_str(">")?;
        }
        if self.optional {
            f.write_str("?")?;
        }
        if self.arity != Arity::Scalar {
            match self.arity {
                Arity::Array => f.write_str("[]")?,
                Arity::Dictionary => f.write_str("{}")?,
                _ => ()
            };
            if self.collection_optional {
                f.write_str("?")?;
            }
        }
        Ok(())
    }
}