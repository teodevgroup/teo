mod entity;
mod enum_type;
mod schema;
pub(crate) mod utils;

use proc_macro::TokenStream;

#[proc_macro_derive(Entity, attributes(teo))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    entity::perform(input)
}

#[proc_macro_derive(Schema, attributes(teo))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    schema::perform(input)
}
