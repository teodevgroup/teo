use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum ActionErrorType {
    KeysUnallowed,
    ActionUnrecognized,
    InvalidInput,
    WrongInputType,
    WrongDateFormat,
    WrongDateTimeFormat,
    WrongEnumChoice,
    ValueRequired,
    InternalServerError,
}

impl ActionErrorType {
    pub fn code(&self) -> u16 {
        match self {
            ActionErrorType::KeysUnallowed => { 400 }
            ActionErrorType::ActionUnrecognized => { 400 }
            ActionErrorType::InvalidInput => { 400 }
            ActionErrorType::WrongInputType => { 400 }
            ActionErrorType::WrongDateFormat => { 400 }
            ActionErrorType::WrongDateTimeFormat => { 400 }
            ActionErrorType::WrongEnumChoice => { 400 }
            ActionErrorType::ValueRequired => { 400 }
            ActionErrorType::InternalServerError => { 500 }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ActionError {
    pub r#type: ActionErrorType,
    pub message: String,
    pub errors: Option<HashMap<&'static str, String>>
}

impl ActionError {
    pub fn keys_unallowed() -> Self {
        ActionError {
            r#type: ActionErrorType::KeysUnallowed,
            message: "Unallowed keys detected.".to_string(),
            errors: None
        }
    }

    pub fn action_unrecognized() -> Self {
        ActionError {
            r#type: ActionErrorType::ActionUnrecognized,
            message: "This action is unrecognized.".to_string(),
            errors: None
        }
    }

    pub fn invalid_input(key: &'static str, reason: String) -> Self {
        let mut fields = HashMap::with_capacity(1);
        fields.insert(key, reason);
        ActionError {
            r#type: ActionErrorType::InvalidInput,
            message: "Invalid value found in input values.".to_string(),
            errors: Some(fields)
        }
    }

    pub fn wrong_input_type() -> Self {
        ActionError {
            r#type: ActionErrorType::WrongInputType,
            message: "Input type is unexpected.".to_string(),
            errors: None
        }
    }

    pub fn wrong_date_format() -> Self {
        ActionError {
            r#type: ActionErrorType::WrongDateFormat,
            message: "Date format is unexpected.".to_string(),
            errors: None
        }
    }

    pub fn wrong_datetime_format() -> Self {
        ActionError {
            r#type: ActionErrorType::WrongDateTimeFormat,
            message: "Datetime format is unexpected.".to_string(),
            errors: None
        }
    }

    pub fn wrong_enum_choice() -> Self {
        ActionError {
            r#type: ActionErrorType::WrongEnumChoice,
            message: "Enum value is unexpected.".to_string(),
            errors: None
        }
    }

    pub fn value_required() -> Self {
        ActionError {
            r#type: ActionErrorType::ValueRequired,
            message: "Value is required.".to_string(),
            errors: None
        }
    }

    pub fn internal_server_error(reason: String) -> Self {
        ActionError {
            r#type: ActionErrorType::InternalServerError,
            message: reason,
            errors: None
        }
    }
}
