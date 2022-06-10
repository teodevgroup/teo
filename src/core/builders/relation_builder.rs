use crate::core::connector::ConnectorBuilder;
use crate::core::relation::Relation;


pub struct RelationBuilder {
    pub(crate) name: String,
    pub(crate) model: String,
    pub(crate) is_vec: bool,
    pub(crate) fields: Vec<String>,
    pub(crate) references: Vec<String>,
    connector_builder: * const Box<dyn ConnectorBuilder>,
}

impl RelationBuilder {
    pub(crate) fn new(name: impl Into<String>, connector_builder: &Box<dyn ConnectorBuilder>) -> Self {
        return RelationBuilder {
            name: name.into(),
            model: "".into(),
            is_vec: false,
            fields: Vec::new(),
            references: Vec::new(),
            connector_builder,
        }
    }

    fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        unsafe {
            &*self.connector_builder
        }
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

    pub(crate) fn build(&self, _connector_builder: &Box<dyn ConnectorBuilder>) -> Relation {
        return Relation {
            name: self.name.clone(),
            model: self.model.clone(),
            is_vec: self.is_vec,
            fields: self.fields.clone(),
            references: self.references.clone(),
        }
    }
}