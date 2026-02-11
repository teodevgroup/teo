use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, spanned::Spanned};
use crate::utils::extract_first_path_argument;

pub(in crate::entity) fn default_mysql_column_type(ty: &Type) -> Result<TokenStream> {
    match ty {
        Type::Array(_)
        | Type::BareFn(_)
        | Type::ImplTrait(_)
        | Type::Infer(_)
        | Type::Macro(_)
        | Type::Never(_)
        | Type::Ptr(_)
        | Type::Reference(_)
        | Type::Slice(_)
        | Type::TraitObject(_)
        | Type::Tuple(_)
        | Type::Verbatim(_) => Err(Error::new(ty.span(), "teo(mysql): Can't figure out default column type.")),

        Type::Group(syn::TypeGroup { elem, .. })
        | Type::Paren(syn::TypeParen { elem, .. })
        | Type::Path(syn::TypePath {
            qself: Some(syn::QSelf { ty: elem, .. }),
            ..
        }) => default_mysql_column_type(elem),

        Type::Path(syn::TypePath { qself: None, path }) => {
            if path.is_ident("bool") {
                Ok(quote! { ::teo::teo_column_type::MySQLColumnType::TinyInt })
            } else if path.is_ident("i32") {
                Ok(quote! { ::teo::teo_column_type::MySQLColumnType::Int })
            } else if path.is_ident("i64") {
                Ok(quote! { ::teo::teo_column_type::MySQLColumnType::BigInt })
            } else if path.is_ident("f32") {
                Ok(quote! { ::teo::teo_column_type::MySQLColumnType::Float })
            } else if path.is_ident("f64") {
                Ok(quote! { ::teo::teo_column_type::MySQLColumnType::Double })
            } else if path.is_ident("String") {
                Ok(quote! { ::teo::teo_column_type::MySQLColumnType::VarChar { m: 191 } })
            } else if path.is_ident("Uuid") {
                Ok(quote! { ::teo::teo_column_type::MySQLColumnType::Char { m: 36 } })
            } else {
                if path.leading_colon.is_none()
                    && path.segments.len() == 1
                    && path.segments[0].ident == "Option" {
                    let inner_ty = extract_first_path_argument(path)?;
                    default_mysql_column_type(inner_ty)
                } else if path.segments.len() == 3
                    && (path.segments[0].ident == "std" || path.segments[0].ident == "core")
                    && path.segments[1].ident == "option"
                    && path.segments[2].ident == "Option" {
                    let inner_ty = extract_first_path_argument(path)?;
                    default_mysql_column_type(inner_ty)
                } else {
                    if path.segments.len() == 2
                        && path.segments[0].ident == "uuid"
                        && path.segments[1].ident == "Uuid" {
                        Ok(quote! { ::teo::teo_column_type::MySQLColumnType::Char { m: 36 } })
                    } else if path.segments.len() == 3
                        && path.segments[0].ident == "std"
                        && path.segments[1].ident == "string"
                        && path.segments[2].ident == "String" {
                        Ok(quote! { ::teo::teo_column_type::MySQLColumnType::VarChar { m: 191 } })
                    } else if path.segments.len() == 1
                        && path.segments[0].ident == "DateTime" {
                        Ok(quote! { ::teo::teo_column_type::MySQLColumnType::Timestamp { fsp: 6 } })
                    } else if path.segments.len() == 2
                        && path.segments[0].ident == "chrono"
                        && path.segments[1].ident == "DateTime" {
                        Ok(quote! { ::teo::teo_column_type::MySQLColumnType::Timestamp { fsp: 6 } })
                    } else {
                        Err(Error::new(ty.span(), "teo(mysql): Can't figure out default column type."))
                    }
                }
            }
        },
        _ => Err(Error::new(ty.span(), "teo(mysql): Can't figure out default column type.")),
    }
}
