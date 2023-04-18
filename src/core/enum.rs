#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub(crate) name: &'static str,
    pub(crate) localized_name: Option<String>,
    pub(crate) description: Option<String>,
}

impl EnumVariant {

    pub(crate) fn new(name: &'static str, localized_name: Option<String>, description: Option<String>) -> Self {
        Self { name, localized_name, description }
    }

    pub(crate) fn name(&self) -> &str {
        self.name
    }

    pub(crate) fn localized_name(&self) -> &str {
        if let Some(l) = &self.localized_name {
            l
        } else {
            self.name()
        }
    }

    pub(crate) fn description(&self) -> Option<&str> {
        if let Some(d) = &self.description {
            Some(d)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub(crate) name: &'static str,
    pub(crate) localized_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) variants: Vec<EnumVariant>,
    pub(crate) values: Vec<String>,
}

impl Enum {

    pub(crate) fn new(name: &'static str, localized_name: Option<String>, description: Option<String>, choices: Vec<EnumVariant>) -> Self {
        let values = choices.iter().map(|c| c.name.to_string()).collect();
        Self {
            name,
            localized_name,
            description,
            variants: choices,
            values,
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name
    }

    pub(crate) fn localized_name(&self) -> &str {
        if let Some(l) = &self.localized_name {
            l
        } else {
            self.name()
        }
    }

    pub(crate) fn description(&self) -> Option<&str> {
        if let Some(d) = &self.description {
            Some(d)
        } else {
            None
        }
    }

    pub(crate) fn variants(&self) -> &Vec<EnumVariant> {
        &self.variants
    }

    pub(crate) fn values(&self) -> &Vec<String> {
        &self.values
    }
}

impl Into<DbEnum> for Enum {
    fn into(self) -> DbEnum {
        DbEnum { choices: self.values }
    }
}

impl Into<DbEnum> for &Enum {
    fn into(self) -> DbEnum {
        DbEnum { choices: self.values.clone() }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DbEnum {
    pub(crate) choices: Vec<String>,
}
