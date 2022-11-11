use crate::core::action::r#type::ActionType;
use crate::core::conf::client::TypeScriptClient;
use crate::core::graph::Graph;

pub(crate) async fn generate_runtime_ts(_graph: &Graph, conf: &TypeScriptClient) -> String {
    let actions = ActionType::iter().map(|a| { String::from("\"") + a.as_url_segment() + "\"" }).collect::<Vec<String>>().join(" | ");
    let url = &conf.host_url;
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

export function setCookie(name: string, value: string | undefined, daysToLive: number = 365) {{
    let cookie = name + "=" + (value ? encodeURIComponent(value!) : '')
    cookie += "; max-age=" + (value ? (daysToLive * 24 * 60 * 60) : 0)
    document.cookie = cookie
}}

export function getCookie(name: string, cookie?: string) : string | undefined {{
    let fullString
    if (typeof window === 'undefined') {{
        fullString = cookie
    }} else {{
        fullString = cookie ?? document.cookie
    }}
    var cookieArr = fullString!.split(";")
    for(var i = 0; i < cookieArr.length; i++) {{
        var cookiePair = cookieArr[i].split("=")
        if(name == cookiePair[0].trim()) {{
            return decodeURIComponent(cookiePair[1])
        }}
    }}
    return undefined
}}

export function setBearerToken(token: string | undefined) {{
    if (typeof window !== 'undefined') {{
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

    type: string
    errors: {{[key: string]: string}} | null

    constructor(responseError: ResponseError) {{
        super(responseError.message)
        this.type = responseError.type
        this.errors = responseError.errors
        Object.setPrototypeOf(this, TeoError.prototype)
    }}

    get name() {{
        return "TeoError"
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