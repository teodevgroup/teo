use std::sync::Arc;
use crate::core::field::field::{Field, Sort};
use crate::core::field::optionality::Optionality;
use crate::core::field::r#type::FieldType;
use crate::core::model::model::Model;
use crate::core::model::index::{ModelIndex, ModelIndexItem, ModelIndexType};
use crate::core::items::string::generation::cuid::CUIDItem;
use crate::core::pipeline::Pipeline;
use crate::prelude::{Graph, Value};

pub(crate) fn define_seeder_models(graph: &'static Graph) {
    let group_record_model_name = "__TeoGroupRecord";
    let mut group_record = Model::new(group_record_model_name, vec![], None, None);
    group_record.set_table_name("__teogrouprecord");
    group_record.set_is_teo_internal();
    install_string_id_and_dataset(&mut group_record);
    install_plain_required_string_field(&mut group_record, "group");
    install_plain_required_string_field(&mut group_record, "name");
    install_plain_required_string_field(&mut group_record, "record");
    graph.add_model(group_record, group_record_model_name);
    graph.model_mut(group_record_model_name).unwrap().finalize();
    let group_relation_model_name = "__TeoGroupRelation";
    let mut group_relation = Model::new(group_relation_model_name, vec![], None, None);
    group_relation.set_table_name("__teogrouprelation");
    group_relation.set_is_teo_internal();
    install_string_id_and_dataset(&mut group_relation);
    install_plain_required_string_field(&mut group_relation, "groupA");
    install_plain_required_string_field(&mut group_relation, "relationA");
    install_plain_required_string_field(&mut group_relation, "nameA");
    install_plain_required_string_field(&mut group_relation, "groupB");
    install_plain_optional_string_field(&mut group_relation, "relationB");
    install_plain_required_string_field(&mut group_relation, "nameB");
    graph.add_model(group_relation, group_relation_model_name);
    graph.model_mut(group_relation_model_name).unwrap().finalize();
}

fn install_string_id_and_dataset(m: &mut Model) {
    let mut id_field = Field::new("id");
    id_field.field_type = Some(FieldType::String);
    id_field.primary = true;
    let mut pipeline = Pipeline::new();
    pipeline.items.push(Arc::new(CUIDItem::new()));
    id_field.default = Some(Value::Pipeline(pipeline));
    id_field.finalize();
    m.add_field(id_field, "id");
    m.add_index(ModelIndex::new(ModelIndexType::Primary, None::<String>, vec![
        ModelIndexItem::new("id", Sort::Asc, None)
    ]));
    let mut data_set_field = Field::new("dataset");
    data_set_field.field_type = Some(FieldType::String);
    data_set_field.finalize();
    m.add_field(data_set_field, "dataset");
}

fn install_plain_required_string_field(m: &mut Model, field_name: &'static str) {
    let mut new_field = Field::new(field_name);
    new_field.field_type = Some(FieldType::String);
    new_field.finalize();
    m.add_field(new_field, field_name);
}

fn install_plain_optional_string_field(m: &mut Model, field_name: &'static str) {
    let mut new_field = Field::new(field_name);
    new_field.field_type = Some(FieldType::String);
    new_field.optionality = Optionality::Optional;
    new_field.finalize();
    m.add_field(new_field, field_name);
}