#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeleteRule {
    Default,
    Nullify,
    Cascade,
    Deny,
}
