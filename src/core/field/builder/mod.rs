

use crate::core::pipeline::argument::Argument;
use crate::core::field::builder::index_builder::FieldIndexBuilder;
use crate::core::permission::builder::PermissionBuilder;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::connector::{ConnectorBuilder};
use crate::core::database::r#type::DatabaseType;
use crate::core::field::*;
use crate::core::field::optionality::Optionality::{PresentIf, PresentWith, PresentWithout};
use crate::core::field::r#type::FieldType;

pub(crate) mod index_builder;

pub struct FieldBuilder {
    pub(crate) name: String,
    pub(crate) localized_name: String,
    pub(crate) description: String,
    pub(crate) field_type: FieldType,
    pub(crate) database_type: DatabaseType,
    pub(crate) optionality: Optionality,
    pub(crate) r#virtual: bool,
    pub(crate) atomic: bool,
    pub(crate) primary: bool,
    pub(crate) read_rule: ReadRule,
    pub(crate) write_rule: WriteRule,
    pub(crate) previous_value_rule: PreviousValueRule,
    pub(crate) input_omissible: bool,
    pub(crate) output_omissible: bool,
    pub(crate) index: FieldIndex,
    pub(crate) query_ability: QueryAbility,
    pub(crate) object_assignment: ObjectAssignment,
    pub(crate) auto: bool,
    pub(crate) auto_increment: bool,
    pub(crate) auth_identity: bool,
    pub(crate) auth_by: bool,
    pub(crate) auth_by_arg: Option<Argument>,
    pub(crate) default: Option<Argument>,
    pub(crate) on_set_pipeline: PipelineBuilder,
    pub(crate) on_save_pipeline: PipelineBuilder,
    pub(crate) on_output_pipeline: PipelineBuilder,
    pub(crate) permission: Option<PermissionBuilder>,
    pub(crate) column_name: Option<String>,
    pub(crate) foreign_key: bool,
    connector_builder: * const Box<dyn ConnectorBuilder>,
}

impl FieldBuilder {
    pub(crate) fn new(name: impl Into<String>, connector_builder: *const Box<dyn ConnectorBuilder>) -> Self {
        return FieldBuilder {
            name: name.into(),
            localized_name: "".to_string(),
            description: "".to_string(),
            field_type: FieldType::Undefined,
            database_type: DatabaseType::Undefined,
            optionality: Optionality::Required,
            r#virtual: false,
            atomic: false,
            primary: false,
            read_rule: ReadRule::Read,
            write_rule: WriteRule::Write,
            index: FieldIndex::NoIndex,
            query_ability: QueryAbility::Queryable,
            object_assignment: ObjectAssignment::Reference,
            auto: false,
            auto_increment: false,
            auth_identity: false,
            auth_by: false,
            auth_by_arg: None,
            default: None,
            on_set_pipeline: PipelineBuilder::new(),
            on_save_pipeline: PipelineBuilder::new(),
            on_output_pipeline: PipelineBuilder::new(),
            permission: None,
            column_name: None,
            previous_value_rule: PreviousValueRule::DontKeep,
            input_omissible: false,
            output_omissible: false,
            foreign_key: false,
            connector_builder,
        }
    }

    fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        unsafe {
            &*self.connector_builder
        }
    }

    #[cfg(feature = "data-source-mongodb")]
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
        let mut builder = FieldBuilder::new("", self.connector_builder());
        build(&mut builder);
        let field = builder.build(self.connector_builder());
        self.field_type = FieldType::Vec(Box::new(field));
        self
    }

    pub fn hashmap<F: Fn(&mut FieldBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = FieldBuilder::new("", self.connector_builder());
        build(&mut builder);
        let field = builder.build(self.connector_builder());
        self.field_type = FieldType::HashMap(Box::new(field));
        self
    }

    pub fn btreemap<F: Fn(&mut FieldBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = FieldBuilder::new("", self.connector_builder());
        build(&mut builder);
        let field = builder.build(self.connector_builder());
        self.field_type = FieldType::BTreeMap(Box::new(field));
        self
    }

    pub fn object(&mut self, model: impl Into<String>) -> &mut Self {
        self.field_type = FieldType::Object(model.into());
        self
    }

    pub fn read_if<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.read_rule = ReadRule::ReadIf(pipeline.build());
        self
    }

    pub fn write_once(&mut self) -> &mut Self {
        self.write_rule = WriteRule::WriteOnce;
        self
    }

    pub fn write_on_create(&mut self) -> &mut Self {
        self.write_rule = WriteRule::WriteOnCreate;
        self
    }

    pub fn write_nonnull(&mut self) -> &mut Self {
        self.write_rule = WriteRule::WriteNonNull;
        self
    }

    pub fn write_if<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.write_rule = WriteRule::WriteIf(pipeline.build());
        self
    }

    pub fn record_previous(&mut self) -> &mut Self {
        self.previous_value_rule = PreviousValueRule::Keep;
        self
    }

    pub fn input_omissible(&mut self) -> &mut Self {
        self.input_omissible = true;
        self
    }

    pub fn output_omissible(&mut self) -> &mut Self {
        self.output_omissible = true;
        self
    }

    pub fn unique(&mut self) -> &mut Self {
        self.index = FieldIndex::Unique(IndexSettings::default());
        self
    }

    pub fn unique_settings<F: Fn(&mut FieldIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut index_builder = FieldIndexBuilder::new();
        build(&mut index_builder);
        self.index = FieldIndex::Unique(index_builder.build());
        self
    }

    pub fn index(&mut self) -> &mut Self {
        self.index = FieldIndex::Index(IndexSettings::default());
        self
    }

    pub fn index_settings<F: Fn(&mut FieldIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut index_builder = FieldIndexBuilder::new();
        build(&mut index_builder);
        self.index = FieldIndex::Index(index_builder.build());
        self
    }

    pub fn optional(&mut self) -> &mut Self {
        self.optionality = Optionality::Optional;
        self.input_omissible = true;
        self.output_omissible = true;
        self
    }

    pub fn required(&mut self) -> &mut Self {
        self.optionality = Optionality::Required;
        self
    }

    pub fn present_with<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let string_keys: Vec<String> = keys.into_iter().map(Into::into).collect();
        self.optionality = PresentWith(string_keys);
        self.input_omissible = true;
        self.output_omissible = true;
        self
    }

    pub fn present_without<I, T>(&mut self, keys: I) -> &mut Self where I: IntoIterator<Item = T>, T: Into<String> {
        let string_keys: Vec<String> = keys.into_iter().map(Into::into).collect();
        self.optionality = PresentWithout(string_keys);
        self
    }

    pub fn present_if<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = PipelineBuilder::new();
        build(&mut builder);
        self.optionality = PresentIf(builder.build());
        self
    }

    pub fn r#virtual(&mut self) -> &mut Self {
        self.r#virtual = true;
        self
    }

    pub fn copy(&mut self) -> &mut Self {
        self.object_assignment = ObjectAssignment::Copy;
        self
    }

    pub fn auth_identity(&mut self) -> &mut Self {
        self.auth_identity = true;
        self
    }

    pub fn auth_by<F>(&mut self, argument: F) -> &mut Self where F: Fn(&mut PipelineBuilder) -> () + Clone + 'static {
        self.auth_by = true;
        self.auth_by_arg = Some(argument.into());
        self
    }

    pub fn auto(&mut self) -> &mut Self {
        self.auto = true;
        self
    }

    pub fn auto_increment(&mut self) -> &mut Self {
        self.auto = true;
        self.auto_increment = true;
        self
    }

    pub fn on_set<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        build(&mut self.on_set_pipeline);
        self
    }

    pub fn on_save<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        build(&mut self.on_save_pipeline);
        self
    }

    pub fn on_output<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        build(&mut self.on_output_pipeline);
        self
    }

    pub fn assign_identity(&mut self) -> &mut Self {
        self
    }

    pub fn default(&mut self, value: impl Into<Argument>) -> &mut Self {
        self.default = Some(value.into());
        self.input_omissible = true;
        self
    }

    pub fn permissions<F: Fn(&mut PermissionBuilder)>(&mut self, build: F) -> &mut Self {
        let mut permission_builder = PermissionBuilder::new();
        build(&mut permission_builder);
        self.permission = Some(permission_builder);
        self
    }

    pub fn column_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.column_name = Some(name.into());
        self
    }

    pub fn db(&mut self, database_type: DatabaseType) -> &mut Self {
        self.database_type = database_type;
        self
    }

    pub fn foreign_key(&mut self) -> &mut Self {
        self.foreign_key = true;
        self
    }

    pub(crate) fn build(&self, connector_builder: &Box<dyn ConnectorBuilder>) -> Field {
        return Field {
            name: self.name.clone(),
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            field_type: self.field_type.clone(),
            database_type: if self.database_type.is_undefined() { connector_builder.default_database_type(&self.field_type) } else { self.database_type.clone() },
            optionality: self.optionality.clone(),
            r#virtual: self.r#virtual,
            atomic: self.atomic,
            primary: self.primary,
            read_rule: self.read_rule.clone(),
            write_rule: self.write_rule.clone(),
            index: self.index.clone(),
            query_ability: self.query_ability,
            object_assignment: self.object_assignment,
            auto: self.auto,
            auto_increment: self.auto_increment,
            auth_identity: self.auth_identity,
            auth_by: self.auth_by,
            auth_by_arg: self.auth_by_arg.clone(),
            default: self.default.clone(),
            on_set_pipeline: self.on_set_pipeline.build(),
            on_save_pipeline: self.on_save_pipeline.build(),
            on_output_pipeline: self.on_output_pipeline.build(),
            permission: if let Some(builder) = &self.permission { Some(builder.build()) } else { None },
            column_name: self.column_name.clone(),
            previous_value_rule: self.previous_value_rule.clone(),
            input_omissible: self.input_omissible,
            output_omissible: self.output_omissible,
            foreign_key: self.foreign_key,
        }
    }
}
