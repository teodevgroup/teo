use std::future::Future;
use std::sync::Arc;
use crate::core::pipeline::argument::Argument;
use crate::core::db_type::builder::DatabaseTypeBuilder;
use crate::core::field::builder::index_builder::FieldIndexBuilder;
use crate::core::permission::builder::PermissionBuilder;
use crate::core::pipeline::builder::PipelineBuilder;
use crate::core::connector::{ConnectorBuilder};
use crate::core::db_type::DatabaseType;
use crate::core::field::*;
use crate::core::field::r#type::FieldType;
use crate::core::object::Object;
use crate::core::previous_value::PreviousValueRule;
use crate::core::value::Value;
use crate::core::error::ActionError;

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
    pub(crate) previous_value_rule: PreviousValueRule,
    connector_builder: * const Box<dyn ConnectorBuilder>,
}

impl FieldBuilder {
    pub(crate) fn new(name: impl Into<String>, connector_builder: &Box<dyn ConnectorBuilder>) -> Self {
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
            connector_builder,
        }
    }

    fn connector_builder(&self) -> &Box<dyn ConnectorBuilder> {
        unsafe {
            &*self.connector_builder
        }
    }

    pub fn localized_name(&mut self, localized_name: impl Into<String>) -> &mut Self {
        self.localized_name = localized_name.into();
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = description.into();
        self
    }

    pub fn object_id(&mut self) -> &mut Self {
        self.field_type = FieldType::ObjectId;
        return self;
    }

    pub fn bool(&mut self) -> &mut Self {
        self.field_type = FieldType::Bool;
        return self;
    }

    pub fn i8(&mut self) -> &mut Self {
        self.field_type = FieldType::I8;
        return self;
    }

    pub fn i16(&mut self) -> &mut Self {
        self.field_type = FieldType::I16;
        return self;
    }

    pub fn i32(&mut self) -> &mut Self {
        self.field_type = FieldType::I32;
        return self;
    }

    pub fn i64(&mut self) -> &mut Self {
        self.field_type = FieldType::I64;
        return self;
    }

    pub fn i128(&mut self) -> &mut Self {
        self.field_type = FieldType::I128;
        return self;
    }

    pub fn u8(&mut self) -> &mut Self {
        self.field_type = FieldType::U8;
        return self;
    }

    pub fn u16(&mut self) -> &mut Self {
        self.field_type = FieldType::U16;
        return self;
    }

    pub fn u32(&mut self) -> &mut Self {
        self.field_type = FieldType::U32;
        return self;
    }

    pub fn u64(&mut self) -> &mut Self {
        self.field_type = FieldType::U64;
        return self;
    }

    pub fn u128(&mut self) -> &mut Self {
        self.field_type = FieldType::U128;
        return self;
    }

    pub fn f32(&mut self) -> &mut Self {
        self.field_type = FieldType::F32;
        return self;
    }

    pub fn f64(&mut self) -> &mut Self {
        self.field_type = FieldType::F64;
        return self;
    }

    pub fn string(&mut self) -> &mut Self {
        self.field_type = FieldType::String;
        return self;
    }

    pub fn date(&mut self) -> &mut Self {
        self.field_type = FieldType::Date;
        return self;
    }

    pub fn datetime(&mut self) -> &mut Self {
        self.field_type = FieldType::DateTime;
        return self;
    }

    pub fn r#enum(&mut self, name: &'static str) -> &mut Self {
        self.field_type = FieldType::Enum(name);
        self
    }

    pub fn vec<F: Fn(&mut FieldBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = FieldBuilder::new("", self.connector_builder());
        build(&mut builder);
        let field = builder.build(self.connector_builder());
        self.field_type = FieldType::Vec(Box::new(field));
        return self;
    }

    pub fn map<F: Fn(&mut FieldBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = FieldBuilder::new("", self.connector_builder());
        build(&mut builder);
        let field = builder.build(self.connector_builder());
        self.field_type = FieldType::Map(Box::new(field));
        return self;
    }

    pub fn object(&mut self, model: &'static str) -> &mut Self {
        self.field_type = FieldType::Object(model);
        self
    }

    pub fn atomic(&mut self) -> &mut Self {
        self.atomic = true;
        self
    }

    pub fn nonatomic(&mut self) -> &mut Self {
        self.atomic = false;
        self
    }

    pub fn primary(&mut self) -> &mut Self {
        self.primary = true;
        return self;
    }

    pub fn internal(&mut self) -> &mut Self {
        self.write_rule = WriteRule::NoWrite;
        self.read_rule = ReadRule::NoRead;
        return self;
    }

    pub fn readonly(&mut self) -> &mut Self {
        self.write_rule = WriteRule::NoWrite;
        self
    }

    pub fn writeonly(&mut self) -> &mut Self {
        self.read_rule = ReadRule::NoRead;
        self.query_ability = QueryAbility::Unqueryable;
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

    pub fn unique(&mut self) -> &mut Self {
        self.index = FieldIndex::Unique(IndexSettings::default());
        return self;
    }

    pub fn unique_settings<F: Fn(&mut FieldIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut index_builder = FieldIndexBuilder::new();
        build(&mut index_builder);
        self.index = FieldIndex::Unique(index_builder.build());
        self
    }

    pub fn index(&mut self) -> &mut Self {
        self.index = FieldIndex::Index(IndexSettings::default());
        return self;
    }

    pub fn index_settings<F: Fn(&mut FieldIndexBuilder)>(&mut self, build: F) -> &mut Self {
        let mut index_builder = FieldIndexBuilder::new();
        build(&mut index_builder);
        self.index = FieldIndex::Index(index_builder.build());
        self
    }

    pub fn optional(&mut self) -> &mut Self {
        self.optionality = Optionality::Optional;
        return self;
    }

    pub fn required(&mut self) -> &mut Self {
        self.optionality = Optionality::Required;
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
        return self;
    }

    pub fn on_output<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        build(&mut self.on_output_pipeline);
        return self;
    }

    pub fn assign_identity(&mut self) -> &mut Self {
        return self;
    }

    pub fn default(&mut self, value: impl Into<Argument>) -> &mut Self {
        self.default = Some(value.into());
        return self;
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

    pub fn db<F, A>(&mut self, build: F) -> &mut Self where F: Fn(&mut DatabaseTypeBuilder) -> A, A: Into<DatabaseType> {
        let mut builder = DatabaseTypeBuilder::new();
        let result = build(&mut builder);
        let db_type = result.into();
        self.database_type = db_type;
        match self.field_type {
            FieldType::Undefined => {
                self.field_type = (&self.database_type).into();
            }
            _ => {}
        }
        self
    }

    pub(crate) fn build(&self, connector_builder: &Box<dyn ConnectorBuilder>) -> Field {
        return Field {
            name: self.name.clone(),
            localized_name: self.localized_name.clone(),
            description: self.description.clone(),
            field_type: self.field_type.clone(),
            database_type: if self.database_type.is_undefined() { connector_builder.inferred_database_type(&self.field_type) } else { self.database_type.clone() },
            optionality: self.optionality,
            r#virtual: self.r#virtual,
            atomic: self.atomic,
            primary: self.primary,
            read_rule: self.read_rule,
            write_rule: self.write_rule,
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
        }
    }
}
