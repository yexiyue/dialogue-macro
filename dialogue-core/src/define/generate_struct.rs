use super::DialogueList;
use quote::quote;
use syn::spanned::Spanned;
use syn::{LitStr, Result};
pub fn generate_struct(input: &DialogueList) -> Result<proc_macro2::TokenStream> {
    let res = proc_macro2::TokenStream::new();

    let struct_impl = quote! {
        struct Dialogue{
        
        }
    };
    Ok(res)
}

fn get_names(DialogueList(st): &DialogueList) -> Vec<(String, String)> {
    st.iter()
        .map(|item| {
            (
                item.field_name.to_string(),
                item.ty.clone().unwrap_or("input".to_string()),
            )
        })
        .collect::<Vec<_>>()
}
