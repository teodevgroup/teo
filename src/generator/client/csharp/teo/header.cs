using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;
using Windows.Web.Http;
using System.Reflection;
using System.Text.Json;
using System.Text.Json.Serialization;

// Enumerable
namespace Teo {
    public struct Enumerable<T> {

        OneOf<T, T[]> _value;

        public OneOf<T, T[]> Value {
            get => _value;
            set {
                _value = value;
            }
        }

        public static implicit operator Enumerable<T>(OneOf<T, T[]> value) => new Enumerable<T> { Value = value };
        public static implicit operator Enumerable<T>(T t) => new Enumerable<T> { Value = (OneOf<T, T[]>)new Enumerable<T> { Value = t } };
        public static implicit operator Enumerable<T>(T[] a) => new Enumerable<T> { Value = (OneOf<T, T[]>)new Enumerable<T> { Value = a } };

        public static explicit operator OneOf<T, T[]>(Enumerable<T> enumerable) {
            return enumerable.Value;
        }

        public override string ToString() {
            return Value.ToString()!;
        }
    }
}

// Filters
namespace Teo {
    public class ObjectIdFilter {
        public new string? Equals { get; set; }
        public string[]? In { get; set; }
        public string[]? NotIn { get; set; }
        public string? Lt { get; set; }
        public string? Lte { get; set; }
        public string? Gt { get; set; }
        public string? Gte { get; set; }
        public OneOf<string, ObjectIdFilter>? Not { get; set; }

