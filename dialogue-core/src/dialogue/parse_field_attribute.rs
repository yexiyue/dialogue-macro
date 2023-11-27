use syn::{ext::IdentExt, punctuated::Punctuated, token::Comma, Lit, Result, Token};

#[derive(Debug)]
pub enum FieldDefault {
    Lit(Lit),
    Vec(Punctuated<Lit, Comma>),
}

/// 解析字段属性
#[derive(Debug)]
pub struct FieldAttributeOptions {
    pub default: Option<FieldDefault>,
    pub confirmation: Option<String>,
    pub prompt: Option<String>,
    pub password: Option<bool>,
    pub options: Punctuated<Lit, Comma>,
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
pub fn parse_field_attribute(field: &syn::Field) -> Result<Option<FieldAttributeOptions>> {
    let attrs = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("dialogue"));

    if let Some(syn::Attribute {
        meta: syn::Meta::List(syn::MetaList { tokens, .. }),
        ..
    }) = attrs
    {
        Ok(Some(syn::parse2(tokens.clone())?))
    } else {
        Ok(None)
    }
}
