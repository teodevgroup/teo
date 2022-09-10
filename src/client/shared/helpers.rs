use inflector::Inflector;
use crate::core::model::Model;

pub(crate) fn model_localized_name(model: &Model) -> String {
    if model.localized_name().is_empty() {
        model.name().to_title_case()
    } else {
        model.localized_name().to_owned()
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
