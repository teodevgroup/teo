use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{Either, Full};
use hyper::body::Body;
use hyper::header::CONTENT_TYPE;
use mime::APPLICATION_JSON;
use teo_result::{Error, Result};
use teo_runtime::request::Request;
use teo_runtime::response::body::BodyInner;
use teo_runtime::response::Response;

pub async fn hyper_response_from(request: Request, response: Response) -> Result<hyper::Response<Either<Full<Bytes>, BoxBody<Bytes, Error>>>> {
    let mut builder = hyper::Response::builder().status(response.code());
    for key in response.headers().keys() {
        builder = builder.header(key.clone(), response.headers().get(&key).unwrap().clone());
    }
    for cookie in response.cookies() {
        builder = builder.header("Set-Cookie", cookie.encoded().to_string());
    }
    match response.body().inner.as_ref() {
        BodyInner::Empty => {
            let body_bytes = "".to_owned();
            Ok(builder.body(Either::Left(body_bytes.into())).unwrap())
        },
        BodyInner::String(content) => {
            let body_bytes = content.to_string();
            Ok(builder.body(Either::Left(body_bytes.into())).unwrap())
        },
        BodyInner::Teon(value) => {
            builder = builder.header(CONTENT_TYPE, APPLICATION_JSON.essence_str());
            let json_value = serde_json::Value::try_from(value).unwrap();
            let string_value = serde_json::to_string(&json_value).unwrap();
            Ok(builder.body(Either::Left(string_value.into())).unwrap())
        },
        BodyInner::File(path_buf) => {
            todo!()
            //send_file(&request, path_buf).await
        }
    }
}
