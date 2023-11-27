use quote::quote;
use syn::Result;
pub enum DialogueType<'a> {
    Confirm {
        prompt: Option<&'a str>,
        default: Option<bool>,
    },
    Input {
        prompt: Option<&'a str>,
        default: Option<String>,
    },
    Number {
        prompt: Option<&'a str>,
        default: Option<syn::Lit>,
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
    pub fn get_dialogue(&self) -> Result<proc_macro2::TokenStream> {
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
            DialogueType::Number { prompt, default } => {
                res.extend(quote!(dialoguer::Input::with_theme(
                    &dialoguer::theme::ColorfulTheme::default()
                )));
                if let Some(default) = default {
                    match default {
                        syn::Lit::Int(i) => res.extend(quote!(
                           .default(#i)
                        )),
                        syn::Lit::Float(f) => res.extend(quote!(
                          .default(#f)
                        )),
                        syn::Lit::Str(s) => res.extend(quote!(
                         .default(#s.parse().expect("expected number"))
                        )),
                        _ => return Err(syn::Error::new(default.span(), "expected number")),
                    }
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
        Ok(res)
    }
}
