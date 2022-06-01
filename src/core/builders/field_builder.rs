use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use crate::action::action::ActionType;
use crate::core::argument::{Argument, FnArgument};
use crate::core::argument::Argument::{PipelineArgument, ValueArgument};
use crate::core::builders::field_index_builder::FieldIndexBuilder;
use crate::core::builders::permission_builder::PermissionBuilder;
use crate::core::builders::pipeline_builder::PipelineBuilder;
use crate::core::connector::{ConnectorBuilder};
use crate::core::field::*;
use crate::core::pipeline::Pipeline;
use crate::core::value::Value;


pub struct FieldBuilder {
    pub(crate) name: &'static str,
    pub(crate) localized_name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) r#type: Type,
    pub(crate) optionality: Optionality,
    pub(crate) store: Store,
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
    pub(crate) column_name: Option<&'static str>,
}

impl FieldBuilder {
    pub fn new(name: &'static str) -> Self {
        return FieldBuilder {
            name,
            localized_name: "",
            description: "",
            r#type: Type::Undefined,
            optionality: Optionality::Required,
            store: Store::Embedded,
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
        }
    }

    pub fn localized_name(&mut self, localized_name: &'static str) {
        self.localized_name = localized_name;
    }

    pub fn description(&mut self, description: &'static str) {
        self.description = description;
    }

    pub fn object_id(&mut self) -> &mut Self {
        self.r#type = Type::ObjectId;
        return self;
    }

    pub fn bool(&mut self) -> &mut Self {
        self.r#type = Type::Bool;
        return self;
    }

    pub fn i8(&mut self) -> &mut Self {
        self.r#type = Type::I8;
        return self;
    }

    pub fn i16(&mut self) -> &mut Self {
        self.r#type = Type::I16;
        return self;
    }

    pub fn i32(&mut self) -> &mut Self {
        self.r#type = Type::I32;
        return self;
    }

    pub fn i64(&mut self) -> &mut Self {
        self.r#type = Type::I64;
        return self;
    }

    pub fn i128(&mut self) -> &mut Self {
        self.r#type = Type::I128;
        return self;
    }

    pub fn u8(&mut self) -> &mut Self {
        self.r#type = Type::U8;
        return self;
    }

    pub fn u16(&mut self) -> &mut Self {
        self.r#type = Type::U16;
        return self;
    }

    pub fn u32(&mut self) -> &mut Self {
        self.r#type = Type::U32;
        return self;
    }

    pub fn u64(&mut self) -> &mut Self {
        self.r#type = Type::U64;
        return self;
    }

    pub fn u128(&mut self) -> &mut Self {
        self.r#type = Type::U128;
        return self;
    }

    pub fn f32(&mut self) -> &mut Self {
        self.r#type = Type::F32;
        return self;
    }

    pub fn f64(&mut self) -> &mut Self {
        self.r#type = Type::F64;
        return self;
    }

    pub fn string(&mut self) -> &mut Self {
        self.r#type = Type::String;
        return self;
    }

    pub fn date(&mut self) -> &mut Self {
        self.r#type = Type::Date;
        return self;
    }

    pub fn datetime(&mut self) -> &mut Self {
        self.r#type = Type::DateTime;
        return self;
    }

    pub fn r#enum(&mut self, name: &'static str) -> &mut Self {
        self.r#type = Type::Enum(name);
        self
    }

    pub fn vec<F: Fn(&mut FieldBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = FieldBuilder::new("");
        build(&mut builder);
        let field = Field::new(&builder);
        self.r#type = Type::Vec(Box::new(field));
        return self;
    }

    pub fn map<F: Fn(&mut FieldBuilder)>(&mut self, build: F) -> &mut Self {
        let mut builder = FieldBuilder::new("");
        build(&mut builder);
        let field = Field::new(&builder);
        self.r#type = Type::Map(Box::new(field));
        return self;
    }

    pub fn object(&mut self, model: &'static str) -> &mut Self {
        self.r#type = Type::Object(model);
        return self;
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

    pub fn linked_by(&mut self, field: &'static str) -> &mut Self {
        self.store = Store::ForeignKey(field);
        self
    }

    pub fn link_to(&mut self) -> &mut Self {
        self.store = Store::LocalKey;
        self
    }

    pub fn temp(&mut self) -> &mut Self {
        self.store = Store::Temp;
        self
    }

    pub fn calculated(&mut self) -> &mut Self {
        self.store = Store::Calculated;
        self.write_rule = WriteRule::NoWrite;
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

    pub fn auth_by(&mut self, argument: impl Into<Argument>) -> &mut Self {
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

    pub fn column_name(&mut self, name: &'static str) -> &mut Self {
        self.column_name = Some(name);
        self
    }
}
