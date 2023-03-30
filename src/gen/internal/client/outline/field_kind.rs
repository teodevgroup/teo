#[derive(Copy, Clone)]
pub(in crate::gen) enum FieldKind {
    Property,
    Relation,
    Field,
    Predefined,
}

impl FieldKind {
    pub(in crate::gen) fn is_property(&self) -> bool {
        match self {
            FieldKind::Property => true,
            _ => false,
        }
    }

    pub(in crate::gen) fn is_relation(&self) -> bool {
        match self {
            FieldKind::Relation => true,
            _ => false,
        }
    }

    pub(in crate::gen) fn is_field(&self) -> bool {
        match self {
            FieldKind::Field => true,
            _ => false,
        }
    }

    pub(in crate::gen) fn is_predefined(&self) -> bool {
        match self {
            FieldKind::Predefined => true,
            _ => false,
        }
    }
}
