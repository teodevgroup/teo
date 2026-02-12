use std::collections::BTreeSet;
use crate::{migration::{ColumnDef, EnumDef, IndexColumnDef, IndexDef, TableDef}, types::Schema};

pub(crate) trait SyncMigration {

    type Err;

    type ColumnType: PartialEq + ToString;

    fn execute_without_params(&mut self, q: &str) -> Result<(), Self::Err>;

    fn ident_quote_char() -> &'static str;

    fn string_quote_char() -> &'static str;

    fn exist_enum_names(&mut self) -> Result<Vec<String>, Self::Err>;

    fn enum_create_statement(&self, enum_def: &EnumDef) -> String;

    fn enum_drop_statement(&self, enum_name: &str) -> String;

    fn add_enum_variant_statement(&self, enum_name: &str, variant_name: &str) -> String;

    fn exist_enum_def(&mut self, enum_name: &'static str) -> Result<EnumDef, Self::Err>;

    fn defined_table_defs<S>(&self) -> Vec<TableDef<Self::ColumnType>> where S: Schema;

    fn exist_table_names(&mut self) -> Result<Vec<String>, Self::Err>;

    fn drop_table_statement(&self, table_name: &str) -> String {
        format!("drop table if exists {}{}{}", Self::ident_quote_char(), table_name, Self::ident_quote_char())
    }

    fn create_table_statement(&self, table_def: &TableDef<Self::ColumnType>) -> String;

    fn column_statement(&self, column_def: &ColumnDef<Self::ColumnType>) -> String;

    fn index_column_statement(&self, index_column_def: &IndexColumnDef) -> String {
        format!("{}{}{} {}", Self::ident_quote_char(), index_column_def.name, Self::ident_quote_char(), index_column_def.order.as_ref())
    }

    fn create_index_statement(&self, table_name: &str, index_def: &IndexDef) -> String {
        let columns: Vec<String> = index_def.columns.iter().map(|c| self.index_column_statement(c)).collect();
        let columns_joined = columns.join(",");
        format!("create index {}{}{} on {}{}{}({})",
            Self::ident_quote_char(),
            index_def.name,
            Self::ident_quote_char(),
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            columns_joined)
    }

    fn exist_table_def(&mut self, table_name: &str) -> Result<TableDef<Self::ColumnType>, Self::Err>;

    fn drop_table_column_statement(&self, table_name: &str, column_name: &str) -> String {
        format!(r#"alter table {}{}{} drop column {}{}{}"#,
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            Self::ident_quote_char(),
            column_name,
            Self::ident_quote_char())
    }

    fn add_table_column_statement(&self, table_name: &str, column_def: &ColumnDef<Self::ColumnType>) -> String {
        format!("alter table {}{}{} add {}",
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            self.column_statement(column_def))
    }

    fn alter_table_column_type_statement(&self, table_name: &str, column_name: &str, column_ty: &Self::ColumnType) -> String {
        format!("alter table {}{}{} alter column {}{}{} type {}",
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            Self::ident_quote_char(),
            column_name,
            Self::ident_quote_char(),
            column_ty.to_string())
    }

    fn alter_table_column_set_not_null_statement(&self, table_name: &str, column_name: &str) -> String {
        format!("alter table {}{}{} alter column {}{}{} set not null",
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            Self::ident_quote_char(),
            column_name,
            Self::ident_quote_char())
    }

    fn alter_table_column_drop_not_null_statement(&self, table_name: &str, column_name: &str) -> String {
        format!("alter table {}{}{} alter column {}{}{} drop not null",
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            Self::ident_quote_char(),
            column_name,
            Self::ident_quote_char())
    }

    fn alter_table_column_set_default_statement(&self, table_name: &str, column_name: &str, default: &str) -> String {
        format!("alter table {}{}{} alter column {}{}{} set default {}",
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            Self::ident_quote_char(),
            column_name,
            Self::ident_quote_char(),
            default)
    }

    fn alter_table_column_drop_default_statement(&self, table_name: &str, column_name: &str) -> String {
        format!("alter table {}{}{} alter column {}{}{} drop default",
            Self::ident_quote_char(),
            table_name,
            Self::ident_quote_char(),
            Self::ident_quote_char(),
            column_name,
            Self::ident_quote_char())
    }

    fn drop_index_statement(&self, index_name: &str) -> String {
        format!("drop index if exists {}{}{}", Self::ident_quote_char(), index_name, Self::ident_quote_char())
    }

    fn migrate<S>(&mut self) -> Result<(), Self::Err> where S: Schema {
        let defined_enum_defs = S::enum_defs();
        let exist_enum_names_vec = self.exist_enum_names()?;
        let defined_enum_names = BTreeSet::from_iter(defined_enum_defs.iter().map(|t| t.name.as_ref()));
        let exist_enum_names = BTreeSet::from_iter(exist_enum_names_vec.iter().map(|s| s.as_str()));
        let enums_to_create = defined_enum_names.difference(&exist_enum_names);
        for enum_name in enums_to_create {
            if let Some(enum_def) = defined_enum_defs.iter().find(|def| def.name == *enum_name) {
                self.create_enum(enum_def)?;
            }
        }
        let enums_to_diff = exist_enum_names.intersection(&defined_enum_names);
        for enum_name in enums_to_diff {
            if let Some(enum_def) = defined_enum_defs.iter().find(|def| def.name == *enum_name) {
                self.diff_enum(enum_def)?;
            }
        }
        self.diff_tables::<S>(&defined_enum_defs)?;
        let enums_to_delete = exist_enum_names.difference(&defined_enum_names);
        for enum_name in enums_to_delete {
            self.delete_enum(*enum_name)?;
        }
        Ok(())

    }

    fn create_enum(&mut self, enum_def: &EnumDef) -> Result<(), Self::Err> {
        let statement = self.enum_create_statement(enum_def);
        self.execute_without_params(&statement)
    }

    fn diff_enum(&mut self, defined_enum_def: &EnumDef) -> Result<(), Self::Err> {
        let exist_enum_def = self.exist_enum_def(defined_enum_def.name)?;
        let defined_varaints: BTreeSet<&str> = defined_enum_def.variants.iter().map(|c| c.as_ref()).collect();
        let exist_variants: BTreeSet<&str> = exist_enum_def.variants.iter().map(|c| c.as_ref()).collect();
        let variants_to_add = defined_varaints.difference(&exist_variants);
        for variant in variants_to_add {
            self.add_enum_variant(exist_enum_def.name, *variant)?;
        }
        Ok(())
    }

    fn delete_enum(&mut self, enum_name: &str) -> Result<(), Self::Err> {
        let statement = self.enum_drop_statement(enum_name);
        self.execute_without_params(&statement)
    }

    fn add_enum_variant(&mut self, enum_name: &str, variant_name: &str) -> Result<(), Self::Err> {
        let add_variant_statement = self.add_enum_variant_statement(enum_name, variant_name);
        self.execute_without_params(&add_variant_statement)
    }

    fn diff_tables<S>(&mut self, _defined_enum_defs: &Vec<EnumDef>) -> Result<(), Self::Err> where S: Schema {
        let defined_table_defs = self.defined_table_defs::<S>();
        let exist_table_names_vec = self.exist_table_names()?;
        let exist_table_names: BTreeSet<&str> = BTreeSet::from_iter(exist_table_names_vec.iter().map(|s| s.as_str()));
        let defined_table_names: BTreeSet<&str> = BTreeSet::from_iter(defined_table_defs.iter().map(|t| t.name.as_ref()));
        let tables_to_delete = exist_table_names.difference(&defined_table_names);
        for table_name in tables_to_delete {
            self.delete_table(*table_name)?;
        }
        let tables_to_create = defined_table_names.difference(&exist_table_names);
        for table_name in tables_to_create {
            if let Some(table_def) = defined_table_defs.iter().find(|def| def.name == *table_name) {
                self.create_table(table_def)?;
            }
        }
        let tables_to_diff = exist_table_names.intersection(&defined_table_names);
        for table_name in tables_to_diff {
            if let Some(defined_table_def) = defined_table_defs.iter().find(|def| def.name == *table_name) {
                let exist_table_def = self.exist_table_def(&defined_table_def.name)?;
                self.diff_table(defined_table_def, &exist_table_def)?;
            }
        }
        Ok(())
    }

    fn delete_table(&mut self, table_name: &str) -> Result<(), Self::Err> {
        let statement = self.drop_table_statement(table_name);
        self.execute_without_params(&statement)
    }

    fn create_table(&mut self, table_def: &TableDef<Self::ColumnType>) -> Result<(), Self::Err> {
        let statement = self.create_table_statement(table_def);
        self.execute_without_params(&statement)?;
        for index_def in &table_def.indexes {
            self.create_index(&table_def.name, index_def)?;
        }
        Ok(())
    }

    fn diff_table(&mut self, defined_table_def: &TableDef<Self::ColumnType>, exist_table_def: &TableDef<Self::ColumnType>) -> Result<(), Self::Err> {
        self.diff_table_columns(defined_table_def, exist_table_def)?;
        self.diff_table_indexes(defined_table_def, exist_table_def)?;
        Ok(())
    }

    fn diff_table_columns(&mut self, defined_table_def: &TableDef<Self::ColumnType>, exist_table_def: &TableDef<Self::ColumnType>) -> Result<(), Self::Err> {
        let defined_column_names: BTreeSet<&str> = defined_table_def.columns.iter().map(|c| c.name.as_ref()).collect();
        let exist_column_names: BTreeSet<&str> = exist_table_def.columns.iter().map(|c| c.name.as_ref()).collect();
        let columns_to_delete = exist_column_names.difference(&defined_column_names);
        for column_name in columns_to_delete {
            self.drop_table_column(&defined_table_def.name, *column_name)?;
        }
        let columns_to_add = defined_column_names.difference(&exist_column_names);
        for column_name in columns_to_add {
            if let Some(defined_column_def) = defined_table_def.columns.iter().find(|def| def.name == *column_name) {
                self.add_table_column(&defined_table_def.name, defined_column_def)?;
            }
        }
        let columns_to_diff = exist_column_names.intersection(&defined_column_names);
        for column_name in columns_to_diff {
            if let Some(defined_column_def) = defined_table_def.columns.iter().find(|def| def.name == *column_name) &&
                    let Some(exist_column_def) = exist_table_def.columns.iter().find(|def| def.name == *column_name) {
                self.diff_table_column(&defined_table_def.name, defined_column_def, exist_column_def)?;
            }
        }
        Ok(())

    }

    fn drop_table_column(&mut self, table_name: &str, column_name: &str) -> Result<(), Self::Err> {
        let statement = self.drop_table_column_statement(table_name, column_name);
        self.execute_without_params(&statement)
    }

    fn add_table_column(&mut self, table_name: &str, column_def: &ColumnDef<Self::ColumnType>) -> Result<(), Self::Err> {
        let statement = self.add_table_column_statement(table_name, column_def);
        self.execute_without_params(&statement)
    }

    fn diff_table_column(&mut self, table_name: &str, defined_column_def: &ColumnDef<Self::ColumnType>, exist_column_def: &ColumnDef<Self::ColumnType>) -> Result<(), Self::Err> {
        if defined_column_def.ty != exist_column_def.ty {
            self.alter_table_column_type(table_name, &defined_column_def.name, &defined_column_def.ty)?;
        }
        if defined_column_def.nullable != exist_column_def.nullable {
            self.alter_table_column_nullable(table_name, &defined_column_def.name, defined_column_def.nullable)?;
        }
        if defined_column_def.default != exist_column_def.default {
            self.alter_table_column_default(table_name, &defined_column_def.name, defined_column_def.default.as_deref())?;
        }
        Ok(())
    }

    fn alter_table_column_type(&mut self, table_name: &str, column_name: &str, column_ty: &Self::ColumnType) -> Result<(), Self::Err> {
        let statement = self.alter_table_column_type_statement(table_name, column_name, column_ty);
        self.execute_without_params(&statement)
    }

    fn alter_table_column_nullable(&mut self, table_name: &str, column_name: &str, nullable: bool) -> Result<(), Self::Err> {
        let statement = if nullable {
            self.alter_table_column_drop_not_null_statement(table_name, column_name)
        } else {
            self.alter_table_column_set_not_null_statement(table_name, column_name)
        };
        self.execute_without_params(&statement)
    }

    fn alter_table_column_default(&mut self, table_name: &str, column_name: &str, default: Option<&str>) -> Result<(), Self::Err> {
        let statement = if let Some(default) = default {
            self.alter_table_column_set_default_statement(table_name, column_name, default)
        } else {
            self.alter_table_column_drop_default_statement(table_name, column_name)
        };
        self.execute_without_params(&statement)
    }

    fn create_index(&mut self, table_name: &str, index_def: &IndexDef) -> Result<(), Self::Err> {
        let statement = self.create_index_statement(table_name, index_def);
        self.execute_without_params(&statement)
    }

    fn diff_table_indexes(&mut self, defined_table_def: &TableDef<Self::ColumnType>, exist_table_def: &TableDef<Self::ColumnType>) -> Result<(), Self::Err> {
        let defined_index_names: BTreeSet<&str> = defined_table_def.indexes.iter().map(|c| c.name.as_ref()).collect();
        let exist_index_names: BTreeSet<&str> = exist_table_def.indexes.iter().map(|c| c.name.as_ref()).collect();
        let indexes_to_delete = exist_index_names.difference(&defined_index_names);
        for index_name in indexes_to_delete {
            self.drop_index(index_name)?;
        }
        let indexes_to_create = defined_index_names.difference(&exist_index_names);
        for index_name in indexes_to_create {
            if let Some(defined_index_def) = defined_table_def.indexes.iter().find(|def| def.name == *index_name) {
                self.create_index(&defined_table_def.name, defined_index_def)?;
            }
        }
        let indexes_to_diff = exist_index_names.intersection(&defined_index_names);
        for index_name in indexes_to_diff {
            if let Some(defined_index_def) = defined_table_def.indexes.iter().find(|def| def.name == *index_name) &&
            let Some(exist_index_def) = exist_table_def.indexes.iter().find(|def| def.name == *index_name) {
                if defined_index_def != exist_index_def {
                    self.drop_index(&exist_index_def.name)?;
                    self.create_index(&defined_table_def.name, defined_index_def)?;
                }
            }
        }
        Ok(())
    }

    fn drop_index(&mut self, index_name: &str) -> Result<(), Self::Err> {
        let statement = self.drop_index_statement(index_name);
        self.execute_without_params(&statement)
    }
}
