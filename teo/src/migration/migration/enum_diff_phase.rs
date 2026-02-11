#[derive(PartialEq, Eq, Clone, Copy)]
pub(in super::super) enum EnumDiffPhase {
    Create,
    Delete,
}
