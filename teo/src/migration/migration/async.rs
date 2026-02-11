use std::collections::BTreeSet;
use crate::{migration::{ColumnDef, EnumDef, IndexDef, TableDef}, types::Schema};

pub(crate) trait AsyncMigration: Sync {

    type Err: Send;

    type ColumnType: Send + Sync + PartialEq;

    fn execute_without_params(&self, q: &str) -> impl Future<Output = Result<(), Self::Err>> + Send;

    fn ident_quote_char() -> &'static str;

    fn string_quote_char() -> &'static str;

    fn exist_enum_names(&self) -> impl Future<Output = Result<Vec<String>, Self::Err>> + Send;

    fn enum_create_statement(&self, enum_def: &EnumDef) -> String;

    fn enum_drop_statement(&self, enum_name: &str) -> String;

    fn add_enum_variant_statement(&self, enum_name: &str, variant_name: &str) -> String;

    fn exist_enum_def(&self, enum_name: &str) -> impl Future<Output = Result<Option<EnumDef>, Self::Err>> + Send;

    fn exist_table_defs<S>(&self) -> Vec<TableDef<Self::ColumnType>> where S: Schema;

    fn exist_table_names(&self) -> impl Future<Output = Result<Vec<String>, Self::Err>> + Send;

    fn delete_table_statement(&self, table_name: &str) -> String;

    fn create_table_statement(&self, table_def: &TableDef<Self::ColumnType>) -> String;

    fn create_index_statement(&self, table_name: &str, index_def: &IndexDef) -> String;

    fn exist_table_def(&self, table_name: &str) -> impl Future<Output = Result<Option<TableDef<Self::ColumnType>>, Self::Err>> + Send;

    fn drop_table_column_statement(&self, table_name: &str, column_name: &str) -> String;

    fn add_table_column_statement(&self, table_name: &str, column_def: &ColumnDef<Self::ColumnType>) -> String;

    fn alter_table_column_type_statement(&self, table_name: &str, column_name: &str, column_ty: &Self::ColumnType) -> String;

    fn alter_table_column_set_not_null_statement(&self, table_name: &str, column_name: &str) -> String;

    fn alter_table_column_drop_not_null_statement(&self, table_name: &str, column_name: &str) -> String;

    fn alter_table_column_set_default_statement(&self, table_name: &str, column_name: &str, default: &str) -> String;

    fn alter_table_column_drop_default_statement(&self, table_name: &str, column_name: &str) -> String;

    fn drop_index_statement(&self, index_name: &str) -> String;

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
            self.diff_tables::<S>(&defined_enum_defs).await?;
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
                self.add_enum_variant(exist_enum_def.name, *variant).await?;
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

