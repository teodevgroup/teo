use crate::core::connector::ConnectorBuilder;
use crate::core::field::optionality::Optionality;
use crate::core::relation::delete_rule::DeleteRule;
use crate::core::relation::Relation;

pub struct RelationBuilder {
    pub(crate) name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) optionality: Optionality,
    pub(crate) model: String,
    pub(crate) through: Option<String>,
    pub(crate) is_vec: bool,
    pub(crate) fields: Vec<String>,
    pub(crate) references: Vec<String>,
    pub(crate) auto: bool,
    pub(crate) delete_rule: DeleteRule,
    connector_builder: * const Box<dyn ConnectorBuilder>,
}

impl RelationBuilder {
    pub(crate) fn new(name: impl Into<String>, connector_builder: &Box<dyn ConnectorBuilder>) -> Self {
        return RelationBuilder {
            name: name.into(),
            localized_name: "".into(),
            description: "".into(),
            optionality: Optionality::Required,
            model: "".into(),
            through: None,
            is_vec: false,
            fields: Vec::new(),
            references: Vec::new(),
            auto: false,
            delete_rule: DeleteRule::Nullify,
            connector_builder,
        }
    }

    fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        unsafe {
            &*self.connector_builder
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

    pub fn optional(&mut self) -> &mut Self {
        self.optionality = Optionality::Optional;
        self
    }

    pub fn required(&mut self) -> &mut Self {
        self.optionality = Optionality::Required;
        self
    }

    pub fn vec(&mut self, model: impl Into<String>) -> &mut Self {
        self.is_vec = true;
        self.model = model.into();
        self
    }

    pub fn object(&mut self, model: impl Into<String>) -> &mut Self {
        self.is_vec = false;
        self.model = model.into();
        self
    }

    pub fn fields<I, T>(&mut self, fields: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let fields_vec: Vec<String> = fields.into_iter().map(Into::into).collect();
        self.fields = fields_vec;
        self
    }

    pub fn references<I, T>(&mut self, fields: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let references_vec: Vec<String> = fields.into_iter().map(Into::into).collect();
        self.references = references_vec;
        self
    }

    pub fn through(&mut self, model: impl Into<String>) -> &mut Self {
        self.through = Some(model.into());
        self
    }

    pub fn local(&mut self, relation: impl Into<String>) -> &mut Self {
        self.fields = vec![relation.into()];
        self
    }

    pub fn foreign(&mut self, relation: impl Into<String>) -> &mut Self {
        self.references = vec![relation.into()];
        self
    }

    pub fn auto(&mut self) -> &mut Self {
        self.auto = true;
        self
    }

    pub fn cascade(&mut self) -> &mut Self {
        self.delete_rule = DeleteRule::Cascade;
        self
    }

    pub fn nullify(&mut self) -> &mut Self {
        self.delete_rule = DeleteRule::Nullify;
        self
    }

    pub fn deny(&mut self) -> &mut Self {
        self.delete_rule = DeleteRule::Deny;
        self
    }

    pub(crate) fn build(&self, _connector_builder: &Box<dyn ConnectorBuilder>) -> Relation {
        return Relation {
            name: self.name.clone(),
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            optionality: self.optionality.clone(),
            model: self.model.clone(),
            through: self.through.clone(),
            is_vec: self.is_vec,
            fields: self.fields.clone(),
            references: self.references.clone(),
        }
    }
}
