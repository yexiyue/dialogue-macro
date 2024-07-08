use super::ParseFieldAttr;
use quote::quote;
use syn::{ parse_quote, ExprClosure, Pat, Result, ReturnType };

pub struct Password {
    pub prompt: Option<String>,
    pub confirmation: Option<String>,
    pub mismatch: Option<String>,
    pub validate_with: Option<ExprClosure>,
}

impl ParseFieldAttr for Password {
    fn parse_field_attr(attr: &syn::Attribute, inner_ty: Option<&syn::Type>) -> Result<Self> {
        let mut res = Self {
            prompt: None,
            confirmation: None,
            mismatch: None,
            validate_with: None,
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

            if meta.path.is_ident("validate_with") {
                meta.value()?;

                let mut value = meta.input.parse::<ExprClosure>()?;
                // 如果没有添加返回类型就添加默认的
                if let ReturnType::Default = value.output {
                    value.output = parse_quote!(->std::result::Result<(),&str>);
                }
                let arg = value.inputs.iter_mut().next().unwrap();
                if let Pat::Ident(i) = arg {
                    let pat: syn::PatType = parse_quote!(#i:&#inner_ty);
                    let _ = std::mem::replace(arg, Pat::Type(pat));
                }

                res.validate_with = Some(value);
                return Ok(());
            }
            Err(meta.error("expected `prompt` , `confirmation` , `mismatch` , `validate_with`"))
        })?;
        Ok(res)
    }

    fn generate_method(
        &self,
        theme: &proc_macro2::TokenStream,
        field_name: &Option<syn::Ident>,
        _inner_ty: Option<&syn::Type>
    ) -> Result<proc_macro2::TokenStream> {
        let mut body = proc_macro2::TokenStream::new();
        let mut params = proc_macro2::TokenStream::new();

        body.extend(
            quote! {
            let res=dialogue_macro::dialoguer::Password::with_theme(#theme)
        }
        );

        let Self { prompt, confirmation, mismatch, validate_with } = self;
        if self.prompt.is_some() {
            body.extend(quote!(
                .with_prompt(#prompt)
            ));
        } else {
            params.extend(quote! {
                prompt: &str,
            });
            body.extend(quote!(
                .with_prompt(prompt)
            ));
        }

        if confirmation.is_some() && mismatch.is_some() {
            body.extend(
                quote!(
                .with_confirmation(#confirmation, #mismatch)
            )
            );
        } else if confirmation.is_some() {
            body.extend(
                quote!(
                .with_confirmation(#confirmation,"The passwords entered do not match!")
            )
            );
        }

        if validate_with.is_some() {
            body.extend(quote!(
                .validate_with(#validate_with)
            ));
        }

        Ok(
            quote! {
            pub fn #field_name(&mut self,#params) -> &mut Self{
                #body.interact().unwrap();
                self.#field_name=Some(res);
                self
            }
        }
        )
    }
}
