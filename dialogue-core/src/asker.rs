use super::dialoguer_list::DialoguerList;
use crate::utils::{get_fields, get_inner_type};
use quote::quote;
use syn::{parse_str, LitStr, Result};
use syn::{DeriveInput, Type};

// 入口函数，用于中间层接受Result
pub fn entrypoint(st: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    res.extend(generate_asker(st)?);
    Ok(res)
}

// 生成asker 结构体
fn generate_asker(st: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let mut asker_fields = proc_macro2::TokenStream::new();
    let mut asker_fields_init = proc_macro2::TokenStream::new();
    let mut finish_clone_fields = proc_macro2::TokenStream::new();

    let fields = get_fields(st)?;
    let st_name = st.ident.to_string();

    let asker_name = syn::Ident::new(&format!("{}Asker", st_name), st.ident.span());
    let mut methods = proc_macro2::TokenStream::new();
    let theme = get_theme(st)?;
    for field in fields {
        let field_name = &field.ident;
        // 生成asker 方法
        let dialogue_list = DialoguerList::parse_field(field)?;
        let skip = get_skip(&field)?;

        if !skip {
            let method = dialogue_list.generate_method(&theme, field_name)?;
            methods.extend(method);
        }

        if let Some(ty) = get_inner_type(&field.ty, "Option") {
            asker_fields_init.extend(quote!(
                #field_name:std::option::Option::None,
            ));
            asker_fields.extend(quote!(
                #field_name:std::option::Option<#ty>,
            ));
            finish_clone_fields.extend(quote!(
                #field_name:self.#field_name.clone(),
            ));
        } else if let Some(ty) = get_inner_type(&field.ty, "Vec") {
            asker_fields_init.extend(quote!(
                #field_name:std::vec::Vec::new(),
            ));
            asker_fields.extend(quote!(
                #field_name:std::vec::Vec<#ty>,
            ));
            finish_clone_fields.extend(quote!(
                #field_name:self.#field_name.clone(),
            ));
        } else {
            let ty = &field.ty;
            asker_fields_init.extend(quote!(
                #field_name:std::option::Option::None,
            ));
            asker_fields.extend(quote!(
                #field_name:std::option::Option<#ty>,
            ));
            finish_clone_fields.extend(quote!(
                #field_name:self.#field_name.clone().expect(&format!("{} is not set",stringify!(#field_name))),
            ));
        }
    }
    let st_name = &st.ident;
    Ok(quote! {
        impl #st_name{
            pub fn asker()-> #asker_name{
                #asker_name{
                    #asker_fields_init
                }
            }
        }
        pub struct #asker_name{
            #asker_fields
        }

        impl #asker_name{
            pub fn finish(&self) -> #st_name{
                #st_name{
                    #finish_clone_fields
                }
            }

            #methods
        }
    })
}

fn get_skip(field: &syn::Field) -> Result<bool> {
    let skip = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("asker"));
    let mut res = false;
    if let Some(attr) = skip {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                res = true;
                return Ok(());
            }
            Err(meta.error("only support skip"))
        })?;
    }

    if res {
        if get_inner_type(&field.ty, "Option").is_none()
            && get_inner_type(&field.ty, "Vec").is_none()
        {
            return Err(syn::Error::new_spanned(
                field.ty.clone(),
                "asker(skip) can only be used on Option or Vec",
            ));
        }
    }
    Ok(res)
}

fn get_theme(st: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let mut res = quote!();
    if let Some(attr) = st.attrs.iter().find(|attr| attr.path().is_ident("asker")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("theme") {
                meta.value()?;
                let v = meta.input.parse::<LitStr>()?;
                let path: Type = parse_str(&v.value())?;
                res = quote!(&#path::default());
                return Ok(());
            }
            return Err(meta.error("expect `theme`"));
        })?;
    } else {
        res = quote!(&dialogue_macro::ColorfulTheme::default())
    }
    Ok(res)
}
