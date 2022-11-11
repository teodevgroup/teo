use crate::parser::ast::identifier::Identifier;

#[derive(Debug)]
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
    pub(crate) identifier: Identifier,
    pub(crate) arity: Arity,
    pub(crate) required: bool,
    pub(crate) resolved: bool,
    pub(crate) type_id: (usize, usize),
    pub(crate) type_class: TypeClass,
}

impl Type {
    pub(crate) fn new(identifier: Identifier, arity: Arity, required: bool) -> Self {
        Self {
            identifier, arity, required,
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
