use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, spanned::Spanned};
use teo_column_type::postgres::ColumnType;
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
            | Type::Verbatim(_) => Err(Error::new(ty.span(), "teo(postgres): Can't figure out default column type.")),

            Type::Group(syn::TypeGroup { elem, .. })
            | Type::Paren(syn::TypeParen { elem, .. })
            | Type::Path(syn::TypePath {
                qself: Some(syn::QSelf { ty: elem, .. }),
                ..
            }) => Self::default_column_type(elem),

            Type::Path(syn::TypePath { qself: None, path }) => {
                if path.is_ident("bool") {
                    Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::Boolean })
                } else if path.is_ident("i32") {
                    Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::Integer })
                } else if path.is_ident("i64") {
                    Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::BigInt })
                } else if path.is_ident("f32") {
                    Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::Real })
                } else if path.is_ident("f64") {
                    Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::DoublePrecision })
                } else if path.is_ident("String") {
                    Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::Text })
                } else if path.is_ident("Uuid") {
                    Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::UUID })
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
                            Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::UUID })
                        } else if path.segments.len() == 3
                            && path.segments[0].ident == "std"
                            && path.segments[1].ident == "string"
                            && path.segments[2].ident == "String" {
                            Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::Text })
                        } else if path.segments.len() == 1
                            && path.segments[0].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::TimestampWithTimeZone { n: 6 } })
                        } else if path.segments.len() == 2
                            && path.segments[0].ident == "chrono"
                            && path.segments[1].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::postgres::ColumnType::TimestampWithTimeZone { n: 6 } })
                        } else {
                            Err(Error::new(ty.span(), "teo(postgres): Can't figure out default column type."))
                        }
                    }
                }
            },
            _ => Err(Error::new(ty.span(), "teo(postgres): Can't figure out default column type.")),
        }
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            ColumnType::BigInt => { quote! { ::teo::teo_column_type::postgres::ColumnType::BigInt } },
            ColumnType::BigSerial => { quote! { ::teo::teo_column_type::postgres::ColumnType::BigSerial } },
            ColumnType::Bit { n } => { quote! { ::teo::teo_column_type::postgres::ColumnType::Bit { n: #n } } },
            ColumnType::BitVarying { n } => { quote! { ::teo::teo_column_type::postgres::ColumnType::BitVarying { n: #n } } },
            ColumnType::Boolean => { quote! { ::teo::teo_column_type::postgres::ColumnType::Boolean } },
            ColumnType::Box => { quote! { ::teo::teo_column_type::postgres::ColumnType::Box } },
            ColumnType::ByteA => { quote! { ::teo::teo_column_type::postgres::ColumnType::ByteA } },
            ColumnType::Character { n } => { quote! { ::teo::teo_column_type::postgres::ColumnType::Character { n: #n } } },
            ColumnType::CharacterVarying { n } => { quote! { ::teo::teo_column_type::postgres::ColumnType::CharacterVarying { n: #n } } },
            ColumnType::CIDR => { quote! { ::teo::teo_column_type::postgres::ColumnType::CIDR } },
            ColumnType::Circle => { quote! { ::teo::teo_column_type::postgres::ColumnType::Circle } },
            ColumnType::Date => { quote! { ::teo::teo_column_type::postgres::ColumnType::Date } },
            ColumnType::DoublePrecision => { quote! { ::teo::teo_column_type::postgres::ColumnType::DoublePrecision } },
            ColumnType::INet => { quote! { ::teo::teo_column_type::postgres::ColumnType::INet } },
            ColumnType::Integer => { quote! { ::teo::teo_column_type::postgres::ColumnType::Integer } },
            ColumnType::JSON => { quote! { ::teo::teo_column_type::postgres::ColumnType::JSON } },
            ColumnType::JSONB => { quote! { ::teo::teo_column_type::postgres::ColumnType::JSONB } },
            ColumnType::Line => { quote! { ::teo::teo_column_type::postgres::ColumnType::Line } },
            ColumnType::LSeg => { quote! { ::teo::teo_column_type::postgres::ColumnType::LSeg } },
            ColumnType::MACAddr => { quote! { ::teo::teo_column_type::postgres::ColumnType::MACAddr } },
            ColumnType::MACAddr8 => { quote! { ::teo::teo_column_type::postgres::ColumnType::MACAddr8 } },
            ColumnType::Money => { quote! { ::teo::teo_column_type::postgres::ColumnType::Money } },
            ColumnType::Numeric { p, s } => { quote! { ::teo::teo_column_type::postgres::ColumnType::Numeric { p: #p, s: #s } } },
            ColumnType::Path => { quote! { ::teo::teo_column_type::postgres::ColumnType::Path } },
            ColumnType::PgLSN => { quote! { ::teo::teo_column_type::postgres::ColumnType::PgLSN } },
            ColumnType::PGSnapshot => { quote! { ::teo::teo_column_type::postgres::ColumnType::PGSnapshot } },
            ColumnType::Point => { quote! { ::teo::teo_column_type::postgres::ColumnType::Point } },
            ColumnType::Polygon => { quote! { ::teo::teo_column_type::postgres::ColumnType::Polygon } },
            ColumnType::Real => { quote! { ::teo::teo_column_type::postgres::ColumnType::Real } },
            ColumnType::SmallInt => { quote! { ::teo::teo_column_type::postgres::ColumnType::SmallInt } },
            ColumnType::SmallSerial => { quote! { ::teo::teo_column_type::postgres::ColumnType::SmallSerial } },
            ColumnType::Serial => { quote! { ::teo::teo_column_type::postgres::ColumnType::Serial } },
            ColumnType::Text => { quote! { ::teo::teo_column_type::postgres::ColumnType::Text } },
            ColumnType::TimeWithoutTimeZone { p } => { quote! { ::teo::teo_column_type::postgres::ColumnType::TimeWithoutTimeZone { p: #p } } },
            ColumnType::TimeWithTimeZone { p } => { quote! { ::teo::teo_column_type::postgres::ColumnType::TimeWithTimeZone { p: #p } } },
            ColumnType::TimestampWithoutTimeZone { p } => { quote! { ::teo::teo_column_type::postgres::ColumnType::TimestampWithoutTimeZone { p: #p } } },
            ColumnType::TimestampWithTimeZone { p } => { quote! { ::teo::teo_column_type::postgres::ColumnType::TimestampWithTimeZone { p: #p } } },
            ColumnType::TSQuery => { quote! { ::teo::teo_column_type::postgres::ColumnType::TSQuery } },
            ColumnType::TSVector => { quote! { ::teo::teo_column_type::postgres::ColumnType::TSVector } },
            ColumnType::TxIDSnapshot => { quote! { ::teo::teo_column_type::postgres::ColumnType::TxIDSnapshot } },
            ColumnType::UUID => { quote! { ::teo::teo_column_type::postgres::ColumnType::UUID } },
            ColumnType::XML => { quote! { ::teo::teo_column_type::postgres::ColumnType::XML } },
        }
    }
}
