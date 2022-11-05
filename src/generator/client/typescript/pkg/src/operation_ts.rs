use crate::core::graph::Graph;


pub(crate) async fn generate_operation_ts(_graph: &Graph) -> String {
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
