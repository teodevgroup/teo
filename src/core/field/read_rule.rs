use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub enum ReadRule {
    Read,
    NoRead,
    ReadIf(Pipeline),
}

impl ReadRule {
    pub fn is_no_read(&self) -> bool {
        match self {
            ReadRule::NoRead => true,
            _ => false
        }
    }
}
