use crate::core::conf::test::ResetMode;
use crate::seeder::data_set::DataSet;

pub(crate) struct TestContext {
    pub(crate) reset_mode: ResetMode,
    pub(crate) datasets: Vec<DataSet>,
}