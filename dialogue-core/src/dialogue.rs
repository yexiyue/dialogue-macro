mod dialogue_type;
use dialogue_type::DialogueType;
mod parse_field_attribute;
use parse_field_attribute::{parse_field_attribute, FieldAttributeOptions, FieldDefault};
use quote::quote;
use syn::{parse2, token::Comma, DeriveInput, Field, Lit, Result};

use crate::DIALOGUE_THEME;

pub fn dialogue_derive(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    let theme = get_theme(&input)?;
    unsafe {
        DIALOGUE_THEME = theme;
    }
    let fields = get_struct(&input.data)?;
    let impl_t = impl_struct(input, fields)?;
    res.extend(impl_t);
    Ok(res)
}

type StructFields = syn::punctuated::Punctuated<Field, Comma>;
/// 判断是否是struct，只支持struct结构体
fn get_struct(data: &syn::Data) -> Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = data
    {
        Ok(named)
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "only support named struct",
        ))
    }
}

/// 为结构体实现方法
fn impl_struct(
    input: &syn::DeriveInput,
    fields: &StructFields,
) -> Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    let (impl_g, type_g, where_case) = input.generics.split_for_impl();
    let impl_method = impl_method(fields)?;
    let res = quote! {
        impl #impl_g #struct_name #type_g #where_case{
            #impl_method
        }
    };
    Ok(res)
}

