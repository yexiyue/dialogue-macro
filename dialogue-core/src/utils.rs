#![allow(unused)]
use syn::{
    punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments, DataStruct, DeriveInput,
    Field, Fields, FieldsNamed, PathArguments, PathSegment,
};
use syn::{Result, TypePath};

// 获取结构体的字段
pub fn get_fields(st: &DeriveInput) -> Result<&Punctuated<Field, Comma>> {
    if let syn::Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = &st.data
    {
        Ok(named)
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "only support named struct",
        ))
    }
}

// 获取结构体字段的范醒类型，例如Option<String>获取到String
pub fn get_inner_type<'a, 'b>(ty: &'a syn::Type, outer_type: &'b str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(TypePath { path, .. }) = ty {
        if let Some(PathSegment { arguments, ident }) = path.segments.last() {
            if ident == outer_type {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    args, ..
                }) = arguments
                {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
