#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PreviousValueRule {
    DontKeep,
    DropAfterSaved,
    KeepAfterSaved,
}