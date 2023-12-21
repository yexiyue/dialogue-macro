use super::DialogueItem;
use quote::{quote, ToTokens};
use syn::Result;

impl DialogueItem {
    pub fn to_tokens(&self) -> Result<proc_macro2::TokenStream> {
        let mut res = proc_macro2::TokenStream::new();
        let fn_name = &self.field_name;

        let dialogue_t = self.ty_to_tokens()?;
        let prompt_t = self.prompt_to_tokens()?;
        let (set_t, head_t) = self.to_set_value_tokens()?;

        if let Some(_) = &self.prompt {
            res.extend(quote! {
                pub fn #fn_name(&mut self)->&mut Self{
                    #head_t
                    let res=#dialogue_t
                    #prompt_t
                    #set_t
                    self
                }
            })
        } else {
            res.extend(quote! {
                pub fn #fn_name(&mut self,prompt:&str)->&mut Self{
                    #head_t
                    let res=#dialogue_t
                    #prompt_t
                    #set_t
                    self
                }
            })
        }

        Ok(res)
    }

    pub fn ty_to_tokens(&self) -> Result<proc_macro2::TokenStream> {
        let mut res = proc_macro2::TokenStream::new();
        if let Some(dialogue_type) = &self.ty {
            match dialogue_type.as_str() {
                "input" => res.extend(quote!(dialoguer::Input::with_theme(
                    &dialogue_macro::ColorfulTheme::default()
                ))),
                "password" => res.extend(quote!(dialoguer::Password::with_theme(
                    &dialogue_macro::ColorfulTheme::default()
                ))),
                "select" => res.extend(quote!(dialoguer::Select::with_theme(
                    &dialogue_macro::ColorfulTheme::default()
                ))),
                "confirm" => res.extend(quote!(dialoguer::Confirm::with_theme(
                    &dialogue_macro::ColorfulTheme::default()
                ))),
                "multiselect" => res.extend(quote!(dialoguer::MultiSelect::with_theme(
                    &dialogue_macro::ColorfulTheme::default()
                ))),
                _ => unreachable!(),
            }
        } else {
            res.extend(quote!(dialoguer::Input::with_theme(
                &dialogue_macro::ColorfulTheme::default()
            )))
        }
        Ok(res)
    }

    pub fn prompt_to_tokens(&self) -> Result<proc_macro2::TokenStream> {
        let mut res = proc_macro2::TokenStream::new();
        if let Some(prompt) = &self.prompt {
            match prompt {
                super::IdentOrLit::Ident(ident) => res.extend(quote!(
                   .with_prompt(#ident)
                )),
                super::IdentOrLit::Lit(lit) => {
                    if let syn::Lit::Str(_) = lit {
                        res.extend(quote!(
                            .with_prompt(#lit)
                        ))
                    } else {
                        return Err(syn::Error::new_spanned(lit, "only support str"));
                    }
                }
                super::IdentOrLit::Lits(lits) => {
                    return Err(syn::Error::new_spanned(lits, "only support str"))
                }
            }
        } else {
            res.extend(quote!(
               .with_prompt(prompt)
            ))
        }
        Ok(res)
    }

    pub fn options_to_tokens(
        &self,
    ) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
        let mut res = proc_macro2::TokenStream::new();
        let mut head_t = proc_macro2::TokenStream::new();
        if let Some(options) = &self.options {
            match options {
                super::IdentOrLit::Ident(ident) => {
                    head_t.extend(quote!(
                      let option = #ident.map(|x| x.to_string());
                    ));
                    res.extend(quote!(
                      .items(&option)
                    ))
                }
                super::IdentOrLit::Lit(lit) => {
                    return Err(syn::Error::new_spanned(lit, "not support"))
                }
                super::IdentOrLit::Lits(lits) => {
                    head_t.extend(quote!(
                      let option = [#lits];
                    ));
                    res.extend(quote!(
                      .items(&option)
                    ))
                }
            }
        } else {
            return Err(syn::Error::new_spanned(
                &self.field_name,
                "ty is select or multiselect options is required",
            ));
        }

        Ok((res, head_t))
    }

    pub fn default_to_tokens(
        &self,
    ) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
        let mut res = proc_macro2::TokenStream::new();
        let mut head_t = proc_macro2::TokenStream::new();
        if let Some(dialogue_type) = &self.ty {
            match dialogue_type.as_str() {
                "select" => {
                    if let Some(default) = &self.default {
                        match default {
                            super::IdentOrLit::Ident(ident) => {
                                head_t.extend(quote!(
                                    let default=option.iter().position(|x| x == &#ident).expect("默认值不在选项中");
                                ));
                                res.extend(quote!(
                                  .default(default)
                                ))
                            }
                            super::IdentOrLit::Lit(lit) => {
                                head_t.extend(quote!(
                                    let default=option.iter().position(|x| x == &#lit).expect("默认值不在选项中");
                                ));
                                res.extend(quote!(
                                  .default(default)
                                ))
                            }
                            super::IdentOrLit::Lits(lits) => {
                                return Err(syn::Error::new_spanned(
                                    lits,
                                    "select not support list",
                                ))
                            }
                        }
                    }
                }
                "confirm" => {
                    if let Some(default) = &self.default {
                        match default {
                            super::IdentOrLit::Ident(ident) => res.extend(quote!(
                              .default(#ident)
                            )),
                            super::IdentOrLit::Lit(lit) => res.extend(quote!(
                               .default(#lit.parse().unwrap())
                            )),
                            super::IdentOrLit::Lits(lits) => {
                                return Err(syn::Error::new_spanned(lits, "input not support list"))
                            }
                        }
                    }
                }
                "input" => {
                    if let Some(default) = &self.default {
                        match default {
                            super::IdentOrLit::Ident(ident) => res.extend(quote!(
                              .default(format!("{}",#ident))
                            )),
                            super::IdentOrLit::Lit(lit) => res.extend(quote!(
                               .default(#lit.parse().unwrap())
                            )),
                            super::IdentOrLit::Lits(lits) => {
                                return Err(syn::Error::new_spanned(lits, "input not support list"))
                            }
                        }
                    }
                }
                "multiselect" => {
                    if let Some(default) = &self.default {
                        match default {
                            super::IdentOrLit::Ident(ident) => {
                                head_t.extend(quote!(
                                    let default=option.iter().map(|y|{
                                        if #ident.iter().find(|x|*x==&y.as_str()).is_some(){
                                            true
                                        }else{
                                            false
                                        }
                                    }).collect::<Vec<_>>();
                                ));
                                res.extend(quote!(
                                  .defaults(&default)
                                ))
                            }
                            super::IdentOrLit::Lit(lit) => {
                                return Err(syn::Error::new_spanned(lit, "should be a list"))
                            }
                            super::IdentOrLit::Lits(lits) => {
                                head_t.extend(quote!(
                                    let default_value=vec![#lits];
                                    let default=option.iter().map(|y|{
                                        if default_value.iter().find(|x|*x==&y.as_str()).is_some(){
                                            true
                                        }else{
                                            false
                                        }
                                    }).collect::<Vec<_>>();
                                ));
                                res.extend(quote!(
                                  .defaults(&default)
                                ))
                            }
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        &self.ty,
                        format!("{:?} not support default", dialogue_type),
                    ))
                }
            }
        } else {
            if let Some(default) = &self.default {
                match default {
                    super::IdentOrLit::Ident(ident) => res.extend(quote!(
                      .default(format!("{}",#ident))
                    )),
                    super::IdentOrLit::Lit(lit) => res.extend(quote!(
                       .default(#lit.parse().unwrap())
                    )),
                    super::IdentOrLit::Lits(lits) => {
                        return Err(syn::Error::new_spanned(lits, "input not support list"))
                    }
                }
            }
        }
        Ok((res, head_t))
    }

    pub fn to_set_value_tokens(
        &self,
    ) -> Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
        let mut res = proc_macro2::TokenStream::new();
        let mut head_token = proc_macro2::TokenStream::new();
        let fn_name = &self.field_name;
        let (default_token, _) = self.default_to_tokens()?;
        if let Some(dialogue_type) = &self.ty {
            match dialogue_type.as_str() {
                "input" => res.extend(quote!(
                    #default_token
                    .interact().unwrap();
                    self.#fn_name=std::option::Option::Some(res);
                )),
                "password" => {
                    let confirmation_t = self.confirmation_to_tokens()?;
                    res.extend(quote!(
                        #confirmation_t
                        .interact().unwrap();
                        self.#fn_name=std::option::Option::Some(res);
                    ))
                }
                "select" => {
                    let (options_t, head_t1) = self.options_to_tokens()?;
                    let (default_t, head_t2) = self.default_to_tokens()?;
                    head_token = head_t1;
                    head_token.extend(head_t2);
                    res.extend(quote!(
                        #options_t
                        #default_t
                        .interact().unwrap();

                        self.#fn_name=std::option::Option::Some(option[res].clone());
                    ))
                }
                "confirm" => res.extend(quote!(
                    #default_token
                    .interact().unwrap();
                    self.#fn_name=std::option::Option::Some(res);
                )),
                "multiselect" => {
                    let (options_t, head_t1) = self.options_to_tokens()?;
                    let (default_t, head_t2) = self.default_to_tokens()?;
                    head_token = head_t1;
                    head_token.extend(head_t2);
                    res.extend(quote!(
                        #options_t
                        #default_t
                        .interact().unwrap();
                        let result=res.iter().map(|x|option[*x].clone()).collect::<Vec<_>>();
                        self.#fn_name=std::option::Option::Some(result);
                    ))
                }
                _ => unreachable!(),
            }
        } else {
            res.extend(quote!(
                #default_token
                .interact().unwrap();
                self.#fn_name=std::option::Option::Some(res);
            ))
        }
        Ok((res, head_token))
    }

    pub fn confirmation_to_tokens(&self) -> Result<proc_macro2::TokenStream> {
        let mut res = proc_macro2::TokenStream::new();
        let mismatch_err: &dyn ToTokens;
        if let Some(mismatch) = &self.mismatch {
            match mismatch {
                super::IdentOrLit::Ident(ident) => mismatch_err = ident,
                super::IdentOrLit::Lit(lit) => mismatch_err = lit,
                super::IdentOrLit::Lits(lits) => {
                    return Err(syn::Error::new_spanned(lits, "only support str"))
                }
            }
        } else {
            mismatch_err = &"两次密码不匹配";
        };

        if let Some(confirmation) = &self.confirmation {
            match confirmation {
                super::IdentOrLit::Ident(ident) => res.extend(quote!(
                  .with_confirmation(#ident,#mismatch_err)
                )),
                super::IdentOrLit::Lit(lit) => res.extend(quote!(
                  .with_confirmation(#lit,#mismatch_err)
                )),
                super::IdentOrLit::Lits(lits) => {
                    return Err(syn::Error::new_spanned(lits, "only support str"))
                }
            }
        }

        Ok(res)
    }
}
