use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub enum WriteRule {
    Write,
    NoWrite,
    WriteOnce,
    WriteOnCreate,
    WriteNonNull,
    WriteIf(Pipeline),
}

impl WriteRule {
    pub fn is_no_write(&self) -> bool {
        match self {
            WriteRule::NoWrite => true,
            _ => false
        }
    }
}
