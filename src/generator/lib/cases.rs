use inflector::Inflector;
use crate::core::field::Field;
use crate::core::model::Model;
use crate::core::relation::Relation;

pub(crate) fn model_localized_name(model: &Model) -> String {
    if model.localized_name().is_empty() {
        model.name().to_title_case()
    } else {
        model.localized_name().to_owned()
    }
}

pub(crate) fn model_localized_name_word_case(model: &Model) -> String {
    if model.localized_name().is_empty() {
        model.name().to_word_case()
    } else {
        model.localized_name().to_owned().to_word_case()
    }
}

pub(crate) fn model_api_object_description(model: &Model) -> String {
    if model.description().is_empty() {
        let m_name = model_localized_name(model).to_word_case();
        format!("Actions for {m_name}.")
    } else {
        model.description().to_owned()
    }
}

pub(crate) fn field_localized_name(field: &Field) -> String {
    if field.localized_name().is_empty() {
        field.name().to_owned().to_sentence_case()
    } else {
        field.localized_name().to_owned()
    }
}

pub(crate) fn field_localized_name_word_case(field: &Field) -> String {
    field_localized_name(field).to_word_case()
}

pub(crate) fn field_description(field: &Field) -> String {
    if field.description().is_empty() {
        "This field doesn't have a description.".to_owned()
    } else {
        field.description().to_owned()
    }
}

pub(crate) fn relation_localized_name(relation: &Relation) -> String {
    if relation.localized_name().is_empty() {
        relation.name().to_owned().to_sentence_case()
    } else {
        relation.localized_name().to_owned()
    }
}

pub(crate) fn relation_localized_name_word_case(relation: &Relation) -> String {
    relation_localized_name(relation).to_word_case()
}

pub(crate) fn relation_description(relation: &Relation) -> String {
    if relation.description().is_empty() {
        "This field doesn't have a description.".to_owned()
    } else {
        relation.description().to_owned()
    }
}
