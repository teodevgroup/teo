use actix_http::body::MessageBody;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::test::{call_service, read_body_json, TestRequest};
use inflector::Inflector;
use serde_json::{Value as JsonValue};
use regex::Regex;

pub fn is_object_id(value: &str) -> bool {
    let regex = Regex::new("[\\da-f]{24}").unwrap();
    regex.is_match(value)
}

pub async fn request<S, B, E>(app: &S, url: &str, action: &str, body: JsonValue) -> S::Response where
    S: Service<Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
{
    let formatted_action = action.to_camel_case().to_kebab_case();
    let req = TestRequest::post().uri(&format!("/{url}/action/{formatted_action}")).set_json(body).to_request();
    call_service(&app, req).await
}

fn match_json_value(object_value: &JsonValue, matcher_value: &JsonValue) {
    //println!("see here object matcher {} {}", object_value, matcher_value);
    for (key, value) in matcher_value.as_object().unwrap().iter() {
        let key: &str = &key;
        match key {
            "is" => {
                let value_class = value.as_str().unwrap();
                match value_class {
                    "objectId" => {
                        assert!(object_value.is_string());
                        assert!(is_object_id(object_value.as_str().unwrap()));
                    }
                    "null" => {
                        assert!(object_value.is_null());
                    }
                    "date" => {
                        assert!(object_value.is_string());
                        // TODO: add assert
                    }
                    "dateTime" => {
                        assert!(object_value.is_string());
                        // TODO: add assert
                    }
                    _ => {
                        if value_class.starts_with("$") {
                            // TODO: variable match
                        } else {
                            assert!(false, "unknown matcher class type '{value_class}'")
                        }
                    }
                }
            }
            "equals" => {
                if object_value != value {
                    println!("see equals: {} {}", object_value, value);
                }
                assert_eq!(object_value, value);
            }
            "and" => {
                for matcher in value.as_array().unwrap() {
                    match_json_value(object_value, matcher);
                }
            }
            _ => {
                assert!(false, "unknown matcher '{key}'")
            }
        }
    }
}

fn match_json_array(object: &JsonValue, matcher: &JsonValue) {
    let object_array = object.as_array().unwrap();
    let matcher_array = matcher.as_array().unwrap();
    assert_eq!(object_array.len(), matcher_array.len(), "array length doesn't match {} {}", object, matcher);
    for (index, object_value) in object_array.iter().enumerate() {
        let matcher_value = matcher_array.get(index).unwrap();
        match object_value {
            JsonValue::Object(_) => match_json_object(object_value, matcher_value),
            JsonValue::Array(_) => match_json_array(object_value, matcher_value),
            _ => match_json_value(object_value, matcher_value),
        }
    }
}

fn match_json_object(object: &JsonValue, matcher: &JsonValue) {
    let object_keys = object.as_object().unwrap().keys();
    let matcher_keys = object.as_object().unwrap().keys();
    println!("see object and matcher o:{} m:{}", object, matcher);
    assert!(object_keys.eq(matcher_keys));
    for (key, object_value) in object.as_object().unwrap().iter() {
        let matcher_value = matcher.as_object().unwrap().get(key).unwrap();
        match object_value {
            JsonValue::Object(_) => match_json_object(object_value, matcher_value),
            JsonValue::Array(_) => match_json_array(object_value, matcher_value),
            _ => match_json_value(object_value, matcher_value),
        }
    }
}

pub async fn assert_json_response<B: MessageBody>(res: ServiceResponse<B>, _code: u16, matcher: JsonValue) {
    let _status = res.status().as_u16();
    //assert_eq!(status, code);
    let json: JsonValue = read_body_json(res).await;
    println!("see json {}", json);
    match_json_object(&json, &matcher);
}

pub trait RequestGetArgument {
    fn paths(&self) -> Vec<&str>;
}

impl RequestGetArgument for &str {
    fn paths(&self) -> Vec<&str> {
        vec![self]
    }
}

impl RequestGetArgument for Vec<&str> {
    fn paths(&self) -> Vec<&str> {
        self.clone()
    }
}

pub async fn request_get<S, B, E, P>(app: &S, url: &str, action: &str, body: JsonValue, code: u16, paths: P) -> JsonValue where
    S: Service<Request, Response = ServiceResponse<B>, Error = E>,
    E: std::fmt::Debug,
    B: MessageBody,
    P: RequestGetArgument
{
    let res = request(app, url, action, body).await;
    let status = res.status().as_u16();
    let json: JsonValue = read_body_json(res).await;
    assert_eq!(status, code, "see wrong json response: {:?}", &json);
    let mut final_ret_val: Vec<JsonValue> = vec![];
    for path in paths.paths() {
        let mut retval = &json;
        let items = path.split(".");
        for item in items {
            if retval.is_object() {
                if retval.as_object().unwrap().get(item).is_none() {
                }
                retval = retval.as_object().unwrap().get(item).unwrap();
            } else if retval.is_array() {
                retval = retval.as_array().unwrap().get(item.parse::<usize>().unwrap()).unwrap();
            } else {
                assert!(false, "{retval} is not object or array.");
            }
        }
        final_ret_val.push(retval.clone());
    }
    if final_ret_val.len() == 1 {
        final_ret_val.get(0).unwrap().clone()
    } else {
        JsonValue::Array(final_ret_val)
    }
}
