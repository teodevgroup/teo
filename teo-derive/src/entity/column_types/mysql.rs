use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, spanned::Spanned};
use teo_column_type::MySQLColumnType;
use crate::{entity::column_types::extended_column_type::ExtendedColumnType, utils::extract_first_path_argument};

impl ExtendedColumnType for MySQLColumnType {
    fn default_column_type(ty: &Type) -> Result<TokenStream> {
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
            }) => Self::default_column_type(elem),

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
                        Self::default_column_type(inner_ty)
                    } else if path.segments.len() == 3
                        && (path.segments[0].ident == "std" || path.segments[0].ident == "core")
                        && path.segments[1].ident == "option"
                        && path.segments[2].ident == "Option" {
                        let inner_ty = extract_first_path_argument(path)?;
                        Self::default_column_type(inner_ty)
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

    fn to_token_stream(&self) -> TokenStream {
        match self {
            MySQLColumnType::TinyInt => { quote! { ::teo::teo_column_type::MySQLColumnType::TinyInt } },
            MySQLColumnType::SmallInt => { quote! { ::teo::teo_column_type::MySQLColumnType::SmallInt } },
            MySQLColumnType::MediumInt => { quote! { ::teo::teo_column_type::MySQLColumnType::MediumInt } },
            MySQLColumnType::Int => { quote! { ::teo::teo_column_type::MySQLColumnType::Int } },
            MySQLColumnType::BigInt => { quote! { ::teo::teo_column_type::MySQLColumnType::BigInt } },
            MySQLColumnType::Decimal { m, d } => { quote! { ::teo::teo_column_type::MySQLColumnType::Decimal { m: #m, d: #d }}},
            MySQLColumnType::Float => { quote! { ::teo::teo_column_type::MySQLColumnType::Float } },
            MySQLColumnType::Double => { quote! { ::teo::teo_column_type::MySQLColumnType::Double } },
            MySQLColumnType::Bit { m } => { quote! { ::teo::teo_column_type::MySQLColumnType::Bit { m: #m } } },
            MySQLColumnType::Date => { quote! { ::teo::teo_column_type::MySQLColumnType::Date } },
            MySQLColumnType::Time { fsp } => { quote! { ::teo::teo_column_type::MySQLColumnType::Time { fsp: #fsp } } },
            MySQLColumnType::DateTime { fsp } => { quote! { ::teo::teo_column_type::MySQLColumnType::DateTime { fsp: #fsp } } },
            MySQLColumnType::Timestamp { fsp } => { quote! { ::teo::teo_column_type::MySQLColumnType::Timestamp { fsp: #fsp } } },
            MySQLColumnType::Year => { quote! { ::teo::teo_column_type::MySQLColumnType::Year } },
            MySQLColumnType::Char { m } => { quote! { ::teo::teo_column_type::MySQLColumnType::Char { m: #m } } },
            MySQLColumnType::VarChar { m } => { quote! { ::teo::teo_column_type::MySQLColumnType::VarChar { m: #m } } },
            MySQLColumnType::Binary { m } => { quote! { ::teo::teo_column_type::MySQLColumnType::Binary { m: #m } } },
            MySQLColumnType::VarBinary { m } => { quote! { ::teo::teo_column_type::MySQLColumnType::VarBinary { m: #m } } },
            MySQLColumnType::TinyBlob => { quote! { ::teo::teo_column_type::MySQLColumnType::TinyBlob } },
            MySQLColumnType::Blob => { quote! { ::teo::teo_column_type::MySQLColumnType::Blob } },
            MySQLColumnType::MediumBlob => { quote! { ::teo::teo_column_type::MySQLColumnType::MediumBlob } },
            MySQLColumnType::LongBlob => { quote! { ::teo::teo_column_type::MySQLColumnType::LongBlob } },
            MySQLColumnType::TinyText => { quote! { ::teo::teo_column_type::MySQLColumnType::TinyText } },
            MySQLColumnType::Text => { quote! { ::teo::teo_column_type::MySQLColumnType::Text } },
            MySQLColumnType::MediumText => { quote! { ::teo::teo_column_type::MySQLColumnType::MediumText } },
            MySQLColumnType::LongText => { quote! { ::teo::teo_column_type::MySQLColumnType::LongText } },
            MySQLColumnType::Geometry => { quote! { ::teo::teo_column_type::MySQLColumnType::Geometry } },
            MySQLColumnType::Point => { quote! { ::teo::teo_column_type::MySQLColumnType::Point } },
            MySQLColumnType::LineString => { quote! { ::teo::teo_column_type::MySQLColumnType::LineString } },
            MySQLColumnType::Polygon => { quote! { ::teo::teo_column_type::MySQLColumnType::Polygon } },
            MySQLColumnType::Multipoint => { quote! { ::teo::teo_column_type::MySQLColumnType::Multipoint } },
            MySQLColumnType::MultilineString => { quote! { ::teo::teo_column_type::MySQLColumnType::MultilineString } },
            MySQLColumnType::Multipolygon => { quote! { ::teo::teo_column_type::MySQLColumnType::Multipolygon } },
            MySQLColumnType::GeometryCollection => { quote! { ::teo::teo_column_type::MySQLColumnType::GeometryCollection } },
            MySQLColumnType::JSON => { quote! { ::teo::teo_column_type::MySQLColumnType::JSON } },
        }
    }
}
