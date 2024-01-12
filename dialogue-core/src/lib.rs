/*!
该crate是dialogue-macro库的核心库，具体的用法请参考[dialogue-macro](https://docs.rs/dialogue-macro/latest/dialogue_macro/)
 */

use dialoguer::entrypoint;
use proc_macro::TokenStream;
mod dialoguer;
pub(crate) mod dialoguer_list;
mod utils;
pub(crate) static mut DIALOGUE_THEME: i32 = 1;

#[proc_macro_derive(
    Asker,
    attributes(input, confirm, password, select, multiselect, theme)
)]
pub fn dialoguer(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    entrypoint(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
