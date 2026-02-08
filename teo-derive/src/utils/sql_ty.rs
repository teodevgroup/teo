use syn::{Error, Result, Type, spanned::Spanned};

use crate::utils::extract_first_path_argument::extract_first_path_argument;

pub (crate) fn sql_ty(ty: &Type) -> Result<String> {
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
        | Type::Verbatim(_) => Err(Error::new(ty.span(), "teo-data-syncer: Unsupported type.")),

        Type::Group(syn::TypeGroup { elem, .. })
        | Type::Paren(syn::TypeParen { elem, .. })
        | Type::Path(syn::TypePath {
            qself: Some(syn::QSelf { ty: elem, .. }),
            ..
        }) => sql_ty(elem),

        Type::Path(syn::TypePath { qself: None, path }) => {
            if path.is_ident("bool") {
                Ok("bool".to_owned())
            } else if path.is_ident("i32") {
                Ok("int4".to_owned())
            } else if path.is_ident("i64") {
                Ok("bigint".to_owned())
            } else if path.is_ident("f32") {
                Ok("real".to_owned())
            } else if path.is_ident("f64") {
                Ok("float8".to_owned())
            } else if path.is_ident("String") {
                Ok("text".to_owned())
            } else if path.is_ident("Uuid") {
                Ok("uuid".to_owned())
            } else {
                if path.leading_colon.is_none()
                    && path.segments.len() == 1
                    && path.segments[0].ident == "Option" {
                    let inner_ty = extract_first_path_argument(path)?;
                    sql_ty(inner_ty)
                } else if path.segments.len() == 3
                    && (path.segments[0].ident == "std" || path.segments[0].ident == "core")
                    && path.segments[1].ident == "option"
                    && path.segments[2].ident == "Option" {
                    let inner_ty = extract_first_path_argument(path)?;
                    sql_ty(inner_ty)
                } else {
                    if path.segments.len() == 2
                        && path.segments[0].ident == "uuid"
                        && path.segments[1].ident == "Uuid" {
                        Ok("uuid".to_owned())
                    } else if path.segments.len() == 3
                        && path.segments[0].ident == "std"
                        && path.segments[1].ident == "string"
                        && path.segments[2].ident == "String" {
                        Ok("text".to_owned())
                    } else if path.segments.len() == 1
                        && path.segments[0].ident == "DateTime" {
                        Ok("timestamptz".to_owned())
                    } else if path.segments.len() == 2
                        && path.segments[0].ident == "chrono"
                        && path.segments[1].ident == "DateTime" {
                        Ok("timestamptz".to_owned())
                    } else {
                        if let Some(last_segment) = path.segments.last() {
                            Ok(last_segment.ident.to_string())
                        } else {
                            Err(Error::new(ty.span(), "teo-data-syncer: Unsupported type."))
                        }
                    }
                }
            }
        },
        _ => Err(Error::new(ty.span(), "teo-data-syncer: Unsupported type.")),
    }
}
