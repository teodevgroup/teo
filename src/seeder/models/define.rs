use std::sync::Arc;
use crate::core::field::{Field, Sort};
use crate::core::field::optionality::Optionality;
use crate::core::field::r#type::FieldType;
use crate::core::model::builder::ModelBuilder;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::pipeline::items::string::generation::cuid::CUIDItem;
use crate::core::pipeline::Pipeline;
use crate::prelude::{GraphBuilder, Value};

pub(crate) fn define_seeder_models(graph_builder: &mut GraphBuilder) {
    graph_builder.model("__TeoGroupRecord", |m| {
        m.table_name = "__teogrouprecord".to_owned();
        m.teo_internal = true;
        install_string_id_and_dataset(m);
        install_plain_required_string_field(m, "group");
        install_plain_required_string_field(m, "name");
        install_plain_required_string_field(m, "record");
    });
    graph_builder.model("__TeoGroupRelation", |m| {
        m.table_name = "__teogrouprelation".to_owned();
        m.teo_internal = true;
        install_string_id_and_dataset(m);
        install_plain_required_string_field(m, "groupA");
        install_plain_required_string_field(m, "relationA");
        install_plain_required_string_field(m, "nameA");
        install_plain_required_string_field(m, "groupB");
        install_plain_optional_string_field(m, "relationB");
        install_plain_required_string_field(m, "nameB");
    });
}

fn install_string_id_and_dataset(m: &mut ModelBuilder) {
    let mut id_field = Field::new("id".to_owned());
    id_field.field_type = Some(FieldType::String);
    id_field.primary = true;
    let mut pipeline = Pipeline::new();
    pipeline.items.push(Arc::new(CUIDItem::new()));
    id_field.default = Some(Value::Pipeline(pipeline));
    m.field(id_field);
    m.primary = Some(ModelIndex::new(ModelIndexType::Primary, None::<String>, vec![
        ModelIndexItem::new("id".to_string(), Sort::Asc, None)
    ]));
    let mut data_set_field = Field::new("dataset".to_owned());
    data_set_field.field_type = Some(FieldType::String);
    m.field(data_set_field);
}

fn install_plain_required_string_field(m: &mut ModelBuilder, field_name: &str) {
    let mut new_field = Field::new(field_name.to_owned());
    new_field.field_type = Some(FieldType::String);
    m.field(new_field);
}

fn install_plain_optional_string_field(m: &mut ModelBuilder, field_name: &str) {
    let mut new_field = Field::new(field_name.to_owned());
    new_field.field_type = Some(FieldType::String);
    new_field.optionality = Optionality::Optional;
    m.field(new_field);
}