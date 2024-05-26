use super::ParseFieldAttr;
use quote::quote;
use syn::{ExprLit, Generics, Lit, Result};

#[derive(Debug)]
pub struct Input {
    pub prompt: Option<String>,
    pub default: Option<ExprLit>,
    pub with_default: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            prompt: None,
            default: None,
            with_default: false,
        }
    }
}

impl ParseFieldAttr for Input {
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
                let value = meta.input.parse::<syn::ExprLit>()?;
                res.default = Some(value);
                return Ok(());
            }

            if meta.path.is_ident("with_default") {
                meta.value()?;
                let value = meta.input.parse::<syn::LitBool>()?;
                res.with_default = value.value();
                return Ok(());
            }

            Err(meta.error("expected `prompt` , `default` , `with_default`"))
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
        // 设置主题
        body.extend(quote! {
            let res=dialogue_macro::dialoguer::Input::with_theme(#theme)
        });

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
            if let Some(ExprLit {
                lit: Lit::Str(_), ..
            }) = default
            {
                body.extend(quote!(
                    .default(#default.parse().unwrap())
                ))
            } else {
                body.extend(quote!(
                    .default(#default)
                ))
            }
        }
        // 使用范型处理input 默认值
        let mut generate = Generics::default();

        if *with_default {
            generate.params.push(syn::parse_quote!(T));
            generate.make_where_clause();
            generate
                .where_clause
                .as_mut()
                .unwrap()
                .predicates
                .push(syn::parse_quote!(T:Into<#inner_ty> + std::clone::Clone + std::fmt::Display));

            params.extend(quote! {
                default: T,
            });

            body.extend(quote!(
                .default(default.into())
            ))
        }
        let (_, type_g, where_g) = generate.split_for_impl();
        Ok(quote! {

            pub fn #field_name #type_g(&mut self,#params) -> &mut Self
                #where_g
            {
                #body.interact().unwrap();
                self.#field_name=Some(res);
                self
            }
        })
    }
}
