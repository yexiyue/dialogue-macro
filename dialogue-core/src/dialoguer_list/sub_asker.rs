use quote::quote;
use syn::Result;

use crate::utils::get_inner_type;

#[derive(Debug)]
pub struct SubAsker {
    ty: syn::Type,
}

impl SubAsker {
    pub fn from(field: &syn::Field) -> Result<Option<Self>> {
        let mut is_sub_asker = false;
        if let Some(attr) = field
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("asker"))
        {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("SubAsker") {
                    is_sub_asker = true;
                    return Ok(());
                }
                Err(meta.error("expected `SubAsker` or `skip`"))
            })?;
        }
        if !is_sub_asker {
            return Ok(None);
        }
        let ty = get_inner_type(&field.ty, "Option")
            .unwrap_or(&field.ty)
            .clone();

        Ok(Some(Self { ty }))
    }

    pub fn generate_method(&self, field_name: &Option<syn::Ident>) -> proc_macro2::TokenStream {
        let ty = &self.ty;
        quote! {
                pub fn #field_name(&mut self) -> &mut Self{
                    let res=#ty::build();
                    self.#field_name=Some(res);
                    self
                }
        }
    }
}
