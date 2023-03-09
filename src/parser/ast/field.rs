use crate::parser::ast::comment_block::CommentBlock;
use crate::parser::ast::decorator::Decorator;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::r#type::Type;
use crate::parser::ast::span::Span;

#[derive(Debug)]
pub(crate) enum FieldClass {
    Unresolved,
    Field,
    DroppedField,
    Relation,
    Property,
}

#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) identifier: Identifier,
    pub(crate) r#type: Type,
    pub(crate) decorators: Vec<Decorator>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) field_class: FieldClass,
}

impl Field {
    pub(crate) fn new(comment_block: Option<CommentBlock>, identifier: Identifier, r#type: Type, decorators: Vec<Decorator>, span: Span) -> Self {
        Self {
            comment_block, identifier, r#type, decorators, span, resolved: false, field_class: FieldClass::Unresolved,
        }
    }

    pub(crate) fn figure_out_class(&mut self) {
        for decorator in self.decorators.iter() {
            match decorator.expression.as_unit() {
                Some(unit) => {
                    let name = unit.expressions.get(0).unwrap().as_identifier().unwrap().name.as_str();
                    match name {
                        "relation" => {
                            self.field_class = FieldClass::Relation;
                            return;
                        }
                        "getter" | "setter" => {
                            self.field_class = FieldClass::Property;
                            return;
                        }
                        "dropped" => {
                            self.field_class = FieldClass::DroppedField;
                            return;
                        }
                        _ => {}
                    }
                }
                _ => {},
            }
        }
        self.field_class = FieldClass::Field;
    }
}
