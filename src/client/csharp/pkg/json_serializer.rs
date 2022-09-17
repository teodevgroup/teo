use inflector::Inflector;
use crate::core::graph::Graph;
use crate::core::r#enum::Enum;

fn enum_read_cases(e: &Enum) -> String {
    let enum_name = &e.name;
    let mut retval = "".to_owned();
    let spaces_string = " ".repeat(16);
    let spaces = &spaces_string;
    for c in &e.choices {
        let c_value = &c.name;
        let pascal = c_value.to_pascal_case();
        retval += format!(r#"{spaces}case "{c_value}": {{"#).as_str();
        retval.push('\n');
        retval += format!("{spaces}    return {enum_name}.{pascal};\n").as_str();
        retval += format!("{spaces}}}\n").as_str();
    }
    retval
}

fn enum_write_cases(e: &Enum) -> String {
    let enum_name = &e.name;
    let mut retval = "".to_owned();
    let spaces_string = " ".repeat(16);
    let spaces = &spaces_string;
    for c in &e.choices {
        let c_value = &c.name;
        let pascal = c_value.to_pascal_case();
        retval += format!(r#"{spaces}case {enum_name}.{pascal}: {{"#).as_str();
        retval.push('\n');
        retval += format!(r#"{spaces}    writer.WriteStringValue("{c_value}");"#).as_str();
        retval.push('\n');
        retval += format!("{spaces}    return;\n").as_str();
        retval += format!("{spaces}}}\n").as_str();
    }
    retval
}

fn serializer_for_enum(e: &Enum) -> String {
    let enum_name = &e.name;
    let read_cases = enum_read_cases(e);
    let write_cases = enum_write_cases(e);
    format!(r#"    public class {enum_name}JsonConverter : JsonConverter<{enum_name}> {{

        public override {enum_name} Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            switch (reader.GetString()) {{
{read_cases}
            }}
            throw new NotImplementedException();
        }}

        public override void Write(Utf8JsonWriter writer, {enum_name} value, JsonSerializerOptions options) {{
            switch (value) {{
{write_cases}
            }}
        }}
    }}
"#)
}

fn serializer_for_enums(g: &Graph) -> String {
    let mut retval = "".to_owned();
    for (_, e) in g.enums() {
        retval += serializer_for_enum(e).as_str();
        retval.push('\n');
    }
    retval
}

fn enum_converters(g: &Graph) -> String {
    let mut retval = "".to_owned();
    for (_, e) in g.enums() {
        let enum_name = &e.name;
        retval += format!("            options.Converters.Add(new {enum_name}JsonConverter());").as_str();
        retval.push('\n');
    }
    retval
}

pub(crate) async fn generate_json_serializer_cs(graph: &Graph) -> String {
    let enums = serializer_for_enums(graph);
    let enum_converters = enum_converters(graph);
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
                typeof(OneOfJsonConverter<,>).MakeGenericType(new Type[] {{ t0, t1 }}),
                BindingFlags.Instance | BindingFlags.Public,
                binder: null,
                args: null,
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
                typeof(OptionalJsonConverter<>).MakeGenericType(new Type[] {{ t0 }}),
                BindingFlags.Instance | BindingFlags.Public,
                binder: null,
                args: null,
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

    public class EnumerableJsonConverterFactory : JsonConverterFactory {{

        public override bool CanConvert(Type typeToConvert) {{
            if (!typeToConvert.IsGenericType) {{
                return false;
            }}
            if (typeToConvert.GetGenericTypeDefinition() != typeof(Enumerable<>)) {{
                return false;
            }}
            return true;
        }}

        public override JsonConverter? CreateConverter(Type typeToConvert, JsonSerializerOptions options) {{
            Type t0 = typeToConvert.GetGenericArguments()[0];

            JsonConverter converter = (JsonConverter)Activator.CreateInstance(
                typeof(EnumerableJsonConverter<>).MakeGenericType(new Type[] {{ t0 }}),
                BindingFlags.Instance | BindingFlags.Public,
                binder: null,
                args: null,
                culture: null)!;

            return converter;
        }}
    }}

    public class EnumerableJsonConverter<T> : JsonConverter<Enumerable<T>> {{

        public override Enumerable<T> Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            throw new NotImplementedException();
        }}

        public override void Write(Utf8JsonWriter writer, Enumerable<T> value, JsonSerializerOptions options) {{
            value.Value.Switch(
                t0 => writer.WriteRawValue(JSJsonSerializer.Serialize(value.Value.AsT0)),
                t1 => writer.WriteRawValue(JSJsonSerializer.Serialize(value.Value.AsT1)));
        }}
    }}

    public class SortOrderJsonConverter: JsonConverter<SortOrder> {{
        public override SortOrder Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options) {{
            throw new NotImplementedException();
        }}

        public override void Write(Utf8JsonWriter writer, SortOrder value, JsonSerializerOptions options) {{
            switch (value) {{
                case SortOrder.Asc: {{
                    writer.WriteStringValue("asc");
                    return;
                }}
                case SortOrder.Desc: {{
                    writer.WriteStringValue("desc");
                    return;
                }}
            }}
        }}
    }}

{enums}    static public class JSJsonSerializer {{
        static private JsonSerializerOptions options() {{
            var options = new JsonSerializerOptions {{
                PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
                DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull,
            }};
            options.Converters.Add(new DateOnlyConverter());
            options.Converters.Add(new DateTimeConverter());
            options.Converters.Add(new DateTimeOffsetConverter());
            options.Converters.Add(new SortOrderJsonConverter());
            options.Converters.Add(new OneOfJsonConverterFactory());
            options.Converters.Add(new OptionalJsonConverterFactory());
            options.Converters.Add(new EnumerableJsonConverterFactory());
{enum_converters}            return options;
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