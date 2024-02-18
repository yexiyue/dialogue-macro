use super::ParseFieldAttr;
use quote::quote;
use syn::Result;

#[derive(Debug, Default)]
pub struct Confirm {
    pub prompt: Option<String>,
    pub default: Option<bool>,
    pub with_default: bool,
}

impl ParseFieldAttr for Confirm {
    fn parse_field_attr(attr: &syn::Attribute) -> Result<Self> {
        let mut res = Self {
            prompt: None,
            default: None,
            with_default: false,
        };
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("prompt") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitStr>()?;
                res.prompt = Some(value.value());
                return Ok(());
            }

            if meta.path.is_ident("default") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitBool>()?;
                res.default = Some(value.value());
                return Ok(());
            }

            if meta.path.is_ident("with_default") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitBool>()?;
                res.with_default = value.value();
                return Ok(());
            }
            Err(meta.error("expected `prompt` or `default`"))
        })?;
        Ok(res)
    }

    fn generate_method(
        &self,
        field_name: &Option<syn::Ident>,
        _inner_ty: Option<&syn::Type>,
    ) -> Result<proc_macro2::TokenStream> {
        let mut body = proc_macro2::TokenStream::new();
        let mut params = proc_macro2::TokenStream::new();
        if let Some(theme) = Self::get_theme() {
            body.extend(quote! {
                let res=dialogue_macro::dialoguer::Confirm::with_theme(#theme)
            });
        } else {
            body.extend(quote! {
                let res=dialogue_macro::dialoguer::Confirm::new()
            });
        }
        let Self {
            prompt,
            default,
            with_default,
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

        if default.is_some() {
            body.extend(quote!(
                .default(#default)
            ))
        }

        if *with_default {
            params.extend(quote! {
                default: bool,
            });
            body.extend(quote!(
                .default(default)
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
