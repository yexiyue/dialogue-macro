use quote::quote;
use std::{default, str::FromStr};
use syn::{ext::IdentExt, punctuated::Punctuated, token::Comma, Field, Lit, Result, Token};

enum DialogueType<'a> {
    Confirm {
        prompt: Option<&'a str>,
        default: Option<bool>,
    },
    Input {
        prompt: Option<&'a str>,
        default: Option<String>,
    },
    Password {
        prompt: Option<&'a str>,
        confirmation: Option<(&'a str, &'a str)>,
    },
    Select {
        prompt: Option<&'a str>,
        default: Option<usize>,
        options: Vec<String>,
    },
    MultiSelect {
        prompt: Option<&'a str>,
        default: Option<Vec<bool>>,
        options: Vec<String>,
    },
}

impl<'a> DialogueType<'a> {
    fn get_dialogue(&self) -> proc_macro2::TokenStream {
        let mut res = proc_macro2::TokenStream::new();
        match self {
            DialogueType::Confirm { prompt, default } => {
                res.extend(quote!(dialoguer::Confirm::with_theme(
                    &dialoguer::theme::ColorfulTheme::default()
                )));
                if let Some(default) = default {
                    res.extend(quote!(
                        .default(#default)
                    ))
                }
                if prompt.is_some() {
                    res.extend(quote!(
                        .with_prompt(#prompt)
                    ))
                }
            }
            DialogueType::Input { prompt, default } => {
                res.extend(quote!(dialoguer::Input::with_theme(
                    &dialoguer::theme::ColorfulTheme::default()
                )));
                if let Some(default) = default {
                    res.extend(quote!(
                       .default(#default.parse().unwrap())
                    ))
                }
                if prompt.is_some() {
                    res.extend(quote!(
                        .with_prompt(#prompt)
                    ))
                }
            }
            DialogueType::Password {
                prompt,
                confirmation,
            } => {
                res.extend(quote!(dialoguer::Password::with_theme(
                    &dialoguer::theme::ColorfulTheme::default()
                )));
                if prompt.is_some() {
                    res.extend(quote!(
                        .with_prompt(#prompt)
                    ))
                };
                if let Some((title, other_prompt)) = confirmation {
                    res.extend(quote!(
                        .with_confirmation(#title, #other_prompt)
                    ))
                }
            }
            DialogueType::Select {
                prompt,
                default,
                options,
            } => {
                res.extend(quote!(dialoguer::Select::with_theme(
                    &dialoguer::theme::ColorfulTheme::default()
                )));
                if let Some(default) = default {
                    res.extend(quote!(
                       .default(#default)
                    ))
                }
                res.extend(quote!(#(.item(#options))*));
                if prompt.is_some() {
                    res.extend(quote!(
                        .with_prompt(#prompt)
                    ))
                }
            }
            DialogueType::MultiSelect {
                prompt,
                default,
                options,
            } => {
                res.extend(quote!(dialoguer::MultiSelect::with_theme(
                    &dialoguer::theme::ColorfulTheme::default()
                )));
                res.extend(quote!(#(.item(#options))*));
                if let Some(default) = default {
                    res.extend(quote!(
                       .defaults(&[#(#default),*])
                    ))
                }
                if prompt.is_some() {
                    res.extend(quote!(
                        .with_prompt(#prompt)
                    ))
                }
            }
        }
        res
    }
}

pub(crate) fn dialogue_derive(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
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
        if path.segments.last().unwrap().ident == "Vec" {
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
                            if let Ok(index) = new_options.binary_search(&y.value()) {
                                default_vector[index] = true;
                            }
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
                    let dialogue_type = temp_input.get_dialogue();

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
                    let dialogue_type = temp_input.get_dialogue();
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
        } else if path.segments.last().unwrap().ident == "bool" {
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
                let dialogue_type = temp_input.get_dialogue();
                res.extend(quote! {
                    pub fn #field_name() ->#field_type  {
                        #dialogue_type
                        .interact()
                        .unwrap()
                    }
                });
            } else {
                let dialogue_type = temp_input.get_dialogue();
                res.extend(quote! {
                    pub fn #field_name(prompt:&str) ->#field_type  {
                        #dialogue_type
                        .with_prompt(prompt)
                        .interact()
                        .unwrap()
                    }
                });
            }
        } else if path.segments.last().unwrap().ident == "String" {
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
                        let idx = new_options.binary_search(&y.value()).map_err(|e| {
                            syn::Error::new_spanned(y, format!("{:?} not in options", y.value()))
                        })?;
                        new_default = Some(idx);
                    }
                    if prompt.is_none() {
                        let temp_input = DialogueType::Select {
                            prompt,
                            default: new_default,
                            options: new_options.clone(),
                        };
                        let dialogue_type = temp_input.get_dialogue();
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
                        let dialogue_type = temp_input.get_dialogue();
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
                    let mut temp_input = DialogueType::Password {
                        prompt,
                        confirmation: confirm_prompt,
                    };

                    let dialogue_type = temp_input.get_dialogue();
                    res.extend(quote! {
                        pub fn #field_name() ->#field_type  {
                            #dialogue_type
                            .interact()
                            .unwrap()
                        }
                    });
                } else {
                    // 没有options也是输入
                    let default = default
                        .as_ref()
                        .map(|x| {
                            if let FieldDefault::Lit(Lit::Str(y)) = x {
                                Some(y.value())
                            } else {
                                None
                            }
                        })
                        .unwrap();

                    temp_input = DialogueType::Input { prompt, default };
                    let dialogue_type = temp_input.get_dialogue();
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
                let dialogue_type = temp_input.get_dialogue();
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
            return Err(syn::Error::new_spanned(
                field,
                format!("don't support {:?}", field.ty),
            ));
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

#[derive(Debug)]
enum FieldDefault {
    Lit(Lit),
    Vec(Punctuated<Lit, Comma>),
}

/// 解析字段属性
#[derive(Debug)]
struct FieldAttributeOptions {
    default: Option<FieldDefault>,
    confirmation: Option<String>,
    prompt: Option<String>,
    password: Option<bool>,
    options: Punctuated<Lit, Comma>,
}
static WHITE_LIST: [&str; 5] = ["default", "prompt", "options", "password", "confirmation"];

/// 实现parse trait
impl syn::parse::Parse for FieldAttributeOptions {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut res = FieldAttributeOptions {
            default: None,
            prompt: None,
            confirmation: None,
            password: None,
            options: Punctuated::new(),
        };
        while input.peek(syn::Ident::peek_any) {
            let ident = input.parse::<syn::Ident>()?;
            let ident_str = ident.to_string();
            if !WHITE_LIST.contains(&ident_str.as_str()) {
                return Err(syn::Error::new_spanned(
                    ident,
                    "only support default, prompt, options",
                ));
            }
            input.parse::<Token![=]>()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Lit) {
                let literal = input.parse::<syn::Lit>()?;
                match ident_str.as_str() {
                    "default" => res.default = Some(FieldDefault::Lit(literal)),
                    "prompt" => {
                        if let syn::Lit::Str(s) = literal {
                            res.prompt = Some(s.value());
                        } else {
                            return Err(syn::Error::new_spanned(
                                &ident,
                                "prompt should be a string",
                            ));
                        }
                    }
                    "password" => {
                        if let syn::Lit::Bool(s) = literal {
                            res.password = Some(s.value());
                        } else {
                            return Err(syn::Error::new_spanned(
                                &ident,
                                "password should be a bool",
                            ));
                        }
                    }
                    "confirmation" => {
                        if let syn::Lit::Str(s) = literal {
                            res.confirmation = Some(s.value());
                        } else {
                            return Err(syn::Error::new_spanned(
                                &ident,
                                "confirmation should be a string",
                            ));
                        }
                    }
                    "options" => {
                        return Err(syn::Error::new_spanned(
                            &ident,
                            "options should be a vector",
                        ))
                    }
                    _ => {}
                }
            } else if lookahead.peek(syn::token::Bracket) {
                let vec_option;
                syn::bracketed!(vec_option in input);
                let punct_list = vec_option.parse_terminated(Lit::parse, Token![,])?;

                match ident_str.as_str() {
                    "default" => res.default = Some(FieldDefault::Vec(punct_list)),
                    "options" => {
                        res.options = punct_list;
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(&ident, "options should be a lit"));
                    }
                }
            } else {
                return Err(lookahead.error());
            }

            // 解析分隔符逗号
            if input.peek(syn::Ident::peek_any) || input.peek2(syn::Ident::peek_any) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(res)
    }
}

/// 解析字段属性
fn parse_field_attribute(field: &syn::Field) -> Result<Option<FieldAttributeOptions>> {
    let attrs = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("dialogue"));

    if let Some(syn::Attribute {
        meta: syn::Meta::List(syn::MetaList { path, tokens, .. }),
        ..
    }) = attrs
    {
        Ok(Some(syn::parse2(tokens.clone())?))
    } else {
        Ok(None)
    }
}
