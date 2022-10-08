use crate::core::field::r#type::FieldType;

pub(crate) mod builder;

// The database internal types.
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseType {

    // This value will be finally altered.
    Undefined,

    // MongoDB only
    ObjectId,

    // In MySQL, it's alias for TINYINT(1).
    // All database supports.
    Bool,

    // Bit(M), M is from 1 - 64. If M is omitted, M is 1 by default. To assign,
    // b'value' is used. For example, assigning a value of b'101' to a BIT(6) column is, in effect,
    // the same as assigning b'000101'.
    // MySQL and PostgreSQL
    Bit(u8),

    // PostgreSQL only
    BitVarying,

    // TinyInt(signed), from -128 to 127. Unsigned version is from 0 - 255.
    // Available on MySQL only.
    TinyInt(bool),

    // SmallInt(signed), from -32768 to 32767. Unsigned version is from 0 - 65535.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    SmallInt(bool),

    // MediumInt(M), from -8388608 to 8388607. Unsigned version is from 0 - 16777215.
    // Available for MySQL only.
    MediumInt(bool),

    // Int(signed), from -2147483648 to 2147483647. Unsigned version is from 0 to 4294967295.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    Int(bool),

    // BigInt(M), from -9223372036854775808 to 9223372036854775807. Unsigned version is from 0 to
    // 18446744073709551615.
    // Available for MySQL and PostgreSQL. The signed option is ignored in PostgreSQL.
    BigInt(bool),

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
    Decimal(Option<u8>, Option<u8>),

    // Float(p)
    // A floating-point number. p represents the precision in bits, but MySQL uses this value only
    // to determine whether to use FLOAT or DOUBLE for the resulting data type. If p is from 0 to
    // 24, the data type becomes FLOAT with no M or D values. If p is from 25 to 53, the data type
    // becomes DOUBLE with no M or D values.
    // MySQL and PostgreSQL only
    Float(u8),

    // Double
    // A double precision. This name is remapped to DOUBLE PRECISION for PostgreSQL.
    // All database supports
    Double,

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

    // Timestamp(p, with timezone)
    // A timestamp. In MySQL, the range is '1970-01-01 00:00:01.000000' UTC to
    // '2038-01-19 03:14:07.999999' UTC. In MySQL, the with timezone option is ignored.
    // All database supports this.
    Timestamp(u8, bool),

    // Time(fsp, with timezone), fsp is from 0 - 6. Time zone is ignored for MySQL.
    // MySQL and PostgreSQL only
    Time(u8, bool),

    // This is mysql only
    Year,

    // String types

    // Char(len, charset, collate)
    // On PostgreSQL, charset and collate are ignored.
    // MySQL and PostgreSQL support this
    Char(u8, Option<String>, Option<String>),

    // VarChar(len, charset, collate)
    // On PostgreSQL, charset and collate are ignored.
    // MySQL and PostgreSQL support this
    VarChar(u16, Option<String>, Option<String>),

    // TinyText(charset, collate)
    // MySQL support this
    TinyText(Option<String>, Option<String>),

    // MediumText(charset, collate)
    // MySQL support this
    MediumText(Option<String>, Option<String>),

    // LongText(charset, collate)
    // MySQL support this
    LongText(Option<String>, Option<String>),

    // Text is different in MySQL and PostgreSQL
    // MySQL and PostgreSQL support this
    Text(Option<u16>, Option<String>, Option<String>),

    // String
    // MongoDB only
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
}

impl Into<FieldType> for &DatabaseType {
    fn into(self) -> FieldType {
        match self {
            DatabaseType::Undefined => FieldType::Undefined,
            #[cfg(feature = "data-source-mongodb")]
            DatabaseType::ObjectId => FieldType::ObjectId,
            DatabaseType::Bool => FieldType::Bool,
            DatabaseType::Bit(_) => todo!(),
            DatabaseType::BitVarying => todo!(),
            DatabaseType::TinyInt(unsigned) => if *unsigned { FieldType::U8 } else { FieldType::I8 },
            DatabaseType::SmallInt(unsigned) => if *unsigned { FieldType::U16 } else { FieldType::I16 },
            DatabaseType::MediumInt(unsigned) => if *unsigned { FieldType::U32 } else { FieldType::I32 },
            DatabaseType::Int(unsigned) => if *unsigned { FieldType::U32 } else { FieldType::I32 },
            DatabaseType::BigInt(unsigned) => if *unsigned { FieldType::U64 } else { FieldType::I64 },
            DatabaseType::Int32 => FieldType::I32,
            DatabaseType::Int64 => FieldType::I64,
            DatabaseType::Decimal(_, _) => FieldType::Decimal,
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
            DatabaseType::String => FieldType::String,
            DatabaseType::Binary(_) => FieldType::String,
            DatabaseType::VarBinary(_) => FieldType::String,
            DatabaseType::TinyBlob => FieldType::String,
            DatabaseType::MediumBlob => FieldType::String,
            DatabaseType::LongBlob => FieldType::String,
            DatabaseType::Blob(_) => FieldType::String,
            DatabaseType::ByteA => FieldType::String,
        }
    }
}

impl DatabaseType {
    pub(crate) fn is_undefined(&self) -> bool {
        match self {
            DatabaseType::Undefined => true,
            _ => false
        }
    }
}
