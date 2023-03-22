use crate::core::graph::Graph;


pub(crate) fn generate_operation_types(server_mode: bool) -> String {
    let decimal_base = if server_mode {
        "Decimal"
    } else {
        "Decimal | string"
    };
    let datetime_base = if server_mode {
        "Date"
    } else {
        "Date | string"
    };
    format!(r#"export type ObjectIdFieldUpdateOperationsInput = {{
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
    set?: {decimal_base}
    increment?: {decimal_base}
    decrement?: {decimal_base}
    multiply?: {decimal_base}
    divide?: {decimal_base}
}}

export type NullableDecimalFieldUpdateOperationsInput = {{
    set?: {decimal_base} | null
    increment?: {decimal_base}
    decrement?: {decimal_base}
    multiply?: {decimal_base}
    divide?: {decimal_base}
}}

export type BoolFieldUpdateOperationsInput = {{
    set?: boolean
}}

export type NullableBoolFieldUpdateOperationsInput = {{
    set?: boolean | null
}}

export type DateFieldUpdateOperationsInput = {{
    set?: string
}}

export type NullableDateFieldUpdateOperationsInput = {{
    set?: string | null
}}

export type DateTimeFieldUpdateOperationsInput = {{
    set?: {datetime_base}
}}

export type NullableDateTimeFieldUpdateOperationsInput = {{
    set?: {datetime_base} | null
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
