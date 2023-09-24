use crate::app::app_ctx::AppCtx;
use crate::parser::ast::comment_block::CommentBlock;
use crate::parser::ast::decorator::ASTDecorator;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::r#type::ASTFieldType;
use crate::parser::ast::span::Span;
use crate::prelude::Value;

#[derive(Debug, Copy, Clone)]
pub(crate) enum ASTFieldClass {
    Unresolved,
    Field,
    DroppedField,
    Relation,
    Property,
}

impl ASTFieldClass {
    pub(crate) fn is_relation(&self) -> bool {
        match self {
            ASTFieldClass::Relation => true,
            _ => false,
        }
    }

    pub(crate) fn is_primitive_field(&self) -> bool {
        match self {
            ASTFieldClass::Field => true,
            _ => false,
        }
    }

    pub(crate) fn is_dropped(&self) -> bool {
        match self {
            ASTFieldClass::DroppedField => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ASTField {
    pub(crate) source_id: usize,
    pub(crate) comment_block: Option<CommentBlock>,
    pub(crate) identifier: ASTIdentifier,
    pub(crate) r#type: ASTFieldType,
    pub(crate) decorators: Vec<ASTDecorator>,
    pub(crate) span: Span,
    pub(crate) resolved: bool,
    pub(crate) field_class: ASTFieldClass,
}

impl ASTField {
    pub(crate) fn new(source_id: usize, comment_block: Option<CommentBlock>, identifier: ASTIdentifier, r#type: ASTFieldType, decorators: Vec<ASTDecorator>, span: Span) -> Self {
        Self {
            source_id, comment_block, identifier, r#type, decorators, span, resolved: false, field_class: ASTFieldClass::Unresolved,
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

    pub(crate) fn name(&self) -> &str {
        self.identifier.name.as_str()
    }

    pub(crate) fn validate_primitive_value(&self, value: &Value) -> Option<String> {
        if self.r#type.type_class.is_enum() {
            let enum_def = AppCtx::get().unwrap().parser().enum_by_id(&self.r#type.type_id);
            let enum_name = enum_def.path().join(".");
            if value.is_raw_enum_choice() {
                let variant_value = value.as_raw_enum_choice().unwrap();
                if enum_def.choices.iter().find(|v| v.identifier.name.as_str() == variant_value).is_some() {
                    None
                } else {
                    Some(format!("Value is not enum variant of `{enum_name}'"))
                }
            } else {
                Some(format!("Value is not enum variant of `{enum_name}'"))
            }
        } else {
            let p = self.r#type.identifiers.path();
            let type_name = p.get(0).unwrap().as_str();
            let valid = match type_name {
                "String" => value.is_string(),
                "Bool" => value.is_bool(),
                "Int" | "Int32" => value.is_i(),
                "Int64" => value.is_i(),
                "Float32" => value.is_f(),
                "Float" | "Float64" => value.is_f(),
                "Date" => value.is_date(),
                "DateTime" => value.is_datetime(),
                "Decimal" => value.is_decimal(),
                #[cfg(feature = "data-source-mongodb")]
                "ObjectId" => value.is_object_id(),
                _ => false,
            };
            if !valid {
                Some(format!("Value is not `{type_name}`"))
            } else {
                None
            }
        }
    }
}
