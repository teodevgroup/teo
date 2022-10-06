use crate::prelude::Value;

pub(crate) fn has_negative_take(json_value: &Value) -> bool {
    if json_value.is_object() {
        let take = json_value.as_object().unwrap().get("take");
        if take.is_some() {
            let take = take.unwrap();
            if take.is_number() {
                let take = take.as_i64().unwrap();
                return take < 0;
            }
        }
    }
    false
}
