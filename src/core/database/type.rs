use crate::core::r#enum::DbEnum;

/// This enum represents the field type in the actual database. This enum is designed to support
/// all SQL databases and MongoDB.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum DatabaseType {

    /// ObjectId
    /// Represents an object's id.
    /// Availability: MongoDB
    ObjectId,

    /// Bool
    /// Represents a bool value.
    /// Note: In MySQL, this type is synonyms only and you should alter this with TINYINT(1).
    /// Availability: All
    Bool,

    /// Bit
    /// Represents the smallest integer.
    /// Arguments:
    ///     m: m is from 1 to 64. If m is omitted, m is 1 by default.
    /// Note: To assign a value to it, b'value' is used. For example, assigning a value of b'101'
    /// to a BIT(6) column is, in effect, the same as assigning b'000101'.
    /// Availability: MySQL, PostgreSQL
    Bit { m: Option<u8> },

    /// BitVarying
    /// Represents a bit but varying in length.
    /// Availability: PostgreSQL
    BitVarying,

    /// TinyInt
    /// Represents a tiny integer from -128 to 127. The unsigned version is from 0 to 255.
    /// Arguments:
    ///     m: column length,
    ///     u: unsigned
    /// Availability: MySQL
    TinyInt { m: Option<u8>, u: bool },

    /// SmallInt
    /// Represents a small integer from -32768 to 32767. The unsigned version is from 0 to 65535.
    /// Arguments:
    ///     m: column length, ignored for PostgreSQL
    ///     u: unsigned, ignored for PostgreSQL
    /// Availability: MySQL, PostgreSQL
    SmallInt { m: Option<u8>, u: bool },

    // MediumInt(M), from -8388608 to 8388607. Unsigned version is from 0 - 16777215.
    // Available for MySQL only.
    MediumInt { m: Option<u8>, u: bool },

    // Int(signed), from -2147483648 to 2147483647. Unsigned version is from 0 to 4294967295.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    Int { m: Option<u8>, u: bool },

    // BigInt(M), from -9223372036854775808 to 9223372036854775807. Unsigned version is from 0 to
    // 18446744073709551615.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    BigInt { m: Option<u8>, u: bool },

    // MongoDB's int type
    // MongoDB only
    Int32,

    // MongoDB's number long type
    // MongoDB only
    Int64,

    // MySQL: Decimal(M, D) PostgreSQL: Decimal(precision, scale)
    // A packed “exact” fixed-point number. M is the total number of digits (the precision) and D
    // is the number of digits after the decimal point (the scale). The decimal point and (for
    // negative numbers) the - sign are not counted in M. If D is 0, values have no decimal point
    // or fractional part. The maximum number of digits (M) for DECIMAL is 65. The maximum number
    // of supported decimals (D) is 30. If D is omitted, the default is 0. If M is omitted, the
    // default is 10.
    // Available for MySQL and PostgreSQL.
    // Numeric, Dec, Fixed are all the same.
    // Supports on all databases. MongoDB doesn't have full support.
    Decimal { m: Option<u8>, d: Option<u8> },

    // Float(p)
    // A floating-point number. p represents the precision in bits, but MySQL uses this value only
    // to determine whether to use FLOAT or DOUBLE for the resulting data type. If p is from 0 to
    // 24, the data type becomes FLOAT with no M or D values. If p is from 25 to 53, the data type
    // becomes DOUBLE with no M or D values.
    // MySQL and PostgreSQL only
    Float { m: Option<u8>, d: Option<u8> },

    // Double
    // A double precision. This name is remapped to DOUBLE PRECISION for PostgreSQL.
    // All database supports
    Double { m: Option<u8>, d: Option<u8> },

    // Real
    // A normal float in MySQL or real in PostgreSQL.
    // MySQL and PostgreSQL only
    Real,

    // A date. In MySQL, the supported range is '1000-01-01' to '9999-12-31'. In MongoDB, this
    // represents datetime.
    // All database supports
    Date,

    // datetime. fsp is from 0 - 6. The supported range is '1000-01-01 00:00:00.000000' to
    // '9999-12-31 23:59:59.999999'
    // This is MySQL only.
    // MongoDB supports this, too.
    DateTime(u8),

    /// Timestamp
    /// Arguments:
    ///     p: precision
    ///     z: time zone info
    /// Represents a timestamp.
    /// Note: In MySQL, the range is '1970-01-01 00:00:01.000000' UTC to
    /// '2038-01-19 03:14:07.999999' UTC. In MySQL, the with timezone option is ignored.
    /// Availability: PostgreSQL, MySQL, SQLite, MongoDB
    Timestamp { p: u8, z: bool },

    // Time(fsp, with timezone), fsp is from 0 - 6. Time zone is ignored for MySQL.
    // MySQL and PostgreSQL only
    Time(u8, bool),

    // This is mysql only
    Year,

    /// MARK: - String types

    /// Char
    /// Represents a fixed-length string.
    /// Arguments:
    ///     m: column length, from 0 to 255, if omitted, m is 1
    ///     n: charset name
    ///     c: collate
    /// Note: In PostgreSQL, charset and collate are ignored.
    /// Availability: MySQL, PostgreSQL
    Char { m: Option<u8>, n: Option<String>, c: Option<String> },

    /// VarChar
    /// Represents a variable-length string.
    /// Arguments:
    ///     m: column length, from 0 to 65,535
    ///     n: charset name
    ///     c: collate
    /// Note: In PostgreSQL, charset and collate are ignored.
    /// Availability: MySQL, PostgreSQL
    VarChar { m: u16, n: Option<String>, c: Option<String> },

    /// TinyText
    /// Represents a tiny text.
    /// Arguments:
    ///     n: charset name
    ///     c: collate
    /// Availability: MySQL
    TinyText { n: Option<String>, c: Option<String> },

    /// MediumText
    /// Represents a medium text.
    /// Arguments:
    ///     n: charset name
    ///     c: collate
    /// Availability: MySQL
    MediumText { n: Option<String>, c: Option<String> },

    /// LongText
    /// Represents a long text.
    /// Arguments:
    ///     n: charset name
    ///     c: collate
    /// Availability: MySQL
    LongText { n: Option<String>, c: Option<String> },

    /// Text
    /// Represents a text.
    ///
    /// Arguments:
    ///     m: m
    ///     n: charset name
    ///     c: collate
    /// Note: In PostgreSQL, arguments are ingored.
    ///
    /// Availability: MySQL, PostgreSQL
    Text { m: Option<u16>, n: Option<String>, c: Option<String> },

    /// String
    /// Represents a string.
    // Availability: MongoDB
    String,

    // MySQL and MongoDB only
    Binary(u8),

    // MySQL only
    VarBinary(u16),

    // MySQL only
    TinyBlob,

    // MySQL only
    MediumBlob,

    // MySQL only
    LongBlob,

    // MySQL only
    Blob(u16),

    // ByteA type
    // PostgreSQL only
    ByteA,

    Vec(Box<DatabaseType>),

    Enum(DbEnum),
}
