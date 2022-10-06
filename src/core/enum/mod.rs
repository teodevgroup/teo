pub(crate) mod builder;

#[derive(Debug, Clone)]
pub struct EnumChoice {
    pub(self) name: String,
    pub(self) localized_name: String,
    pub(self) description: String,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub(self) name: String,
    pub(self) localized_name: String,
    pub(self) description: String,
    pub(self) choices: Vec<EnumChoice>,
    pub(self) values: Vec<String>,
}

impl EnumChoice {

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn localized_name(&self) -> &str {
        &self.localized_name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }
}

impl Enum {

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn localized_name(&self) -> &str {
        &self.localized_name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn choices(&self) -> &Vec<EnumChoice> {
        &self.choices
    }

    pub(crate) fn values(&self) -> &Vec<String> {
        &self.values
    }
}
