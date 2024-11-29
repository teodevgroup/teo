use bigdecimal::BigDecimal;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use futures_util::future::try_join_all;
use teo_runtime::request::Request;
use teo_runtime::response::Response;
use teo_runtime::{pipeline, request, teon, Value};
use teo_runtime::arguments::Arguments;
use teo_runtime::pipeline::item::templates::validator::{ValidatorResult, Validity};
use teo::app::App;
use teo::result::{Error, Result};
use teo::test::schema_path::schema_path_args;
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
    app.main_namespace().define_transform_pipeline_item("transformInt32Array", |_| {
        return Ok(|value: Vec<i32>| async move {
            value.into_iter().map(|v| v * 10).collect::<Vec<i32>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformInt64Array", |_| {
        return Ok(|value: Vec<i64>| async move {
            value.into_iter().map(|v| v * 10).collect::<Vec<i64>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat32Array", |_| {
        return Ok(|value: Vec<f32>| async move {
            value.into_iter().map(|v| v * 10.0).collect::<Vec<f32>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat64Array", |_| {
        return Ok(|value: Vec<f64>| async move {
            value.into_iter().map(|v| v * 10.0).collect::<Vec<f64>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformBoolArray", |_| {
        return Ok(|value: Vec<bool>| async move {
            value.into_iter().map(|v| !v).collect::<Vec<bool>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformStringArray", |_| {
        return Ok(|value: Vec<String>| async move {
            value.into_iter().map(|v| format!("*{}*", v)).collect::<Vec<String>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDateArray", |_| {
        return Ok(|value: Vec<NaiveDate>| async move {
            value.into_iter().map(|v| v + Duration::days(1)).collect::<Vec<NaiveDate>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDateTimeArray", |_| {
        return Ok(|value: Vec<DateTime<Utc>>| async move {
            value.into_iter().map(|v| v + Duration::days(1)).collect::<Vec<DateTime<Utc>>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDecimalArray", |_| {
        return Ok(|value: Vec<BigDecimal>| async move {
            value.into_iter().map(|v| v * BigDecimal::from(10)).collect::<Vec<BigDecimal>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformStatusArray", |_| {
        return Ok(|value: Vec<Status>| async move {
            value.into_iter().map(|v| {
                Ok::<Status, Error>(if v.is_open() {
                    Status::pending()
                } else if v.is_pending() {
                    Status::in_progress()
                } else if v.is_in_progress() {
                    Status::waiting_for_review()
                } else if v.is_waiting_for_review() {
                    Status::done()
                } else if v.is_done() {
                    Status::open()
                } else {
                    Err(Error::new(format!("unknown status {:?}", v)))?
                })
            }).collect::<Result<Vec<Status>>>()
        })
    });


    // app.main_namespace().define_validator_pipeline_item("myValidator", |args: Arguments| {
    //     return Ok(|_: Value, ctx: pipeline::Ctx| async move {
    //         ValidatorResult::Validity(Validity::Valid)
    //     })
    // });
    Ok(app)
}