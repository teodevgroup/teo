use crate::parser::ast::span::Span;
use crate::parser::ast::type_with_generic::TypeWithGenerics;

#[derive(Debug)]
pub(crate) struct InterfaceDeclaration {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) name: TypeWithGenerics,
    pub(crate) args: Vec<TypeWithGenerics>,
    pub(crate) span: Span,
}