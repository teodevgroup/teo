use crate::core::graph::Graph;


pub(crate) async fn generate_operation_d_ts(_graph: &Graph) -> String {
    format!(r#"import Decimal from "./decimal"

export type ObjectIdFieldUpdateOperationsInput = {{
    set?: string
}}

export type NullableObjectIdFieldUpdateOperationsInput = {{
    set?: string | null
}}

export type StringFieldUpdateOperationsInput = {{
    set?: string
}}

export type NullableStringFieldUpdateOperationsInput = {{
    set?: string | null
}}

export type NumberFieldUpdateOperationsInput = {{
    set?: number
    increment?: number
    decrement?: number
    multiply?: number
    divide?: number
}}

export type NullableNumberFieldUpdateOperationsInput = {{
    set?: number | null
    increment?: number
    decrement?: number
    multiply?: number
    divide?: number
}}

export type DecimalFieldUpdateOperationsInput = {{
    set?: string | Decimal
    increment?: string | Decimal
    decrement?: string | Decimal
    multiply?: string | Decimal
    divide?: string | Decimal
}}

export type NullableDecimalFieldUpdateOperationsInput = {{
    set?: string | Decimal | null
    increment?: string | Decimal
    decrement?: string | Decimal
    multiply?: string | Decimal
    divide?: string | Decimal
}}

export type BoolFieldUpdateOperationsInput = {{
    set?: boolean
}}

export type NullableBoolFieldUpdateOperationsInput = {{
    set?: boolean | null
}}

export type DateFieldUpdateOperationsInput = {{
    set?: string | Date
}}

export type NullableDateFieldUpdateOperationsInput = {{
    set?: string | Date | null
}}

export type DateTimeFieldUpdateOperationsInput = {{
    set?: string | Date
}}

export type NullableDateTimeFieldUpdateOperationsInput = {{
    set?: string | Date | null
}}

export type EnumFieldUpdateOperationsInput<T> = {{
    set?: T
}}

export type NullableEnumFieldUpdateOperationsInput<T> = {{
    set?: T | null
}}

export type ArrayFieldUpdateOperationsInput<T> = {{
    set?: T[],
    push?: T
}}

export type NullableArrayFieldUpdateOperationsInput<T> = {{
    set?: T[] | null,
    push?: T
}}
"#)
}
