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
        Ok(|value: i32| async move {
            value * 10
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformInt64", |_| {
        Ok(|value: i64| async move {
            value * 10
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat32", |_| {
        Ok(|value: f32| async move {
            value * 10.0
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat64", |_| {
        Ok(|value: f64| async move {
            value * 10.0
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformBool", |_| {
        Ok(|value: bool| async move {
            !value
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformString", |_| {
        Ok(|value: String| async move {
            format!("*{}*", value)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDate", |_| {
        Ok(|value: NaiveDate| async move {
            value + Duration::days(1)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDateTime", |_| {
        Ok(|value: DateTime<Utc>| async move {
            value + Duration::days(1)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDecimal", |_| {
        Ok(|value: BigDecimal| async move {
            value * BigDecimal::from(10)
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformStatus", |_| {
        Ok(|value: Status| async move {
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
        Ok(|value: Vec<i32>| async move {
            value.into_iter().map(|v| v * 10).collect::<Vec<i32>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformInt64Array", |_| {
        Ok(|value: Vec<i64>| async move {
            value.into_iter().map(|v| v * 10).collect::<Vec<i64>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat32Array", |_| {
        Ok(|value: Vec<f32>| async move {
            value.into_iter().map(|v| v * 10.0).collect::<Vec<f32>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformFloat64Array", |_| {
        Ok(|value: Vec<f64>| async move {
            value.into_iter().map(|v| v * 10.0).collect::<Vec<f64>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformBoolArray", |_| {
        Ok(|value: Vec<bool>| async move {
            value.into_iter().map(|v| !v).collect::<Vec<bool>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformStringArray", |_| {
        Ok(|value: Vec<String>| async move {
            value.into_iter().map(|v| format!("*{}*", v)).collect::<Vec<String>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDateArray", |_| {
        Ok(|value: Vec<NaiveDate>| async move {
            value.into_iter().map(|v| v + Duration::days(1)).collect::<Vec<NaiveDate>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDateTimeArray", |_| {
        Ok(|value: Vec<DateTime<Utc>>| async move {
            value.into_iter().map(|v| v + Duration::days(1)).collect::<Vec<DateTime<Utc>>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformDecimalArray", |_| {
        Ok(|value: Vec<BigDecimal>| async move {
            value.into_iter().map(|v| v * BigDecimal::from(10)).collect::<Vec<BigDecimal>>()
        })
    });
    app.main_namespace().define_transform_pipeline_item("transformStatusArray", |_| {
        Ok(|value: Vec<Status>| async move {
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
        Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterInt64", |args: Arguments| {
        let to: i64 = args.get("to")?;
        Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat32", |args: Arguments| {
        let to: f32 = args.get("to")?;
        Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat64", |args: Arguments| {
        let to: f64 = args.get("to")?;
        Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterBool", |args: Arguments| {
        let to: bool = args.get("to")?;
        Ok(move || async move { to })
    });
    app.main_namespace().define_transform_pipeline_item("alterString", |args: Arguments| {
        let to: String = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDate", |args: Arguments| {
        let to: NaiveDate = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDateTime", |args: Arguments| {
        let to: DateTime<Utc> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDecimal", |args: Arguments| {
        let to: BigDecimal = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterStatus", |args: Arguments| {
        let to: Status = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterInt32Array", |args: Arguments| {
        let to: Vec<i32> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterInt64Array", |args: Arguments| {
        let to: Vec<i64> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat32Array", |args: Arguments| {
        let to: Vec<f32> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterFloat64Array", |args: Arguments| {
        let to: Vec<f64> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterBoolArray", |args: Arguments| {
        let to: Vec<bool> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterStringArray", |args: Arguments| {
        let to: Vec<String> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDateArray", |args: Arguments| {
        let to: Vec<NaiveDate> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDateTimeArray", |args: Arguments| {
        let to: Vec<DateTime<Utc>> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterDecimalArray", |args: Arguments| {
        let to: Vec<BigDecimal> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_transform_pipeline_item("alterStatusArray", |args: Arguments| {
        let to: Vec<Status> = args.get("to")?;
        Ok(move || {
            let to = to.clone();
            async move {
                to
            }
        })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt32", |_| {
        Ok(|_: i32| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt64", |_| {
        Ok(|_: i64| async move { })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat32", |_| {
        Ok(|_: f32| async move { Ok::<bool, Error>(true) })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat64", |_| {
        Ok(|_: f64| async move { Ok::<(), Error>(()) })
    });
    app.main_namespace().define_validator_pipeline_item("validateBool", |_| {
        Ok(|_: bool| async move { Ok::<Validity, Error>(Validity::Valid) })
    });
    app.main_namespace().define_validator_pipeline_item("validateString", |_| {
        Ok(|s: String| async move {
            if s.len() > 1 {
                Validity::Valid
            } else {
                Validity::Invalid("string is too short, expect length > 1".to_owned())
            }
        })
    });
    app.main_namespace().define_validator_pipeline_item("validateDate", |_| {
        Ok(|_: NaiveDate| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDateTime", |_| {
        Ok(|_: DateTime<Utc>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDecimal", |_| {
        Ok(|_: BigDecimal| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateStatus", |_| {
        Ok(|_: Status| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt32Array", |_| {
        Ok(|_: Vec<i32>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateInt64Array", |_| {
        Ok(|_: Vec<i64>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat32Array", |_| {
        Ok(|_: Vec<f32>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateFloat64Array", |_| {
        Ok(|_: Vec<f64>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateBoolArray", |_| {
        Ok(|_: Vec<bool>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateStringArray", |_| {
        Ok(|_: Vec<String>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDateArray", |_| {
        Ok(|_: Vec<NaiveDate>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDateTimeArray", |_| {
        Ok(|_: Vec<DateTime<Utc>>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateDecimalArray", |_| {
        Ok(|_: Vec<BigDecimal>| async move { true })
    });
    app.main_namespace().define_validator_pipeline_item("validateStatusArray", |_| {
        Ok(|_: Vec<Status>| async move { true })
    });
    app.main_namespace().define_callback_pipeline_item("int32Callback", |_| {
        Ok(|value: i32, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("int64Callback", |_| {
        Ok(|value: i64, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float32Callback", |_| {
        Ok(|value: f32, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float64Callback", |_| {
        Ok(|value: f64, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("boolCallback", |_| {
        Ok(|value: bool, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("stringCallback", |_| {
        Ok(|value: String, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateCallback", |_| {
        Ok(|value: NaiveDate, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateTimeCallback", |_| {
        Ok(|value: DateTime<Utc>, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("decimalCallback", |_| {
        Ok(|value: BigDecimal, container: Container| async move {
            container.set_message(Some(format!("{}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("statusCallback", |_| {
        Ok(|value: Status, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("int32ArrayCallback", |_| {
        Ok(|value: Vec<i32>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("int64ArrayCallback", |_| {
        Ok(|value: Vec<i64>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float32ArrayCallback", |_| {
        Ok(|value: Vec<f32>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("float64ArrayCallback", |_| {
        Ok(|value: Vec<f64>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("boolArrayCallback", |_| {
        Ok(|value: Vec<bool>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("stringArrayCallback", |_| {
        Ok(|value: Vec<String>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateArrayCallback", |_| {
        Ok(|value: Vec<NaiveDate>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("dateTimeArrayCallback", |_| {
        Ok(|value: Vec<DateTime<Utc>>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("decimalArrayCallback", |_| {
        Ok(|value: Vec<BigDecimal>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_callback_pipeline_item("statusArrayCallback", |_| {
        Ok(|value: Vec<Status>, container: Container| async move {
            container.set_message(Some(format!("{:?}", value)))?;
            Ok(())
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareInt32", |_| {
        Ok(|old: i32, new: i32| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareInt64", |_| {
        Ok(|old: i64, new: i64| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareFloat32", |_| {
        Ok(|old: f32, new: f32| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareFloat64", |_| {
        Ok(|old: f64, new: f64| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareBool", |_| {
        Ok(|old: bool, new: bool| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareString", |_| {
        Ok(|old: String, new: String| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareDate", |_| {
        Ok(|old: NaiveDate, new: NaiveDate| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareDateTime", |_| {
        Ok(|old: DateTime<Utc>, new: DateTime<Utc>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareDecimal", |_| {
        Ok(|old: BigDecimal, new: BigDecimal| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareStatus", |_| {
        Ok(|old: Status, new: Status| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareInt32Array", |_| {
        Ok(|old: Vec<i32>, new: Vec<i32>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareInt64Array", |_| {
        Ok(|old: Vec<i64>, new: Vec<i64>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareFloat32Array", |_| {
        Ok(|old: Vec<f32>, new: Vec<f32>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareFloat64Array", |_| {
        Ok(|old: Vec<f64>, new: Vec<f64>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareBoolArray", |_| {
        Ok(|old: Vec<bool>, new: Vec<bool>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareStringArray", |_| {
        Ok(|old: Vec<String>, new: Vec<String>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareDateArray", |_| {
        Ok(|old: Vec<NaiveDate>, new: Vec<NaiveDate>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareDateTimeArray", |_| {
        Ok(|old: Vec<DateTime<Utc>>, new: Vec<DateTime<Utc>>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareDecimalArray", |_| {
        Ok(|old: Vec<BigDecimal>, new: Vec<BigDecimal>| async move {
            old != new
        })
    });
    app.main_namespace().define_compare_pipeline_item("compareStatusArray", |_| {
        Ok(|old: Vec<Status>, new: Vec<Status>| async move {
            old != new
        })
    });
    Ok(app)
}