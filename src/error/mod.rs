use std::collections::HashMap;
use serde::Serialize;


#[derive(Debug, PartialEq, Serialize)]
pub enum ActionErrorType {
    KeysUnallowed,
    ActionUnrecognized,
    InvalidInput,
    WrongInputType,
    WrongWhereType,
    WrongDateFormat,
    WrongDateTimeFormat,
    WrongEnumChoice,
    WrongJSONFormat,
    ValueRequired,
    ValidationError,
    UnknownDatabaseWriteError,
    UnknownDatabaseDeleteError,
    UnknownDatabaseFindError,
    UnknownDatabaseFindUniqueError,
    UnknownDatabaseCountError,
    NotFound,
    InternalServerError,
    MissingActionName,
    UndefinedAction,
    UnallowedAction,
    MissingInputSection,
    ObjectNotFound,
    ObjectIsNotSaved,
    FieldIsNotUnique,
    UnmatchedDataTypeInDatabase,
    UndefinedEnumValue,
    MissingCredentials,
    MultipleAuthIdentityProvided,
    MultipleAuthCheckerProvided,
    MissingAuthIdentity,
    MissingAuthChecker,
    AuthenticationFailed,
    InvalidAuthorizationFormat,
    InvalidJWTToken,
    IdentityIsNotFound,
}

