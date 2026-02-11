use proc_macro2::TokenStream;
use syn::{Result, Type};

pub(in crate::entity) trait ExtendedColumnType {
    fn default_column_type(ty: &Type) -> Result<TokenStream>;
    fn to_token_stream(&self) -> TokenStream;
}
