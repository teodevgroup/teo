use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::{entity::types::EntityDef, utils::{is_std_option, sql_ty}};

pub(in super::super) fn generate_impl_entity(opts: EntityDef) -> Result<TokenStream> {
    let table_name = opts.table_name();
    let struct_ident = opts.ident;
    let fields = opts.data.take_struct().unwrap().fields;
    let columns: Vec<TokenStream> = fields.iter().filter_map(|field_def| {
        if let Some(ident) = &field_def.ident {
            let column_name = field_def.column_name.clone().unwrap_or(ident.to_string());
            let nullable = is_std_option(&field_def.ty);
            let ty = field_def.column_type.clone().unwrap_or( sql_ty(&field_def.ty).unwrap());
            let default = if let Some(default) = &field_def.default {
                Some(quote! { Some(std::borrow::Cow::Borrowed(#default)) })
            } else {
                Some(quote! { None })
            };
            Some(quote! {
                columns.push(::teo::migration::ColumnDef {
                    name: std::borrow::Cow::Borrowed(#column_name),
                    ty: std::borrow::Cow::Borrowed(#ty),
                    nullable: #nullable,
                    default: #default,
                });
            })
        } else {
            None
        }
    }).collect();
    Ok(quote! {
        impl ::teo::types::Entity for #struct_ident {

        }
    })
}

            // fn table_def() -> ::teo::migration::TableDef {
            //     let mut columns = Vec::new();
            //     #( #columns )*
            //     let mut indexes = Vec::new();
            //     ::teo::migration::TableDef {
            //         name: #table_name,
            //         columns,
            //         indexes
            //     }
            // }
