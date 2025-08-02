use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn model(_attrs: TokenStream, input: TokenStream) -> TokenStream {
  input
}
