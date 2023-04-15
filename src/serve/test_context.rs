use crate::seeder::data_set::DataSet;

pub(crate) enum ResetMode {
    AfterQuery,
    AfterMutation,
}

impl ResetMode {
    pub(crate) fn is_after_query(&self) -> bool {
        match self {
            ResetMode::AfterQuery => true,
            _ => false,
        }
    }

    pub(crate) fn is_after_mutation(&self) -> bool {
        match self {
            ResetMode::AfterMutation => true,
            _ => false,
        }
    }
}

pub(crate) struct TestContext {
    pub(crate) reset_mode: ResetMode,
    pub(crate) datasets: Vec<DataSet>,
}