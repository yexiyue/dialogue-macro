mod dialogue;
use define::generate_struct;
use proc_macro::TokenStream;
mod define;

pub(crate) static mut DIALOGUE_THEME: i32 = 1;
#[proc_macro_derive(Dialogue, attributes(dialogue, theme))]
pub fn dialogue_derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    dialogue::dialogue_derive(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro]
pub fn dialogue_define(input: TokenStream) -> TokenStream {
    let dialogue_list = syn::parse_macro_input!(input as define::DialogueList);
    generate_struct::generate_struct(&dialogue_list)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
