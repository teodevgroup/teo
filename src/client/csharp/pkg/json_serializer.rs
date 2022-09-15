use crate::core::graph::Graph;

pub(crate) async fn generate_json_serializer_cs(_graph: &Graph) -> String {
    format!(r#"using System;
using System.Text.Json;
using System.Text.Json.Serialization;

#nullable enable
namespace Teo {{

    public class DateTimeOffsetConverter : JsonConverter<DateTimeOffset> {{
        public override DateTimeOffset Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            return DateTimeOffset.Parse(reader.GetString() ?? string.Empty);
        }}

        public override void Write(Utf8JsonWriter writer, DateTimeOffset value, JsonSerializerOptions options) {{
            writer.WriteStringValue(value.UtcDateTime.ToString("yyyy-MM-ddTHH:mm:ss.fffK"));
        }}
    }}

    public class OneOfJsonConverter<T0, T1> : JsonConverter<OneOf<T0, T1>> {{

        public override OneOf<T0, T1> Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            throw new NotImplementedException();
        }}

        public override void Write(Utf8JsonWriter writer, OneOf<T0, T1> value, JsonSerializerOptions options) {{
            value.Switch(
                t0 => {{ writer.WriteRawValue(JSJsonSerializer.Serialize(t0)); }},
                t1 => {{ writer.WriteRawValue(JSJsonSerializer.Serialize(t1)); }});
        }}
    }}

    static public class JSJsonSerializer {{
        static private JsonSerializerOptions options() {{
            var options = new JsonSerializerOptions {{
                PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
                DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
            }};
            options.Converters.Add(new DateTimeOffsetConverter());
            options.Converters.Add(new OptionalJsonConverter<string>());
            options.Converters.Add(new OptionalJsonConverter<int>());
            return options;
        }}
        static public string Serialize<T>(T value) {{
            return JsonSerializer.Serialize(value, options());
        }}
        static public T? Deserialize<T>(string value) {{
            return JsonSerializer.Deserialize<T>(value, options());
        }}
    }}
}}
"#)
}