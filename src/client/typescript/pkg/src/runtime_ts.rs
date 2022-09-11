use crate::core::action::r#type::ActionType;
use crate::app::app::ClientConfiguration;
use crate::core::graph::Graph;


pub(crate) async fn generate_runtime_ts(_graph: &Graph, conf: &ClientConfiguration) -> String {
    let actions = ActionType::iter().map(|a| { String::from("\"") + a.as_url_segment() + "\"" }).collect::<Vec<String>>().join(" | ");
    let url = match &conf.host_url {
        Some(h) => h.as_str(),
        None => ""
    };
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

let bearerToken: string | undefined = undefined
let bearerTokenLoaded: boolean = false

export const setCookie = (key: string, value: string | undefined) => {{
    if (!document) return
    let retval = document.cookie.replace(new RegExp(`${{key}}:(.*?); ?`), '')
    if (value) {{
        retval = retval + " " + key + "=" + value + ";";
    }}
    document.cookie = retval
}}

export const getCookie = (key: string, cookie?: string): string | undefined => {{
    let fullString
    if (typeof window === 'undefined') {{
        fullString = cookie
    }} else {{
        fullString = cookie ?? document.cookie
    }}
    if (!fullString) return undefined
    const result = fullString.match(new RegExp(`${{key}}=([^; ]*);?`))
    if (result === null) return undefined
    return result[1]
}}

export function setBearerToken(token: string | undefined) {{
    if (typeof window === 'undefined') {{
        // local storage
        if (token === undefined) {{
            localStorage.removeItem("__teo_bearer_token")
        }} else {{
            localStorage.setItem("__teo_bearer_token", token)
        }}
        // cookie
        setCookie("__teo_bearer_token", token)
        bearerToken = token
        bearerTokenLoaded = true
    }}
}}

export function getBearerToken(): string | undefined {{
    if (typeof window === 'undefined') {{
        bearerTokenLoaded = true
        return undefined
    }} else {{
        if (!bearerTokenLoaded) {{
            if (localStorage) {{
                let token = localStorage.getItem("__teo_bearer_token")
                if (token != null) {{
                    bearerToken = token
                }}
            }}
            if (document && !bearerToken) {{
                bearerToken = getCookie("__teo_bearer_token")
            }}
            bearerTokenLoaded = true
        }}
        return bearerToken
    }}
}}

export class TeoError extends Error {{

    responseError: ResponseError

    constructor(responseError: ResponseError) {{
        super()
        this.responseError = responseError
    }}

    get type() {{
        return this.responseError.type
    }}

    get message() {{
        return this.responseError.message
    }}

    get errors() {{
        return this.responseError.errors
    }}
}}

export async function request(urlSegmentName: string, action: Action, args: any, token: string | undefined = getBearerToken()): Promise<any> {{
    let url = "{url}/" + urlSegmentName + "/action/" + action
    let response = await fetch(url, {{
        method: "POST",
        headers: token ? {{ "Authorization": `Bearer ${{token}}` }} : undefined,
        body: JSON.stringify(args)
    }})
    let response_json = await response.json()
    if (400 <= response.status) {{
        throw new TeoError(response_json.error)
    }}
    return response_json
}}
"#)
}
