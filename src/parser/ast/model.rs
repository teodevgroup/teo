use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::field::Field;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub struct Model {
    pub(crate) id: usize,
    pub(crate) source_id: usize,
    pub(crate) identifier: Identifier,
    pub(crate) fields: Vec<Field>,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) scalar_field_enum: Vec<String>,
    pub(crate) scalar_field_and_cached_property_enum: Vec<String>,
    pub(crate) direct_relation_enum: Vec<String>,
}

impl Model {
    pub(crate) fn new(id: usize, source_id: usize, identifier: Identifier, fields: Vec<Field>, decorators: Vec<Decorator>, span: Span) -> Self {
        Self {
            id, source_id, identifier, fields, decorators, span, resolved: false,
            scalar_field_enum: vec![], scalar_field_and_cached_property_enum: vec![],
            direct_relation_enum: vec![],
        }
    }

    pub(crate) fn resolve(&mut self, scalar_field_enum: Vec<String>, scalar_field_and_cached_property_enum: Vec<String>, direct_relation_enum: Vec<String>) {
        self.scalar_field_enum = scalar_field_enum;
        self.scalar_field_and_cached_property_enum = scalar_field_and_cached_property_enum;
        self.direct_relation_enum = direct_relation_enum;
    }
}
