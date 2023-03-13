use mongodb::IndexModel;
use crate::core::field::Sort;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};

impl From<&IndexModel> for ModelIndex {
    fn from(index_model: &IndexModel) -> Self {
        let unique_result = index_model.options.as_ref().unwrap().unique;
        let unique = match unique_result {
            Some(bool) => bool,
            None => false
        };
        let mut items: Vec<ModelIndexItem> = Vec::new();
        for (k, v) in &index_model.keys {
            let item = ModelIndexItem::new(k, if v.as_i32().unwrap() == 1 { Sort::Asc } else { Sort::Desc }, None);
            items.push(item);
        }
        ModelIndex::new(if unique { ModelIndexType::Unique } else { ModelIndexType::Index }, Some(index_model.options.as_ref().unwrap().name.as_ref().unwrap().to_string()), items)
    }
}
