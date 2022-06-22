use crate::core::r#enum::{Enum, EnumChoice};

pub struct EnumChoiceBuilder {
    name: String,
    localized_name: String,
    description: String,
}

impl EnumChoiceBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        EnumChoiceBuilder {
            name: name.into(),
            localized_name: "".into(),
            description: "".into()
        }
    }

    pub fn localized_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.localized_name = name.into();
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = description.into();
        self
    }
}

pub struct EnumBuilder {
    name: String,
    localized_name: String,
    description: String,
    choices: Vec<EnumChoiceBuilder>
}

impl EnumBuilder {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        EnumBuilder {
            name: name.into(),
            localized_name: "".into(),
            description: "".into(),
            choices: vec![]
        }
    }

    pub fn localized_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.localized_name = name.into();
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = description.into();
        self
    }

    pub fn choice<F: Fn(&mut EnumChoiceBuilder)>(&mut self, value: impl Into<String>, build: F) -> &mut Self {
        let mut choice = EnumChoiceBuilder::new(value.into());
        build(&mut choice);
        self.choices.push(choice);
        self
    }
}

impl Into<EnumChoice> for EnumChoiceBuilder {
    fn into(self) -> EnumChoice {
        EnumChoice {
            name: self.name.clone(),
            localized_name: self.localized_name.clone(),
            description: self.description.clone()
        }
    }
}

impl Into<EnumChoice> for &EnumChoiceBuilder {
    fn into(self) -> EnumChoice {
        EnumChoice {
            name: self.name.clone(),
            localized_name: self.localized_name.clone(),
            description: self.description.clone()
        }
    }
}

impl Into<Enum> for &EnumBuilder {
    fn into(self) -> Enum {
        Enum {
            name: self.name.clone(),
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            choices: self.choices.iter().map(|c| c.into()).collect::<Vec<EnumChoice>>(),
            values: self.choices.iter().map(|c| c.name.clone()).collect(),
        }
    }
}

impl Into<Enum> for EnumBuilder {
    fn into(self) -> Enum {
        Enum {
            name: self.name.clone(),
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            choices: self.choices.iter().map(|c| c.into()).collect::<Vec<EnumChoice>>(),
            values: self.choices.iter().map(|c| c.name.clone()).collect(),
        }
    }
}
