use crate::core::field::Optionality;
use crate::core::field::r#type::FieldType;
use crate::core::pipeline::Pipeline;

pub mod builder;

pub struct Property {
    pub(crate) name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) optionality: Optionality,
    pub(crate) field_type: FieldType,
    pub(crate) dependencies: Vec<String>,
    pub(crate) setter: Option<Pipeline>,
    pub(crate) getter: Option<Pipeline>,
    pub(crate) input_omissible: bool,
}
