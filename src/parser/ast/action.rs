use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;
use crate::parser::ast::type_with_generic::TypeWithGenerics;

#[derive(Debug)]
pub(crate) struct ActionGroupDeclaration {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) span: Span,
}

#[derive(Debug)]
pub(crate) struct ActionDeclaration {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) group_identifier: ASTIdentifier,
    pub(crate) name_identifier: ASTIdentifier,
    pub(crate) input_type: TypeWithGenerics,
    pub(crate) output_type: TypeWithGenerics,
    pub(crate) span: Span,
}