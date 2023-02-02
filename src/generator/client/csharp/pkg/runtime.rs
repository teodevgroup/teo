use crate::core::app::conf::ClientGeneratorConf;
use crate::core::graph::Graph;
use crate::parser::ast::client::Client;

pub(crate) async fn generate_runtime_cs(_graph: &Graph, client: &ClientGeneratorConf) -> String {
    let url = &client.host;
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

    public class TeoException : Exception {{

        public override string Message {{ get; }}

        public string Type {{ get; }}

        public Dictionary<string, string>? Errors {{ get; }}

        public TeoException(ResponseError responseError) {{
            Message = responseError.Message;
            Type = responseError.Type;
            Errors = responseError.Errors;
        }}
    }}

    public class Delegate {{

        protected static readonly string HOST = "{url}";

        protected async Task<T> Request<T>(string urlSegmentName, string action, object args, string? token = null) {{
            // not handle our own errors yet
            var uri = new Uri(HOST + "/" + urlSegmentName + "/action/" + action);
            var client = new HttpClient();
            if (token != null) {{
                client.DefaultRequestHeaders.Add("Authorization", $"Bearer {{token}}");
            }}
            var content = JSJsonSerializer.Serialize(args)!;
            var response = await client.PostAsync(uri, new HttpStringContent(content));
            var httpResponseBody = await response.Content.ReadAsStringAsync();
            if (((int)response.StatusCode) >= 400) {{
                throw new TeoException(JSJsonSerializer.Deserialize<ResponseError>(httpResponseBody)!);
            }}
            return JSJsonSerializer.Deserialize<T>(httpResponseBody)!;
        }}
    }}
}}
"#)
}
