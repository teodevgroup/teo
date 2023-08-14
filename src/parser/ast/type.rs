use crate::parser::ast::identifier::ASTIdentifier;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Arity {
    Scalar,
    Array,
    Dictionary,
}

#[derive(Debug)]
pub(crate) enum TypeClass {
    Unresolved,
    Builtin,
    Enum,
    Model,
}

#[derive(Debug)]
pub(crate) struct Type {
    pub(crate) identifier: ASTIdentifier,
    pub(crate) arity: Arity,
    pub(crate) item_required: bool,
    pub(crate) collection_required: bool,
    pub(crate) resolved: bool,
    pub(crate) type_id: (usize, usize),
    pub(crate) type_class: TypeClass,
}

impl Type {
    pub(crate) fn new(identifier: ASTIdentifier, arity: Arity, item_required: bool, collection_required: bool) -> Self {
        Self {
            identifier, arity, item_required, collection_required,
            resolved: false,
            type_id: (0, 0),
            type_class: TypeClass::Unresolved,
        }
    }

    pub(crate) fn resolve(&mut self, type_id: (usize, usize), type_class: TypeClass) {
        self.resolved = true;
        self.type_id = type_id;
        self.type_class = type_class;
    }
}
