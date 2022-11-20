#[derive(Debug, Clone)]
pub(crate) struct IdReference {
    source_id: usize,
    top_id: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum Reference {
    ModelReference(IdReference),
    ConstantReference(IdReference),
}
