use super::DialogueList;
use quote::quote;
use syn::Result;
pub fn generate_struct(input: &DialogueList) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    let names = get_names(input);
    let (fields, where_case) = generate_field(&names)?;
    let struct_dialogue = quote! {
        #[derive(Debug,Clone)]
        pub struct Dialogue where #where_case
        {
            #fields
        }
    };
    let struct_impl = generate_impl(&input)?;
    res.extend(quote!(
        #struct_dialogue
        impl Dialogue where #where_case{
            #struct_impl
        }
    ));
    Ok(res)
}

fn get_names(DialogueList(st): &DialogueList) -> Vec<(&syn::Ident, String, Option<syn::Type>)> {
    st.iter()
        .map(|item| {
            (
                &item.field_name,
                item.ty.clone().unwrap_or("input".to_string()),
                item.generic.clone(),
            )
        })
        .collect::<Vec<_>>()
}

fn generate_field(
    names: &Vec<(&syn::Ident, String, Option<syn::Type>)>,
) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let mut where_case = proc_macro2::TokenStream::new();
    let mut res = proc_macro2::TokenStream::new();
    for item in names {
        let dialogue_type = item.1.as_str();
        let field_name = item.0;
        let field_type = item.2.clone();
        match dialogue_type {
            "input" => {
                if let Some(syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                })) = &item.2
                {
                    let last_ident = segments.last().unwrap().ident.to_string();
                    match last_ident.as_str() {
                        "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "isize" | "usize"
                        | "String" => res.extend(quote!(
                            pub #field_name: std::option::Option<#field_type>,
                        )),
                        _ => {
                            return Err(syn::Error::new(
                                item.0.span(),
                                format!("{} is not supported", last_ident),
                            ));
                        }
                    }
                } else {
                    res.extend(quote!(
                        pub #field_name: std::option::Option<String>,
                    ))
                }
            }
            "confirm" => {
                if let Some(syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                })) = &item.2
                {
                    let last_ident = segments.last().unwrap().ident.to_string();
                    match last_ident.as_str() {
                        "bool" => res.extend(quote!(
                            pub #field_name: std::option::Option<#field_type>,
                        )),
                        _ => {
                            return Err(syn::Error::new(
                                item.0.span(),
                                format!("{} is not supported", last_ident),
                            ));
                        }
                    }
                } else {
                    res.extend(quote!(
                        pub #field_name: std::option::Option<bool>,
                    ))
                }
            }
            "select"=> {
                if let Some(syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                })) = &item.2
                {
                    let last_ident = segments.last().unwrap().ident.to_string();
                    match last_ident.as_str() {
                        "String" => res.extend(quote!(
                            pub #field_name: std::option::Option<#field_type>,
                        )),
                        _ => {
                            res.extend(quote!(
                                pub #field_name: std::option::Option<#field_type>,
                            ));
                            where_case.extend(quote!(
                                #field_type: ToString + std::fmt::Debug,
                            ))
                        }
                    }
                } else {
                    res.extend(quote!(
                        pub #field_name: std::option::Option<String>,
                    ))
                }
            }
             "multiselect" => {
                if let Some(syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                })) = &item.2
                {
                    let last_ident = segments.last().unwrap().ident.to_string();
                    match last_ident.as_str() {
                        "String" => res.extend(quote!(
                            pub #field_name: std::option::Option<Vec<#field_type>>,
                        )),
                        _ => {
                            res.extend(quote!(
                                pub #field_name: std::option::Option<Vec<#field_type>>,
                            ));
                            where_case.extend(quote!(
                                #field_type: ToString + std::fmt::Debug,
                            ))
                        }
                    }
                } else {
                    res.extend(quote!(
                        pub #field_name: std::option::Option<Vec<String>>,
                    ))
                }
            }
            "password" => {
                if let Some(syn::Type::Path(syn::TypePath {
                    path: syn::Path { segments, .. },
                    ..
                })) = &item.2
                {
                    let last_ident = segments.last().unwrap().ident.to_string();
                    match last_ident.as_str() {
                        "String" => res.extend(quote!(
                            pub #field_name: std::option::Option<#field_type>,
                        )),
                        _ => {
                            return Err(syn::Error::new(
                                item.0.span(),
                                format!("{} is not supported", last_ident),
                            ));
                        }
                    }
                } else {
                    res.extend(quote!(
                        pub #field_name: std::option::Option<String>,
                    ))
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    Ok((res, where_case))
}

fn generate_impl(DialogueList(st): &DialogueList) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    let field_names = st
        .iter()
        .map(|item| &item.field_name)
        .collect::<Vec<&syn::Ident>>();
    res.extend(quote!(
        pub fn new()->Self{
            Self{
                #(#field_names:None),*
            }
        }
    ));
    for dialogue_item in st.iter() {
        res.extend(dialogue_item.to_tokens())
    }
    Ok(res)
}
