use async_trait::async_trait;
use crate::core::modifier::Modifier;
use crate::core::value::Value;
use crate::core::object::Object;
use crate::core::stage::Stage;


#[derive(Debug, Copy, Clone)]
pub struct AddIModifier {
    addend: i128
}

impl AddIModifier {
    pub fn new(addend: i128) -> Self {
        return AddIModifier {
            addend
        };
    }
}

#[async_trait]
impl Modifier for AddIModifier {

    fn name(&self) -> &'static str {
        "addi"
    }

    async fn call(&self, stage: Stage, _object: Object) -> Stage {
        return if let Some(value) = stage.value() {
            return if let Value::I8(i) = value {
                Stage::Value(Value::I8(i + self.addend as i8))
            } else if let Value::I16(i) = value {
                Stage::Value(Value::I16(i + self.addend as i16))
            } else if let Value::I32(i) = value {
                Stage::Value(Value::I32(i + self.addend as i32))
            } else if let Value::I64(i) = value {
                Stage::Value(Value::I64(i + self.addend as i64))
            } else if let Value::I128(i) = value {
                Stage::Value(Value::I128(i + self.addend))
            } else if let Value::U8(i) = value {
                Stage::Value(Value::U8(i + self.addend as u8))
            } else if let Value::U16(i) = value {
                Stage::Value(Value::U16(i + self.addend as u16))
            } else if let Value::U32(i) = value {
                Stage::Value(Value::U32(i + self.addend as u32))
            } else if let Value::U64(i) = value {
                Stage::Value(Value::U64(i + self.addend as u64))
            } else if let Value::U128(i) = value {
                Stage::Value(Value::U128(i + self.addend as u128))
            } else if let Value::F32(f) = value {
                Stage::Value(Value::F32(f + self.addend as f32))
            } else if let Value::F64(f) = value {
                Stage::Value(Value::F64(f + self.addend as f64))
            } else {
                Stage::Value(value)
            }
        } else {
            stage
        }
    }
}
