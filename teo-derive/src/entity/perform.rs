use proc_macro::TokenStream;
use darling::FromDeriveInput;
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Error};
use crate::entity::types::EntityDef;

pub(crate) fn perform(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _opts = match EntityDef::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(err) => return Error::from(err).to_compile_error().into()
    };
    let output = quote! {

    };
    output.into()
}

// fn _perform(input: TokenStream) -> Result<TokenStream2, Error> {

//     let Ok(input) = parse::<ItemStruct>(input) else {
//         Err(Error::new(
//             Span::call_site(),
//             "`Entity` can only be applied to struct.",
//         ))?
//     };
//     let new_struct = quote! {
//         struct NewStruct { }
//     };
//     Ok(quote! { #new_struct })
// }
