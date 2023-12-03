use syn::{ext::IdentExt, token::Comma, Lit, Token};
pub mod generate_struct;
// 定义结构体，把数据解析出来
#[derive(Debug, Clone)]
pub struct DialogueList(syn::punctuated::Punctuated<DialogueItem, Comma>);

impl syn::parse::Parse for DialogueList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::braced!(content in input);
        let list = content.parse_terminated(DialogueItem::parse, Token![,])?;
        Ok(Self(list))
    }
}

#[derive(Debug, Clone)]
pub enum IdentOrLit {
    Ident(syn::Ident),
    Lit(syn::Lit),
    Lits(syn::punctuated::Punctuated<syn::Lit, Comma>),
}

impl syn::parse::Parse for IdentOrLit {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident::peek_any) {
            Ok(IdentOrLit::Ident(input.parse()?))
        } else if lookahead.peek(Lit) {
            Ok(IdentOrLit::Lit(input.parse()?))
        } else if lookahead.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            Ok(IdentOrLit::Lits(
                content.parse_terminated(Lit::parse, Token![,])?,
            ))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug, Clone)]
pub struct DialogueItem {
    pub field_name: syn::Ident,
    pub generic: Option<syn::Type>,
    pub ty: Option<String>,
    pub default: Option<IdentOrLit>,
    pub confirmation: Option<IdentOrLit>,
    pub prompt: Option<IdentOrLit>,
    pub password: Option<IdentOrLit>,
    pub options: Option<IdentOrLit>,
}

static TYPE_LIST: [&str; 5] = ["input", "password", "select", "confirm", "multiselect"];

impl syn::parse::Parse for DialogueItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let field_name = input.parse::<syn::Ident>()?;
        let mut res = Self {
            field_name,
            generic: None,
            ty: None,
            default: None,
            confirmation: None,
            prompt: None,
            password: None,
            options: None,
        };
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            res.generic = Some(input.parse()?);
            input.parse::<Token![>]>()?;
        }
        input.parse::<Token![=>]>()?;

        let content;
        syn::braced!(content in input);
        let options = content.parse_terminated(FieldValue::parse, Comma)?;
        for item in options {
            match item.0.to_string().as_str() {
                "default" => res.default = Some(item.1),
                "confirmation" => res.confirmation = Some(item.1),
                "prompt" => res.prompt = Some(item.1),
                "password" => res.password = Some(item.1),
                "options" => res.options = Some(item.1),
                "ty" => match item.1 {
                    IdentOrLit::Lit(lit) => {
                        if let Lit::Str(ty) = lit {
                            let v = ty.value();
                            if v.is_empty() {
                                return Err(syn::Error::new_spanned(
                                    ty,
                                    "ty field can not be empty",
                                ));
                            }
                            if !TYPE_LIST.contains(&v.trim().to_lowercase().as_str()) {
                                return Err(syn::Error::new_spanned(
                                    ty,
                                    "ty field can only be one of input, password, select, confirm, multiselect",
                                ));
                            }

                            res.ty = Some(v);
                        } else {
                            return Err(syn::Error::new_spanned(
                                lit,
                                "ty field can only be a string",
                            ));
                        }
                    }
                    IdentOrLit::Ident(ident) => {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "ty field can only be a string",
                        ));
                    }
                    IdentOrLit::Lits(lits) => {
                        return Err(syn::Error::new_spanned(
                            lits,
                            "ty field can only be a string",
                        ));
                    }
                },
                _ => {}
            }
        }
        Ok(res)
    }
}

#[derive(Debug, Clone)]
pub struct FieldValue(syn::Ident, IdentOrLit);

impl syn::parse::Parse for FieldValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let field_name = input.parse::<syn::Ident>()?;
        input.parse::<Token![:]>()?;
        let field_value = input.parse::<IdentOrLit>()?;
        Ok(Self(field_name, field_value))
    }
}
