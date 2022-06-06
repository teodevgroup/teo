use std::cmp::Ordering;
use std::collections::HashMap;
use mysql_async::{Row, Value};
use crate::connectors::mysql::mysql_column::ValueHelpers;
use crate::connectors::sql_shared::sql::{SQLColumnDef, SQLDialect};
use crate::core::field::Sort;
use crate::core::model::{ModelIndex, ModelIndexType};


#[derive(PartialEq, Copy, Clone)]
pub(crate) enum MySQLIndexItemCollation {
    A,
    D
}

#[derive(PartialEq)]
pub(crate) struct MySQLIndexItem {
    pub(crate) column_name: String,
    pub(crate) collation: MySQLIndexItemCollation,
}

#[derive(PartialEq)]
pub struct MySQLIndex {
    pub(crate) non_unique: bool,
    pub(crate) key_name: String,
    pub(crate) items: Vec<MySQLIndexItem>,
}

pub fn mysql_indices_from_rows(rows: &Vec<Row>) -> Vec<MySQLIndex> {
    let mut retval: Vec<MySQLIndex> = Vec::new();
    let mut result: HashMap<String, Vec<HashMap<String, &Value>>> = HashMap::new();
    for row in rows {
        let key_name = row["Key_name"].to_string();
        if result.get(&key_name).is_none() {
            let mut columns: Vec<HashMap<String, &Value>> = Vec::new();
            let mut column: HashMap<String, &Value> = HashMap::new();
            column.insert("seq_in_index".to_string(), &row["Seq_in_index"]);
            column.insert("collation".to_string(), &row["Collation"]);
            column.insert("column_name".to_string(), &row["Column_name"]);
            columns.push(column);
            result.insert(key_name, columns);
        } else {
            let mut columns = result.get_mut(&key_name).unwrap();
            let mut column: HashMap<String, &Value> = HashMap::new();
            column.insert("seq_in_index".to_string(), &row["Seq_in_index"]);
            column.insert("collation".to_string(), &row["Collation"]);
            column.insert("column_name".to_string(), &row["Column_name"]);
            columns.push(column);
        }
    }
    for (key, mut value) in result {
        let any_row = rows.iter().find(|r| r["Key_name"].to_string() == *key).unwrap();
        let non_unique_value = (&any_row["Non_unique"]).to_i64();
        let mut items: Vec<MySQLIndexItem> = Vec::new();
        value.sort_by(|v1, v2| {
            let a = v1.get("seq_in_index").unwrap().to_i64();
            let b = v2.get("seq_in_index").unwrap().to_i64();
            if a < b {
                Ordering::Less
            } else if a > b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        for column in value {
            let collation_string = column.get("collation").unwrap().to_string();
            let item = MySQLIndexItem {
                column_name: column.get("column_name").unwrap().to_string(),
                collation: if collation_string == "A" { MySQLIndexItemCollation::A } else { MySQLIndexItemCollation::D }
            };
            items.push(item);
        }
        retval.push(MySQLIndex {
            non_unique: non_unique_value == 1,
            key_name: key.clone(),
            items
        })
    }
    retval
}

impl From<&ModelIndex> for MySQLIndex {
    fn from(idx: &ModelIndex) -> Self {
        MySQLIndex {
            key_name: idx.name.clone(),
            non_unique: idx.index_type == ModelIndexType::Index,
            items: idx.items.iter().map(|item| {
                MySQLIndexItem {
                    column_name: item.field_name.clone(),
                    collation: if item.sort == Sort::Asc { MySQLIndexItemCollation::A } else { MySQLIndexItemCollation::D }
                }
            }).collect()
        }
    }
}
