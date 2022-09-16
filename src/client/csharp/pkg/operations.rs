use crate::core::graph::Graph;

pub(crate) async fn generate_operations_cs(_graph: &Graph) -> String {
    format!(r#"namespace Teo {{
    public class ObjectIdFieldUpdateOperationsInput {{
        public string? Set {{ get; set; }}
        public ObjectIdFieldUpdateOperationsInput(string? set) {{ Set = set; }}
    }}

    public class NullableObjectIdFieldUpdateOperationsInput {{
        public Optional<string>? Set {{ get; set; }}
        public NullableObjectIdFieldUpdateOperationsInput(Optional<string>? set) {{ Set = set; }}
    }}

    public class StringFieldUpdateOperationsInput {{
        public string? Set {{ get; set; }}
        public StringFieldUpdateOperationsInput(string? set) {{ Set = set; }}
    }}

    public class NullableStringFieldUpdateOperationsInput {{
        public Optional<string>? Set {{ get; set; }}
        public NullableStringFieldUpdateOperationsInput(Optional<string>? set) {{ Set = set; }}
    }}

    public class NumberFieldUpdateOperationsInput<T> where T: struct {{
        public T? Set {{ get; set; }}
        public T? Increment {{ get; set; }}
        public T? Decrement {{ get; set; }}
        public T? Multiply {{ get; set; }}
        public T? Divide {{ get; set; }}
        public NumberFieldUpdateOperationsInput(
            T? set = null, 
            T? increment = null, 
            T? decrement = null, 
            T? multiply = null, 
            T? divide = null
        ) {{
            Set = set;
            Increment = increment;
            Decrement = decrement;
            Multiply = multiply;
            Divide = divide;
        }}
    }}

    public class NullableNumberFieldUpdateOperationsInput<T> where T : struct {{
        public Optional<T>? Set {{ get; set; }}
        public T? Increment {{ get; set; }}
        public T? Decrement {{ get; set; }}
        public T? Multiply {{ get; set; }}
        public T? Divide {{ get; set; }}
        public NullableNumberFieldUpdateOperationsInput(
            Optional<T>? set = null,
            T? increment = null,
            T? decrement = null,
            T? multiply = null,
            T? divide = null
        ) {{
            Set = set;
            Increment = increment;
            Decrement = decrement;
            Multiply = multiply;
            Divide = divide;
        }}
    }}

    public class BoolFieldUpdateOperationsInput {{
        public bool? Set {{ get; set; }}
        public BoolFieldUpdateOperationsInput(bool? set) {{ Set = set; }}
    }}

    public class NullableBoolFieldUpdateOperationsInput {{
        public Optional<bool>? Set {{ get; set; }}
        public NullableBoolFieldUpdateOperationsInput(Optional<bool>? set) {{ Set = set; }}
    }}

    public class DateFieldUpdateOperationsInput {{
        public DateOnly? Set {{ get; set; }}
        public DateFieldUpdateOperationsInput(DateOnly? set) {{ Set = set; }}
    }}

    public class NullableDateFieldUpdateOperationsInput {{
        public Optional<DateOnly>? Set {{ get; set; }}
        public NullableDateFieldUpdateOperationsInput(Optional<DateOnly>? set) {{ Set = set; }}
    }}

    public class DateTimeFieldUpdateOperationsInput {{
        public DateTime? Set {{ get; set; }}
        public DateTimeFieldUpdateOperationsInput(DateTime? set) {{ Set = set; }}
    }}

    public class NullableDateTimeFieldUpdateOperationsInput {{
        public Optional<DateTime>? Set {{ get; set; }}
        public NullableDateTimeFieldUpdateOperationsInput(Optional<DateTime>? set) {{ Set = set; }}
    }}

    public class EnumFieldUpdateOperationsInput<T> where T: struct {{
        public T? Set {{ get; set; }}
        public EnumFieldUpdateOperationsInput(T? set) {{ Set = set; }}
    }}

    public class NullableEnumFieldUpdateOperationsInput<T> where T : struct {{
        public Optional<T>? Set {{ get; set; }}
        public NullableEnumFieldUpdateOperationsInput(Optional<T>? set) {{ Set = set; }}
    }}

    public class ValueArrayFieldUpdateOperationsInput<T> where T : struct {{
        public T[]? Set {{ get; set; }}
        public T? Push {{ get; set; }}
        public ValueArrayFieldUpdateOperationsInput(T[]? set = null, T? push = null) {{ 
            Set = set;
            Push = push;
        }}
    }}

    public class NullableValueArrayFieldUpdateOperationsInput<T> where T : struct {{
        public Optional<T[]>? Set {{ get; set; }}
        public T? Push {{ get; set; }}
        public NullableValueArrayFieldUpdateOperationsInput(Optional<T[]>? set = null, T? push = null) {{
            Set = set;
            Push = push;
        }}
    }}

    public class RefArrayFieldUpdateOperationsInput<T> where T : class {{
        public T[]? Set {{ get; set; }}
        public T? Push {{ get; set; }}
        public RefArrayFieldUpdateOperationsInput(T[]? set = null, T? push = null) {{
            Set = set;
            Push = push;
        }}
    }}

    public class NullableRefArrayFieldUpdateOperationsInput<T> where T : class {{
        public Optional<T[]>? Set {{ get; set; }}
        public T? Push {{ get; set; }}
        public NullableRefArrayFieldUpdateOperationsInput(Optional<T[]>? set = null, T? push = null) {{
            Set = set;
            Push = push;
        }}
    }}
}}"#)
}