use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use serde_json::{Value as JsonValue};
use crate::core::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: JsonValue,
    pub model: Vec<String>,
    pub exp: usize
}

impl Claims {
    pub(crate) fn model_path(&self) -> Vec<&str> {
        self.model.iter().map(|s| s.as_str()).collect()
    }
}

pub fn encode_token(claims: Claims, secret: &str) -> String {
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()));
    token.unwrap()
}

pub fn decode_token(token: &String, secret: &str) -> Result<Claims, Error> {
    let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());
    return match token {
        Ok(token) => {
            Ok(token.claims)
        }
        Err(_) => {
            Err(Error::invalid_auth_token())
        }
    }
}
