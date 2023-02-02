use crate::core::action::r#type::ActionType;
use crate::core::app::conf::ClientGeneratorConf;
use crate::core::graph::Graph;
use crate::parser::ast::client::Client;

pub(crate) async fn generate_runtime_d_ts(_graph: &Graph, conf: &ClientGeneratorConf) -> String {
    let actions = ActionType::iter().map(|a| { String::from("\"") + a.as_url_segment() + "\"" }).collect::<Vec<String>>().join(" | ");
    let url = &conf.host;
    format!(r#"type Action = {actions}

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

export interface Response<Meta, Data> {{
    meta: Meta
    data: Data
}}

export interface ResponseError {{
    type: string
    message: string
    errors: {{[key: string]: string}} | null
}}

export type PagingInfo = {{
    count: number
    numberOfPages?: number
}}

export type TokenInfo = {{
    token: string
}}
"#)
}
