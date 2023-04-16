use crate::parser::ast::comment_block::CommentBlock;
use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::r#type::Type;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) enum ASTFieldClass {
    Unresolved,
    Field,
    DroppedField,
    Relation,
    Property,
}

#[derive(Debug)]
pub(crate) struct ASTField {
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) r#type: Type,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) field_class: ASTFieldClass,
}

impl ASTField {
    pub(crate) fn new(comment_block: Option<CommentBlock>, identifier: ASTIdentifier, r#type: Type, decorators: Vec<Decorator>, span: Span) -> Self {
        Self {
            comment_block, identifier, r#type, decorators, span, resolved: false, field_class: ASTFieldClass::Unresolved,
        }
    }

    pub(crate) fn figure_out_class(&mut self) {
        for decorator in self.decorators.iter() {
            match decorator.expression.as_unit() {
                Some(unit) => {
                    let name = unit.expressions.get(0).unwrap().as_identifier().unwrap().name.as_str();
                    match name {
                        "relation" => {
                            self.field_class = ASTFieldClass::Relation;
                            return;
                        }
                        "getter" | "setter" => {
                            self.field_class = ASTFieldClass::Property;
                            return;
                        }
                        "dropped" => {
                            self.field_class = ASTFieldClass::DroppedField;
                            return;
                        }
                        _ => {}
                    }
                }
                _ => {},
            }
        }
        self.field_class = ASTFieldClass::Field;
    }
}
