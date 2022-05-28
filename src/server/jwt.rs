use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};


#[derive(Debug, Clone)]
pub struct TokenDecodeError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub model: String,
    pub exp: usize
}

pub fn encode_token(claims: Claims, secret: &str) -> String {
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()));
    token.unwrap()
}

pub fn decode_token(token: &String, secret: &str) -> Result<Claims, TokenDecodeError> {
    let token = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());
    return match token {
        Ok(token) => {
            Ok(token.claims)
        }
        Err(err) => {
            println!("{}", err);
            Err(TokenDecodeError)
        }
    }
}
