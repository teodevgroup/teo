mod entity;
mod enum_type;
pub(crate) mod utils;

use proc_macro::TokenStream;

#[proc_macro_derive(Entity, attributes(teo))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    entity::perform(input)
}
