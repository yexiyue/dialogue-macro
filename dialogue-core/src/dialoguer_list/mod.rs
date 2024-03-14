mod confirm;
mod input;
mod multiselect;
mod password;
mod select;
mod sub_asker;
use crate::utils::get_inner_type;
use syn::Result;

trait ParseFieldAttr
where
    Self: Sized,
{
    #[allow(unused)]
    fn parse_field_attr(attr: &syn::Attribute) -> Result<Self> {
        unimplemented!()
    }
    #[allow(unused)]
    fn parse_field_attr_with_inner_ty(attr: &syn::Attribute, inner_ty: &syn::Type) -> Result<Self> {
        unimplemented!()
    }

    fn generate_method(
        &self,
        theme: &proc_macro2::TokenStream,
        field_name: &Option<syn::Ident>,
        inner_ty: Option<&syn::Type>,
    ) -> Result<proc_macro2::TokenStream>;
}

pub enum DialoguerList {
    Input(input::Input),
    Password(password::Password),
    Confirm(confirm::Confirm),
    Select(select::Select, syn::Type),
    MultiSelect(multiselect::MultiSelect, syn::Type),
    SubAsker(sub_asker::SubAsker),
}

impl DialoguerList {
    fn get_dialogue(attr: &syn::Attribute) -> Option<&'static str> {
        if attr.path().is_ident("input") {
            return Some("Input");
        }
        if attr.path().is_ident("password") {
            return Some("Password");
        }
        if attr.path().is_ident("confirm") {
            return Some("Confirm");
        }
        if attr.path().is_ident("select") {
            return Some("Select");
        }
        if attr.path().is_ident("multiselect") {
            return Some("MultiSelect");
        }
        None
    }

    // 嵌套匹配
    fn is_some_type(ty: &syn::Type, name: &str, outer: &str) -> bool {
        if let Some(syn::Type::Path(syn::TypePath { path, .. })) = get_inner_type(ty, outer) {
            if path.is_ident(name) {
                return true;
            }
        } else {
            if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
                if path.is_ident(name) {
                    return true;
                }
            }
        }
        false
    }

    pub fn parse_field(field: &syn::Field) -> Result<Self> {
        if let Some(sub_asker) = sub_asker::SubAsker::from(&field)? {
            return Ok(Self::SubAsker(sub_asker));
        }
        for attr in &field.attrs {
            if let Some(dialogue) = DialoguerList::get_dialogue(attr) {
                match dialogue {
                    "Input" => {
                        return Ok(DialoguerList::Input(
                            input::Input::parse_field_attr_with_inner_ty(attr, &field.ty)?,
                        ));
                    }
                    "Password" => {
                        if Self::is_some_type(&field.ty, "String", "Option") {
                            return Ok(DialoguerList::Password(
                                password::Password::parse_field_attr(attr)?,
                            ));
                        }
                        return Err(syn::Error::new_spanned(
                            &field.ty,
                            "Password only support String or Option<String> type",
                        ));
                    }
                    "Confirm" => {
                        if Self::is_some_type(&field.ty, "bool", "Option") {
                            return Ok(DialoguerList::Confirm(confirm::Confirm::parse_field_attr(
                                attr,
                            )?));
                        }
                        return Err(syn::Error::new_spanned(
                            &field.ty,
                            "Confirm only support bool or Option<bool> type",
                        ));
                    }
                    "Select" => {
                        if let Some(ty) = get_inner_type(&field.ty, "Option") {
                            return Ok(DialoguerList::Select(
                                select::Select::parse_field_attr(attr)?,
                                ty.clone(),
                            ));
                        }
                        return Ok(DialoguerList::Select(
                            select::Select::parse_field_attr(attr)?,
                            field.ty.clone(),
                        ));
                    }
                    "MultiSelect" => {
                        if let Some(ty) = get_inner_type(&field.ty, "Vec") {
                            return Ok(DialoguerList::MultiSelect(
                                multiselect::MultiSelect::parse_field_attr(attr)?,
                                ty.clone(),
                            ));
                        }
                        return Err(syn::Error::new_spanned(
                            &field.ty,
                            "multiselect only support Vec type",
                        ));
                    }
                    _ => unreachable!(),
                }
            }
            if attr.path().is_ident("multiselect") {
                if let syn::Type::Path(syn::TypePath { path, .. }) = &field.ty {
                    if path.is_ident("Vec") {
                        return Ok(DialoguerList::MultiSelect(
                            multiselect::MultiSelect::parse_field_attr(attr)?,
                            get_inner_type(&field.ty, "Vec").unwrap().clone(),
                        ));
                    }
                    return Err(syn::Error::new_spanned(
                        &field.ty,
                        "multiselect only support Vec type",
                    ));
                }
                return Err(syn::Error::new_spanned(
                    &field.ty,
                    "multiselect only support Vec type",
                ));
            }
        }

        // 没有匹配到属性就给默认值
        if let Some(ty) = get_inner_type(&field.ty, "Vec") {
            Ok(DialoguerList::MultiSelect(
                multiselect::MultiSelect::default(),
                ty.clone(),
            ))
        } else {
            if Self::is_some_type(&field.ty, "bool", "Option") {
                Ok(DialoguerList::Confirm(confirm::Confirm::default()))
            } else {
                Ok(DialoguerList::Input(input::Input::default()))
            }
        }
    }

    pub fn generate_method(
        &self,
        theme: &proc_macro2::TokenStream,
        field_name: &Option<syn::Ident>,
    ) -> Result<proc_macro2::TokenStream> {
        match self {
            Self::Input(input) => input.generate_method(theme, field_name, None),
            Self::Confirm(confirm) => confirm.generate_method(theme, field_name, None),
            Self::Password(password) => password.generate_method(theme, field_name, None),
            Self::Select(select, ty) => select.generate_method(theme, field_name, Some(ty)),
            Self::MultiSelect(multiselect, ty) => {
                multiselect.generate_method(theme, field_name, Some(ty))
            }
            Self::SubAsker(sub_asker) => Ok(sub_asker.generate_method(field_name)),
        }
    }
}
