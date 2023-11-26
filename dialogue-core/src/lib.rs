#![allow(unused)]

mod dialogue;
use proc_macro::TokenStream;

#[proc_macro_derive(Dialogue, attributes(dialogue))]
pub fn dialogue_derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    dialogue::dialogue_derive(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