    fn add_enum_variant(&self, enum_name: &str, variant_name: &str) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let add_variant_statement = self.add_enum_variant_statement(enum_name, variant_name);
            self.execute_without_params(&add_variant_statement).await
        }
    }

    fn diff_tables<S>(&self, _defined_enum_defs: &Vec<EnumDef>) -> impl Future<Output = Result<(), Self::Err>> + Send where S: Schema {
        async {
            let defined_table_defs = self.exist_table_defs::<S>();
            let exist_table_names_vec = self.exist_table_names().await?;
            let exist_table_names: BTreeSet<&str> = BTreeSet::from_iter(exist_table_names_vec.iter().map(|s| s.as_str()));
            let defined_table_names: BTreeSet<&str> = BTreeSet::from_iter(defined_table_defs.iter().map(|t| t.name.as_ref()));
            let tables_to_delete = exist_table_names.difference(&defined_table_names);
            for table_name in tables_to_delete {
                self.delete_table(*table_name).await?;
            }
            let tables_to_create = defined_table_names.difference(&exist_table_names);
            for table_name in tables_to_create {
                if let Some(table_def) = defined_table_defs.iter().find(|def| def.name == *table_name) {
                    self.create_table(table_def).await?;
                }
            }
            let tables_to_diff = exist_table_names.intersection(&defined_table_names);
            for table_name in tables_to_diff {
                if let Some(defined_table_def) = defined_table_defs.iter().find(|def| def.name == *table_name) &&
                      let Some(exist_table_def) = self.exist_table_def(*table_name).await? {
                    self.diff_table(defined_table_def, &exist_table_def).await?;
                }
            }
            Ok(())
        }
    }

    fn delete_table(&self, table_name: &str) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.delete_table_statement(table_name);
            self.execute_without_params(&statement).await
        }
    }

    fn create_table(&self, table_def: &TableDef<Self::ColumnType>) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.create_table_statement(table_def);
            self.execute_without_params(&statement).await?;
            for index_def in &table_def.indexes {
                self.create_index(table_def.name, index_def).await?;
            }
            Ok(())
        }
    }

    fn diff_table(&self, defined_table_def: &TableDef<Self::ColumnType>, exist_table_def: &TableDef<Self::ColumnType>) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            self.diff_table_columns(defined_table_def, exist_table_def).await?;
            self.diff_table_indexes(defined_table_def, exist_table_def).await?;
            Ok(())
        }
    }

    fn diff_table_columns(&self, defined_table_def: &TableDef<Self::ColumnType>, exist_table_def: &TableDef<Self::ColumnType>) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let defined_column_names: BTreeSet<&str> = defined_table_def.columns.iter().map(|c| c.name.as_ref()).collect();
            let exist_column_names: BTreeSet<&str> = exist_table_def.columns.iter().map(|c| c.name.as_ref()).collect();
            let columns_to_delete = exist_column_names.difference(&defined_column_names);
            for column_name in columns_to_delete {
                self.drop_table_column(defined_table_def.name, *column_name).await?;
            }
            let columns_to_add = defined_column_names.difference(&exist_column_names);
            for column_name in columns_to_add {
                if let Some(defined_column_def) = defined_table_def.columns.iter().find(|def| def.name == *column_name) {
                    self.add_table_column(defined_table_def.name, defined_column_def).await?;
                }
            }
            let columns_to_diff = exist_column_names.intersection(&defined_column_names);
            for column_name in columns_to_diff {
                if let Some(defined_column_def) = defined_table_def.columns.iter().find(|def| def.name == *column_name) &&
                      let Some(exist_column_def) = exist_table_def.columns.iter().find(|def| def.name == *column_name) {
                    self.diff_table_column(defined_table_def.name, defined_column_def, exist_column_def).await?;
                }
            }
            Ok(())
        }
    }

    fn drop_table_column(&self, table_name: &str, column_name: &str) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.drop_table_column_statement(table_name, column_name);
            self.execute_without_params(&statement).await
        }
    }

    fn add_table_column(&self, table_name: &str, column_def: &ColumnDef<Self::ColumnType>) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.add_table_column_statement(table_name, column_def);
            self.execute_without_params(&statement).await
        }
    }

    fn diff_table_column(&self, table_name: &str, defined_column_def: &ColumnDef<Self::ColumnType>, exist_column_def: &ColumnDef<Self::ColumnType>) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            if defined_column_def.ty != exist_column_def.ty {
                self.alter_table_column_type(table_name, &defined_column_def.name, &defined_column_def.ty).await?;
            }
            Ok(())
        }
    }

    fn alter_table_column_type(&self, table_name: &str, column_name: &str, column_ty: &Self::ColumnType) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.alter_table_column_type_statement(table_name, column_name, column_ty);
            self.execute_without_params(&statement).await
        }
    }

    fn alter_table_column_nullable(&self, table_name: &str, column_name: &str, nullable: bool) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async move {
            let statement = if nullable {
                self.alter_table_column_drop_not_null_statement(table_name, column_name)
            } else {
                self.alter_table_column_set_not_null_statement(table_name, column_name)
            };
            self.execute_without_params(&statement).await
        }
    }

    fn alter_table_column_default(&self, table_name: &str, column_name: &str, default: Option<&str>) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async move {
            let statement = if let Some(default) = default {
                self.alter_table_column_set_default_statement(table_name, column_name, default)
            } else {
                self.alter_table_column_drop_default_statement(table_name, column_name)
            };
            self.execute_without_params(&statement).await
        }
    }

    fn create_index(&self, table_name: &str, index_def: &IndexDef) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.create_index_statement(table_name, index_def);
            self.execute_without_params(&statement).await
        }
    }

    fn diff_table_indexes(&self, defined_table_def: &TableDef<Self::ColumnType>, exist_table_def: &TableDef<Self::ColumnType>) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let defined_index_names: BTreeSet<&str> = defined_table_def.indexes.iter().map(|c| c.name.as_ref()).collect();
            let exist_index_names: BTreeSet<&str> = exist_table_def.indexes.iter().map(|c| c.name.as_ref()).collect();
            let indexes_to_delete = exist_index_names.difference(&defined_index_names);
            for index_name in indexes_to_delete {
                self.drop_index(index_name).await?;
            }
            let indexes_to_create = defined_index_names.difference(&exist_index_names);
            for index_name in indexes_to_create {
                if let Some(defined_index_def) = defined_table_def.indexes.iter().find(|def| def.name == *index_name) {
                    self.create_index(defined_table_def.name, defined_index_def).await?;
                }
            }
            let indexes_to_diff = exist_index_names.intersection(&defined_index_names);
            for index_name in indexes_to_diff {
                if let Some(defined_index_def) = defined_table_def.indexes.iter().find(|def| def.name == *index_name) &&
                let Some(exist_index_def) = exist_table_def.indexes.iter().find(|def| def.name == *index_name) {
                    if defined_index_def != exist_index_def {
                        self.drop_index(&exist_index_def.name).await?;
                        self.create_index(defined_table_def.name, defined_index_def).await?;
                    }
                }
            }
            Ok(())
        }
    }

    fn drop_index(&self, index_name: &str) -> impl Future<Output = Result<(), Self::Err>> + Send {
        async {
            let statement = self.drop_index_statement(index_name);
            self.execute_without_params(&statement).await
        }
    }
}
