use crate::core::graph::Graph;

pub(crate) fn generate_filter_types(server_mode: bool) -> String {
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
    format!(r#"export type ObjectIdFilter = {{
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
    equals?: {decimal_base}
    in?: ({decimal_base})[]
    notIn?: ({decimal_base})[]
    lt?: {decimal_base}
    lte?: {decimal_base}
    gt?: {decimal_base}
    gte?: {decimal_base}
    not?: DecimalFilter | {decimal_base}
}}

export type DecimalNullableFilter = {{
    equals?: {decimal_base} | null
    in?: ({decimal_base} | null)[]
    notIn?: ({decimal_base} | null)[]
    lt?: {decimal_base}
    lte?: {decimal_base}
    gt?: {decimal_base}
    gte?: {decimal_base}
    not?: DecimalNullableFilter | {decimal_base} | null
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
    equals?: {datetime_base}
    in?: {datetime_base}[]
    notIn?: {datetime_base}[]
    lt?: {datetime_base}
    lte?: {datetime_base}
    gt?: {datetime_base}
    gte?: {datetime_base}
    not?: DateFilter | {datetime_base}
}}

export type DateTimeNullableFilter = {{
    equals?: {datetime_base} | null
    in?: ({datetime_base} | null)[]
    notIn?: ({datetime_base} | null)[]
    lt?: {datetime_base}
    lte?: {datetime_base}
    gt?: {datetime_base}
    gte?: {datetime_base}
    not?: DateNullableFilter | {datetime_base} | null
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
