use std::cmp::Ordering;
use std::collections::HashMap;
use maplit::hashmap;
use sqlx::any::AnyRow;
use sqlx::Row;
use crate::connectors::sql::schema::dialect::SQLDialect;
use crate::core::field::Sort;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::model::Model;
use crate::prelude::Value;

pub(crate) struct IndexDecoder { }

impl IndexDecoder {
    fn decode(model: &Model, rows: Vec<AnyRow>, dialect: SQLDialect) -> Vec<ModelIndex> {
        let mut indices: Vec<ModelIndex> = Vec::new();
        let mut items: HashMap<String, HashMap<i32, ModelIndexItem>> = HashMap::new();
        for row in rows.iter() {
            let index_name: String = row.get("Key_name");
            let non_unique: i32 = row.get("Non_unique");
            if !items.contains_key(&index_name) {
                items.insert(index_name.to_string(), hashmap!{});
                let r#type = if &index_name == "PRIMARY" {
                    ModelIndexType::Primary
                } else if non_unique == 0 {
                    ModelIndexType::Unique
                } else {
                    ModelIndexType::Index
                };
                indices.push(ModelIndex::new(r#type, index_name.clone(), vec![]));
            }
            let column_name: String = row.get("Column_name");
            let field_name = model.field_with_column_name(&column_name).unwrap().name();
            let collation: String = row.get("Collation");
            let sort = if &collation == "A" { Sort::Asc } else { Sort::Desc };
            let seq: i32 = row.get("Seq_in_index");
            items.get_mut(&index_name).unwrap().insert(seq, ModelIndexItem::new(field_name.to_string(), sort, None));
        }
        let mut retval: Vec<ModelIndex> = vec![];
        for index in indices.iter() {
            let mut items = items.get(index.name()).unwrap().iter().collect::<Vec<(&i32, &ModelIndexItem)>>();
            items.sort_by(|(k1, _), (k2, _)| {
                k1.cmp(k2)
            });
            let items = items.into_iter().map(|(k, v)| v.clone()).collect::<Vec<ModelIndexItem>>();
            retval.push(ModelIndex::new(index.r#type(), index.name(), items));
        }
        retval
    }
}
