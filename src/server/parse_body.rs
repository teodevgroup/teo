use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use http_body_util::{BodyStream, BodyExt};
use futures_util::{StreamExt, TryStreamExt};
use hyper::header::CONTENT_TYPE;
use regex::Regex;
use serde_json::{json, Value as JsonValue};
use teo_result::{Result, Error};
use teo_runtime::request::Request;
use multer::Multipart;

pub(super) async fn parse_json_body(incoming: hyper::body::Incoming) -> Result<JsonValue> {
    let body = match incoming.collect().await {
        Ok(body) => body,
        Err(_) => return Err(Error::internal_server_error_message("cannot read HTTP body")),
    }.to_bytes();
    let parsed_json_body_result: std::result::Result<JsonValue, serde_json::Error> = serde_json::from_slice(&body);
    let parsed_json_body = match parsed_json_body_result {
        Ok(b) => b,
        Err(_) => {
            return Err(Error::invalid_request_message("incorrect json format"));
        }
    };
    if !parsed_json_body.is_object() {
        return Err(Error::invalid_request_message("expect json root object"));
    }
    Ok(parsed_json_body)
}

pub(super) async fn parse_form_body(request: &Request, incoming: hyper::body::Incoming) -> Result<JsonValue> {
    let boundary = request
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|ct| ct.to_str().ok())
        .and_then(|ct| multer::parse_boundary(ct).ok());
    if boundary.is_none() {
        return Err(Error::invalid_request_message("missing boundary in header"));
    }
    let boundary = boundary.unwrap();

    let body_stream = BodyStream::new(incoming)
        .filter_map(|result| async move { result.map(|frame| frame.into_data().ok()).transpose() });

    // Create a Multipart instance from the request body.
    let mut multipart = Multipart::new(body_stream, boundary);

    let mut result_value = json!({});

    while let Some(mut field) = multipart.next_field().await? {

        // Get the field name.
        let name = field.name();
        if name.is_none() {
            return Err(Error::invalid_request_message("missing field name"));
        }
        let owned_field_name = name.unwrap().to_owned();

        // Get the field's filename if provided in "Content-Disposition" header.
        let file_name = field.file_name().map(ToString::to_string);

        // Get the "Content-Type" header as `mime::Mime` type.
        let content_type = field.content_type();

        if let Some(file_name) = file_name {
            let file_name_ext = Path::new(&file_name)
                .extension()
                .and_then(OsStr::to_str);
            let filepath = std::env::temp_dir().join(file_name.clone()).to_str().unwrap().to_owned();
            let mut file = std::fs::File::create(&filepath)?;
            while let Some(field_chunk) = field.chunk().await? {
                file.write_all(&field_chunk)?;
            }
            if owned_field_name.ends_with("[]") {
                let field_name_without_suffix = owned_field_name.strip_suffix("[]").unwrap();
                if !result_value.as_object_mut().unwrap().contains_key(field_name_without_suffix) {
                    result_value.as_object_mut().unwrap().insert(field_name_without_suffix.to_owned(), json!([]));
                }

                result_value.as_object_mut().unwrap().get_mut(field_name_without_suffix).unwrap().as_array_mut().unwrap().push(json!({
                    "filepath": filepath.clone(),
                    "contentType": field.content_type().map(|c| c.to_string()),
                    "filename": file_name,
                    "filenameExt": file_name_ext,
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
                    "filepath": filepath.clone(),
                    "contentType": field.content_type().map(|c| c.to_string()),
                    "filename": file_name,
                    "filenameExt": file_name_ext,
                }));
            } else {
                result_value.as_object_mut().unwrap().insert(field.name().unwrap().to_owned(), json!({
                    "filepath": filepath.clone(),
                    "contentType": field.content_type().map(|c| c.to_string()),
                    "filename": file_name,
                    "filenameExt": file_name_ext,
                }));
            }
        } else {
            result_value.as_object_mut().unwrap().insert(field.name().unwrap().to_owned(), serde_json::Value::String(match field.text().await {
                Ok(text) => text,
                Err(_) => return Err(Error::invalid_request_message("cannot read text content")),
            }));
        }
    }
    Ok(result_value)
}
