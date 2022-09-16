use crate::client::csharp::pkg::filters::numbers::generate_number_filters;
use crate::core::graph::Graph;

pub(crate) mod numbers;

pub(crate) async fn generate_filters_cs(_graph: &Graph) -> String {
    let number_filters = generate_number_filters().await;
    format!(r#"namespace Teo {{
    public class ObjectIdFilter {{
        public new string? Equals {{ get; set; }}
        public string[]? In {{ get; set; }}
        public string[]? NotIn {{ get; set; }}
        public string? Lt {{ get; set; }}
        public string? Lte {{ get; set; }}
        public string? Gt {{ get; set; }}
        public string? Gte {{ get; set; }}
        public OneOf<string, ObjectIdFilter>? Not {{ get; set; }}

        public ObjectIdFilter(
            string? equals = null,
            string[]? @in = null,
            string[]? notIn = null,
            string? lt = null,
            string? lte = null,
            string? gt = null,
            string? gte = null,
            OneOf<string, ObjectIdFilter>? not = null
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

    public class ObjectIdNullableFilter {{
        public new Optional<string>? Equals {{ get; set; }}
        public Optional<string>[]? In {{ get; set; }}
        public Optional<string>[]? NotIn {{ get; set; }}
        public string? Lt {{ get; set; }}
        public string? Lte {{ get; set; }}
        public string? Gt {{ get; set; }}
        public string? Gte {{ get; set; }}

        [JsonConverter(typeof(OneOfJsonConverter<Optional<string>, ObjectIdNullableFilter>))]
        public OneOf<Optional<string>, ObjectIdNullableFilter>? Not {{ get; set; }}

        public ObjectIdNullableFilter(
            Optional<string>? equals = null,
            Optional<string>[]? @in = null,
            Optional<string>[]? notIn = null,
            string? lt = null,
            string? lte = null,
            string? gt = null,
            string? gte = null,
            OneOf<Optional<string>, ObjectIdNullableFilter>? not = null
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

    public class BoolFilter {{
        public new bool? Equals {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<bool, ObjectIdFilter>))]
        public OneOf<bool, BoolFilter>? Not {{ get; set; }}

        public BoolFilter(
            bool? equals = null,
            OneOf<bool, BoolFilter>? not = null
        ) {{
            Equals = equals;
            Not = not;
        }}
    }}

    public class BoolNullableFilter {{
        public new Optional<bool>? Equals {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<Optional<bool>, BoolNullableFilter>))]
        public OneOf<Optional<bool>, BoolNullableFilter>? Not {{ get; set; }}

        public BoolNullableFilter(
            bool? equals = null,
            OneOf<Optional<bool>, BoolNullableFilter>? not = null
        ) {{
            Equals = equals;
            Not = not;
        }}
    }}

{number_filters}

    public class StringFilter {{
        public new string? Equals {{ get; set; }}
        public string[]? In {{ get; set; }}
        public string[]? NotIn {{ get; set; }}
        public string? Lt {{ get; set; }}
        public string? Lte {{ get; set; }}
        public string? Gt {{ get; set; }}
        public string? Gte {{ get; set; }}
        public string? Contains {{ get; set; }}
        public string? StartsWith {{ get; set; }}
        public string? EndsWith {{ get; set; }}
        public string? Matches {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<string, StringFilter>))]
        public OneOf<string, StringFilter>? Not {{ get; set; }}

        public StringFilter(
            string? equals = null,
            string[]? @in = null,
            string[]? notIn = null,
            string? lt = null,
            string? lte = null,
            string? gt = null,
            string? gte = null,
            string? contains = null,
            string? startsWith = null,
            string? endsWith = null,
            string? matches = null,
            OneOf<string, StringFilter>? not = null
        ) {{
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Contains = contains;
            StartsWith = startsWith;
            EndsWith = endsWith;
            Matches = matches;
            Not = not;
        }}
    }}

    public class StringNullableFilter {{
        public new Optional<string>? Equals {{ get; set; }}
        public Optional<string>[]? In {{ get; set; }}
        public Optional<string>[]? NotIn {{ get; set; }}
        public string? Lt {{ get; set; }}
        public string? Lte {{ get; set; }}
        public string? Gt {{ get; set; }}
        public string? Gte {{ get; set; }}
        public string? Contains {{ get; set; }}
        public string? StartsWith {{ get; set; }}
        public string? EndsWith {{ get; set; }}
        public string? Matches {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<string, StringNullableFilter>))]
        public OneOf<Optional<string>, StringNullableFilter>? Not {{ get; set; }}

        public StringNullableFilter(
            Optional<string>? equals = null,
            Optional<string>[]? @in = null,
            Optional<string>[]? notIn = null,
            string? lt = null,
            string? lte = null,
            string? gt = null,
            string? gte = null,
            string? contains = null,
            string? startsWith = null,
            string? endsWith = null,
            string? matches = null,
            OneOf<Optional<string>, StringNullableFilter>? not = null
        ) {{
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Contains = contains;
            StartsWith = startsWith;
            EndsWith = endsWith;
            Matches = matches;
            Not = not;
        }}
    }}

    public class DateTimeFilter {{
        public new DateTime? Equals {{ get; set; }}
        public DateTime[]? In {{ get; set; }}
        public DateTime[]? NotIn {{ get; set; }}
        public DateTime? Lt {{ get; set; }}
        public DateTime? Lte {{ get; set; }}
        public DateTime? Gt {{ get; set; }}
        public DateTime? Gte {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<DateTime, DateTimeFilter>))]
        public OneOf<DateTime, DateTimeFilter>? Not {{ get; set; }}

        public DateTimeFilter(
            DateTime? equals = null,
            DateTime[]? @in = null,
            DateTime[]? notIn = null,
            DateTime? lt = null,
            DateTime? lte = null,
            DateTime? gt = null,
            DateTime? gte = null,
            OneOf<DateTime, DateTimeFilter>? not = null
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

    public class DateTimeNullableFilter {{
        public new Optional<DateTime>? Equals {{ get; set; }}
        public Optional<DateTime>[]? In {{ get; set; }}
        public Optional<DateTime>[]? NotIn {{ get; set; }}
        public DateTime? Lt {{ get; set; }}
        public DateTime? Lte {{ get; set; }}
        public DateTime? Gt {{ get; set; }}
        public DateTime? Gte {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<Optional<DateTime>, DateTimeNullableFilter>))]
        public OneOf<Optional<DateTime>, DateTimeNullableFilter>? Not {{ get; set; }}

        public DateTimeNullableFilter(
            Optional<DateTime>? equals = null,
            Optional<DateTime>[]? @in = null,
            Optional<DateTime>[]? notIn = null,
            DateTime? lt = null,
            DateTime? lte = null,
            DateTime? gt = null,
            DateTime? gte = null,
            OneOf<Optional<DateTime>, DateTimeNullableFilter>? not = null
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

    public class DateOnlyFilter {{
        public new DateOnly? Equals {{ get; set; }}
        public DateOnly[]? In {{ get; set; }}
        public DateOnly[]? NotIn {{ get; set; }}
        public DateOnly? Lt {{ get; set; }}
        public DateOnly? Lte {{ get; set; }}
        public DateOnly? Gt {{ get; set; }}
        public DateOnly? Gte {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<DateOnly, DateOnlyFilter>))]
        public OneOf<DateOnly, DateOnlyFilter>? Not {{ get; set; }}

        public DateOnlyFilter(
            DateOnly? equals = null,
            DateOnly[]? @in = null,
            DateOnly[]? notIn = null,
            DateOnly? lt = null,
            DateOnly? lte = null,
            DateOnly? gt = null,
            DateOnly? gte = null,
            OneOf<DateOnly, DateOnlyFilter>? not = null
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

    public class DateOnlyNullableFilter {{
        public new Optional<DateOnly>? Equals {{ get; set; }}
        public Optional<DateOnly>[]? In {{ get; set; }}
        public Optional<DateOnly>[]? NotIn {{ get; set; }}
        public DateOnly? Lt {{ get; set; }}
        public DateOnly? Lte {{ get; set; }}
        public DateOnly? Gt {{ get; set; }}
        public DateOnly? Gte {{ get; set; }}
        [JsonConverter(typeof(OneOfJsonConverter<Optional<DateOnly>, DateOnlyNullableFilter>))]
        public OneOf<Optional<DateOnly>, DateOnlyNullableFilter>? Not {{ get; set; }}

        public DateOnlyNullableFilter(
            Optional<DateOnly>? equals = null,
            Optional<DateOnly>[]? @in = null,
            Optional<DateOnly>[]? notIn = null,
            DateOnly? lt = null,
            DateOnly? lte = null,
            DateOnly? gt = null,
            DateOnly? gte = null,
            OneOf<Optional<DateOnly>, DateOnlyNullableFilter>? not = null
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

    public class ValueArrayFilter<T> where T: struct {{
        public new T? Equals {{ set; get; }}
        public T? Has {{ set; get; }}
        public T[]? HasSome {{ set; get; }}
        public T[]? HasEvery {{ set; get; }}
        public bool? IsEmpty {{ get; set; }}
        public int? Length {{ get; set; }}

        public ValueArrayFilter(
            T? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {{
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }}
    }}

    public class ValueArrayNullableFilter<T> where T: struct {{
        public new Optional<T>? Equals {{ set; get; }}
        public T? Has {{ set; get; }}
        public T[]? HasSome {{ set; get; }}
        public T[]? HasEvery {{ set; get; }}
        public bool? IsEmpty {{ get; set; }}
        public int? Length {{ get; set; }}

        public ValueArrayNullableFilter(
            Optional<T>? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {{
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }}
    }}

    public class RefArrayFilter<T> where T: class {{
        public new T? Equals {{ set; get; }}
        public T? Has {{ set; get; }}
        public T[]? HasSome {{ set; get; }}
        public T[]? HasEvery {{ set; get; }}
        public bool? IsEmpty {{ get; set; }}
        public int? Length {{ get; set; }}

        public RefArrayFilter(
            T? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {{
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }}
    }}

    public class RefArrayNullableFilter<T> where T: class {{
        public new Optional<T>? Equals {{ set; get; }}
        public T? Has {{ set; get; }}
        public T[]? HasSome {{ set; get; }}
        public T[]? HasEvery {{ set; get; }}
        public bool? IsEmpty {{ get; set; }}
        public int? Length {{ get; set; }}

        public RefArrayNullableFilter(
            Optional<T>? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {{
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }}
    }}
}}
"#)
}
