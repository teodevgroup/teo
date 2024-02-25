use std::io::Write;
use actix_multipart::Multipart;
use actix_web::{FromRequest, HttpRequest, web};
use futures_util::{StreamExt, TryStreamExt};
use regex::Regex;
use serde_json::{json, Value as JsonValue};
use teo_result::{Result, Error};
use teo_runtime::error_runtime_ext::ErrorRuntimeExt;

pub(super) async fn parse_json_body(mut payload: web::Payload) -> Result<JsonValue> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > 262_144usize {
            return Err(Error::internal_server_error_message_only("memory overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let parsed_json_body_result: std::result::Result<JsonValue, serde_json::Error> = serde_json::from_slice(&body);
    let parsed_json_body = match parsed_json_body_result {
        Ok(b) => b,
        Err(_) => {
            return Err(Error::value_error_message_only("incorrect json format"));
        }
    };
    if !parsed_json_body.is_object() {
        return Err(Error::value_error_message_only("expect json root object"));
    }
    Ok(parsed_json_body)
}

pub(super) async fn parse_form_body(http_request: HttpRequest, mut payload: web::Payload) -> Result<JsonValue> {
    let mut inner_payload = payload.into_inner();
    let multipart_result = Multipart::from_request(&http_request, &mut inner_payload).await;
    let mut multipart = match multipart_result {
        Ok(multipart) => multipart,
        Err(err) => return Err(Error::value_error_message_only("incorrect form format")),
    };
    let mut result_value = json!({});
    while let Some(mut field) = multipart.try_next().await.unwrap() {
        // A multipart/form-data stream has to contain `content_disposition`
        if let Some(filename) = field.content_disposition().get_filename().map(|f| f.to_owned()) {
            let filepath = std::env::temp_dir().join(filename.clone()).to_str().unwrap().to_owned();
            let filepath2 = filepath.clone();
            // File::create is blocking operation, use threadpool
            let mut f = web::block(move || std::fs::File::create(&filepath)).await.unwrap().unwrap();
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.try_next().await.unwrap() {
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap().unwrap();
            }
            let owned_field_name = field.name().to_owned();
            if owned_field_name.ends_with("[]") {
                let field_name_without_suffix = owned_field_name.strip_suffix("[]").unwrap();
                if !result_value.as_object_mut().unwrap().contains_key(field_name_without_suffix) {
                    result_value.as_object_mut().unwrap().insert(field_name_without_suffix.to_owned(), json!([]));
                }
                result_value.as_object_mut().unwrap().get_mut(field_name_without_suffix).unwrap().as_array_mut().unwrap().push(json!({
                    "filepath": filepath2,
                    "contentType": field.content_type().map(|c| c.to_string()),
                    "filename": filename,
                    "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
                }));
            } else if owned_field_name.ends_with("]") {
                let regex = Regex::new("(.*)\\[(.*)\\]").unwrap();
                let found = regex.captures(&owned_field_name).unwrap();
                let field_name = found.get(1).unwrap().as_str().to_owned();
                let dict_name = found.get(2).unwrap().as_str().to_owned();
                if !result_value.as_object_mut().unwrap().contains_key(&field_name) {
                    result_value.as_object_mut().unwrap().insert(field_name.clone(), json!([]));
                }
                result_value.as_object_mut().unwrap().get_mut(&field_name).unwrap().as_object_mut().unwrap().insert(dict_name, json!({
                    "filepath": filepath2,
                    "contentType": field.content_type().map(|c| c.to_string()),
                    "filename": filename,
                    "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
                }));
            } else {
                result_value.as_object_mut().unwrap().insert(field.name().to_owned(), json!({
                    "filepath": filepath2,
                    "contentType": field.content_type().map(|c| c.to_string()),
                    "filename": filename,
                    "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
                }));
            }
        } else {
            let mut body = web::BytesMut::new();
            while let Some(chunk) = field.try_next().await.unwrap() {
                body.extend_from_slice(&chunk);
            }
            result_value.as_object_mut().unwrap().insert(field.name().to_owned(), serde_json::Value::String(String::from_utf8(body.as_ref().to_vec()).unwrap()));
        }
    }
    Ok(result_value)
}
