use bson::DateTime;
use crate::core::database_type::DatabaseType;

pub struct DatabaseTypeTerminationBuilder {
    value: DatabaseType
}

impl DatabaseTypeTerminationBuilder {
    pub(crate) fn new(value: DatabaseType) -> Self {
        Self { value }
    }
}

impl Into<DatabaseType> for DatabaseTypeTerminationBuilder {
    fn into(self) -> DatabaseType {
        self.value
    }
}

pub struct StringTypeBuilder {
    value: DatabaseType
}

impl StringTypeBuilder {
    pub(crate) fn new(value: DatabaseType) -> Self {
        Self { value }
    }

    pub fn charset(&self, charset: impl Into<String>) -> Self {
        let value = match &self.value {
            DatabaseType::Char(len, _, collation) => DatabaseType::Char(*len, Some(charset.into()), collation.clone()),
            DatabaseType::VarChar(len, _, collation) => DatabaseType::VarChar(*len, Some(charset.into()), collation.clone()),
            DatabaseType::TinyText(_, collation) => DatabaseType::TinyText(Some(charset.into()), collation.clone()),
            DatabaseType::MediumText(_, collation) => DatabaseType::MediumText(Some(charset.into()), collation.clone()),
            DatabaseType::LongText(_, collation) => DatabaseType::LongText(Some(charset.into()), collation.clone()),
            DatabaseType::Text(len, _, collation) => DatabaseType::Text(*len, Some(charset.into()), collation.clone()),
            _ => self.value.clone()
        };
        Self { value }
    }

    pub fn collation(&self, collation: impl Into<String>) -> DatabaseTypeTerminationBuilder {
        let value = match &self.value {
            DatabaseType::Char(len, charset, _) => DatabaseType::Char(*len, charset.clone(), Some(collation.into())),
            DatabaseType::VarChar(len, charset, _) => DatabaseType::VarChar(*len, charset.clone(), Some(collation.into())),
            DatabaseType::TinyText(charset, _) => DatabaseType::TinyText(charset.clone(), Some(collation.into())),
            DatabaseType::MediumText(charset, _) => DatabaseType::MediumText(charset.clone(), Some(collation.into())),
            DatabaseType::LongText(charset, _) => DatabaseType::LongText(charset.clone(), Some(collation.into())),
            DatabaseType::Text(len, charset, _) => DatabaseType::Text(*len, charset.clone(), Some(collation.into())),
            _ => self.value.clone()
        };
        DatabaseTypeTerminationBuilder { value }
    }
}

impl Into<DatabaseType> for StringTypeBuilder {
    fn into(self) -> DatabaseType {
        self.value
    }
}

pub struct SignedIntegerTypeBuilder {
    value: DatabaseType
}

impl SignedIntegerTypeBuilder {
    pub(crate) fn new(value: DatabaseType) -> Self {
        Self { value }
    }

    pub fn unsigned(&mut self) -> DatabaseTypeTerminationBuilder {
        let value = match &self.value {
            DatabaseType::BigInt(_) => DatabaseType::BigInt(true),
            DatabaseType::Int(_) => DatabaseType::Int(true),
            DatabaseType::TinyInt(_) => DatabaseType::TinyInt(true),
            DatabaseType::SmallInt(_) => DatabaseType::SmallInt(true),
            DatabaseType::MediumInt(_) => DatabaseType::MediumInt(true),
            _ => self.value.clone()
        };
        DatabaseTypeTerminationBuilder { value }
    }
}

impl Into<DatabaseType> for SignedIntegerTypeBuilder {
    fn into(self) -> DatabaseType {
        self.value
    }
}

pub struct DateTimeTypeBuilder {
    value: DatabaseType
}

impl DateTimeTypeBuilder {
    pub(crate) fn new(value: DatabaseType) -> Self {
        Self { value }
    }

    pub fn with_timezone(&self) -> DatabaseTypeTerminationBuilder {
        let value = match &self.value {
            DatabaseType::Timestamp(fsp, _tz) => DatabaseType::Timestamp(*fsp, true),
            DatabaseType::Time(fsp, _tz) => DatabaseType::Time(*fsp, true),
            _ => self.value.clone()
        };
        DatabaseTypeTerminationBuilder { value }
    }
}

impl Into<DatabaseType> for DateTimeTypeBuilder {
    fn into(self) -> DatabaseType {
        self.value
    }
}

pub struct DatabaseTypeBuilder {
    value: DatabaseType
}

impl DatabaseTypeBuilder {

    pub(crate) fn new() -> Self {
        Self { value: DatabaseType::Undefined }
    }

    pub fn tiny_int(&mut self) -> SignedIntegerTypeBuilder {
        SignedIntegerTypeBuilder { value: DatabaseType::TinyInt(false) }
    }

    pub fn small_int(&mut self) -> SignedIntegerTypeBuilder {
        SignedIntegerTypeBuilder { value: DatabaseType::SmallInt(false) }
    }

    pub fn medium_int(&mut self) -> SignedIntegerTypeBuilder {
        SignedIntegerTypeBuilder { value: DatabaseType::MediumInt(false) }
    }

    pub fn int(&mut self) -> SignedIntegerTypeBuilder {
        SignedIntegerTypeBuilder { value: DatabaseType::Int(false) }
    }

    pub fn big_int(&mut self) -> SignedIntegerTypeBuilder {
        SignedIntegerTypeBuilder { value: DatabaseType::BigInt(false) }
    }

    pub fn real(&mut self) -> DatabaseTypeTerminationBuilder {
        DatabaseTypeTerminationBuilder { value: DatabaseType::Real }
    }

    pub fn double(&mut self) -> DatabaseTypeTerminationBuilder {
        DatabaseTypeTerminationBuilder { value: DatabaseType::Double }
    }

    pub fn float(&mut self, precision: u8) -> DatabaseTypeTerminationBuilder {
        DatabaseTypeTerminationBuilder { value: DatabaseType::Float(precision) }
    }

    pub fn date(&mut self) -> DatabaseTypeTerminationBuilder {
        DatabaseTypeTerminationBuilder { value: DatabaseType::Date }
    }

    pub fn timestamp(&mut self, fsp: u8) -> DateTimeTypeBuilder {
        DateTimeTypeBuilder { value: DatabaseType::Timestamp(fsp, false) }
    }

    pub fn char(&mut self, len: u8) -> StringTypeBuilder {
        StringTypeBuilder { value: DatabaseType::Char(len, None, None) }
    }

    pub fn var_char(&mut self, len: u16) -> StringTypeBuilder {
        StringTypeBuilder { value: DatabaseType::VarChar(len, None, None) }
    }

    pub fn tiny_text(&mut self) -> StringTypeBuilder {
        StringTypeBuilder { value: DatabaseType::TinyText(None, None) }
    }

    pub fn medium_text(&mut self) -> StringTypeBuilder {
        StringTypeBuilder { value: DatabaseType::MediumText(None, None) }
    }

    pub fn long_text(&mut self) -> StringTypeBuilder {
        StringTypeBuilder { value: DatabaseType::LongText(None, None) }
    }

    pub fn text(&mut self) -> StringTypeBuilder {
        StringTypeBuilder { value: DatabaseType::Text(None, None, None) }
    }
}

impl Into<DatabaseType> for DatabaseTypeBuilder {
    fn into(self) -> DatabaseType {
        self.value
    }
}
