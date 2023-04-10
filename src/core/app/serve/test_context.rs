use crate::seeder::data_set::DataSet;

pub(crate) enum ResetMode {
    AfterQuery,
    AfterMutation,
}

pub(crate) struct TestContext {
    pub(crate) reset_mode: ResetMode,
    pub(crate) datasets: Vec<DataSet>,
}