/// 实现同名方法带前缀get
fn impl_method(fields: &StructFields) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    for field in fields {
        // 获取参数
        let field_attribute_options = parse_field_attribute(field)?;

        // 获取类型
        let path = get_dialogue_type(field)?;
        let field_name = &field.ident;
        let field_type = &field.ty;

        match path.segments.last().unwrap().ident.to_string().as_str() {
            "Vec" => {
                if let Some(FieldAttributeOptions {
                    default,
                    prompt,
                    options,
                    ..
                }) = field_attribute_options
                {
                    let prompt = prompt.as_ref().map(|x| x.as_str());
                    let mut new_options = vec![];
                    for option in options.iter() {
                        if let Lit::Str(y) = option {
                            new_options.push(y.value());
                        } else {
                            return Err(syn::Error::new_spanned(option, "must be string"));
                        }
                    }

                    let mut new_default: Option<Vec<bool>> = None;
                    if let Some(FieldDefault::Vec(p)) = &default {
                        let mut default_vector = Vec::new();
                        default_vector.resize(new_options.len(), false);
                        for i in p {
                            if let Lit::Str(y) = i {
                                let index = new_options
                                    .iter()
                                    .position(|x| x == &y.value())
                                    .ok_or(syn::Error::new_spanned(
                                        y,
                                        format!("{:?} not in options", y.value()),
                                    ))?;
                                default_vector[index] = true;
                            } else {
                                return Err(syn::Error::new_spanned(i, "must be string"));
                            }
                        }
                        new_default = Some(default_vector)
                    }

                    if prompt.is_none() {
                        let temp_input = DialogueType::MultiSelect {
                            prompt,
                            default: new_default,
                            options: new_options.clone(),
                        };
                        let dialogue_type = temp_input.get_dialogue()?;

                        res.extend(quote! {
                            pub fn #field_name(prompt:&str) -> #field_type {
                                let options=[#(#new_options),*];
                                let select_usize=#dialogue_type
                                .with_prompt(prompt)
                                .interact()
                                .unwrap();
                                let selected_items = select_usize
                                    .iter()
                                    .map(|i| options.get(*i).unwrap().clone().to_string())
                                    .collect::<Vec<_>>();
                                selected_items
                            }
                        });
                    } else {
                        let temp_input = DialogueType::MultiSelect {
                            prompt,
                            default: new_default,
                            options: new_options.clone(),
                        };
                        let dialogue_type = temp_input.get_dialogue()?;
                        res.extend(quote! {
                            pub fn #field_name() -> #field_type {
                                let options=[#(#new_options),*];
                                let select_usize=#dialogue_type
                                .interact()
                                .unwrap();
                                let selected_items = select_usize
                                    .iter()
                                    .map(|i| options.get(*i).unwrap().clone().to_string())
                                    .collect::<Vec<_>>();
                                selected_items
                            }
                        });
                    }
                } else {
                    return Err(syn::Error::new_spanned(field, "Vec type must have options"));
                }
            }
            "String" => {
                let mut temp_input = DialogueType::Input {
                    prompt: None,
                    default: None,
                };

                if let Some(FieldAttributeOptions {
                    default,
                    prompt,
                    options,
                    password,
                    confirmation,
                    ..
                }) = &field_attribute_options
                {
                    let prompt = prompt.as_ref().map(|x| x.as_str());
                    if options.len() > 0 {
                        // 如果有options代表的是选择
                        let mut new_options = vec![];
                        for option in options.iter() {
                            if let Lit::Str(y) = option {
                                new_options.push(y.value());
                            } else {
                                return Err(syn::Error::new_spanned(option, "must be string"));
                            }
                        }

                        let mut new_default: Option<usize> = None;
                        if let Some(FieldDefault::Lit(Lit::Int(y))) = &default {
                            new_default = Some(y.base10_parse()?);
                        } else if let Some(FieldDefault::Lit(Lit::Str(y))) = &default {
                            let idx = new_options.iter().position(|x| x == &y.value()).ok_or(
                                syn::Error::new_spanned(
                                    y,
                                    format!("{:?} not in options", y.value()),
                                ),
                            )?;
                            new_default = Some(idx);
                        }
                        if prompt.is_none() {
                            let temp_input = DialogueType::Select {
                                prompt,
                                default: new_default,
                                options: new_options.clone(),
                            };
                            let dialogue_type = temp_input.get_dialogue()?;
                            res.extend(quote! {
                                pub fn #field_name(prompt:&str) -> #field_type {
                                    let options=[#(#new_options),*];
                                    let select_usize=#dialogue_type
                                    .with_prompt(prompt)
                                    .interact()
                                    .unwrap();
                                    options[select_usize].to_string()
                                }
                            });
                        } else {
                            let temp_input = DialogueType::Select {
                                prompt,
                                default: new_default,
                                options: new_options.clone(),
                            };
                            let dialogue_type = temp_input.get_dialogue()?;
                            res.extend(quote! {
                                pub fn #field_name() -> #field_type {
                                    let options=[#(#new_options),*];
                                    let select_usize=#dialogue_type
                                    .interact()
                                    .unwrap();
                                    options[select_usize].to_string()
                                }
                            });
                        }
                    } else if password.is_some() && password.unwrap() {
                        let confirm_prompt = confirmation
                            .as_ref()
                            .map(|p| ("Confirm password", p.as_str()));
                        let temp_input = DialogueType::Password {
                            prompt,
                            confirmation: confirm_prompt,
                        };

                        let dialogue_type = temp_input.get_dialogue()?;
                        res.extend(quote! {
                            pub fn #field_name() ->#field_type  {
                                #dialogue_type
                                .interact()
                                .unwrap()
                            }
                        });
                    } else {
                        // 没有options也是输入
                        let default = default.as_ref().and_then(|x| {
                            if let FieldDefault::Lit(Lit::Str(y)) = x {
                                Some(y.value())
                            } else {
                                None
                            }
                        });

                        temp_input = DialogueType::Input { prompt, default };
                        let dialogue_type = temp_input.get_dialogue()?;
                        res.extend(quote! {
                            pub fn #field_name() ->#field_type  {
                                #dialogue_type
                                .interact()
                                .unwrap()
                            }
                        });
                    }
                } else {
                    // 如果没有attribute 默认是输入
                    let dialogue_type = temp_input.get_dialogue()?;
                    res.extend(quote! {
                        pub fn #field_name(prompt:&str) ->#field_type  {
                            #dialogue_type
                            .with_prompt(prompt)
                            .interact()
                            .unwrap()
                        }
                    });
                }
            }
            "bool" => {
                let mut temp_input = DialogueType::Confirm {
                    prompt: None,
                    default: None,
                };

                if let Some(FieldAttributeOptions {
                    default, prompt, ..
                }) = &field_attribute_options
                {
                    let prompt = prompt.as_ref().map(|x| x.as_str());
                    let default = default.as_ref().map(|x| {
                        if let FieldDefault::Lit(Lit::Bool(y)) = x {
                            y.value()
                        } else {
                            false
                        }
                    });
                    temp_input = DialogueType::Confirm { prompt, default };
                    let dialogue_type = temp_input.get_dialogue()?;
                    res.extend(quote! {
                        pub fn #field_name() ->#field_type  {
                            #dialogue_type
                            .interact()
                            .unwrap()
                        }
                    });
                } else {
                    let dialogue_type = temp_input.get_dialogue()?;
                    res.extend(quote! {
                        pub fn #field_name(prompt:&str) ->#field_type  {
                            #dialogue_type
                            .with_prompt(prompt)
                            .interact()
                            .unwrap()
                        }
                    });
                }
            }
            "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "isize" | "usize" => {
                if let Some(FieldAttributeOptions {
                    prompt, default, ..
                }) = field_attribute_options
                {
                    let prompt = prompt.as_ref().map(|x| x.as_str());
                    let default = default.and_then(|d| {
                        if let FieldDefault::Lit(y) = d {
                            Some(y)
                        } else {
                            None
                        }
                    });
                    let temp_number = DialogueType::Number { prompt, default };
                    if prompt.is_some() {
                        let dialogue_type = temp_number.get_dialogue()?;
                        res.extend(quote! {
                            pub fn #field_name() ->#field_type  {
                                #dialogue_type
                                .interact()
                                .unwrap()
                            }
                        });
                    } else {
                        let dialogue_type = temp_number.get_dialogue()?;
                        res.extend(quote! {
                            pub fn #field_name(prompt:&str) ->#field_type  {
                                #dialogue_type
                                .with_prompt(prompt)
                                .interact()
                                .unwrap()
                            }
                        });
                    }
                } else {
                    // 如果没有attribute 默认是输入
                    let temp_number = DialogueType::Number {
                        prompt: None,
                        default: None,
                    };
                    let dialogue_type = temp_number.get_dialogue()?;
                    res.extend(quote! {
                        pub fn #field_name(prompt:&str) ->#field_type  {
                            #dialogue_type
                            .with_prompt(prompt)
                            .interact()
                            .unwrap()
                        }
                    });
                }
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    field,
                    format!("don't support {:?}", field.ty),
                ));
            }
        }
    }
    Ok(res)
}

/// 获取字段类型
fn get_dialogue_type(field: &syn::Field) -> Result<&syn::Path> {
    if let syn::Type::Path(syn::TypePath { path, .. }) = &field.ty {
        Ok(path)
    } else {
        Err(syn::Error::new_spanned(field, "only support Path type"))
    }
}

/// 获取主题信息
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
