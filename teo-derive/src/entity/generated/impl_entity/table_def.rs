macro_rules! make_table_def {
    ($fn_name:ident, $return_type:path, $config_name:ident) => {
        pub(in crate::entity) fn $fn_name(
            opts: crate::entity::types::EntityDef,
        ) -> ::syn::Result<::proc_macro2::TokenStream> {
            let table_name = opts.table_name();
            let fields = opts.data.take_struct().unwrap().fields;
            let columns: Vec<::proc_macro2::TokenStream> = fields.iter().filter_map(|field_def| {
                if let Some(ident) = &field_def.ident {
                    let column_name = field_def.column_name.clone().unwrap_or(ident.to_string());
                    let nullable = crate::utils::is_std_option(&field_def.ty);
                    //let ty = field_def.postgres.clone().unwrap_or( sql_ty(&field_def.ty).unwrap());
                    let default = if let Some(default) = &field_def.default {
                        Some(::quote::quote! { Some(std::borrow::Cow::Borrowed(#default)) })
                    } else {
                        Some(::quote::quote! { None })
                    };
                    Some(::quote::quote! {
                        columns.push(::teo::migration::ColumnDef {
                            name: std::borrow::Cow::Borrowed(#column_name),
                            //ty: std::borrow::Cow::Borrowed(#ty),
                            nullable: #nullable,
                            default: #default,
                        });
                    })
                } else {
                    None
                }
            }).collect();
            Ok(::quote::quote! {
                fn $fn_name() -> ::teo::migration::TableDef<$return_type> {
                    let mut columns = Vec::new();
                    #( #columns )*
                    let mut indexes = Vec::new();
                    ::teo::migration::TableDef {
                        name: #table_name,
                        columns,
                        indexes
                    }
                }
            })
        }
    };
}
