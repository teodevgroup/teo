use std::path::PathBuf;
use crate::gen::interface::client::kind::Kind as ClientKind;
use crate::parser::ast::span::Span;
use crate::parser::ast::item::Item;
use crate::parser::ast::identifier::ASTIdentifier;

#[derive(Debug, Clone)]
pub struct ASTClient {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Option<ASTIdentifier>,
    pub(crate) items: Vec<Item>,
    pub(crate) span: Span,
    pub(crate) provider: Option<ClientKind>,
    pub(crate) dest: Option<PathBuf>,
    pub(crate) package: Option<bool>,
    pub(crate) host: Option<String>,
    pub(crate) object_name: String,
    pub(crate) git_commit: bool,
}

impl ASTClient {
    pub(crate) fn new(id: usize, source_id: usize, identifier: Option<ASTIdentifier>, items: Vec<Item>, span: Span) -> Self {
        Self {
            id,
            source_id,
            identifier,
            items,
            span,
            provider: None,
            dest: None,
            package: Some(true),
            host: None,
            object_name: "teo".to_owned(),
            git_commit: false,
        }
    }
}
