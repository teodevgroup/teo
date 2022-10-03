use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize, Serializer};
use maplit::hashmap;
use key_path::KeyPath;
use crate::core::model::Model;

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
    ValueRequired,
    ValidationError,
    UnknownDatabaseWriteError,
    UnknownDatabaseDeleteError,
    UnknownDatabaseFindError,
    UnknownDatabaseFindUniqueError,
    UnknownDatabaseCountError,
    NotFound,
    InternalServerError,
    UndefinedAction,
    UnallowedAction,
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
    IdentityIsNotFound,
    UnexpectedNull,
    WrongInputUpdator,
    UnexpectedFieldType,
    InvalidQueryInput,
    RequiredRelationCannotDisconnect,
    NewObjectCannotDisconnect,
    SaveCallingError,
    CustomError,
    ModelNotFound,
    WrongIdentityModel,
    PropertySetterError,

    // new errors

    // request format
    IncorrectJSONFormat,
    UnexpectedInputRootType,
    UnexpectedInputType,
    UnexpectedInputKey,
    UnexpectedInputValue,
    MissingRequiredInput,
    UnexpectedObjectLength,

    // request token
    InvalidJWTToken,

    // object api
    InvalidKey,
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
            ActionErrorType::IncorrectJSONFormat => { 400 }
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
            ActionErrorType::UnexpectedNull => { 400 }
            ActionErrorType::WrongInputUpdator => { 400 }
            ActionErrorType::UnexpectedFieldType => { 400 }
            ActionErrorType::InvalidQueryInput => { 400 }
            ActionErrorType::RequiredRelationCannotDisconnect => { 400 }
            ActionErrorType::NewObjectCannotDisconnect => { 400 }
            ActionErrorType::SaveCallingError => { 500 }
            ActionErrorType::CustomError => { 500 }
            ActionErrorType::ModelNotFound => { 500 }
            ActionErrorType::WrongIdentityModel => { 401 }
            ActionErrorType::PropertySetterError => { 400 }
            ActionErrorType::UnexpectedInputRootType => { 400 }
            ActionErrorType::UnexpectedInputType => { 400 }
            ActionErrorType::UnexpectedInputKey => { 400 }
            ActionErrorType::UnexpectedInputValue => { 400 }
            ActionErrorType::MissingRequiredInput => { 400 }
            ActionErrorType::UnexpectedObjectLength => { 400 }
            ActionErrorType::InvalidKey => { 500 }
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

    pub fn invalid_query_input(reason: impl Into<String>) -> Self {
        ActionError {
            r#type: ActionErrorType::InvalidQueryInput,
            message: reason.into(),
            errors: None
        }
    }

    pub fn invalid_input(key: impl Into<String>, reason: impl Into<String>) -> Self {
        let mut fields = HashMap::with_capacity(1);
        fields.insert(key.into(), reason.into());
        ActionError {
            r#type: ActionErrorType::InvalidInput,
            message: "Invalid value found in input values.".into(),
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
            message: "The request destination is not found.".to_string(),
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

    pub fn unexpected_null(field: impl Into<String>) -> Self {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.into(), "Unexpected null.".to_string());
        ActionError {
            r#type: ActionErrorType::UnexpectedNull,
            message: "Unexpected null.".to_string(),
            errors: Some(errors)
        }
    }

    pub fn wrong_input_updator() -> Self {
        // let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        // errors.insert(field.into(), "Wrong input updator.".to_string());
        ActionError {
            r#type: ActionErrorType::WrongInputUpdator,
            message: "Wrong input updator.".to_string(),
            errors: None
            // errors: Some(errors)
        }
    }

    pub fn expected(expected_json_type: impl Into<String>, field: impl Into<String>) -> Self {
        let expected_json_type = expected_json_type.into();
        let mut errors: HashMap<String, String> = HashMap::with_capacity(1);
        errors.insert(field.into(), "Unexpected field type.".to_string());
        ActionError {
            r#type: ActionErrorType::UnexpectedFieldType,
            message: format!("Expected '{expected_json_type}'."),
            errors: Some(errors)
        }
    }

    pub fn required_relation_cannot_disconnect() -> Self {
        ActionError {
            r#type: ActionErrorType::RequiredRelationCannotDisconnect,
            message: "Required relation cannot disconnect.".to_string(),
            errors: None
        }
    }

    pub fn new_object_cannot_disconnect() -> Self {
        ActionError {
            r#type: ActionErrorType::NewObjectCannotDisconnect,
            message: "New object cannot disconnect.".to_string(),
            errors: None
        }
    }

    pub fn save_calling_error(model_name: impl AsRef<str> + Display) -> Self {
        ActionError {
            r#type: ActionErrorType::SaveCallingError,
            message: format!("Model `{model_name}', save method cannot be called inside before save callbacks."),
            errors: None
        }
    }

    pub fn custom_error(message: impl Into<String>) -> Self {
        ActionError {
            r#type: ActionErrorType::CustomError,
            message: message.into(),
            errors: None
        }
    }

    pub fn model_not_found(name: impl AsRef<str> + Display) -> Self {
        ActionError {
            r#type: ActionErrorType::ModelNotFound,
            message: format!("Model named `{name}' is not found."),
            errors: None
        }
    }

    pub fn wrong_identity_model() -> Self {
        ActionError {
            r#type: ActionErrorType::WrongIdentityModel,
            message: format!("This identity is valid but is not of this model."),
            errors: None
        }
    }

    pub fn property_setter_error(reason: impl Into<String>) -> Self {
        ActionError {
            r#type: ActionErrorType::PropertySetterError,
            message: reason.into(),
            errors: None
        }
    }

    // new error types which should be used across the project

    pub fn incorrect_json_format() -> Self {
        ActionError {
            r#type: ActionErrorType::IncorrectJSONFormat,
            message: "Incorrect JSON format.".to_string(),
            errors: None
        }
    }

    pub fn unexpected_input_root_type<'a>(expected: impl AsRef<str>) -> Self {
        ActionError {
            r#type: ActionErrorType::UnexpectedInputRootType,
            message: format!("Unexpected root input type. Expect {}.", expected.as_ref()),
            errors: None
        }
    }

    pub fn unexpected_input_type<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        ActionError {
            r#type: ActionErrorType::UnexpectedInputType,
            message: "Unexpected input type found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect {}.", expected.into())}),
        }
    }

    pub fn unexpected_input_key<'a>(unexpected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        ActionError {
            r#type: ActionErrorType::UnexpectedInputKey,
            message: "Unexpected key found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Unexpected key '{}'.", unexpected.into())}),
        }
    }

    pub fn unexpected_input_value<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        ActionError {
            r#type: ActionErrorType::UnexpectedInputValue,
            message: "Unexpected value found.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect `{}'.", expected.into())}),
        }
    }

    pub fn missing_required_input<'a>(expected: impl Into<String>, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        ActionError {
            r#type: ActionErrorType::MissingRequiredInput,
            message: "Missing required input.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect `{}'.", expected.into())})
        }
    }

    pub fn unexpected_object_length<'a>(expected: usize, key_path: impl AsRef<KeyPath<'a>>) -> Self {
        ActionError {
            r#type: ActionErrorType::UnexpectedObjectLength,
            message: "Unexpected object length.".to_string(),
            errors: Some(hashmap!{key_path.as_ref().to_string() => format!("Expect length {}.", expected)})
        }
    }

    pub fn invalid_key(unexpected_key: impl AsRef<str>, model: &Model) -> Self {
        ActionError {
            r#type: ActionErrorType::InvalidKey,
            message: format!("Invalid key '{}' accessed on model `{}'", unexpected_key.as_ref(), model.name()),
            errors: None
        }
    }
}

impl Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.r#type.fmt(f)
    }
}

impl Error for ActionError { }

unsafe impl Sync for ActionError {}
unsafe impl Send for ActionError {}
