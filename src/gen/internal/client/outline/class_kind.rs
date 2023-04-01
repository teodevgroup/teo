#[derive(Copy, Clone)]
pub(in crate::gen) enum ClassKind {
    DataOutput,
    WhereInput,
    WhereUniqueInput,
    RelationFilter,
    ListRelationFilter,
    CreateInput,
    UpdateInput,
    OrderByInput,
    SelectInput,
    IncludeInput,
    ActionArgs,
    Other,
    Enum,
    ConnectOrCreateInput,
    CreateNestedOneInput,
    CreateNestedManyInput,
    UpdateNestedOneInput,
    UpdateNestedManyInput,
    UpdateWithWhereUniqueInput,
    UpdateManyWithWhereInput,
    ScalarFieldEnum,
}

impl ClassKind {
    pub(in crate::gen) fn is_where_input(&self) -> bool {
        match self {
            ClassKind::WhereInput => true,
            _ => false,
        }
    }

    pub(in crate::gen) fn is_any_kind_of_enum(&self) -> bool {
        match self {
            ClassKind::Enum | ClassKind::ScalarFieldEnum => true,
            _ => false,
        }
    }

    pub(in crate::gen) fn is_order_by_input(&self) -> bool {
        match self {
            ClassKind::OrderByInput => true,
            _ => false,
        }
    }

    pub(in crate::gen) fn is_output(&self) -> bool {
        match self {
            ClassKind::DataOutput => true,
            _ => false,
        }
    }
}
