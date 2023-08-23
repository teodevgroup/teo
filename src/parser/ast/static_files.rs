use crate::parser::ast::expression::Expression;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) struct StaticFiles {
    pub(crate) source_id: usize,
    pub(crate) id: usize,
    pub(crate) span: Span,
    pub(crate) path: Expression,
    pub(crate) map: Expression,
    pub(crate) resolved_path: Option<String>,
    pub(crate) resolved_map: Option<String>,
}

impl StaticFiles {
    pub(crate) fn new(source_id: usize, id: usize, span: Span, path: Expression, map: Expression) -> Self {
        Self {
            source_id,
            id,
            span,
            path,
            map,
            resolved_path: None,
            resolved_map: None,
        }
    }
}
