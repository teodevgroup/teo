#[derive(Copy, Clone)]
pub(in crate::gen) enum ClassKind {
    WhereInput,
    WhereUniqueInput,
    CreateInput,
    UpdateInput,
    DataOutput,
    SortOrderInput,
    SelectInput,
    IncludeInput,
    ActionArgs,
    Other,
    Enum,
}

impl ClassKind {
    pub(in crate::gen) fn is_where_input(&self) -> bool {
        match self {
            ClassKind::WhereInput => true,
            _ => false,
        }
    }

    pub(in crate::gen) fn is_enum(&self) -> bool {
        match self {
            ClassKind::Enum => true,
            _ => false,
        }
    }
}
