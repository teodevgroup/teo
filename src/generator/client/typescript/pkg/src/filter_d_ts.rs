use crate::core::graph::Graph;

pub(crate) async fn generate_filter_d_ts(_graph: &Graph, use_local_decimal: bool) -> String {
    let decimal = if use_local_decimal {
        "./decimal"
    } else {
        "decimal.js"
    };
    format!(r#"import Decimal from "{decimal}"

export type ObjectIdFilter = {{
    equals?: string
    in?: string[]
    notIn?: string[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    not?: ObjectIdFilter | string
}}

export type ObjectIdNullableFilter = {{
    equals?: string | null
    in?: (string | null)[]
    notIn?: (string | null)[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    not?: ObjectIdNullableFilter | string | null
}}

export type BoolFilter = {{
    equals?: boolean
    not?: BoolFilter | boolean
}}

export type BoolNullableFilter = {{
    equals?: boolean | null
    not?: BoolNullableFilter | boolean | null
}}

export type NumberFilter = {{
    equals?: number
    in?: number[]
    notIn?: number[]
    lt?: number
    lte?: number
    gt?: number
    gte?: number
    not?: NumberFilter | number
}}

export type NumberNullableFilter = {{
    equals?: number | null
    in?: (number | null)[]
    notIn?: (number | null)[]
    lt?: number
    lte?: number
    gt?: number
    gte?: number
    not?: NumberNullableFilter | number | null
}}

export type DecimalFilter = {{
    equals?: Decimal | string
    in?: (Decimal | string)[]
    notIn?: (Decimal | string)[]
    lt?: Decimal | string
    lte?: Decimal | string
    gt?: Decimal | string
    gte?: Decimal | string
    not?: DecimalFilter | Decimal | string
}}

export type DecimalNullableFilter = {{
    equals?: Decimal | string | null
    in?: (Decimal | string | null)[]
    notIn?: (Decimal | string | null)[]
    lt?: Decimal | string
    lte?: Decimal | string
    gt?: Decimal | string
    gte?: Decimal | string
    not?: DecimalNullableFilter | Decimal | string | null
}}

export type StringFilter = {{
    equals?: string
    in?: string[]
    notIn?: string[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    contains?: string
    startsWith?: string
    endsWith?: string
    matches?: string
    not?: StringFilter | string
}}

export type StringNullableFilter = {{
    equals?: string | null
    in?: (string | null)[]
    notIn?: (string | null)[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    contains?: string
    startsWith?: string
    endsWith?: string
    matches?: string
    not?: StringNullableFilter | string | null
}}

export type DateFilter = {{
    equals?: string
    in?: string[]
    notIn?: string[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    not?: DateFilter | string
}}

export type DateNullableFilter = {{
    equals?: string | null
    in?: (string | null)[]
    notIn?: (string | null)[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    not?: DateNullableFilter | string | null
}}

export type DateTimeFilter = {{
    equals?: string
    in?: string[]
    notIn?: string[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    not?: DateFilter | string
}}

export type DateTimeNullableFilter = {{
    equals?: string | null
    in?: (string | null)[]
    notIn?: (string | null)[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
    not?: DateNullableFilter | string | null
}}

export type EnumFilter<T> = {{
    equals?: T
    in?: T[]
    notIn?: T[]
    not?: EnumFilter<T>
}}

export type EnumNullableFilter<T> = {{
    equals?: T | null
    in?: (T | null)[]
    notIn?: (T | null)[]
    not?: EnumNullableFilter<T> | T | null
}}

export type ArrayFilter<T> = {{
    equals?: T[]
    has?: T
    hasSome?: T[]
    hasEvery?: T[]
    isEmpty?: boolean
    length?: number
}}

export type ArrayNullableFilter<T> = {{
    equals?: T[] | null
    has?: T
    hasSome?: T[]
    hasEvery?: T[]
    isEmpty?: boolean
    length?: number
}}
"#)
}
