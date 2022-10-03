use crate::core::error::ActionError;

pub type ActionResult<T> = Result<T, ActionError>;
