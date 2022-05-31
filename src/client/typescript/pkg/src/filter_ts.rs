use inflector::Inflector;
use crate::action::action::ActionType;
use crate::client::shared::code::Code;
use crate::client::typescript::r#type::ToTypeScriptType;
use crate::core::field::Availability;
use crate::core::graph::Graph;


pub(crate) async fn generate_filter_ts(_graph: &'static Graph) -> String {
    format!(r#"export type ObjectIdFilter = {{
    equals?: string
    in?: string[]
    notIn?: string[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
}}

export type ObjectIdNullableFilter = {{
    equals?: string | null
    in?: (string | null)[]
    notIn?: (string | null)[]
    lt?: string
    lte?: string
    gt?: string
    gte?: string
}}

export type BooleanFilter = {{
    equals?: boolean
}}

export type BooleanNullableFilter = {{
    equals?: boolean | null
}}

export type NumberFilter = {{
    equals?: number
    in?: number[]
    notIn?: number[]
    lt?: number
    lte?: number
    gt?: number
    gte?: number
}}

export type NumberNullableFilter = {{
    equals?: number | null
    in?: (number | null)[]
    notIn?: (number | null)[]
    lt?: number
    lte?: number
    gt?: number
    gte?: number
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
}}
"#)
}
