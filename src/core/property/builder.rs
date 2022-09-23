use crate::core::connector::ConnectorBuilder;
use crate::core::field::builder::FieldBuilder;
use crate::core::field::Optionality;
use crate::core::field::r#type::FieldType;
use crate::core::pipeline::Pipeline;
use crate::prelude::PipelineBuilder;

pub struct PropertyBuilder {
    pub(crate) name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) optionality: Optionality,
    pub(crate) field_type: FieldType,
    pub(crate) dependencies: Vec<String>,
    pub(crate) setter: Option<Pipeline>,
    pub(crate) getter: Option<Pipeline>,
    connector_builder: * const Box<dyn ConnectorBuilder>,
}

impl PropertyBuilder {

    pub(crate) fn new(name: String, connector_builder: &Box<dyn ConnectorBuilder>) -> Self {
        PropertyBuilder {
            name,
            localized_name: "".to_owned(),
            description: "".to_owned(),
            optionality: Optionality::Optional,
            field_type: FieldType::Undefined,
            dependencies: vec![],
            setter: None,
            getter: None,
            connector_builder,
        }
    }

    pub fn optional(&mut self) -> &mut Self {
        self.optionality = Optionality::Optional;
        self
    }

    pub fn required(&mut self) -> &mut Self {
        self.optionality = Optionality::Required;
        self
    }

    pub fn object_id(&mut self) -> &mut Self {
        self.field_type = FieldType::ObjectId;
        self
    }

    pub fn bool(&mut self) -> &mut Self {
        self.field_type = FieldType::Bool;
        self
    }

    pub fn i8(&mut self) -> &mut Self {
        self.field_type = FieldType::I8;
        self
    }

    pub fn i16(&mut self) -> &mut Self {
        self.field_type = FieldType::I16;
        self
    }

    pub fn i32(&mut self) -> &mut Self {
        self.field_type = FieldType::I32;
        self
    }

    pub fn i64(&mut self) -> &mut Self {
        self.field_type = FieldType::I64;
        self
    }

    pub fn i128(&mut self) -> &mut Self {
        self.field_type = FieldType::I128;
        self
    }

    pub fn u8(&mut self) -> &mut Self {
        self.field_type = FieldType::U8;
        self
    }

    pub fn u16(&mut self) -> &mut Self {
        self.field_type = FieldType::U16;
        self
    }

    pub fn u32(&mut self) -> &mut Self {
        self.field_type = FieldType::U32;
        self
    }

    pub fn u64(&mut self) -> &mut Self {
        self.field_type = FieldType::U64;
        self
    }

    pub fn u128(&mut self) -> &mut Self {
        self.field_type = FieldType::U128;
        self
    }

    pub fn f32(&mut self) -> &mut Self {
        self.field_type = FieldType::F32;
        self
    }

    pub fn f64(&mut self) -> &mut Self {
        self.field_type = FieldType::F64;
        self
    }

    pub fn decimal(&mut self) -> &mut Self {
        self.field_type = FieldType::Decimal;
        self
    }

    pub fn string(&mut self) -> &mut Self {
        self.field_type = FieldType::String;
        self
    }

    pub fn date(&mut self) -> &mut Self {
        self.field_type = FieldType::Date;
        self
    }

    pub fn datetime(&mut self) -> &mut Self {
        self.field_type = FieldType::DateTime;
        self
    }

    pub fn r#enum(&mut self, name: impl Into<String>) -> &mut Self {
        self.field_type = FieldType::Enum(name.into());
        self
    }

    pub fn vec<F: Fn(&mut FieldBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = FieldBuilder::new("", self.connector_builder);
        build(&mut builder);
        let field = builder.build(self.connector_builder());
        self.field_type = FieldType::Vec(Box::new(field));
        self
    }

    pub fn setter<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        self.setter = Some(builder.build());
        self
    }

    pub fn getter<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        self.getter = Some(builder.build());
        self
    }

    pub fn dependencies<I, T>(&mut self, dependencies: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let dependencies_vec: Vec<String> = dependencies.into_iter().map(Into::into).collect();
        self.dependencies = dependencies_vec;
        self
    }

    fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        unsafe {
            &*self.connector_builder
        }
    }
}
