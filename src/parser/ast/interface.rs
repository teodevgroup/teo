use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;
use crate::parser::ast::type_with_generic::TypeWithGenerics;

#[derive(Debug)]
pub(crate) struct InterfaceDeclaration {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) name: TypeWithGenerics,
    pub(crate) extends: Vec<TypeWithGenerics>,
    pub(crate) items: Vec<InterfaceItemDeclaration>,
    pub(crate) span: Span,
}

impl InterfaceDeclaration {
    pub(crate) fn args(&self) -> &Vec<TypeWithGenerics> {
        &self.name.args
    }
}

#[derive(Debug)]
pub(crate) struct InterfaceItemDeclaration {
    pub(crate) name: ASTIdentifier,
    pub(crate) kind: TypeWithGenerics,
    pub(crate) span: Span,
}