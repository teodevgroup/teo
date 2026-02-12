pub(in crate::schema) fn generate_mongo_table_defs(
    opts: crate::schema::types::SchemaDef,
) -> ::syn::Result<::proc_macro2::TokenStream> {
    let entities: Vec<::proc_macro2::TokenStream> = opts.entities.iter().map(|entity| {
        let path = &entity.path;
        ::quote::quote! {
            table_defs.push(<#path as ::teo::types::Entity>::mongo_table_def());
        }
    }).collect();
    Ok(::quote::quote! {
        fn mongo_table_defs() -> Vec<::teo::migration::TableDef<::teo::teo_column_type::mongo::ColumnType>> {
            let mut table_defs = Vec::new();
            #( #entities )*
            table_defs
        }
    })
}
