use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use sqlx::types::JsonValue;
use crate::core::error::ActionError;
use crate::prelude::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: JsonValue,
    pub model: String,
    pub exp: usize
}

pub fn encode_token(claims: Claims, secret: &str) -> String {
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()));
    token.unwrap()
}

pub fn decode_token(token: &String, secret: &str) -> Result<Claims, ActionError> {
    let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());
    return match token {
        Ok(token) => {
            Ok(token.claims)
        }
        Err(_) => {
            Err(ActionError::invalid_auth_token())
        }
    }
}
