use crate::app::app::ClientConfiguration;
use crate::core::graph::Graph;

pub(crate) async fn generate_runtime_cs(_graph: &Graph, conf: &ClientConfiguration) -> String {
    let url = match &conf.host_url {
        Some(h) => h.as_str(),
        None => ""
    };
    format!(r#"using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Windows.Web.Http;

namespace Teo {{
    public struct Response<D> {{
        public D Data {{ get; set; }}
    }}

    public struct Response<M, D> {{
        public M Meta {{ get; set; }}
        public D Data {{ get; set; }}
    }}

    public struct ResponseError {{
        public string Type {{ get; set; }}
        public string Message {{ get; set; }}
        public Dictionary<string, string>? Errors {{ get; set; }}
    }}

    public struct PagingInfo {{
        public uint Count {{ get; set; }}
        public uint? NumberOfPages {{ get; set; }}
    }}

    public struct TokenInfo {{
        public string Token {{ get; set; }}
    }}

    public class Delegate {{

        protected static readonly string HOST = "{url}";

        protected async Task<T> Request<T>(string urlSegmentName, string action, object args, string? token = null) {{
            // not install token yet
            // not handle our own errors yet
            var uri = new Uri(HOST + "/" + urlSegmentName + "/action/" + action);
            var client = new HttpClient();
            var content = JSJsonSerializer.Serialize(args)!;
            var response = await client.PostAsync(uri, new HttpStringContent(content));
            var httpResponseBody = await response.Content.ReadAsStringAsync();
            return JSJsonSerializer.Deserialize<T>(httpResponseBody)!;
        }}
    }}
}}
"#)
}