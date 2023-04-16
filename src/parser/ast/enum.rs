use crate::parser::ast::comment_block::CommentBlock;
use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) struct ASTEnum {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) identifier: Identifier,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) choices: Vec<EnumChoice>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl ASTEnum {
    pub(crate) fn new(item_id: usize, source_id: usize, comment_block: Option<CommentBlock>, identifier: Identifier, decorators: Vec<Decorator>, choices: Vec<EnumChoice>, span: Span) -> Self {
        Self {
            id: item_id,
            source_id,
            comment_block,
            identifier,
            decorators,
            choices,
            span,
            resolved: false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EnumChoice {
    pub(crate) identifier: Identifier,
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
}

impl EnumChoice {
    pub(crate) fn new(identifier: Identifier, comment_block: Option<CommentBlock>, decorators: Vec<Decorator>, span: Span) -> Self {
        Self { identifier, decorators, span, comment_block, resolved: false }
    }
}
