use crate::core::database_type::DatabaseType;

pub struct DatabaseTypeTerminationBuilder {
    value: DatabaseType
}

impl DatabaseTypeTerminationBuilder {
    pub(crate) fn new(value: DatabaseType) -> Self {
        Self { value }
    }
}

#[cfg(feature = "mysql")]
pub struct SignedIntegerTypeBuilder {
    value: DatabaseType
}

#[cfg(feature = "mysql")]
impl SignedIntegerTypeBuilder {
    pub(crate) fn new(value: DatabaseType) -> Self {
        Self { value }
    }

    pub fn unsigned(&mut self) -> DatabaseTypeTerminationBuilder {
        let value = match self.value {
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
}
