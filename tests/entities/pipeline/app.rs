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
use crate::entities::pipeline::entities::{Container, Status};

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
    app.main_namespace().define_transform_pipeline_item("alterInt32", |args: Arguments| {
        let to: i32 = args.get("to")?;
        return Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterInt64", |args: Arguments| {
        let to: i64 = args.get("to")?;
        return Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat32", |args: Arguments| {
        let to: f32 = args.get("to")?;
        return Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat64", |args: Arguments| {
        let to: f64 = args.get("to")?;
        return Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterBool", |args: Arguments| {
        let to: bool = args.get("to")?;
        return Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterString", |args: Arguments| {
        let to: String = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDate", |args: Arguments| {
        let to: NaiveDate = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDateTime", |args: Arguments| {
        let to: DateTime<Utc> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDecimal", |args: Arguments| {
        let to: BigDecimal = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterStatus", |args: Arguments| {
        let to: Status = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterInt32Array", |args: Arguments| {
        let to: Vec<i32> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterInt64Array", |args: Arguments| {
        let to: Vec<i64> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat32Array", |args: Arguments| {
        let to: Vec<f32> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat64Array", |args: Arguments| {
        let to: Vec<f64> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterBoolArray", |args: Arguments| {
        let to: Vec<bool> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterStringArray", |args: Arguments| {
        let to: Vec<String> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDateArray", |args: Arguments| {
        let to: Vec<NaiveDate> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDateTimeArray", |args: Arguments| {
        let to: Vec<DateTime<Utc>> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDecimalArray", |args: Arguments| {
        let to: Vec<BigDecimal> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterStatusArray", |args: Arguments| {
        let to: Vec<Status> = args.get("to")?;
        return Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt32", |_| {
        return Ok(|_: i32| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt64", |_| {
        return Ok(|_: i64| async move { })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat32", |_| {
        return Ok(|_: f32| async move { Ok::<bool, Error>(true) })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat64", |_| {
        return Ok(|_: f64| async move { Ok::<(), Error>(()) })
    });
    app.main_namespace().define_validator_pipeline_item("validateBool", |_| {
        return Ok(|_: bool| async move { Ok::<Validity, Error>(Validity::Valid) })
    });
    app.main_namespace().define_validator_pipeline_item("validateString", |_| {
        return Ok(|s: String| async move {
            if s.len() > 1 {
                Validity::Valid
            } else {
                Validity::Invalid("string is too short, expect length > 1".to_owned())
            }
        })
    });
    app.main_namespace().define_validator_pipeline_item("validateDate", |_| {
        return Ok(|_: NaiveDate| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDateTime", |_| {
        return Ok(|_: DateTime<Utc>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDecimal", |_| {
        return Ok(|_: BigDecimal| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateStatus", |_| {
        return Ok(|_: Status| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt32Array", |_| {
        return Ok(|_: Vec<i32>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt64Array", |_| {
        return Ok(|_: Vec<i64>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat32Array", |_| {
        return Ok(|_: Vec<f32>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat64Array", |_| {
        return Ok(|_: Vec<f64>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateBoolArray", |_| {
        return Ok(|_: Vec<bool>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateStringArray", |_| {
        return Ok(|_: Vec<String>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDateArray", |_| {
        return Ok(|_: Vec<NaiveDate>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDateTimeArray", |_| {
        return Ok(|_: Vec<DateTime<Utc>>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDecimalArray", |_| {
        return Ok(|_: Vec<BigDecimal>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateStatusArray", |_| {
        return Ok(|_: Vec<Status>| async move { true })
    });
    app.main_namespace().define_callback_pipeline_item("int32Callback", |_| {
        return Ok(|value: i32, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("int64Callback", |_| {
        return Ok(|value: i64, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float32Callback", |_| {
        return Ok(|value: f32, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float64Callback", |_| {
        return Ok(|value: f64, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("boolCallback", |_| {
        return Ok(|value: bool, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("stringCallback", |_| {
        return Ok(|value: String, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateCallback", |_| {
        return Ok(|value: NaiveDate, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateTimeCallback", |_| {
        return Ok(|value: DateTime<Utc>, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("decimalCallback", |_| {
        return Ok(|value: BigDecimal, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("statusCallback", |_| {
        return Ok(|value: Status, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("int32ArrayCallback", |_| {
        return Ok(|value: Vec<i32>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("int64ArrayCallback", |_| {
        return Ok(|value: Vec<i64>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float32ArrayCallback", |_| {
        return Ok(|value: Vec<f32>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float64ArrayCallback", |_| {
        return Ok(|value: Vec<f64>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("boolArrayCallback", |_| {
        return Ok(|value: Vec<bool>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("stringArrayCallback", |_| {
        return Ok(|value: Vec<String>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateArrayCallback", |_| {
        return Ok(|value: Vec<NaiveDate>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateTimeArrayCallback", |_| {
        return Ok(|value: Vec<DateTime<Utc>>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("decimalArrayCallback", |_| {
        return Ok(|value: Vec<BigDecimal>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("statusArrayCallback", |_| {
        return Ok(|value: Vec<Status>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    Ok(app)
}