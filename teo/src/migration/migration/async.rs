use crate::types::Schema;

pub(crate) trait AsyncMigration {

    type Err;

    fn migrate<S>(&self) -> impl Future<Output = Result<(), Self::Err>> + Send where S: Schema;

    fn execute_without_params(&self, q: &str) -> impl Future<Output = Result<(), Self::Err>>;

    fn ident_quote_char() -> &'static str;

    fn string_quote_char() -> &'static str;
}
