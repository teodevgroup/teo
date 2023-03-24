#[derive(Debug, Clone)]
pub struct EnumChoice {
    pub(self) name: String,
    pub(self) localized_name: Option<String>,
    pub(self) description: Option<String>,
}

impl EnumChoice {

    pub(crate) fn new(name: String, localized_name: Option<String>, description: Option<String>) -> Self {
        Self { name, localized_name, description }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
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
    pub(self) name: String,
    pub(self) localized_name: Option<String>,
    pub(self) description: Option<String>,
    pub(self) choices: Vec<EnumChoice>,
    pub(self) values: Vec<String>,
}

impl Enum {

    pub(crate) fn new(name: String, localized_name: Option<String>, description: Option<String>, choices: Vec<EnumChoice>) -> Self {
        let values = choices.iter().map(|c| c.name.clone()).collect();
        Self {
            name,
            localized_name,
            description,
            choices,
            values,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
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

    pub(crate) fn choices(&self) -> &Vec<EnumChoice> {
        &self.choices
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