impl ActionErrorType {
    pub fn code(&self) -> u16 {
        match self {
            ActionErrorType::KeysUnallowed => { 400 }
            ActionErrorType::ActionUnrecognized => { 400 }
            ActionErrorType::InvalidInput => { 400 }
            ActionErrorType::WrongInputType => { 400 }
            ActionErrorType::WrongWhereType => { 400 }
            ActionErrorType::WrongDateFormat => { 400 }
            ActionErrorType::WrongDateTimeFormat => { 400 }
            ActionErrorType::WrongEnumChoice => { 400 }
            ActionErrorType::ValueRequired => { 400 }
            ActionErrorType::ValidationError => { 400 }
            ActionErrorType::WrongJSONFormat => { 400 }
            ActionErrorType::MissingActionName => { 400 }
            ActionErrorType::UndefinedAction => { 400 }
            ActionErrorType::UnallowedAction => { 400 }
            ActionErrorType::ObjectIsNotSaved => { 400 }
            ActionErrorType::UndefinedEnumValue => { 400 }
            ActionErrorType::UnknownDatabaseWriteError => { 500 }
            ActionErrorType::UnknownDatabaseDeleteError => { 500 }
            ActionErrorType::UnknownDatabaseFindError => { 500 }
            ActionErrorType::UnknownDatabaseFindUniqueError => { 500 }
            ActionErrorType::UnknownDatabaseCountError => { 500 }
            ActionErrorType::UnmatchedDataTypeInDatabase => { 500 }
            ActionErrorType::NotFound => { 404 }
            ActionErrorType::InternalServerError => { 500 }
            ActionErrorType::MissingInputSection => { 400 }
            ActionErrorType::ObjectNotFound => { 404 }
            ActionErrorType::FieldIsNotUnique => { 400 }
            ActionErrorType::MultipleAuthCheckerProvided => { 400 }
            ActionErrorType::MultipleAuthIdentityProvided => { 400 }
            ActionErrorType::MissingAuthIdentity => { 400 }
            ActionErrorType::MissingAuthChecker => { 400 }
            ActionErrorType::MissingCredentials => { 400 }
            ActionErrorType::AuthenticationFailed => { 401 }
            ActionErrorType::InvalidAuthorizationFormat => { 401 }
            ActionErrorType::InvalidJWTToken => { 401 }
            ActionErrorType::IdentityIsNotFound => { 401 }
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ActionError {
    pub r#type: ActionErrorType,
    pub message: String,
    pub errors: Option<HashMap<String, String>>
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

    pub fn invalid_input(key: impl Into<String>, reason: String) -> Self {
        let mut fields = HashMap::with_capacity(1);
        fields.insert(key.into(), reason);
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

    pub fn wrong_where_type() -> Self {
        ActionError {
            r#type: ActionErrorType::WrongWhereType,
            message: "Where type is unexpected.".to_string(),
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
            message: "Wrong enum choice".to_string(),
            errors: None
        }
    }

    pub fn unexpected_enum_value(field: impl Into<String>) -> Self {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.into(), "Enum value is unexpected.".to_string());
        ActionError {
            r#type: ActionErrorType::ValidationError,
            message: "Enum value is unexpected.".to_string(),
            errors: Some(errors)
        }
    }

    pub fn value_required(field: impl Into<String>) -> Self {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.into(), "Value is required.".to_string());
        ActionError {
            r#type: ActionErrorType::ValidationError,
            message: "Value is required.".to_string(),
            errors: Some(errors)
        }
    }

    pub fn unique_value_duplicated(field: impl Into<String>) -> Self {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.into(), "Unique value duplicated.".to_string());
        ActionError {
            r#type: ActionErrorType::ValidationError,
            message: "Input is not valid.".to_string(),
            errors: Some(errors)
        }
    }

    pub fn internal_server_error(reason: String) -> Self {
        ActionError {
            r#type: ActionErrorType::InternalServerError,
            message: reason,
            errors: None
        }
    }

    pub fn unknown_database_write_error() -> Self {
        ActionError {
            r#type: ActionErrorType::UnknownDatabaseWriteError,
            message: "An unknown database write error occurred.".to_string(),
            errors: None
        }
    }

    pub fn unknown_database_delete_error() -> Self {
        ActionError {
            r#type: ActionErrorType::UnknownDatabaseDeleteError,
            message: "An unknown database delete error occurred.".to_string(),
            errors: None
        }
    }

    pub fn not_found() -> Self {
        ActionError {
            r#type: ActionErrorType::NotFound,
            message: "Not found.".to_string(),
            errors: None
        }
    }

    pub fn wrong_json_format() -> Self {
        ActionError {
            r#type: ActionErrorType::WrongJSONFormat,
            message: "Wrong JSON format.".to_string(),
            errors: None
        }
    }

    pub fn missing_action_name() -> Self {
        ActionError {
            r#type: ActionErrorType::MissingActionName,
            message: "Missing action name.".to_string(),
            errors: None
        }
    }

    pub fn undefined_action() -> Self {
        ActionError {
            r#type: ActionErrorType::UndefinedAction,
            message: "Undefined action.".to_string(),
            errors: None
        }
    }

    pub fn unallowed_action() -> Self {
        ActionError {
            r#type: ActionErrorType::UnallowedAction,
            message: "Unallowed action.".to_string(),
            errors: None
        }
    }

    pub fn missing_input_section() -> Self {
        ActionError {
            r#type: ActionErrorType::MissingInputSection,
            message: "Input incomplete.".to_string(),
            errors: None
        }
    }

    pub fn object_not_found() -> Self {
        ActionError {
            r#type: ActionErrorType::ObjectNotFound,
            message: "The requested object is not exist.".to_string(),
            errors: None
        }
    }

    pub fn object_is_not_saved() -> Self {
        ActionError {
            r#type: ActionErrorType::ObjectIsNotSaved,
            message: "This object is not saved thus can't be deleted.".to_string(),
            errors: None
        }
    }

    pub fn field_is_not_unique() -> Self {
        ActionError {
            r#type: ActionErrorType::FieldIsNotUnique,
            message: format!("Unique where input is not unique."),
            errors: None
        }
    }

    pub fn unknown_database_find_error() -> Self {
        ActionError {
            r#type: ActionErrorType::UnknownDatabaseFindError,
            message: "An unknown query error occurred.".to_string(),
            errors: None
        }
    }

    pub fn unknown_database_find_unique_error() -> Self {
        ActionError {
            r#type: ActionErrorType::UnknownDatabaseFindUniqueError,
            message: "An unknown query unique error occurred.".to_string(),
            errors: None
        }
    }

    pub fn unknown_database_count_error() -> Self {
        ActionError {
            r#type: ActionErrorType::UnknownDatabaseCountError,
            message: "An unknown count error occurred.".to_string(),
            errors: None
        }
    }

    pub fn unmatched_data_type_in_database(field_name: &str) -> Self {
        ActionError {
            r#type: ActionErrorType::UnmatchedDataTypeInDatabase,
            message: format!("Unmatched data type for field '{field_name}' in database."),
            errors: None
        }
    }

    pub fn undefined_enum_value() -> Self {
        ActionError {
            r#type: ActionErrorType::UndefinedEnumValue,
            message: "Undefined enum value is not acceptable.".to_string(),
            errors: None
        }
    }

    pub fn missing_credentials() -> Self {
        ActionError {
            r#type: ActionErrorType::MissingCredentials,
            message: "Credentials are missing.".to_string(),
            errors: None
        }
    }

    pub fn multiple_auth_identity_provided() -> Self {
        ActionError {
            r#type: ActionErrorType::MultipleAuthIdentityProvided,
            message: "Multiple auth identity provided.".to_string(),
            errors: None
        }
    }

    pub fn multiple_auth_checker_provided() -> Self {
        ActionError {
            r#type: ActionErrorType::MultipleAuthCheckerProvided,
            message: "Multiple auth checker provided.".to_string(),
            errors: None
        }
    }

    pub fn missing_auth_identity() -> Self {
        ActionError {
            r#type: ActionErrorType::MissingAuthIdentity,
            message: "Missing auth identity.".to_string(),
            errors: None
        }
    }

    pub fn missing_auth_checker() -> Self {
        ActionError {
            r#type: ActionErrorType::MissingAuthChecker,
            message: "Missing auth checker.".to_string(),
            errors: None
        }
    }

    pub fn authentication_failed() -> Self {
        ActionError {
            r#type: ActionErrorType::AuthenticationFailed,
            message: "Authentication failed.".to_string(),
            errors: None
        }
    }

    pub fn invalid_authorization_format() -> Self {
        ActionError {
            r#type: ActionErrorType::InvalidAuthorizationFormat,
            message: "Invalid authorization header format.".to_string(),
            errors: None
        }
    }

    pub fn invalid_jwt_token() -> Self {
        ActionError {
            r#type: ActionErrorType::InvalidJWTToken,
            message: "This token is malformed.".to_string(),
            errors: None
        }
    }

    pub fn identity_is_not_found() -> Self {
        ActionError {
            r#type: ActionErrorType::IdentityIsNotFound,
            message: "Identity is not found.".to_string(),
            errors: None
        }
    }
}
