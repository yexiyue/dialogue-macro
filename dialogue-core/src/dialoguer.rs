use super::dialoguer_list::DialoguerList;
use crate::{
    utils::{get_fields, get_inner_type},
    DIALOGUE_THEME,
};
use quote::quote;
use syn::Result;
use syn::{parse2, DeriveInput};

// 入口函数，用于中间层接受Result
pub fn entrypoint(st: &DeriveInput) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    unsafe {
        DIALOGUE_THEME = get_theme(st)?;
    }
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

    for field in fields {
        let field_name = &field.ident;
        // 生成asker 方法
        let dialogue_list = DialoguerList::parse_field(field)?;
        methods.extend(dialogue_list.generate_method(field_name));

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
        struct #asker_name{
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

// 获取主题信息
fn get_theme(st: &DeriveInput) -> Result<i32> {
    let theme = st.attrs.iter().find(|attr| attr.path().is_ident("theme"));
    if let Some(syn::Attribute {
        meta: syn::Meta::List(syn::MetaList { tokens, .. }),
        ..
    }) = theme
    {
        let ident: syn::Ident = parse2(tokens.clone())?;

        if ident == "simple" {
            return Ok(0);
        } else if ident == "colorful" {
            return Ok(1);
        } else if ident == "colorful_macro" {
            return Ok(2);
        } else {
            return Err(syn::Error::new_spanned(
                ident,
                "only support simple,colorful,colorful_macro",
            ));
        }
    }
    Ok(2)
}
