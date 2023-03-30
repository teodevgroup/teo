#[derive(Copy, Clone)]
pub(in crate::gen) enum Kind {
    WhereInput,
    WhereUniqueInput,
    CreateInput,
    UpdateInput,
    DataOutput,
    SortOrderInput,
    SelectInput,
    IncludeInput,
    Other,
}
