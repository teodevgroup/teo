use crate::core::action::Action;
use crate::core::app::conf::ClientGeneratorConf;
use crate::core::graph::Graph;


pub(crate) fn generate_runtime_d_ts() -> String {
    format!(r#"
export type ExistKeys<T> = {{
    [key in keyof T]: T[key] extends false | undefined | null ? never : key
}}[keyof T]

type HasSelect = {{
    select: any
}}

type HasInclude = {{
    include: any
}}

export type CheckSelectInclude<T, S, U> = T extends HasSelect
    ? U
    : T extends HasInclude
    ? U
    : S

export type SelectSubset<T, U> = U extends HasSelect
    ? {{
        [K in ExistKeys<U['select']>]: K extends keyof T ? T[K] : never
    }}
    : T

export type Enumerable<T> = T | Array<T>

export type SortOrder = "asc" | "desc"
"#)
}
