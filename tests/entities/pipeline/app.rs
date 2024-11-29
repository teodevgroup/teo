use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use futures_util::future::try_join_all;
use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{pipeline, request, teon, Value};
use teo_runtime::arguments::Arguments;
use teo_runtime::pipeline::item::templates::validator::{ValidatorResult, Validity};
use teo::app::App;
use teo::result::Result;
use teo::test::schema_path::schema_path_args;
use teo::prelude::Error;
use crate::entities::models::types::entities::{SupportCreateInput, SupportFindManyArgs, Teo};
use crate::entities::pipeline::entities::Status;

pub fn load_app() -> Result<App> {
    let app = App::new_with_argv(
        schema_path_args(file!(), "schema.teo")
    )?;
    app.main_namespace().define_transform_pipeline_item("transformInt32", |_| {
        return Ok(|value: i32| async move {
            value * 10
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformInt64", |_| {
        return Ok(|value: i64| async move {
            value * 10
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat32", |_| {
        return Ok(|value: f32| async move {
            value * 10.0
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat64", |_| {
        return Ok(|value: f64| async move {
            value * 10.0
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformBool", |_| {
        return Ok(|value: bool| async move {
            !value
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformString", |_| {
        return Ok(|value: String| async move {
            format!("*{}*", value)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDate", |_| {
        return Ok(|value: NaiveDate| async move {
            value + Duration::days(1)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDateTime", |_| {
        return Ok(|value: DateTime<Utc>| async move {
            value + Duration::days(1)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDecimal", |_| {
        return Ok(|value: BigDecimal| async move {
            value * BigDecimal::from(10)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformStatus", |_| {
        return Ok(|value: Status| async move {
            Ok::<Status, Error>(if value.is_open() {
                Status::pending()
            } else if value.is_pending() {
                Status::in_progress()
            } else if value.is_in_progress() {
                Status::waiting_for_review()
            } else if value.is_waiting_for_review() {
                Status::done()
            } else if value.is_done() {
                Status::open()
            } else {
                Err(Error::new(format!("unknown status {:?}", value)))?
            })
        })
    });

    // declare pipeline item transformInt32Array: Int[] -> Int[]
    // declare pipeline item transformInt64Array: Int64[] -> Int64[]
    // declare pipeline item transformFloat32Array: Float32[] -> Float32[]
    // declare pipeline item transformFloat64Array: Float[] -> Float[]
    // declare pipeline item transformBoolArray: Bool[] -> Bool[]
    // declare pipeline item transformStringArray: String[] -> String[]
    // declare pipeline item transformDateArray: Date[] -> Date[]
    // declare pipeline item transformDateTimeArray: DateTime[] -> DateTime[]
    // declare pipeline item transformDecimalArray: Decimal[] -> Decimal[]
    // declare pipeline item transformStatusArray: Status[] -> Status[]

    // app.main_namespace().define_validator_pipeline_item("myValidator", |args: Arguments| {
    //     return Ok(|_: Value, ctx: pipeline::Ctx| async move {
    //         ValidatorResult::Validity(Validity::Valid)
    //     })
    // });
    Ok(app)
}