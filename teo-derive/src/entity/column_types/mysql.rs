use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, spanned::Spanned};
use teo_column_type::mysql::ColumnType;
use crate::{entity::column_types::extended_column_type::ExtendedColumnType, utils::extract_first_path_argument};

impl ExtendedColumnType for ColumnType {
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
                    Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::TinyInt })
                } else if path.is_ident("i32") {
                    Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::Int })
                } else if path.is_ident("i64") {
                    Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::BigInt })
                } else if path.is_ident("f32") {
                    Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::Float })
                } else if path.is_ident("f64") {
                    Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::Double })
                } else if path.is_ident("String") {
                    Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::VarChar { m: 191 } })
                } else if path.is_ident("Uuid") {
                    Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::Char { m: 36 } })
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
                            Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::Char { m: 36 } })
                        } else if path.segments.len() == 3
                            && path.segments[0].ident == "std"
                            && path.segments[1].ident == "string"
                            && path.segments[2].ident == "String" {
                            Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::VarChar { m: 191 } })
                        } else if path.segments.len() == 1
                            && path.segments[0].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::Timestamp { fsp: 6 } })
                        } else if path.segments.len() == 2
                            && path.segments[0].ident == "chrono"
                            && path.segments[1].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::mysql::ColumnType::Timestamp { fsp: 6 } })
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
            ColumnType::TinyInt => { quote! { ::teo::teo_column_type::mysql::ColumnType::TinyInt } },
            ColumnType::SmallInt => { quote! { ::teo::teo_column_type::mysql::ColumnType::SmallInt } },
            ColumnType::MediumInt => { quote! { ::teo::teo_column_type::mysql::ColumnType::MediumInt } },
            ColumnType::Int => { quote! { ::teo::teo_column_type::mysql::ColumnType::Int } },
            ColumnType::BigInt => { quote! { ::teo::teo_column_type::mysql::ColumnType::BigInt } },
            ColumnType::Decimal { m, d } => { quote! { ::teo::teo_column_type::mysql::ColumnType::Decimal { m: #m, d: #d }}},
            ColumnType::Float => { quote! { ::teo::teo_column_type::mysql::ColumnType::Float } },
            ColumnType::Double => { quote! { ::teo::teo_column_type::mysql::ColumnType::Double } },
            ColumnType::Bit { m } => { quote! { ::teo::teo_column_type::mysql::ColumnType::Bit { m: #m } } },
            ColumnType::Date => { quote! { ::teo::teo_column_type::mysql::ColumnType::Date } },
            ColumnType::Time { fsp } => { quote! { ::teo::teo_column_type::mysql::ColumnType::Time { fsp: #fsp } } },
            ColumnType::DateTime { fsp } => { quote! { ::teo::teo_column_type::mysql::ColumnType::DateTime { fsp: #fsp } } },
            ColumnType::Timestamp { fsp } => { quote! { ::teo::teo_column_type::mysql::ColumnType::Timestamp { fsp: #fsp } } },
            ColumnType::Year => { quote! { ::teo::teo_column_type::mysql::ColumnType::Year } },
            ColumnType::Char { m } => { quote! { ::teo::teo_column_type::mysql::ColumnType::Char { m: #m } } },
            ColumnType::VarChar { m } => { quote! { ::teo::teo_column_type::mysql::ColumnType::VarChar { m: #m } } },
            ColumnType::Binary { m } => { quote! { ::teo::teo_column_type::mysql::ColumnType::Binary { m: #m } } },
            ColumnType::VarBinary { m } => { quote! { ::teo::teo_column_type::mysql::ColumnType::VarBinary { m: #m } } },
            ColumnType::TinyBlob => { quote! { ::teo::teo_column_type::mysql::ColumnType::TinyBlob } },
            ColumnType::Blob => { quote! { ::teo::teo_column_type::mysql::ColumnType::Blob } },
            ColumnType::MediumBlob => { quote! { ::teo::teo_column_type::mysql::ColumnType::MediumBlob } },
            ColumnType::LongBlob => { quote! { ::teo::teo_column_type::mysql::ColumnType::LongBlob } },
            ColumnType::TinyText => { quote! { ::teo::teo_column_type::mysql::ColumnType::TinyText } },
            ColumnType::Text => { quote! { ::teo::teo_column_type::mysql::ColumnType::Text } },
            ColumnType::MediumText => { quote! { ::teo::teo_column_type::mysql::ColumnType::MediumText } },
            ColumnType::LongText => { quote! { ::teo::teo_column_type::mysql::ColumnType::LongText } },
            ColumnType::Geometry => { quote! { ::teo::teo_column_type::mysql::ColumnType::Geometry } },
            ColumnType::Point => { quote! { ::teo::teo_column_type::mysql::ColumnType::Point } },
            ColumnType::LineString => { quote! { ::teo::teo_column_type::mysql::ColumnType::LineString } },
            ColumnType::Polygon => { quote! { ::teo::teo_column_type::mysql::ColumnType::Polygon } },
            ColumnType::Multipoint => { quote! { ::teo::teo_column_type::mysql::ColumnType::Multipoint } },
            ColumnType::MultilineString => { quote! { ::teo::teo_column_type::mysql::ColumnType::MultilineString } },
            ColumnType::Multipolygon => { quote! { ::teo::teo_column_type::mysql::ColumnType::Multipolygon } },
            ColumnType::GeometryCollection => { quote! { ::teo::teo_column_type::mysql::ColumnType::GeometryCollection } },
            ColumnType::JSON => { quote! { ::teo::teo_column_type::mysql::ColumnType::JSON } },
        }
    }
}
