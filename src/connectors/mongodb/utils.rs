use std::collections::HashMap;
use bson::{Bson, doc, Document};
use crate::connectors::mongodb::aggregation_builder::ToBsonValue;
use crate::core::model::Model;
use crate::core::value::Value;

pub fn bson_identifier(identifier: &HashMap<&str, Value>, model: &Model) -> Document {
    let mut val = doc! {};
    for (key, value) in identifier {
        let field = model.field(key).unwrap();
        let column_name = field.column_name();
        val.insert(column_name, value.to_bson_value());
    }
    val
}
