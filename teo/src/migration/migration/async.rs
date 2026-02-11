use std::collections::BTreeSet;
use crate::{migration::EnumDef, types::Schema};

pub(crate) trait AsyncMigration: Sync {

    type Err;

    fn execute_without_params(&self, q: &str) -> impl Future<Output = Result<(), Self::Err>> + Send;

    fn ident_quote_char() -> &'static str;

    fn string_quote_char() -> &'static str;

    fn exist_enum_names(&self) -> impl Future<Output = Result<Vec<String>, Self::Err>> + Send;

    fn enum_create_statement(&self, enum_def: &EnumDef) -> String;

    fn enum_drop_statement(&self, enum_name: &str) -> String;

    fn add_enum_variant_statement(&self, enum_name: &str, variant_name: &str) -> String;

    fn exist_enum_def(&self, enum_name: &str) -> impl Future<Output = Result<Option<EnumDef>, Self::Err>> + Send;

    fn migrate<S>(&self) -> impl Future<Output = Result<(), Self::Err>> + Send where S: Schema {
        async {
            let defined_enum_defs = S::enum_defs();
            let exist_enum_names_vec = self.exist_enum_names().await?;
            let defined_enum_names = BTreeSet::from_iter(defined_enum_defs.iter().map(|t| t.name.as_ref()));
            let exist_enum_names = BTreeSet::from_iter(exist_enum_names_vec.iter().map(|s| s.as_str()));
            let enums_to_create = defined_enum_names.difference(&exist_enum_names);
            for enum_name in enums_to_create {
                if let Some(enum_def) = defined_enum_defs.iter().find(|def| def.name == *enum_name) {
                    self.create_enum(enum_def).await?;
                }
            }
            let enums_to_diff = exist_enum_names.intersection(&defined_enum_names);
            for enum_name in enums_to_diff {
                if let Some(enum_def) = defined_enum_defs.iter().find(|def| def.name == *enum_name) {
                    self.diff_enum(enum_def).await?;
                }
            }
            // db tables here
            let enums_to_delete = exist_enum_names.difference(&defined_enum_names);
            for enum_name in enums_to_delete {
                self.delete_enum(*enum_name).await?;
            }
            Ok(())
        }
    }

    fn create_enum(&self, enum_def: &EnumDef) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.enum_create_statement(enum_def);
            self.execute_without_params(&statement).await
        }
    }

    fn diff_enum(&self, defined_enum_def: &EnumDef) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let Some(exist_enum_def) = self.exist_enum_def(defined_enum_def.name).await? else { return Ok(()) };
            let defined_varaints: BTreeSet<&str> = defined_enum_def.variants.iter().map(|c| c.as_ref()).collect();
            let exist_variants: BTreeSet<&str> = exist_enum_def.variants.iter().map(|c| c.as_ref()).collect();
            let variants_to_add = defined_varaints.difference(&exist_variants);
            for variant in variants_to_add {
                let add_variant_statement = self.add_enum_variant_statement(exist_enum_def.name, *variant);
                self.execute_without_params(&add_variant_statement).await?;
            }
            Ok(())
        }
    }

    fn delete_enum(&self, enum_name: &str) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.enum_drop_statement(enum_name);
            self.execute_without_params(&statement).await
        }
    }
}
