use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result, Type, spanned::Spanned};
use teo_column_type::mongo::{self, ColumnType};
use crate::{entity::column_types::extended_column_type::ExtendedColumnType, utils::extract_first_path_argument};

impl ExtendedColumnType for mongo::ColumnType {
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
            | Type::Verbatim(_) => Err(Error::new(ty.span(), "teo(mongo): Can't figure out default column type.")),

            Type::Group(syn::TypeGroup { elem, .. })
            | Type::Paren(syn::TypeParen { elem, .. })
            | Type::Path(syn::TypePath {
                qself: Some(syn::QSelf { ty: elem, .. }),
                ..
            }) => Self::default_column_type(elem),

            Type::Path(syn::TypePath { qself: None, path }) => {
                if path.is_ident("bool") {
                    Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::Bool })
                } else if path.is_ident("i32") {
                    Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::Int32 })
                } else if path.is_ident("i64") {
                    Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::Long })
                } else if path.is_ident("f32") {
                    Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::Double })
                } else if path.is_ident("f64") {
                    Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::Double })
                } else if path.is_ident("String") {
                    Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::String })
                } else if path.is_ident("Uuid") {
                    Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::UUID })
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
                            Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::UUID })
                        } else if path.segments.len() == 3
                            && path.segments[0].ident == "std"
                            && path.segments[1].ident == "string"
                            && path.segments[2].ident == "String" {
                            Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::String })
                        } else if path.segments.len() == 1
                            && path.segments[0].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::Date })
                        } else if path.segments.len() == 2
                            && path.segments[0].ident == "chrono"
                            && path.segments[1].ident == "DateTime" {
                            Ok(quote! { ::teo::teo_column_type::mongo::ColumnType::Date })
                        } else {
                            Err(Error::new(ty.span(), "teo(mongo): Can't figure out default column type."))
                        }
                    }
                }
            },
            _ => Err(Error::new(ty.span(), "teo(mongo): Can't figure out default column type.")),
        }
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            ColumnType::Bool => quote! { ::teo::teo_column_type::mongo::ColumnType::Bool },
            ColumnType::String => quote! { ::teo::teo_column_type::mongo::ColumnType::String },
            ColumnType::Date => quote! { ::teo::teo_column_type::mongo::ColumnType::Date },
            ColumnType::ObjectId => quote! { ::teo::teo_column_type::mongo::ColumnType::ObjectId },
            ColumnType::Double => quote! { ::teo::teo_column_type::mongo::ColumnType::Double },
            ColumnType::Int32 => quote! { ::teo::teo_column_type::mongo::ColumnType::Int32 },
            ColumnType::Long => quote! { ::teo::teo_column_type::mongo::ColumnType::Long },
            ColumnType::Decimal128 => quote! { ::teo::teo_column_type::mongo::ColumnType::Decimal128 },
            ColumnType::Timestamp => quote! { ::teo::teo_column_type::mongo::ColumnType::Timestamp },
            ColumnType::UUID => quote! { ::teo::teo_column_type::mongo::ColumnType::UUID },
        }
    }
}
