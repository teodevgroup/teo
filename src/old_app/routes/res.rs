use std::collections::HashMap;
use std::path::PathBuf;
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse};
use actix_files::NamedFile;
use crate::prelude::Value;
use teo_teon::teon;


pub enum Res {
    EmptyRes,
    TeonRes(Value),
    TeonDataRes(Value),
    TeonDataMetaRes(Value, Value),
    TeonErrorRes { code: u16, kind: String, message: String, fields: Option<HashMap<String, String>> },
    File(PathBuf),
}

impl Res {

    pub fn is_file(&self) -> bool {
        match self {
            Res::File(_) => true,
            _ => false,
        }
    }

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

    pub(crate) fn into_response(&self, req: &HttpRequest) -> HttpResponse {
        match self {
            Res::File(path) => {
                NamedFile::open(path).unwrap().into_response(req)
            },
            _ => unreachable!()
        }
    }
}

fn j(v: Value) -> serde_json::Value {
    v.into()
}

