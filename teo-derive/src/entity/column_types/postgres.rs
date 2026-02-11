use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, spanned::Spanned};
use teo_column_type::PostgresColumnType;
use crate::{entity::column_types::extended_column_type::ExtendedColumnType, utils::extract_first_path_argument};

impl ExtendedColumnType for PostgresColumnType {
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
                    Ok(quote! { ::teo::teo_column_type::PostgresColumnType::Boolean })
                } else if path.is_ident("i32") {
                    Ok(quote! { ::teo::teo_column_type::PostgresColumnType::Integer })
                } else if path.is_ident("i64") {
                    Ok(quote! { ::teo::teo_column_type::PostgresColumnType::BigInt })
                } else if path.is_ident("f32") {
                    Ok(quote! { ::teo::teo_column_type::PostgresColumnType::Real })
                } else if path.is_ident("f64") {
                    Ok(quote! { ::teo::teo_column_type::PostgresColumnType::DoublePrecision })
                } else if path.is_ident("String") {
                    Ok(quote! { ::teo::teo_column_type::PostgresColumnType::Text })
                } else if path.is_ident("Uuid") {
                    Ok(quote! { ::teo::teo_column_type::PostgresColumnType::UUID })
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
                            Ok(quote! { ::teo::teo_column_type::PostgresColumnType::UUID })
                        } else if path.segments.len() == 3
                            && path.segments[0].ident == "std"
                            && path.segments[1].ident == "string"
                            && path.segments[2].ident == "String" {
                            Ok(quote! { ::teo::teo_column_type::PostgresColumnType::Text })
                        } else if path.segments.len() == 1
                            && path.segments[0].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::PostgresColumnType::TimestampWithTimeZone { n: 6 } })
                        } else if path.segments.len() == 2
                            && path.segments[0].ident == "chrono"
                            && path.segments[1].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::PostgresColumnType::TimestampWithTimeZone { n: 6 } })
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
            PostgresColumnType::BigInt => { quote! { ::teo::teo_column_type::PostgresColumnType::BigInt } },
            PostgresColumnType::BigSerial => { quote! { ::teo::teo_column_type::PostgresColumnType::BigSerial } },
            PostgresColumnType::Bit { n } => { quote! { ::teo::teo_column_type::PostgresColumnType::Bit { n: #n } } },
            PostgresColumnType::BitVarying { n } => { quote! { ::teo::teo_column_type::PostgresColumnType::BitVarying { n: #n } } },
            PostgresColumnType::Boolean => { quote! { ::teo::teo_column_type::PostgresColumnType::Boolean } },
            PostgresColumnType::Box => { quote! { ::teo::teo_column_type::PostgresColumnType::Box } },
            PostgresColumnType::ByteA => { quote! { ::teo::teo_column_type::PostgresColumnType::ByteA } },
            PostgresColumnType::Character { n } => { quote! { ::teo::teo_column_type::PostgresColumnType::Character { n: #n } } },
            PostgresColumnType::CharacterVarying { n } => { quote! { ::teo::teo_column_type::PostgresColumnType::CharacterVarying { n: #n } } },
            PostgresColumnType::CIDR => { quote! { ::teo::teo_column_type::PostgresColumnType::CIDR } },
            PostgresColumnType::Circle => { quote! { ::teo::teo_column_type::PostgresColumnType::Circle } },
            PostgresColumnType::Date => { quote! { ::teo::teo_column_type::PostgresColumnType::Date } },
            PostgresColumnType::DoublePrecision => { quote! { ::teo::teo_column_type::PostgresColumnType::DoublePrecision } },
            PostgresColumnType::INet => { quote! { ::teo::teo_column_type::PostgresColumnType::INet } },
            PostgresColumnType::Integer => { quote! { ::teo::teo_column_type::PostgresColumnType::Integer } },
            PostgresColumnType::JSON => { quote! { ::teo::teo_column_type::PostgresColumnType::JSON } },
            PostgresColumnType::JSONB => { quote! { ::teo::teo_column_type::PostgresColumnType::JSONB } },
            PostgresColumnType::Line => { quote! { ::teo::teo_column_type::PostgresColumnType::Line } },
            PostgresColumnType::LSeg => { quote! { ::teo::teo_column_type::PostgresColumnType::LSeg } },
            PostgresColumnType::MACAddr => { quote! { ::teo::teo_column_type::PostgresColumnType::MACAddr } },
            PostgresColumnType::MACAddr8 => { quote! { ::teo::teo_column_type::PostgresColumnType::MACAddr8 } },
            PostgresColumnType::Money => { quote! { ::teo::teo_column_type::PostgresColumnType::Money } },
            PostgresColumnType::Numeric { p, s } => { quote! { ::teo::teo_column_type::PostgresColumnType::Numeric { p: #p, s: #s } } },
            PostgresColumnType::Path => { quote! { ::teo::teo_column_type::PostgresColumnType::Path } },
            PostgresColumnType::PgLSN => { quote! { ::teo::teo_column_type::PostgresColumnType::PgLSN } },
            PostgresColumnType::PGSnapshot => { quote! { ::teo::teo_column_type::PostgresColumnType::PGSnapshot } },
            PostgresColumnType::Point => { quote! { ::teo::teo_column_type::PostgresColumnType::Point } },
            PostgresColumnType::Polygon => { quote! { ::teo::teo_column_type::PostgresColumnType::Polygon } },
            PostgresColumnType::Real => { quote! { ::teo::teo_column_type::PostgresColumnType::Real } },
            PostgresColumnType::SmallInt => { quote! { ::teo::teo_column_type::PostgresColumnType::SmallInt } },
            PostgresColumnType::SmallSerial => { quote! { ::teo::teo_column_type::PostgresColumnType::SmallSerial } },
            PostgresColumnType::Serial => { quote! { ::teo::teo_column_type::PostgresColumnType::Serial } },
            PostgresColumnType::Text => { quote! { ::teo::teo_column_type::PostgresColumnType::Text } },
            PostgresColumnType::TimeWithoutTimeZone { p } => { quote! { ::teo::teo_column_type::PostgresColumnType::TimeWithoutTimeZone { p: #p } } },
            PostgresColumnType::TimeWithTimeZone { p } => { quote! { ::teo::teo_column_type::PostgresColumnType::TimeWithTimeZone { p: #p } } },
            PostgresColumnType::TimestampWithoutTimeZone { p } => { quote! { ::teo::teo_column_type::PostgresColumnType::TimestampWithoutTimeZone { p: #p } } },
            PostgresColumnType::TimestampWithTimeZone { p } => { quote! { ::teo::teo_column_type::PostgresColumnType::TimestampWithTimeZone { p: #p } } },
            PostgresColumnType::TSQuery => { quote! { ::teo::teo_column_type::PostgresColumnType::TSQuery } },
            PostgresColumnType::TSVector => { quote! { ::teo::teo_column_type::PostgresColumnType::TSVector } },
            PostgresColumnType::TxIDSnapshot => { quote! { ::teo::teo_column_type::PostgresColumnType::TxIDSnapshot } },
            PostgresColumnType::UUID => { quote! { ::teo::teo_column_type::PostgresColumnType::UUID } },
            PostgresColumnType::XML => { quote! { ::teo::teo_column_type::PostgresColumnType::XML } },
        }
    }
}
