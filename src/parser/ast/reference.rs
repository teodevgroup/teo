#[derive(Debug, Clone)]
pub(crate) enum Reference {
    ModelReference((usize, usize, String)),
    ConstantReference((usize, usize)),
    DataSetReference((usize, usize)),
}

impl Reference {

    pub(crate) fn as_model_ref(&self) -> Option<(usize, usize, String)> {
        match self {
            Reference::ModelReference(r) => Some(r.clone()),
            _ => None,
        }
    }

    pub(crate) fn is_model_ref(&self) -> bool {
        self.as_model_ref().is_some()
    }

    pub(crate) fn as_constant_ref(&self) -> Option<(usize, usize)> {
        match self {
            Reference::ConstantReference(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub(crate) fn is_constant_ref(&self) -> bool {
        self.as_constant_ref().is_some()
    }

    pub(crate) fn as_dataset_ref(&self) -> Option<(usize, usize)> {
        match self {
            Reference::DataSetReference(c) => Some(c.clone()),
            _ => None,
        }
    }

    pub(crate) fn is_dataset_ref(&self) -> bool {
        self.as_dataset_ref().is_some()
    }
}
