use crate::core::pipeline::Pipeline;

pub(crate) mod builder;

#[derive(Debug, Clone)]
pub(crate) struct Permission {
    can_read: Option<Pipeline>,
    can_update: Option<Pipeline>,
    can_create: Option<Pipeline>,
    can_delete: Option<Pipeline>,
}

impl Permission {

    pub(crate) fn new(r: Option<Pipeline>, u: Option<Pipeline> ,c: Option<Pipeline>, d: Option<Pipeline>) -> Self {
        Self {
            can_read: r,
            can_update: u,
            can_create: c,
            can_delete: d,
        }
    }

    pub(crate) fn can_read(&self) -> Option<&Pipeline> {
        self.can_read.as_ref()
    }

    pub(crate) fn can_update(&self) -> Option<&Pipeline> {
        self.can_update.as_ref()
    }

    pub(crate) fn can_create(&self) -> Option<&Pipeline> {
        self.can_create.as_ref()
    }

    pub(crate) fn can_delete(&self) -> Option<&Pipeline> {
        self.can_delete.as_ref()
    }
}
