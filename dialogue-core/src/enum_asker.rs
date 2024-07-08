use proc_macro2::TokenStream;
use quote::quote;
use syn::{ parse_str, DeriveInput, Fields, Ident, LitStr, Result, Type, Variant };
#[derive(Debug)]
struct EnumVariant {
    label: Option<String>,
    ident: Ident,
    types: Vec<Type>,
}

impl EnumVariant {
    fn get_label(&mut self, st: &Variant) -> Result<()> {
        if let Some(attr) = &st.attrs.iter().find(|attr| attr.path().is_ident("asker")) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("label") {
                    meta.value()?;
                    let v = meta.input.parse::<LitStr>()?;
                    self.label = Some(v.value());
                    return Ok(());
                }
                Err(meta.error("expected `label`"))
            })?;
        }
        Ok(())
    }

    fn from(st: &Variant) -> Result<Self> {
        let ident = st.ident.clone();
        let mut types = vec![];
        match &st.fields {
            Fields::Named(_) => {
                return Err(
                    syn::Error::new(ident.span(), "asker don't supports enums with named fields")
                );
            }
            Fields::Unnamed(_) => {
                for field in &st.fields {
                    types.push(field.ty.clone());
                }
            }
            _ => {}
        }
        let mut res = Self {
            label: None,
            ident,
            types,
        };
        res.get_label(st)?;

        Ok(res)
    }
}

#[derive(Debug)]
struct EnumAsker {
    theme: Option<Type>,
    prompt: Option<String>,
    default: Option<String>,
    variants: Vec<EnumVariant>,
}

impl EnumAsker {
    fn get_prompt(&mut self, st: &DeriveInput) -> Result<()> {
        if let Some(attr) = &st.attrs.iter().find(|attr| attr.path().is_ident("asker")) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prompt") {
                    meta.value()?;
                    let v = meta.input.parse::<LitStr>()?;
                    self.prompt = Some(v.value());
                    return Ok(());
                }
                if meta.path.is_ident("default") {
                    meta.value()?;
                    let v = meta.input.parse::<LitStr>()?;
                    self.default = Some(v.value());
                    return Ok(());
                }
                if meta.path.is_ident("theme") {
                    meta.value()?;
                    let v = meta.input.parse::<LitStr>()?.value();
                    let path: Type = parse_str(&v)?;
                    self.theme = Some(path);
                    return Ok(());
                }
                Err(meta.error("expected `prompt` , `default` , `theme`"))
            })?;
        }
        Ok(())
    }

    fn from(st: &DeriveInput) -> Result<Self> {
        let mut res = Self {
            theme: None,
            prompt: None,
            default: None,
            variants: vec![],
        };
        res.get_prompt(st)?;
        match &st.data {
            syn::Data::Enum(enum_) => {
                for variant in &enum_.variants {
                    let v = EnumVariant::from(variant)?;
                    res.variants.push(v);
                }
            }
            _ => {
                return Err(syn::Error::new(st.ident.span(), "EnumAsker only supports enums"));
            }
        }
        Ok(res)
    }

    fn options(&self) -> proc_macro2::TokenStream {
        let options = self.variants
            .iter()
            .map(|i| i.ident.to_string())
            .collect::<Vec<_>>();
        quote!(let options = vec![#(#options),*];)
    }

    fn arms(&self) -> proc_macro2::TokenStream {
        let mut res = proc_macro2::TokenStream::new();
        for i in self.variants.iter() {
            let pat = i.ident.to_string();
            let types = &i.types;
            let ident = &i.ident;
            let arm = if types.is_empty() {
                quote!(#pat => Self::#ident,)
            } else {
                quote!(#pat => Self::#ident(#(#types::build()),*),)
            };
            res.extend(arm);
        }
        res.extend(quote!(_=>panic!("Unknown option"),));
        res
    }

    fn default_index(&self) -> Option<proc_macro2::TokenStream> {
        if self.default.is_some() {
            let default = self.default.as_ref().unwrap();
            return Some(
                quote! {
                let default_index = options
                    .iter()
                    .position(|i| *i == #default)
                    .expect(&format!("{} not found in options", #default));
            }
            );
        }
        None
    }

    fn selected(&self) -> proc_macro2::TokenStream {
        let mut res = quote!(let i = dialoguer::Select::with_theme);
        if self.theme.is_some() {
            let theme = self.theme.as_ref().unwrap();
            res.extend(quote!((&#theme::default())));
        } else {
            res.extend(quote!((&dialogue_macro::ColorfulTheme::default())));
        }

        if self.prompt.is_some() {
            let prompt = self.prompt.as_ref().unwrap();
            res.extend(quote!(.with_prompt(#prompt)));
        }
        res.extend(quote!(.items(&options)));

        if self.default.is_some() {
            res.extend(quote!(.default(default_index)));
        }

        res.extend(quote!(
            .interact()
            .unwrap();
        ));

        res
    }
}

pub fn enum_asker_build(st: DeriveInput) -> Result<TokenStream> {
    let asker = EnumAsker::from(&st)?;
    let options = asker.options();
    let arms = asker.arms();
    let default_index = asker.default_index();
    let selected = asker.selected();
    let ident = st.ident;

    let res =
        quote!(
        impl dialogue_macro::Build for #ident {
            fn build() -> Self {
                #options
                #default_index
                #selected
                let res = match options[i] {
                    #arms
                };
                res
            }
        }
    );
    Ok(res)
}
