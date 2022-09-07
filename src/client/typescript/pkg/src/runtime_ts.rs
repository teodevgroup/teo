use crate::action::action::ActionType;
use crate::app::app::ClientConfiguration;
use crate::core::graph::Graph;


pub(crate) async fn generate_runtime_ts(_graph: &Graph, conf: &ClientConfiguration) -> String {
    let actions = ActionType::iter().map(|a| { String::from("\"") + a.as_url_segment() + "\"" }).collect::<Vec<String>>().join(" | ");
    let url = match &conf.host_url {
        Some(h) => h.as_str(),
        None => ""
    };
    format!(r#"type Action = {actions}

export type Enumerable<T> = T | Array<T>

export type SortOrder = "asc" | "desc"

export interface Response<Meta, Data> {{
    meta: Meta
    data: Data
}}

export type PagingInfo = {{
    count: number
    numberOfPages?: number
}}

export type TokenInfo = {{
    token: string
}}

let bearerToken: string | undefined = undefined;
let bearerTokenLoaded: boolean = false;

export function setBearerToken(token: string | undefined) {{
    if (localStorage) {{
        if (token === undefined) {{
            localStorage.removeItem("__teo_bearer_token")
        }} else {{
            localStorage.setItem("__teo_bearer_token", token)
        }}
        bearerToken = token;
        bearerTokenLoaded = true;
    }}
}}

function getBearerToken(): string | undefined {{
    if (!bearerTokenLoaded) {{
        if (localStorage) {{
            let token = localStorage.getItem("__teo_bearer_token")
            if (token != null) {{
                bearerToken = token
            }}
        }}
        bearerTokenLoaded = true
    }}
    return bearerToken
}}

export async function request(urlSegmentName: string, action: Action, args: any): Promise<any> {{
    let url = "{url}/" + urlSegmentName + "/action/" + action
    let response = await fetch(url, {{
        method: "POST",
        headers: getBearerToken() ? {{ "Authorization": `Bearer ${{getBearerToken()}}` }} : undefined,
        body: JSON.stringify(args)
    }})
    return await response.json()
}}
"#)
}
