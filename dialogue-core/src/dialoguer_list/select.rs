use super::ParseFieldAttr;
use quote::quote;
use syn::{ExprArray, Result};

#[derive(Debug, Default)]
pub struct Select {
    pub prompt: Option<String>,
    pub default: Option<usize>,
    pub options: Option<ExprArray>,
    pub with_default: bool,
}

impl ParseFieldAttr for Select {
    fn parse_field_attr(attr: &syn::Attribute) -> Result<Self> {
        let mut res = Self {
            prompt: None,
            default: None,
            options: None,
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
                let value = meta.input.parse::<syn::LitInt>()?;
                res.default = Some(value.base10_parse()?);
                return Ok(());
            }

            if meta.path.is_ident("options") {
                meta.value()?;
                let value = meta.input.parse::<syn::ExprArray>()?;
                res.options = Some(value);
                return Ok(());
            }

            if meta.path.is_ident("with_default") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitBool>()?;
                res.with_default = value.value();
                return Ok(());
            }
            Err(meta.error("expected `prompt` , `default` or  `options`"))
        })?;
        Ok(res)
    }

    fn generate_method(
        &self,
        theme: &proc_macro2::TokenStream,
        field_name: &Option<syn::Ident>,
        inner_ty: Option<&syn::Type>,
    ) -> Result<proc_macro2::TokenStream> {
        let mut body = proc_macro2::TokenStream::new();
        let mut params = proc_macro2::TokenStream::new();
        let mut gen_options = proc_macro2::TokenStream::new();
        // 设置主题
        body.extend(quote! {
            let res=dialogue_macro::dialoguer::Select::with_theme(#theme)
        });
        let Self {
            prompt,
            default,
            options,
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

        if options.is_some() {
            gen_options.extend(quote!(
                let options=&vec!#options;
            ));
        } else {
            params.extend(quote! {
                options: &[#inner_ty],
            });
        }

        if default.is_some() {
            body.extend(quote!(
                .default(#default)
            ))
        }

        if *with_default {
            params.extend(quote! {
                default: usize,
            });
            body.extend(quote!(
                .default(default)
            ))
        }

        body.extend(quote!(
            .items(options)
        ));

        Ok(quote! {
            pub fn #field_name(&mut self,#params) -> &mut Self{
                #gen_options
                #body.interact().unwrap();
                self.#field_name=Some(options[res].clone().into());
                self
            }
        })
    }
}
