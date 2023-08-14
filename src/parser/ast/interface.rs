use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;
use crate::parser::ast::interface_type::InterfaceType;

#[derive(Debug)]
pub(crate) struct InterfaceDeclaration {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) name: InterfaceType,
    pub(crate) extends: Vec<InterfaceType>,
    pub(crate) items: Vec<InterfaceItemDeclaration>,
    pub(crate) span: Span,
}

impl InterfaceDeclaration {
    pub(crate) fn args(&self) -> &Vec<InterfaceType> {
        &self.name.args
    }
}

#[derive(Debug)]
pub(crate) struct InterfaceItemDeclaration {
    pub(crate) name: ASTIdentifier,
    pub(crate) kind: InterfaceType,
    pub(crate) span: Span,
}