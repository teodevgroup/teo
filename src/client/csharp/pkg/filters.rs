use crate::client::csharp::pkg::filters::numbers::generate_number_filters;
use crate::core::graph::Graph;

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
        public OneOf<Optional<bool>, BoolNullableFilter>? Not {{ get; set; }}

        public BoolNullableFilter(
            bool? equals = null,
            OneOf<Optional<bool>, BoolNullableFilter>? not = null
        ) {{
            Equals = equals;
            Not = not;
        }}
    }}

    public class NumberFilter<T> where T: struct {{
        public new T? Equals {{ get; set; }}
        public T[]? In {{ get; set; }}
        public T[]? NotIn {{ get; set; }}
        public T? Lt {{ get; set; }}
        public T? Lte {{ get; set; }}
        public T? Gt {{ get; set; }}
        public T? Gte {{ get; set; }}
        public OneOf<T, NumberFilter<T>>? Not {{ get; set; }}

        public NumberFilter(
            T? equals = null,
            T[]? @in = null,
            T[]? notIn = null,
            T? lt = null,
            T? lte = null,
            T? gt = null,
            T? gte = null,
            OneOf<T, NumberFilter<T>>? not = null
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

    public class NumberNullableFilter<T> where T: struct {{
        public new Optional<T>? Equals {{ get; set; }}
        public Optional<T>[]? In {{ get; set; }}
        public Optional<T>[]? NotIn {{ get; set; }}
        public T? Lt {{ get; set; }}
        public T? Lte {{ get; set; }}
        public T? Gt {{ get; set; }}
        public T? Gte {{ get; set; }}
        public OneOf<Optional<T>, NumberNullableFilter<T>>? Not {{ get; set; }}

        public NumberNullableFilter(
            Optional<T>? equals = null,
            Optional<T>[]? @in = null,
            Optional<T>[]? notIn = null,
            T? lt = null,
            T? lte = null,
            T? gt = null,
            T? gte = null,
            OneOf<Optional<T>, NumberNullableFilter<T>>? not = null
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

    public class EnumFilter<T> where T: struct {{
        public new T? Equals {{ get; set; }}
        public T[]? In {{ get; set; }}
        public T[]? NotIn {{ get; set; }}
        public OneOf<T, EnumFilter<T>>? Not {{ get; set; }}

        public EnumFilter(
            T? equals,
            T[]? @in,
            T[]? notIn,
            OneOf<T, EnumFilter<T>>? not
        ) {{
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Not = not;
        }}
    }}

    public class EnumNullableFilter<T> where T : struct {{
        public new Optional<T>? Equals {{ get; set; }}
        public Optional<T>[]? In {{ get; set; }}
        public Optional<T>[]? NotIn {{ get; set; }}
        public OneOf<Optional<T>, EnumNullableFilter<T>>? Not {{ get; set; }}

        public EnumNullableFilter(
            Optional<T>? equals,
            Optional<T>[]? @in,
            Optional<T>[]? notIn,
            OneOf<Optional<T>, EnumNullableFilter<T>>? not
        ) {{
            Equals = equals;
            In = @in;
            NotIn = notIn;
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
