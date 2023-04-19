#[derive(Debug)]
pub(crate) struct TestConf {
    pub(crate) reset: Option<Reset>,
}

#[derive(Debug)]
pub(crate) struct Reset {
    pub(crate) mode: ResetMode,
    pub(crate) datasets: ResetDatasets,
}

#[derive(Debug)]
pub(crate) enum ResetMode {
    AfterQuery,
    AfterMutation,
}

impl ResetMode {

    pub(crate) fn after_query(&self) -> bool {
        use ResetMode::*;
        match self {
            AfterQuery => true,
            _ => false,
        }
    }

    pub(crate) fn after_mutation(&self) -> bool {
        use ResetMode::*;
        match self {
            AfterMutation => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ResetDatasets {
    Auto,
    Names(Vec<&'static str>),
}