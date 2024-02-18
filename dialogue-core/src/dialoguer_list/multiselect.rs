use super::ParseFieldAttr;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, token::Comma, ExprArray, LitInt, Result};

#[derive(Debug, Default)]
pub struct MultiSelect {
    pub prompt: Option<String>,
    pub default: Option<Punctuated<LitInt, Comma>>,
    pub options: Option<ExprArray>,
    pub with_default: bool,
}

impl ParseFieldAttr for MultiSelect {
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
                let content;
                syn::bracketed!(content in meta.input);
                let values = content.parse_terminated(syn::LitInt::parse, Comma)?;
                res.default = Some(values);
                return Ok(());
            }

            if meta.path.is_ident("options") {
                meta.value()?;
                let value = meta.input.parse::<ExprArray>()?;
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
        field_name: &Option<syn::Ident>,
        inner_ty: Option<&syn::Type>,
    ) -> Result<proc_macro2::TokenStream> {
        let mut body = proc_macro2::TokenStream::new();
        let mut params = proc_macro2::TokenStream::new();
        let mut gen_options = proc_macro2::TokenStream::new();
        let mut gen_default = proc_macro2::TokenStream::new();
        // 设置主题
        if let Some(theme) = Self::get_theme() {
            body.extend(quote! {
                let res=dialogue_macro::dialoguer::MultiSelect::with_theme(#theme)
            });
        } else {
            body.extend(quote! {
                let res=dialogue_macro::dialoguer::MultiSelect::new()
            });
        }
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
        body.extend(quote!(
            .items(options)
        ));

        if default.is_some() {
            gen_default.extend(quote!(
                let mut default=vec![false;options.len()];
                let temp=vec![#default];
                for i in 0..temp.len(){
                    default[temp[i]]=true;
                }
            ));
            body.extend(quote!(
                .defaults(&default)
            ))
        }

        if *with_default {
            params.extend(quote! {
                defaults: &[bool],
            });
            body.extend(quote!(
                .defaults(&defaults)
            ))
        }

        Ok(quote! {
            pub fn #field_name(&mut self,#params) -> &mut Self{
                #gen_options
                #gen_default
                #body.interact().unwrap();
                let res=res.iter().map(|x|options[*x].clone().into()).collect::<Vec<#inner_ty>>();
                self.#field_name=res;
                self
            }
        })
    }
}
