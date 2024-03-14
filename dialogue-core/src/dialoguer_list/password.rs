use super::ParseFieldAttr;
use quote::quote;
use syn::Result;

pub struct Password {
    pub prompt: Option<String>,
    pub confirmation: Option<String>,
    pub mismatch: Option<String>,
}

impl ParseFieldAttr for Password {
    fn parse_field_attr(attr: &syn::Attribute) -> Result<Self> {
        let mut res = Self {
            prompt: None,
            confirmation: None,
            mismatch: None,
        };
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("prompt") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitStr>()?;
                res.prompt = Some(value.value());
                return Ok(());
            }
            if meta.path.is_ident("confirmation") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitStr>()?;
                res.confirmation = Some(value.value());
                return Ok(());
            }
            if meta.path.is_ident("mismatch") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitStr>()?;
                res.mismatch = Some(value.value());
                return Ok(());
            }
            Err(meta.error("expected `prompt` , `confirmation` or `mismatch`"))
        })?;
        Ok(res)
    }

    fn generate_method(
        &self,
        theme: &proc_macro2::TokenStream,
        field_name: &Option<syn::Ident>,
        _inner_ty: Option<&syn::Type>,
    ) -> Result<proc_macro2::TokenStream> {
        let mut body = proc_macro2::TokenStream::new();
        let mut params = proc_macro2::TokenStream::new();

        body.extend(quote! {
            let res=dialogue_macro::dialoguer::Password::with_theme(#theme)
        });

        let Self {
            prompt,
            confirmation,
            mismatch,
        } = self;
        if self.prompt.is_some() {
            body.extend(quote!(
                .with_prompt(#prompt)
            ))
        } else {
            params.extend(quote! {
                prompt: &str,
            });
            body.extend(quote!(
                .with_prompt(prompt)
            ))
        }

        if confirmation.is_some() && mismatch.is_some() {
            body.extend(quote!(
                .with_confirmation(#confirmation, #mismatch)
            ))
        } else if confirmation.is_some() {
            body.extend(quote!(
                .with_confirmation(#confirmation,"The passwords entered do not match!")
            ))
        }

        Ok(quote! {
            pub fn #field_name(&mut self,#params) -> &mut Self{
                #body.interact().unwrap();
                self.#field_name=Some(res);
                self
            }
        })
    }
}