        public ObjectIdFilter(
            string? equals = null,
            string[]? @in = null,
            string[]? notIn = null,
            string? lt = null,
            string? lte = null,
            string? gt = null,
            string? gte = null,
            OneOf<string, ObjectIdFilter>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class ObjectIdNullableFilter {
        public new Optional<string>? Equals { get; set; }
        public Optional<string>[]? In { get; set; }
        public Optional<string>[]? NotIn { get; set; }
        public string? Lt { get; set; }
        public string? Lte { get; set; }
        public string? Gt { get; set; }
        public string? Gte { get; set; }
        public OneOf<Optional<string>, ObjectIdNullableFilter>? Not { get; set; }

        public ObjectIdNullableFilter(
            Optional<string>? equals = null,
            Optional<string>[]? @in = null,
            Optional<string>[]? notIn = null,
            string? lt = null,
            string? lte = null,
            string? gt = null,
            string? gte = null,
            OneOf<Optional<string>, ObjectIdNullableFilter>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class BoolFilter {
        public new bool? Equals { get; set; }
        public OneOf<bool, BoolFilter>? Not { get; set; }

        public BoolFilter(
            bool? equals = null,
            OneOf<bool, BoolFilter>? not = null
        ) {
            Equals = equals;
            Not = not;
        }
    }

    public class BoolNullableFilter {
        public new Optional<bool>? Equals { get; set; }
        public OneOf<Optional<bool>, BoolNullableFilter>? Not { get; set; }

        public BoolNullableFilter(
            bool? equals = null,
            OneOf<Optional<bool>, BoolNullableFilter>? not = null
        ) {
            Equals = equals;
            Not = not;
        }
    }

    public class NumberFilter<T> where T: struct {
        public new T? Equals { get; set; }
        public T[]? In { get; set; }
        public T[]? NotIn { get; set; }
        public T? Lt { get; set; }
        public T? Lte { get; set; }
        public T? Gt { get; set; }
        public T? Gte { get; set; }
        public OneOf<T, NumberFilter<T>>? Not { get; set; }

        public NumberFilter(
            T? equals = null,
            T[]? @in = null,
            T[]? notIn = null,
            T? lt = null,
            T? lte = null,
            T? gt = null,
            T? gte = null,
            OneOf<T, NumberFilter<T>>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class NumberNullableFilter<T> where T: struct {
        public new Optional<T>? Equals { get; set; }
        public Optional<T>[]? In { get; set; }
        public Optional<T>[]? NotIn { get; set; }
        public T? Lt { get; set; }
        public T? Lte { get; set; }
        public T? Gt { get; set; }
        public T? Gte { get; set; }
        public OneOf<Optional<T>, NumberNullableFilter<T>>? Not { get; set; }

        public NumberNullableFilter(
            Optional<T>? equals = null,
            Optional<T>[]? @in = null,
            Optional<T>[]? notIn = null,
            T? lt = null,
            T? lte = null,
            T? gt = null,
            T? gte = null,
            OneOf<Optional<T>, NumberNullableFilter<T>>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class StringFilter {
        public new string? Equals { get; set; }
        public string[]? In { get; set; }
        public string[]? NotIn { get; set; }
        public string? Lt { get; set; }
        public string? Lte { get; set; }
        public string? Gt { get; set; }
        public string? Gte { get; set; }
        public string? Contains { get; set; }
        public string? StartsWith { get; set; }
        public string? EndsWith { get; set; }
        public string? Matches { get; set; }
        public OneOf<string, StringFilter>? Not { get; set; }

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
        ) {
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
        }
    }

    public class StringNullableFilter {
        public new Optional<string>? Equals { get; set; }
        public Optional<string>[]? In { get; set; }
        public Optional<string>[]? NotIn { get; set; }
        public string? Lt { get; set; }
        public string? Lte { get; set; }
        public string? Gt { get; set; }
        public string? Gte { get; set; }
        public string? Contains { get; set; }
        public string? StartsWith { get; set; }
        public string? EndsWith { get; set; }
        public string? Matches { get; set; }
        public OneOf<Optional<string>, StringNullableFilter>? Not { get; set; }

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
        ) {
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
        }
    }

    public class DateTimeFilter {
        public new DateTime? Equals { get; set; }
        public DateTime[]? In { get; set; }
        public DateTime[]? NotIn { get; set; }
        public DateTime? Lt { get; set; }
        public DateTime? Lte { get; set; }
        public DateTime? Gt { get; set; }
        public DateTime? Gte { get; set; }
        public OneOf<DateTime, DateTimeFilter>? Not { get; set; }

        public DateTimeFilter(
            DateTime? equals = null,
            DateTime[]? @in = null,
            DateTime[]? notIn = null,
            DateTime? lt = null,
            DateTime? lte = null,
            DateTime? gt = null,
            DateTime? gte = null,
            OneOf<DateTime, DateTimeFilter>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class DateTimeNullableFilter {
        public new Optional<DateTime>? Equals { get; set; }
        public Optional<DateTime>[]? In { get; set; }
        public Optional<DateTime>[]? NotIn { get; set; }
        public DateTime? Lt { get; set; }
        public DateTime? Lte { get; set; }
        public DateTime? Gt { get; set; }
        public DateTime? Gte { get; set; }
        public OneOf<Optional<DateTime>, DateTimeNullableFilter>? Not { get; set; }

        public DateTimeNullableFilter(
            Optional<DateTime>? equals = null,
            Optional<DateTime>[]? @in = null,
            Optional<DateTime>[]? notIn = null,
            DateTime? lt = null,
            DateTime? lte = null,
            DateTime? gt = null,
            DateTime? gte = null,
            OneOf<Optional<DateTime>, DateTimeNullableFilter>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class DateOnlyFilter {
        public new DateOnly? Equals { get; set; }
        public DateOnly[]? In { get; set; }
        public DateOnly[]? NotIn { get; set; }
        public DateOnly? Lt { get; set; }
        public DateOnly? Lte { get; set; }
        public DateOnly? Gt { get; set; }
        public DateOnly? Gte { get; set; }
        public OneOf<DateOnly, DateOnlyFilter>? Not { get; set; }

        public DateOnlyFilter(
            DateOnly? equals = null,
            DateOnly[]? @in = null,
            DateOnly[]? notIn = null,
            DateOnly? lt = null,
            DateOnly? lte = null,
            DateOnly? gt = null,
            DateOnly? gte = null,
            OneOf<DateOnly, DateOnlyFilter>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class DateOnlyNullableFilter {
        public new Optional<DateOnly>? Equals { get; set; }
        public Optional<DateOnly>[]? In { get; set; }
        public Optional<DateOnly>[]? NotIn { get; set; }
        public DateOnly? Lt { get; set; }
        public DateOnly? Lte { get; set; }
        public DateOnly? Gt { get; set; }
        public DateOnly? Gte { get; set; }
        public OneOf<Optional<DateOnly>, DateOnlyNullableFilter>? Not { get; set; }

        public DateOnlyNullableFilter(
            Optional<DateOnly>? equals = null,
            Optional<DateOnly>[]? @in = null,
            Optional<DateOnly>[]? notIn = null,
            DateOnly? lt = null,
            DateOnly? lte = null,
            DateOnly? gt = null,
            DateOnly? gte = null,
            OneOf<Optional<DateOnly>, DateOnlyNullableFilter>? not = null
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Lt = lt;
            Lte = lte;
            Gt = gt;
            Gte = gte;
            Not = not;
        }
    }

    public class EnumFilter<T> where T: struct {
        public new T? Equals { get; set; }
        public T[]? In { get; set; }
        public T[]? NotIn { get; set; }
        public OneOf<T, EnumFilter<T>>? Not { get; set; }

        public EnumFilter(
            T? equals,
            T[]? @in,
            T[]? notIn,
            OneOf<T, EnumFilter<T>>? not
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Not = not;
        }
    }

    public class EnumNullableFilter<T> where T : struct {
        public new Optional<T>? Equals { get; set; }
        public Optional<T>[]? In { get; set; }
        public Optional<T>[]? NotIn { get; set; }
        public OneOf<Optional<T>, EnumNullableFilter<T>>? Not { get; set; }

        public EnumNullableFilter(
            Optional<T>? equals,
            Optional<T>[]? @in,
            Optional<T>[]? notIn,
            OneOf<Optional<T>, EnumNullableFilter<T>>? not
        ) {
            Equals = equals;
            In = @in;
            NotIn = notIn;
            Not = not;
        }
    }

    public class ValueArrayFilter<T> where T: struct {
        public new T? Equals { set; get; }
        public T? Has { set; get; }
        public T[]? HasSome { set; get; }
        public T[]? HasEvery { set; get; }
        public bool? IsEmpty { get; set; }
        public int? Length { get; set; }

        public ValueArrayFilter(
            T? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }
    }

    public class ValueArrayNullableFilter<T> where T: struct {
        public new Optional<T>? Equals { set; get; }
        public T? Has { set; get; }
        public T[]? HasSome { set; get; }
        public T[]? HasEvery { set; get; }
        public bool? IsEmpty { get; set; }
        public int? Length { get; set; }

        public ValueArrayNullableFilter(
            Optional<T>? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }
    }

    public class RefArrayFilter<T> where T: class {
        public new T? Equals { set; get; }
        public T? Has { set; get; }
        public T[]? HasSome { set; get; }
        public T[]? HasEvery { set; get; }
        public bool? IsEmpty { get; set; }
        public int? Length { get; set; }

        public RefArrayFilter(
            T? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }
    }

    public class RefArrayNullableFilter<T> where T: class {
        public new Optional<T>? Equals { set; get; }
        public T? Has { set; get; }
        public T[]? HasSome { set; get; }
        public T[]? HasEvery { set; get; }
        public bool? IsEmpty { get; set; }
        public int? Length { get; set; }

        public RefArrayNullableFilter(
            Optional<T>? equals = null,
            T? has = null,
            T[]? hasSome = null,
            T[]? hasEvery = null,
            bool? isEmpty = null,
            int? length = null
        ) {
            Equals = equals;
            Has = has;
            HasSome = hasSome;
            HasEvery = hasEvery;
            IsEmpty = isEmpty;
            Length = length;
        }
    }
}

// One of
#nullable disable
namespace Teo {

    public interface IOneOf {
        object Value { get; }
        int Index { get; }
    }

    internal static class Functions {
        internal static string FormatValue<T>(T value) => $"{typeof(T).FullName}: {value?.ToString()}";
        internal static string FormatValue<T>(object @this, object @base, T value) =>
            ReferenceEquals(@this, value) ?
                @base.ToString() :
                $"{typeof(T).FullName}: {value?.ToString()}";
    }

    public struct OneOf<T0, T1> : IOneOf {
        readonly T0 _value0;
        readonly T1 _value1;
        readonly int _index;

        OneOf(int index, T0 value0 = default, T1 value1 = default) {
            _index = index;
            _value0 = value0;
            _value1 = value1;
        }

        public object Value =>
            _index switch {
                0 => _value0,
                1 => _value1,
                _ => throw new InvalidOperationException()
            };

        public int Index => _index;

        public bool IsT0 => _index == 0;
        public bool IsT1 => _index == 1;

        public T0 AsT0 =>
            _index == 0 ?
                _value0 :
                throw new InvalidOperationException($"Cannot return as T0 as result is T{_index}");
        public T1 AsT1 =>
            _index == 1 ?
                _value1 :
                throw new InvalidOperationException($"Cannot return as T1 as result is T{_index}");

        public static implicit operator OneOf<T0, T1>(T0 t) => new OneOf<T0, T1>(0, value0: t);
        public static implicit operator OneOf<T0, T1>(T1 t) => new OneOf<T0, T1>(1, value1: t);

        public void Switch(Action<T0> f0, Action<T1> f1) {
            if (_index == 0 && f0 != null) {
                f0(_value0);
                return;
            }
            if (_index == 1 && f1 != null) {
                f1(_value1);
                return;
            }
            throw new InvalidOperationException();
        }

        public TResult Match<TResult>(Func<T0, TResult> f0, Func<T1, TResult> f1) {
            if (_index == 0 && f0 != null) {
                return f0(_value0);
            }
            if (_index == 1 && f1 != null) {
                return f1(_value1);
            }
            throw new InvalidOperationException();
        }

        public static OneOf<T0, T1> FromT0(T0 input) => input;
        public static OneOf<T0, T1> FromT1(T1 input) => input;


        public OneOf<TResult, T1> MapT0<TResult>(Func<T0, TResult> mapFunc) {
            if (mapFunc == null) {
                throw new ArgumentNullException(nameof(mapFunc));
            }
            return _index switch {
                0 => mapFunc(AsT0),
                1 => AsT1,
                _ => throw new InvalidOperationException()
            };
        }

        public OneOf<T0, TResult> MapT1<TResult>(Func<T1, TResult> mapFunc) {
            if (mapFunc == null) {
                throw new ArgumentNullException(nameof(mapFunc));
            }
            return _index switch {
                0 => AsT0,
                1 => mapFunc(AsT1),
                _ => throw new InvalidOperationException()
            };
        }

        public bool TryPickT0(out T0 value, out T1 remainder) {
            value = IsT0 ? AsT0 : default;
            remainder = _index switch {
                0 => default,
                1 => AsT1,
                _ => throw new InvalidOperationException()
            };
            return this.IsT0;
        }

        public bool TryPickT1(out T1 value, out T0 remainder) {
            value = IsT1 ? AsT1 : default;
            remainder = _index switch {
                0 => AsT0,
                1 => default,
                _ => throw new InvalidOperationException()
            };
            return this.IsT1;
        }

        bool Equals(OneOf<T0, T1> other) =>
            _index == other._index &&
            _index switch {
                0 => Equals(_value0, other._value0),
                1 => Equals(_value1, other._value1),
                _ => false
            };

        public override bool Equals(object obj) {
            if (ReferenceEquals(null, obj)) {
                return false;
            }

            return obj is OneOf<T0, T1> o && Equals(o);
        }

        public override string ToString() =>
            _index switch {
                0 => Functions.FormatValue(_value0),
                1 => Functions.FormatValue(_value1),
                _ => throw new InvalidOperationException("Unexpected index, which indicates a problem in the OneOf codegen.")
            };

        public override int GetHashCode() {
            unchecked {
                int hashCode = _index switch {
                    0 => _value0?.GetHashCode(),
                    1 => _value1?.GetHashCode(),
                    _ => 0
                } ?? 0;
                return (hashCode * 397) ^ _index;
            }
        }
    }
}

// Operations
using System;

namespace Teo {
    public class ObjectIdFieldUpdateOperationsInput {
        public string? Set { get; set; }
        public ObjectIdFieldUpdateOperationsInput(string? set) { Set = set; }
    }

    public class NullableObjectIdFieldUpdateOperationsInput {
        public Optional<string>? Set { get; set; }
        public NullableObjectIdFieldUpdateOperationsInput(Optional<string>? set) { Set = set; }
    }

    public class StringFieldUpdateOperationsInput {
        public string? Set { get; set; }
        public StringFieldUpdateOperationsInput(string? set) { Set = set; }
    }

    public class NullableStringFieldUpdateOperationsInput {
        public Optional<string>? Set { get; set; }
        public NullableStringFieldUpdateOperationsInput(Optional<string>? set) { Set = set; }
    }

    public class NumberFieldUpdateOperationsInput<T> where T: struct {
        public T? Set { get; set; }
        public T? Increment { get; set; }
        public T? Decrement { get; set; }
        public T? Multiply { get; set; }
        public T? Divide { get; set; }
        public NumberFieldUpdateOperationsInput(
            T? set = null,
            T? increment = null,
            T? decrement = null,
            T? multiply = null,
            T? divide = null
        ) {
            Set = set;
            Increment = increment;
            Decrement = decrement;
            Multiply = multiply;
            Divide = divide;
        }
    }

    public class NullableNumberFieldUpdateOperationsInput<T> where T : struct {
        public Optional<T>? Set { get; set; }
        public T? Increment { get; set; }
        public T? Decrement { get; set; }
        public T? Multiply { get; set; }
        public T? Divide { get; set; }
        public NullableNumberFieldUpdateOperationsInput(
            Optional<T>? set = null,
            T? increment = null,
            T? decrement = null,
            T? multiply = null,
            T? divide = null
        ) {
            Set = set;
            Increment = increment;
            Decrement = decrement;
            Multiply = multiply;
            Divide = divide;
        }
    }

    public class BoolFieldUpdateOperationsInput {
        public bool? Set { get; set; }
        public BoolFieldUpdateOperationsInput(bool? set) { Set = set; }
    }

    public class NullableBoolFieldUpdateOperationsInput {
        public Optional<bool>? Set { get; set; }
        public NullableBoolFieldUpdateOperationsInput(Optional<bool>? set) { Set = set; }
    }

    public class DateOnlyFieldUpdateOperationsInput {
        public DateOnly? Set { get; set; }
        public DateOnlyFieldUpdateOperationsInput(DateOnly? set) { Set = set; }
    }

    public class NullableDateOnlyFieldUpdateOperationsInput {
        public Optional<DateOnly>? Set { get; set; }
        public NullableDateOnlyFieldUpdateOperationsInput(Optional<DateOnly>? set) { Set = set; }
    }

    public class DateTimeFieldUpdateOperationsInput {
        public DateTime? Set { get; set; }
        public DateTimeFieldUpdateOperationsInput(DateTime? set) { Set = set; }
    }

    public class NullableDateTimeFieldUpdateOperationsInput {
        public Optional<DateTime>? Set { get; set; }
        public NullableDateTimeFieldUpdateOperationsInput(Optional<DateTime>? set) { Set = set; }
    }

    public class EnumFieldUpdateOperationsInput<T> where T: struct {
        public T? Set { get; set; }
        public EnumFieldUpdateOperationsInput(T? set) { Set = set; }
    }

    public class NullableEnumFieldUpdateOperationsInput<T> where T : struct {
        public Optional<T>? Set { get; set; }
        public NullableEnumFieldUpdateOperationsInput(Optional<T>? set) { Set = set; }
    }

    public class ValueArrayFieldUpdateOperationsInput<T> where T : struct {
        public T[]? Set { get; set; }
        public T? Push { get; set; }
        public ValueArrayFieldUpdateOperationsInput(T[]? set = null, T? push = null) {
            Set = set;
            Push = push;
        }
    }

    public class NullableValueArrayFieldUpdateOperationsInput<T> where T : struct {
        public Optional<T[]>? Set { get; set; }
        public T? Push { get; set; }
        public NullableValueArrayFieldUpdateOperationsInput(Optional<T[]>? set = null, T? push = null) {
            Set = set;
            Push = push;
        }
    }

    public class RefArrayFieldUpdateOperationsInput<T> where T : class {
        public T[]? Set { get; set; }
        public T? Push { get; set; }
        public RefArrayFieldUpdateOperationsInput(T[]? set = null, T? push = null) {
            Set = set;
            Push = push;
        }
    }

    public class NullableRefArrayFieldUpdateOperationsInput<T> where T : class {
        public Optional<T[]>? Set { get; set; }
        public T? Push { get; set; }
        public NullableRefArrayFieldUpdateOperationsInput(Optional<T[]>? set = null, T? push = null) {
            Set = set;
            Push = push;
        }
    }
}

// Optional
namespace Teo {

    public class Null {
        public Null() { }

        public static readonly Null NULL = new();

        public override string ToString() => "Null";
    }

    public struct Optional<T> {
        OneOf<T, Null> _value;

        public OneOf<T, Null> Value {
            get => _value;
            set {
                _value = value;
            }
        }

        public static implicit operator Optional<T>(OneOf<T, Null> value) => new Optional<T> { Value = value };
        public static implicit operator Optional<T>(T t) => new Optional<T> { Value = (OneOf<T, Null>)new Optional<T> { Value = t } };
        public static implicit operator Optional<T>(Null n) => new Optional<T> { Value = (OneOf<T, Null>)new Optional<T> { Value = n } };

        public static explicit operator OneOf<T, Null>(Optional<T> optional) {
            return optional.Value;
        }

        public override string ToString() {
            return Value.ToString()!;
        }
    }
}

// Sort order
namespace Teo {
    public enum SortOrder {
        Asc,
        Desc,
    }
}

// Resp
namespace Teo {
    public struct Response<D> {
        public D Data { get; set; }
    }

    public struct Response<M, D> {
        public M Meta { get; set; }
        public D Data { get; set; }
    }

    public struct ResponseError {
        public string Type { get; set; }
        public string Message { get; set; }
        public Dictionary<string, string>? Errors { get; set; }
    }

    public struct PagingInfo {
        public uint Count { get; set; }
        public uint? NumberOfPages { get; set; }
    }

    public struct TokenInfo {
        public string Token { get; set; }
    }

    public class TeoException : Exception {

        public override string Message { get; }

        public string Type { get; }

        public Dictionary<string, string>? Errors { get; }

        public TeoException(ResponseError responseError) {
            Message = responseError.Message;
            Type = responseError.Type;
            Errors = responseError.Errors;
        }
    }
}