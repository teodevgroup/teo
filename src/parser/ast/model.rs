use std::cmp::Ordering;
use itertools::Itertools;
use crate::parser::ast::comment_block::CommentBlock;
use crate::parser::ast::decorator::ASTDecorator;
use crate::parser::ast::field::ASTField;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub struct ASTModel {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) id_path: Vec<usize>,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) ns_path: Vec<String>,
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) fields: Vec<ASTField>,
    pub(crate) decorators: Vec<ASTDecorator>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) scalar_field_enum: Vec<String>,
    pub(crate) scalar_field_and_cached_property_enum: Vec<String>,
    pub(crate) direct_relation_enum: Vec<String>,
}

impl ASTModel {
    pub(crate) fn new(id: usize, source_id: usize, id_path: Vec<usize>, identifier: ASTIdentifier, ns_path: Vec<String>, comment_block: Option<CommentBlock>, fields: Vec<ASTField>, decorators: Vec<ASTDecorator>, span: Span) -> Self {
        Self {
            id, source_id, id_path, identifier, comment_block, fields, ns_path, decorators, span, resolved: false,
            scalar_field_enum: vec![], scalar_field_and_cached_property_enum: vec![],
            direct_relation_enum: vec![],
        }
    }

    pub(crate) fn sorted_fields(&self) -> Vec<&ASTField> {
        self.fields.iter().sorted_by(|a, b| if a.field_class.is_relation() {
            Ordering::Greater
        } else if b.field_class.is_relation() {
            Ordering::Less
        } else {
            Ordering::Less
        }).collect()
    }

    pub(crate) fn resolve(&mut self, scalar_field_enum: Vec<String>, scalar_field_and_cached_property_enum: Vec<String>, direct_relation_enum: Vec<String>) {
        self.scalar_field_enum = scalar_field_enum;
        self.scalar_field_and_cached_property_enum = scalar_field_and_cached_property_enum;
        self.direct_relation_enum = direct_relation_enum;
    }

    pub(crate) fn path(&self) -> Vec<&str> {
        let mut retval: Vec<&str> = self.ns_path.iter().map(|s| s.as_str()).collect();
        retval.push(self.identifier.name.as_str());
        retval
    }
}
