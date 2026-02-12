macro_rules! make_table_defs {
    ($name:ident, $fn_name:ident, $table_def_fn_name:ident, $config_name:ident) => {
        pub(in crate::schema) fn $name(
            opts: crate::schema::types::SchemaDef,
        ) -> ::syn::Result<::proc_macro2::TokenStream> {
            let entities: Vec<::proc_macro2::TokenStream> = opts.entities.iter().map(|entity| {
                let path = &entity.path;
                ::quote::quote! {
                    table_defs.push(<#path as ::teo::types::Entity>::$table_def_fn_name());
                }
            }).collect();
            Ok(::quote::quote! {
                fn $fn_name() -> Vec<::teo::migration::TableDef<::teo::teo_column_type::$config_name::ColumnType>> {
                    let mut table_defs = Vec::new();
                    #( #entities )*
                    table_defs
                }
            })
        }
    };
}
