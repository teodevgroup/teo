#[derive(Debug, Clone, Copy)]
pub enum DeleteRule {
    Nullify,
    Cascade,
    Deny,
}
