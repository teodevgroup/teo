use std::borrow::Cow;

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
        self.description.as_ref().map(|d| d.as_str())
    }

    pub(crate) fn description_with_default(&self) -> &str {
        self.description().unwrap_or("This enum variant doesn't have a description.")
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub(crate) name: &'static str,
    pub(crate) ns_path: Vec<String>,
    pub(crate) db_name: Cow<'static, str>,
    pub(crate) localized_name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) variants: Vec<EnumVariant>,
    pub(crate) values: Vec<String>,
}

impl Enum {

    pub(crate) fn new(name: &'static str, ns_path: Vec<String>, localized_name: Option<String>, description: Option<String>, choices: Vec<EnumVariant>) -> Self {
        let values = choices.iter().map(|c| c.name.to_string()).collect();
        Self {
            name,
            ns_path,
            db_name: if ns_path.is_empty() { Cow::Borrowed(name) } else { Cow::Owned("_".to_owned() + ns_path.join("_").as_str() + "_" + name) },
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

    pub(crate) fn db_name(&self) -> &str {
        self.db_name.as_ref()
    }

    pub(crate) fn description(&self) -> Option<&str> {
        if let Some(d) = &self.description {
            Some(d)
        } else {
            None
        }
    }

    pub(crate) fn description_with_default(&self) -> &str {
        self.description().unwrap_or("This enum doesn't have a description.")
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
