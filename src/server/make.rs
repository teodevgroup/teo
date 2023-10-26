use std::time::SystemTime;
use chrono::{DateTime, Local};
use colored::Colorize;
use futures_util::future;
use teo_result::Result;
use teo_runtime::config::server::Server;
use teo_runtime::namespace::Namespace;
use actix_http::body::MessageBody;
use actix_http::{HttpMessage, Method};
use actix_multipart::Multipart;
use actix_web::{App, FromRequest, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::DefaultHeaders;
use crate::cli::entrance::Entrance;
use crate::cli::runtime_version::RuntimeVersion;

fn make_server_app(
    namespace: &'static Namespace,
    conf: &'static Server,
    // middlewares: &'static IndexMap<&'static str, &'static dyn Middleware>,
) -> App<impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = actix_web::Error,
> + 'static> {
    //let combined_middleware = combine_middleware(middlewares.clone());
    let app = App::new()
        .wrap(DefaultHeaders::new()
            .add(("Access-Control-Allow-Origin", "*"))
            .add(("Access-Control-Allow-Methods", "OPTIONS, POST, GET"))
            .add(("Access-Control-Allow-Headers", "*"))
            .add(("Access-Control-Max-Age", "86400")))
        .default_service(web::route().to(move |http_request: HttpRequest, mut payload: web::Payload| async move {
            // Validate method
            let method = http_request.method();
            HttpResponse::Ok()
            //
            //     // Validate path
            //     let path_components = match parse_path(http_request.path(), conf.path_prefix) {
            //         Ok(components) => components,
            //         Err(_err) => return log_err_and_return_response(start, method.as_str(), http_request.path(), Error::destination_not_found()),
            //     };
            //     // Return for OPTIONS
            //     if http_request.method() == Method::OPTIONS {
            //         return Res::EmptyRes.into();
            //     }
            //     // Pass data
            //     // Acquire a connection
            //     let connection = AppCtx::get().unwrap().connector().unwrap().connection().await.unwrap();
            //     let teo_req = Req::new(http_request.clone());
            //     let user_ctx = UserCtx::new(connection.clone(), Some(teo_req.clone()));
            //     // Parse body
            //     let format = if http_request.content_type() == "multipart/form-data" {
            //         ActionInputFormat::Form
            //     } else {
            //         ActionInputFormat::Json
            //     };
            //     let parsed_json_body = if format.is_json() {
            //         let mut body = web::BytesMut::new();
            //         while let Some(chunk) = payload.next().await {
            //             let chunk = chunk.unwrap();
            //             // limit max size of in-memory payload
            //             if (body.len() + chunk.len()) > 262_144usize {
            //                 return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::internal_server_error("Memory overflow."));
            //             }
            //             body.extend_from_slice(&chunk);
            //         }
            //         let parsed_json_body_result: std::result::Result<JsonValue, serde_json::Error> = serde_json::from_slice(&body);
            //         let parsed_json_body = match parsed_json_body_result {
            //             Ok(b) => b,
            //             Err(_) => {
            //                 return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::incorrect_json_format());
            //             }
            //         };
            //         if !parsed_json_body.is_object() {
            //             return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::unexpected_input_root_type("object"));
            //         }
            //         parsed_json_body
            //     } else {
            //         let mut inner_payload = payload.into_inner();
            //         let multipart_result = Multipart::from_request(&http_request, &mut inner_payload).await;
            //         let mut multipart = match multipart_result {
            //             Ok(multipart) => multipart,
            //             Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::incorrect_form_format(err.to_string()))
            //         };
            //         let mut result_value = json!({});
            //         while let Some(mut field) = multipart.try_next().await.unwrap() {
            //             // A multipart/form-data stream has to contain `content_disposition`
            //             if let Some(filename) = field.content_disposition().get_filename().map(|f| f.to_owned()) {
            //                 let filepath = env::temp_dir().join(filename.clone()).to_str().unwrap().to_owned();
            //                 let filepath2 = filepath.clone();
            //                 // File::create is blocking operation, use threadpool
            //                 let mut f = web::block(move || std::fs::File::create(&filepath)).await.unwrap().unwrap();
            //                 // Field in turn is stream of *Bytes* object
            //                 while let Some(chunk) = field.try_next().await.unwrap() {
            //                     // filesystem operations are blocking, we have to use threadpool
            //                     f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap().unwrap();
            //                 }
            //                 let owned_field_name = field.name().to_owned();
            //                 if owned_field_name.ends_with("[]") {
            //                     let field_name_without_suffix = owned_field_name.strip_suffix("[]").unwrap();
            //                     if !result_value.as_object_mut().unwrap().contains_key(field_name_without_suffix) {
            //                         result_value.as_object_mut().unwrap().insert(field_name_without_suffix.to_owned(), json!([]));
            //                     }
            //                     result_value.as_object_mut().unwrap().get_mut(field_name_without_suffix).unwrap().as_array_mut().unwrap().push(json!({
            //                         "filepath": filepath2,
            //                         "contentType": field.content_type().map(|c| c.to_string()),
            //                         "filename": filename,
            //                         "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
            //                     }));
            //                 } else if owned_field_name.ends_with("]") {
            //                     let regex = Regex::new("(.*)\\[(.*)\\]").unwrap();
            //                     let found = regex.captures(&owned_field_name).unwrap();
            //                     let field_name = found.get(1).unwrap().as_str().to_owned();
            //                     let dict_name = found.get(2).unwrap().as_str().to_owned();
            //                     if !result_value.as_object_mut().unwrap().contains_key(&field_name) {
            //                         result_value.as_object_mut().unwrap().insert(field_name.clone(), json!([]));
            //                     }
            //                     result_value.as_object_mut().unwrap().get_mut(&field_name).unwrap().as_object_mut().unwrap().insert(dict_name, json!({
            //                         "filepath": filepath2,
            //                         "contentType": field.content_type().map(|c| c.to_string()),
            //                         "filename": filename,
            //                         "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
            //                     }));
            //                 } else {
            //                     result_value.as_object_mut().unwrap().insert(field.name().to_owned(), json!({
            //                         "filepath": filepath2,
            //                         "contentType": field.content_type().map(|c| c.to_string()),
            //                         "filename": filename,
            //                         "filenameExt": field.content_disposition().get_filename_ext().map(|e| e.to_string()),
            //                     }));
            //                 }
            //             } else {
            //                 let mut body = web::BytesMut::new();
            //                 while let Some(chunk) = field.try_next().await.unwrap() {
            //                     body.extend_from_slice(&chunk);
            //                 }
            //                 result_value.as_object_mut().unwrap().insert(field.name().to_owned(), serde_json::Value::String(String::from_utf8(body.as_ref().to_vec()).unwrap()));
            //             }
            //         }
            //         result_value
            //     };
            //
            //     // Teo Req
            //     let teo_req = Req::new(http_request.clone());
            //     // Identity
            //     let identity = match get_identity(&http_request, &graph, conf, connection.clone(), teo_req.clone()).await {
            //         Ok(identity) => { identity },
            //         Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            //     };
            //     let original_action = Action::handler_from_name(path_components.action.as_str());
            //     let model_def = AppCtx::get().unwrap().model(path_components.model_path()).unwrap();
            //     let original_teon_body = if let (Some(original_action), Some(model_def)) = (original_action, model_def) {
            //         // Check whether this action is supported by this model
            //         if !model_def.has_action(original_action) || model_def.is_teo_internal() {
            //             return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::destination_not_found());
            //         }
            //         // Parse body the predefined way
            //         match Decoder::decode_action_arg(model_def, graph, original_action, &parsed_json_body) {
            //             Ok(body) => body,
            //             Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            //         }
            //     } else {
            //         // Parse body the user defined way
            //         let app_ctx = AppCtx::get().unwrap();
            //         if app_ctx.main_namespace().has_custom_action_declaration_for(path_components.model.as_str(), path_components.action.as_str()) {
            //             let custom_action_declaration = app_ctx.main_namespace().get_custom_action_declaration_for(path_components.model.as_str(), path_components.action.as_str());
            //             let input = &custom_action_declaration.input_fields;
            //             let result = transform_custom_action_json_into_teon(&parsed_json_body, input, &path![]);
            //             match result {
            //                 Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            //                 Ok(value) => value,
            //             }
            //         } else {
            //             return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), Error::destination_not_found());
            //         }
            //     };
            //     let (transformed_teon_body, transformed_action) = if let (Some(original_action), Some(model_def)) = (original_action, model_def) {
            //         if model_def.has_action_transformers() || original_teon_body.as_hashmap().unwrap().get("include").is_some() {
            //             if ((original_action.to_u32() == CREATE_MANY_HANDLER) || (original_action.to_u32() == CREATE_HANDLER)) && (original_teon_body.get("create").unwrap().is_vec()) {
            //                 // create with many items
            //                 let entries = original_teon_body.get("create").unwrap().as_vec().unwrap();
            //                 let mut transformed_entries: Vec<Value> = vec![];
            //                 let mut new_action = original_action;
            //                 for (_index, entry) in entries.iter().enumerate() {
            //                     let ctx = PipelineCtx::initial_state_with_value(teon!({"create": entry}), connection.clone(), Some(teo_req.clone())).with_action(original_action);
            //                     match model_def.transformed_action(ctx).await {
            //                         Ok(result) => {
            //                             transformed_entries.push(result.0.get("create").unwrap().clone());
            //                             new_action = result.1;
            //                         },
            //                         Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            //                     }
            //                 }
            //                 let mut new_val = original_teon_body.clone();
            //                 new_val.as_hashmap_mut().unwrap().insert("create".to_owned(), Value::Vec(transformed_entries));
            //                 (new_val, Some(new_action))
            //             } else {
            //                 let ctx = PipelineCtx::initial_state_with_value(original_teon_body, connection.clone(), Some(teo_req.clone())).with_action(original_action);
            //                 match model_def.transformed_action(ctx).await {
            //                     Ok(result) => (result.0, Some(result.1)),
            //                     Err(err) => return log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            //                 }
            //             }
            //         } else {
            //             (original_teon_body, Some(original_action))
            //         }
            //     } else {
            //         (original_teon_body, original_action)
            //     };
            //     // Save the request local data into the extension
            //     let req_ctx = ReqCtx {
            //         start,
            //         connection,
            //         path_components: path_components.clone(),
            //         req: teo_req,
            //         user_ctx,
            //         transformed_action,
            //         transformed_teon_body,
            //         identity,
            //         req_local: ReqLocal::new()
            //     };
            //     let result = if AppCtx::get().unwrap().main_namespace().has_action_handler_for(&path_components.model, &path_components.action) {
            //         combined_middleware.call(req_ctx, &|req_ctx: ReqCtx| async {
            //             let path_components = req_ctx.path_components.clone();
            //             let action_def = AppCtx::get().unwrap().main_namespace().get_action_handler(&path_components.model, &path_components.action);
            //             action_def.call(req_ctx).await
            //         }).await
            //     } else {
            //         combined_middleware.call(req_ctx, &handler).await
            //     };
            //     match result {
            //         Ok(res) => log_req_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), res),
            //         Err(err) => log_err_and_return_response(start, path_components.model.as_str(), path_components.action.as_str(), err),
            //     }
            // }
        }));
    app
}

