use std::collections::HashMap;
use actix_http::StatusCode;
use actix_web::HttpResponse;
use crate::prelude::Value;
use crate::teon;


pub enum Res {
    EmptyRes,
    TeonRes(Value),
    TeonDataRes(Value),
    TeonDataMetaRes(Value, Value),
    TeonErrorRes { code: u16, kind: String, message: String, fields: Option<HashMap<String, String>> },
}

impl Res {

    pub fn empty() -> Self {
        Self::EmptyRes
    }

    pub fn teon(value: Value) -> Self {
        Self::TeonRes(value)
    }

    pub fn teon_data(data: Value) -> Self {
        Self::TeonDataRes(data)
    }

    pub fn teon_data_meta(data: Value, meta: Value) -> Self {
        Self::TeonDataMetaRes(data, meta)
    }

    pub fn teon_error(code: u16, kind: impl Into<String>, message: impl Into<String>, fields: Option<HashMap<String, String>>) -> Self {
        Self::TeonErrorRes { code, kind: kind.into(), message: message.into(), fields }
    }

    pub(crate) fn code(&self) -> u16 {
        match self {
            Self::TeonErrorRes{ code, kind: _, message: _, fields: _ } => *code,
            _ => 200,
        }
    }
}

fn j(v: Value) -> serde_json::Value {
    v.into()
}

impl Into<HttpResponse> for Res {
    fn into(self) -> HttpResponse {
        match self {
            Res::EmptyRes => HttpResponse::Ok().into(),
            Res::TeonRes(value) => HttpResponse::Ok().json(j(value)),
            Res::TeonDataRes(data) => HttpResponse::Ok().json(j(teon!({"data": data}))),
            Res::TeonDataMetaRes(data, meta) => HttpResponse::Ok().json(j(teon!({"data": data, "meta": meta}))),
            Res::TeonErrorRes { code, kind, message, fields } => {
                let mut inner = teon!({
                    "type": kind,
                    "message": message,
                });
                if let Some(fields) = fields {
                    inner.as_hashmap_mut().unwrap().insert("fields".to_string(), Value::HashMap(fields.iter().map(|(k, v)| {
                        (k.to_string(), Value::String(v.to_string()))
                    }).collect()));
                }
                HttpResponse::build(StatusCode::from_u16(code).unwrap()).json(j(teon!({
                    "error": inner
                })))
            }
        }
    }
}