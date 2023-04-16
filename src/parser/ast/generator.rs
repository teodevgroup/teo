use std::path::PathBuf;
use crate::app::program::ProgramLang;
use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;
use crate::parser::ast::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct ASTEntity {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Option<Identifier>,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
    pub(crate) provider: Option<ProgramLang>,
    pub(crate) dest: Option<PathBuf>,
}

impl ASTEntity {
    pub(crate) fn new(id: usize, source_id: usize, identifier: Option<Identifier>, items: Vec<Item>, span: Span) -> Self {
        Self {
            id, source_id, identifier, items, span, provider: None, dest: None,
        }
    }
}
