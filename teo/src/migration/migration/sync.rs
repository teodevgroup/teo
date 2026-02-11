use crate::types::Schema;

pub(crate) trait SyncMigration {

    type Err;

    fn migrate<S>(&self) -> Result<(), Self::Err> where S: Schema;

    fn execute_without_params(&self, q: &str) -> Result<(), Self::Err>;

    fn ident_quote_char() -> &'static str;

    fn string_quote_char() -> &'static str;

}
