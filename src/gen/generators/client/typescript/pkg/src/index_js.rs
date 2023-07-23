use inflector::Inflector;
use crate::gen::interface::client::conf::Conf;
use crate::prelude::Graph;

pub(crate) async fn generate_index_js(graph: &Graph, conf: &Conf) -> String {
    let mut name_map = "".to_owned();
    let host = &conf.host;
    let object_name = conf.object_name.clone();
    let mut class_name = object_name.to_pascal_case();
    if object_name == class_name { // in case of object name is capitalized
        class_name = class_name + "Class";
    }
    for model in graph.models() {
        if model.is_teo_internal() {
            continue
        }
        if model.name() != &model.name().to_camel_case() {
            name_map += &format!("  '{}': '{}',\n", model.name().to_camel_case(), model.name());
        }
    }
    format!(r#"const Decimal = require('decimal.js')

const nameMap = {{
{name_map}}}

let bearerToken = undefined
let bearerTokenLoaded = false

function setCookie(name, value, daysToLive = 365) {{
    let cookie = name + "=" + (value ? encodeURIComponent(value) : '')
    cookie += "; max-age=" + (value ? (daysToLive * 24 * 60 * 60) : 0)
    document.cookie = cookie
}}

function getCookie(name, cookie) {{
    let fullString
    if (typeof window === 'undefined') {{
        fullString = cookie
    }} else {{
        fullString = cookie ?? document.cookie
    }}
    var cookieArr = fullString.split(";")
    for(var i = 0; i < cookieArr.length; i++) {{
        var cookiePair = cookieArr[i].split("=")
        if(name == cookiePair[0].trim()) {{
            return decodeURIComponent(cookiePair[1])
        }}
    }}
    return undefined
}}

function setBearerToken(token) {{
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

function getBearerToken() {{
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

async function request(urlSegmentName, action, args, token = getBearerToken()) {{
  let url = "{host}/" + urlSegmentName + "/action/" + action
  let response = await fetch(url, {{
      method: "POST",
      headers: token ? {{ "Authorization": `Bearer ${{token}}` }} : undefined,
      body: JSON.stringify(args)
  }})
  let response_text = await response.text()
  let response_json = JSON.parse(response_text, (key, value) => {{
    if (typeof value === 'object' && value != null) {{
      if (value['$date']) {{
        return new Date(value['$date'])
      }} else if (value['$decimal']) {{
        return new Decimal(value['$decimal'])
      }}  else {{
        return value
      }}
    }} else {{
      return value
    }}
  }})
  if (400 <= response.status) {{
      throw new TeoError(response_json.error)
  }}
  return response_json
}}

class TeoError extends Error {{

  constructor(responseError) {{
      super(responseError.message)
      this.type = responseError.type
      this.errors = responseError.errors
      Object.setPrototypeOf(this, TeoError.prototype)
  }}

  get name() {{
      return "TeoError"
  }}
}}


class Delegate {{

  constructor(urlSegmentName, token) {{
    this._urlSegmentName = urlSegmentName
    this._token = token
    return new Proxy(this, {{
      get(target, name, receiver) {{
        return function (args) {{
          return request(
            target._urlSegmentName,
            name,
            args ?? {{}},
            target._token)
        }}
      }}
    }})
  }}

  $withToken(token) {{
    let retval = new Delegate(this._urlSegmentName, this._token)
    retval._token = token
    return retval
  }}
}}

class Teo {{

  constructor() {{
    this._token = undefined
    return new Proxy(this, {{
      get(target, name, receiver) {{
        if (name === '$withToken') {{
          return (token) => {{
            let retval = new Teo()
            retval._token = token
            return retval
          }}
        }} else {{
          return new Delegate(nameMap[name] || name, target._token)
        }}
      }},
    }})
  }}

}}

const {object_name} = new {class_name}()

module.exports = {{
  Decimal,
  setBearerToken,
  getBearerToken,
  TeoError,
  {object_name},
}}
"#)
}
