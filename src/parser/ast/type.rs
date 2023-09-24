use crate::parser::ast::identifier_path::ASTIdentifierPath;
use crate::parser::ast::span::Span;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Arity {
    Scalar,
    Array,
    Dictionary,
}

impl Arity {

    pub(crate) fn is_scalar(&self) -> bool {
        match self {
            Arity::Scalar => true,
            _ => false,
        }
    }

    pub(crate) fn is_array(&self) -> bool {
        match self {
            Arity::Array => true,
            _ => false,
        }
    }

    pub(crate) fn is_dictionary(&self) -> bool {
        match self {
            Arity::Dictionary => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub(crate) enum TypeClass {
    Unresolved,
    Builtin,
    Enum,
    Model,
}

impl TypeClass {
    pub(crate) fn is_enum(&self) -> bool {
        match self {
            TypeClass::Enum => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct ASTFieldType {
    pub(crate) span: Span,
    pub(crate) identifiers: ASTIdentifierPath,
    pub(crate) arity: Arity,
    pub(crate) item_required: bool,
    pub(crate) collection_required: bool,
    pub(crate) resolved: bool,
    pub(crate) type_id: Vec<usize>,
    pub(crate) type_class: TypeClass,
}

impl ASTFieldType {
    pub(crate) fn new(span: Span, identifiers: ASTIdentifierPath, arity: Arity, item_required: bool, collection_required: bool) -> Self {
        Self {
            span, identifiers, arity, item_required, collection_required,
            resolved: false,
            type_id: vec![],
            type_class: TypeClass::Unresolved,
        }
    }

    pub(crate) fn resolve(&mut self, type_id: Vec<usize>, type_class: TypeClass) {
        self.resolved = true;
        self.type_id = type_id;
        self.type_class = type_class;
    }
}
