use crate::core::field_type::FieldType;

// The database internal types.
#[derive(Debug, Clone)]
pub enum DatabaseType {

    // This value will be finally altered.
    Undefined,

    // In MySQL, it's alias for TINYINT(1).
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Bool,

    // Bit(M), M is from 1 - 64. If M is omitted, M is 1 by default. To assign,
    // b'value' is used. For example, assigning a value of b'101' to a BIT(6) column is, in effect,
    // the same as assigning b'000101'.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Bit(u8),

    // PostgreSQL only
    #[cfg(feature = "postgres")]
    BitVarying,

    // TinyInt(signed), from -128 to 127. Unsigned version is from 0 - 255.
    // Available on MySQL only.
    #[cfg(feature = "mysql")]
    TinyInt(bool),

    // SmallInt(signed), from -32768 to 32767. Unsigned version is from 0 - 65535.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    SmallInt(bool),

    // MediumInt(M), from -8388608 to 8388607. Unsigned version is from 0 - 16777215.
    // Available for MySQL only.
    #[cfg(feature = "mysql")]
    MediumInt(bool),

    // Int(signed), from -2147483648 to 2147483647. Unsigned version is from 0 to 4294967295.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Int(bool),

    // BigInt(M), from -9223372036854775808 to 9223372036854775807. Unsigned version is from 0 to
    // 18446744073709551615.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    #[cfg(all(feature = "mysql", feature = "postgres", feature = "mongo"))]
    BigInt(bool),

    // MySQL: Decimal(M, D) PostgreSQL: Decimal(precision, scale)
    // A packed “exact” fixed-point number. M is the total number of digits (the precision) and D
    // is the number of digits after the decimal point (the scale). The decimal point and (for
    // negative numbers) the - sign are not counted in M. If D is 0, values have no decimal point
    // or fractional part. The maximum number of digits (M) for DECIMAL is 65. The maximum number
    // of supported decimals (D) is 30. If D is omitted, the default is 0. If M is omitted, the
    // default is 10.
    // Available for MySQL and PostgreSQL.
    // Numeric, Dec, Fixed are all the same.
    #[cfg(all(feature = "mysql", feature = "postgres", feature = "mongo"))]
    Decimal(Option<u8>, Option<u8>),

    // Float(p)
    // A floating-point number. p represents the precision in bits, but MySQL uses this value only
    // to determine whether to use FLOAT or DOUBLE for the resulting data type. If p is from 0 to
    // 24, the data type becomes FLOAT with no M or D values. If p is from 25 to 53, the data type
    // becomes DOUBLE with no M or D values.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Float(u8),

    // Double
    // A double precision. This name is remapped to DOUBLE PRECISION for PostgreSQL.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Double,

    // Real
    // A normal float in MySQL or real in PostgreSQL.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Real,

    // A date. In MySQL, the supported range is '1000-01-01' to '9999-12-31'. In MongoDB, this
    // represents datetime.
    #[cfg(all(feature = "mysql", feature = "postgres", feature = "mongo"))]
    Date,

    // datetime. fsp is from 0 - 6. The supported range is '1000-01-01 00:00:00.000000' to
    // '9999-12-31 23:59:59.999999'
    // This is MySQL only.
    #[cfg(feature = "mysql")]
    DateTime(u8),

    // Timestamp(p, with timezone)
    // A timestamp. In MySQL, the range is '1970-01-01 00:00:01.000000' UTC to
    // '2038-01-19 03:14:07.999999' UTC. In MySQL, the with timezone option is ignored.
    #[cfg(all(feature = "mysql", feature = "postgres", feature = "mongo"))]
    Timestamp(u8, bool),

    // Time(fsp, with timezone), fsp is from 0 - 6. Time zone is ignored for MySQL.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Time(u8, bool),

    // This is mysql only
    #[cfg(feature = "mysql")]
    Year,

    // String types

    // Char(len, charset, collate)
    // On PostgreSQL, charset and collate are ignored.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Char(u8, Option<String>, Option<String>),

    // VarChar(len, charset, collate)
    // On PostgreSQL, charset and collate are ignored.
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    VarChar(u16, Option<String>, Option<String>),

    // TinyText(charset, collate)
    #[cfg(feature = "mysql")]
    TinyText(Option<String>, Option<String>),

    // MediumText(charset, collate)
    #[cfg(feature = "mysql")]
    MediumText(Option<String>, Option<String>),

    // LongText(charset, collate)
    #[cfg(feature = "mysql")]
    LongText(Option<String>, Option<String>),

    // Text is different in MySQL and PostgreSQL
    #[cfg(all(feature = "mysql", feature = "postgres"))]
    Text(Option<u16>, Option<String>, Option<String>),

    #[cfg(feature = "mysql")]
    Binary(u8),

    #[cfg(feature = "mysql")]
    VarBinary(u16),

    #[cfg(feature = "mysql")]
    TinyBlob,

    #[cfg(feature = "mysql")]
    MediumBlob,

    #[cfg(feature = "mysql")]
    LongBlob,

    #[cfg(feature = "mysql")]
    Blob(u16),

    // Bytea type
    #[cfg(feature = "postgres")]
    Bytea,
}

impl Into<FieldType> for &DatabaseType {
    fn into(self) -> FieldType {
        match self {
            DatabaseType::Undefined => FieldType::Undefined,
            DatabaseType::Bool => FieldType::Bool,
            DatabaseType::Bit(_) => todo!(),
            DatabaseType::BitVarying => todo!(),
            DatabaseType::TinyInt(unsigned) => if *unsigned { FieldType::U8 } else { FieldType::I8 },
            DatabaseType::SmallInt(unsigned) => if *unsigned { FieldType::U16 } else { FieldType::I16 },
            DatabaseType::MediumInt(unsigned) => if *unsigned { FieldType::U32 } else { FieldType::I32 },
            DatabaseType::Int(unsigned) => if *unsigned { FieldType::U32 } else { FieldType::I32 },
            DatabaseType::BigInt(unsigned) => if *unsigned { FieldType::U64 } else { FieldType::I64 },
            DatabaseType::Decimal(_, _) => todo!(),
            DatabaseType::Float(precision) => if *precision >= 25 { FieldType::F64 } else { FieldType::F32 },
            DatabaseType::Double => FieldType::F64,
            DatabaseType::Real => FieldType::F32,
            DatabaseType::Date => FieldType::Date,
            DatabaseType::DateTime(_) => FieldType::DateTime,
            DatabaseType::Timestamp(_, _) => FieldType::DateTime,
            DatabaseType::Time(_, _) => todo!(),
            DatabaseType::Year => FieldType::String,
            DatabaseType::Char(_, _, _) => FieldType::String,
            DatabaseType::VarChar(_, _, _) => FieldType::String,
            DatabaseType::TinyText(_, _) => FieldType::String,
            DatabaseType::MediumText(_, _) => FieldType::String,
            DatabaseType::LongText(_, _) => FieldType::String,
            DatabaseType::Text(_, _, _) => FieldType::String,
            DatabaseType::Binary(_) => FieldType::String,
            DatabaseType::VarBinary(_) => FieldType::String,
            DatabaseType::TinyBlob => FieldType::String,
            DatabaseType::MediumBlob => FieldType::String,
            DatabaseType::LongBlob => FieldType::String,
            DatabaseType::Blob(_) => FieldType::String,
            DatabaseType::Bytea => FieldType::String,
        }
    }
}
