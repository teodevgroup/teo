#[derive(Debug, Clone, Copy)]
pub enum DeleteRule {
    Default,
    Nullify,
    Cascade,
    Deny,
}
