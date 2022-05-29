use crate::core::pipeline::Pipeline;


#[derive(Debug, Clone)]
pub(crate) struct Permission {
    pub(crate) can_read: Option<Pipeline>,
    pub(crate) can_update: Option<Pipeline>,
    pub(crate) can_create: Option<Pipeline>,
    pub(crate) can_delete: Option<Pipeline>,
}
