use crate::core::graph::Graph;

pub(crate) async fn generate_optional_cs(_graph: &Graph) -> String {
    format!(r#"using System;
using System.Text.Json;
using System.Text.Json.Serialization;
using OneOf;

namespace Teo {{

    public class Null {{
        public Null() {{ }}

        public static readonly Null NULL = new();

        public override string ToString() => "Null";
    }}

    public struct Optional<T> {{
        OneOf<T, Null> _value;

        public OneOf<T, Null> Value {{
            get => _value;
            set {{
                _value = value;
            }}
        }}

        public static implicit operator Optional<T>(OneOf<T, Null> value) => new Optional<T> {{ Value = value }};
        public static implicit operator Optional<T>(T t) => new Optional<T> {{ Value = (OneOf<T, Null>)new Optional<T> {{ Value = t }} }};
        public static implicit operator Optional<T>(Null n) => new Optional<T> {{ Value = (OneOf<T, Null>)new Optional<T> {{ Value = n }} }};

        public static explicit operator OneOf<T, Null>(Optional<T> optional) {{
            return optional.Value;
        }}

        public override string ToString() {{
            return Value.ToString()!;
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
}}
"#)
}