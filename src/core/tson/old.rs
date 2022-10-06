pub(crate) fn number_from_f64(num: f64, r#type: &FieldType) -> Value {
    match r#type {
        FieldType::F32 => Value::F32(num as f32),
        FieldType::F64 => Value::F64(num),
        _ => panic!()
    }
}

pub(crate) fn number_from_i64(num: i64, r#type: &FieldType) -> Value {
    match r#type {
        FieldType::I8 => Value::I8(num as i8),
        FieldType::I16 => Value::I16(num as i16),
        FieldType::I32 => Value::I32(num as i32),
        FieldType::I64 => Value::I64(num as i64),
        FieldType::I128 => Value::I128(num as i128),
        FieldType::U8 => Value::U8(num as u8),
        FieldType::U16 => Value::U16(num as u16),
        FieldType::U32 => Value::U32(num as u32),
        FieldType::U64 => Value::U64(num as u64),
        FieldType::U128 => Value::U128(num as u128),
        _ => panic!()
    }
}

#[async_recursion]
pub(crate) async fn to_object_json_value(&self) -> Option<Value> {
    match self {
        Value::Object(o) => {
            match o.to_json().await {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        }
        _ => None
    }
}

#[async_recursion]
pub(crate) async fn to_object_vec_json_value(&self) -> Option<Value> {
    match self {
        Value::Vec(vec) => {
            let mut result: Vec<Value> = vec![];
            for object in vec {
                result.push(object.to_object_json_value().await.unwrap());
            }
            Some(Value::Array(result))
        }
        _ => None
    }
}


pub fn is_object(&self) -> bool {
    match self {
        Value::Object(_) => true,
        _ => false,
    }
}

pub fn is_object_vec(&self) -> bool {
    match self {
        Value::Vec(v) => {
            if v.is_empty() {
                false
            } else {
                v.get(0).unwrap().is_object()
            }
        }
        _ => false,
    }
}
