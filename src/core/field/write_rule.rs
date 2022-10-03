#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WriteRule {
    Write,
    NoWrite,
    WriteOnce,
    WriteOnCreate,
    WriteNonNull
}
