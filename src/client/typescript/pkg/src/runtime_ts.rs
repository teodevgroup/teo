use inflector::Inflector;
use crate::action::action::ActionType;
use crate::client::shared::code::Code;
use crate::client::typescript::r#type::ToTypeScriptType;
use crate::core::field::Optionality;
use crate::core::graph::Graph;


pub(crate) async fn generate_runtime_ts(graph: &'static Graph) -> String {
    let actions = ActionType::iter().map(|a| { String::from("\"") + a.as_str() + "\"" }).collect::<Vec<String>>().join(" | ");
    let url = "localhost:5000";
    format!(r#"export interface Response<Meta, Data> {{
    meta: Meta
    data: Data
}}

export type PagingInfo = {{
    count: number
    pageSize?: number
    numberOfPages?: number
}}

export type Token = {{
    token: string
}}

type Action = {actions}

type Order = "asc" | "desc"

let bearerToken: string | undefined = undefined;
let bearerTokenLoaded: boolean = false;

export function setBearerToken(token: string) {{
    localStorage.setItem("__teo_bearer_token", token);
    bearerToken = token;
    bearerTokenLoaded = true;
}}

function getBearerToken(): string | undefined {{
    if (!bearerTokenLoaded) {{
        let token = localStorage.getItem("__teo_bearer_token");
        if (token != null) {{
            bearerToken = token
        }}
        bearerTokenLoaded = true
    }}
    return bearerToken
}}

export async function request(url_segment_name: string, action: Action, args: any): Promise<any> {{
    let url = "{url}" + url_segment_name + "/action"
    let response = await fetch(url, {{
        method: "POST",
        headers: getBearerToken() ? {{ "Authorization": `Bearer ${{getBearerToken()}}` }} : undefined,
        body: JSON.stringify({{ action, ...args }})
    }});
    let result = await response.json();
    result
}}
"#)
}
