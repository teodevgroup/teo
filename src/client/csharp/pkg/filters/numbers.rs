use inflector::Inflector;

static CSHARP_NUMBER_TYPES: [&str; 8] = ["sbyte", "byte", "short", "ushort", "int", "uint", "long", "ulong"];

pub(crate) async fn generate_number_filters() -> String {
    let mut retval = "";
    for t in CSHARP_NUMBER_TYPES {
        let cap_t = t.to_pascal_case();
        retval = retval + format!(r#"    public class {cap_t}Filter {{
        public new {t}? Equals {{ get; set; }}
        public {t}[]? In {{ get; set; }}
        public {t}[]? NotIn {{ get; set; }}
        public {t}? Lt {{ get; set; }}
        public {t}? Lte {{ get; set; }}
        public {t}? Gt {{ get; set; }}
        public {t}? Gte {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<{t}, {cap_t}Filter>))]
        public OneOf<{t}, {cap_t}Filter>? Not {{ get; set; }}

        public {cap_t}Filter(
            {t}? equals = null,
            {t}[]? @in = null,
            {t}[]? notIn = null,
            {t}? lt = null,
            {t}? lte = null,
            {t}? gt = null,
            {t}? gte = null,
            OneOf<{t}, {cap_t}Filter>? not = null
        ) {{
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }}
    }}

    public class {cap_t}NullableFilter {{
        public new Optional<{t}>? Equals {{ get; set; }}
        public Optional<{t}>[]? In {{ get; set; }}
        public Optional<{t}>[]? NotIn {{ get; set; }}
        public {t}? Lt {{ get; set; }}
        public {t}? Lte {{ get; set; }}
        public {t}? Gt {{ get; set; }}
        public {t}? Gte {{ get; set; }}
        public OneOf<Optional<{t}>, {cap_t}NullableFilter>? Not {{ get; set; }}

        public {cap_t}NullableFilter(
            Optional<{t}>? equals = null,
            Optional<{t}>[]? @in = null,
            Optional<{t}>[]? notIn = null,
            {t}? lt = null,
            {t}? lte = null,
            {t}? gt = null,
            {t}? gte = null,
            OneOf<Optional<{t}>, {cap_t}NullableFilter>? not = null
        ) {{
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }}
    }}
"#)
    }
    retval.to_owned()
}