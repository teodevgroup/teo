use crate::core::graph::Graph;

pub(crate) async fn generate_json_serializer_cs(_graph: &Graph) -> String {
    format!(r#"using System;
using System.Reflection;
using System.Text.Json;
using System.Text.Json.Serialization;

#nullable enable
namespace Teo {{

    public class DateOnlyConverter : JsonConverter<DateOnly> {{
        public override DateOnly Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            return DateOnly.Parse(reader.GetString() ?? string.Empty);
        }}

        public override void Write(Utf8JsonWriter writer, DateOnly value, JsonSerializerOptions options) {{
            writer.WriteStringValue(value.ToString("yyyy-MM-dd"));
        }}
    }}

    public class DateTimeConverter : JsonConverter<DateTime> {{
        public override DateTime Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            return DateTime.Parse(reader.GetString() ?? string.Empty);
        }}

        public override void Write(Utf8JsonWriter writer, DateTime value, JsonSerializerOptions options) {{
            writer.WriteStringValue(value.ToString("yyyy-MM-ddTHH:mm:ss.fffK"));
        }}
    }}

    public class DateTimeOffsetConverter : JsonConverter<DateTimeOffset> {{
        public override DateTimeOffset Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            return DateTimeOffset.Parse(reader.GetString() ?? string.Empty);
        }}

        public override void Write(Utf8JsonWriter writer, DateTimeOffset value, JsonSerializerOptions options) {{
            writer.WriteStringValue(value.UtcDateTime.ToString("yyyy-MM-ddTHH:mm:ss.fffK"));
        }}
    }}

    public class OneOfJsonConverterFactory : JsonConverterFactory {{
        public override bool CanConvert(Type typeToConvert) {{
            if (!typeToConvert.IsGenericType) {{
                return false;
            }}
            if (typeToConvert.GetGenericTypeDefinition() != typeof(OneOf<,>)) {{
                return false;
            }}
            return true;
        }}

        public override JsonConverter? CreateConverter(Type typeToConvert, JsonSerializerOptions options) {{
            Type t0 = typeToConvert.GetGenericArguments()[0];
            Type t1 = typeToConvert.GetGenericArguments()[1];

            JsonConverter converter = (JsonConverter)Activator.CreateInstance(
                typeof(OneOfJsonConverter<,>).MakeGenericType(
                    new Type[] {{ t0, t1 }}),
                BindingFlags.Instance | BindingFlags.Public,
                binder: null,
                args: new object[] {{ options }},
                culture: null)!;

            return converter;
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

    public class OptionalJsonConverterFactory : JsonConverterFactory {{

        public override bool CanConvert(Type typeToConvert) {{
            if (!typeToConvert.IsGenericType) {{
                return false;
            }}
            if (typeToConvert.GetGenericTypeDefinition() != typeof(Optional<>)) {{
                return false;
            }}
            return true;
        }}

        public override JsonConverter? CreateConverter(Type typeToConvert, JsonSerializerOptions options) {{
            Type t0 = typeToConvert.GetGenericArguments()[0];

            JsonConverter converter = (JsonConverter)Activator.CreateInstance(
                typeof(Optional<>).MakeGenericType(
                    new Type[] {{ t0 }}),
                BindingFlags.Instance | BindingFlags.Public,
                binder: null,
                args: new object[] {{ options }},
                culture: null)!;

            return converter;
        }}
    }}

    public class OptionalJsonConverter<T> : JsonConverter<Optional<T>> {{

        public override Optional<T> Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            throw new NotImplementedException();
        }}

        public override void Write(Utf8JsonWriter writer, Optional<T> value, JsonSerializerOptions options) {{
            value.Value.Switch(
                t0 => writer.WriteRawValue(JSJsonSerializer.Serialize(value.Value.AsT0)),
                t1 => writer.WriteRawValue("null"));
        }}
    }}

    static public class JSJsonSerializer {{
        static private JsonSerializerOptions options() {{
            var options = new JsonSerializerOptions {{
                PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
                DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
            }};
            options.Converters.Add(new OneOfJsonConverterFactory());
            options.Converters.Add(new DateOnlyConverter());
            options.Converters.Add(new DateTimeConverter());
            options.Converters.Add(new DateTimeOffsetConverter());
            options.Converters.Add(new OptionalJsonConverterFactory());
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