pub(crate) async fn serve(
    namespace: &'static Namespace,
    conf: &'static Server,
    runtime_version: &'static RuntimeVersion,
    entrance: &'static Entrance,
    //middlewares: &'static IndexMap<&'static str, &'static dyn Middleware>,
) -> Result<()> {
    let bind = conf.bind.clone();
    let port = bind.1;
    let server = HttpServer::new(move || {
        make_server_app(namespace, conf)
    })
        .bind((bind.0, bind.1 as u16))
        .unwrap()
        .run();
    let result = future::join(server, server_start_message(port as u16, runtime_version, entrance)).await;
    result.1
}

async fn server_start_message(port: u16, runtime_version: &'static RuntimeVersion, entrance: &'static Entrance) -> Result<()> {
    // Introducing
    let now: DateTime<Local> = Local::now();
    let now_formatted = format!("{now}").dimmed();
    let teo_version = env!("CARGO_PKG_VERSION");
    let teo = format!("Teo {}", teo_version);
    println!("{} {} ({}, {})", now_formatted, teo, runtime_version.to_string(), entrance.to_str());
    // Listening
    let now: DateTime<Local> = Local::now();
    let now_formatted = format!("{now}").dimmed();
    let port_str = format!("{port}").bold();
    let text = "Listening";
    println!("{} {} on port {}", now_formatted, text, port_str);
    Ok(())
}