use futures_util::future::try_join_all;
use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{request, teon, Value};
use teo::app::App;
use teo::result::Result;
use teo::test::schema_path::schema_path_args;
use crate::entities::models::types::entities::{SupportCreateInput, SupportFindManyArgs, Teo};

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    app.main_namespace().define_model_handler_group("Support", |group| {
        group.define_handler("myCreate", |input: SupportCreateInput, teo: Teo| async move {
            let object = teo.support().create_object(input).await?;
            object.save().await?;
            Ok(Response::data(object.to_teon().await?))
        });
        group.define_handler("myFindManyObjects", |input: SupportFindManyArgs, teo: Teo| async move {
            let objects = teo.support().find_many_objects(input).await?;
            let values = try_join_all(objects.iter().map(|object| object.to_teon())).await?;
            Ok(Response::data(Value::Array(values)))
        });
    });
    Ok(app)
}