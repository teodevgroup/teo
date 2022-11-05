use crate::core::graph::Graph;

pub(crate) async fn generate_enumerable_cs(_graph: &Graph) -> String {
    format!(r#"namespace Teo {{
    public struct Enumerable<T> {{

        OneOf<T, T[]> _value;

        public OneOf<T, T[]> Value {{
            get => _value;
            set {{
                _value = value;
            }}
        }}

        public static implicit operator Enumerable<T>(OneOf<T, T[]> value) => new Enumerable<T> {{ Value = value }};
        public static implicit operator Enumerable<T>(T t) => new Enumerable<T> {{ Value = (OneOf<T, T[]>)new Enumerable<T> {{ Value = t }} }};
        public static implicit operator Enumerable<T>(T[] a) => new Enumerable<T> {{ Value = (OneOf<T, T[]>)new Enumerable<T> {{ Value = a }} }};

        public static explicit operator OneOf<T, T[]>(Enumerable<T> enumerable) {{
            return enumerable.Value;
        }}

        public override string ToString() {{
            return Value.ToString()!;
        }}
    }}
}}
"#)
}