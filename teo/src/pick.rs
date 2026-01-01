use serde::{Deserialize, Serialize};

use crate::select::Select;

#[repr(transparent)]
#[derive(Serialize, Deserialize)]
struct Pick<T>(T::Value) where T: Select;
