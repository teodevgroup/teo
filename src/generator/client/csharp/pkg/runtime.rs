use crate::core::app::conf::ClientGeneratorConf;
use crate::core::graph::Graph;

pub(crate) async fn generate_runtime_cs(_graph: &Graph, client: &ClientGeneratorConf) -> String {
    let url = &client.host;
    format!(r#"
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
