pub(crate) mod custom_action_decoder;
pub(crate) mod decoder;

use maplit::hashmap;
use teo_teon::value::Value;
use crate::core::field::r#type::FieldType;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;

pub(crate) fn number_from_i32(num: i32, r#type: &FieldType) -> Value {
    match r#type {
        FieldType::I32 => Value::I32(num as i32),
        _ => panic!(),
    }
}

pub(crate) fn number_from_f64(num: f64, r#type: &FieldType) -> Value {
    match r#type {
        FieldType::F32 => Value::F32(num as f32),
        FieldType::F64 => Value::F64(num),
        _ => panic!()
    }
}
pub(crate) fn number_from_f32(num: f32, r#type: &FieldType) -> Value {
    match r#type {
        FieldType::F32 => Value::F32(num),
        FieldType::F64 => Value::F64(num as f64),
        _ => panic!()
    }
}

pub(crate) fn number_from_i64(num: i64, r#type: &FieldType) -> Value {
    match r#type {
        FieldType::I32 => Value::I32(num as i32),
        FieldType::I64 => Value::I64(num),
        _ => panic!()
    }
}

// resolve pipeline as value
pub(crate) async fn resolve(value: &Value, context: PipelineCtx<'_>) -> Result<Value> {
    match value {
        Value::Pipeline(p) => p.process(context).await,
        Value::HashMap(map) => {
            let mut new_map = hashmap!{};
            for (key, value) in map {
                if let Some(p) = value.as_pipeline() {
                    new_map.insert(key.clone(), p.process(context.clone()).await?);
                } else {
                    new_map.insert(key.clone(), value.clone());
                }
            }
            Ok(Value::HashMap(new_map))
        }
        Value::Vec(vec) => {
            let mut new_vec = vec![];
            for val in vec {
                if let Some(p) = val.as_pipeline() {
                    new_vec.push(p.process(context.clone()).await?);
                } else {
                    new_vec.push(val.clone());
                }
            }
            Ok(Value::Vec(new_vec))
        }
        _ => Ok(value.clone()),
    }
}