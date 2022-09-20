
use std::sync::{Arc};
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::Validity;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::logical::all::AllModifier;
use crate::core::pipeline::modifiers::logical::and::AndModifier;
use crate::core::pipeline::modifiers::bcrypt::bcrypt_salt::BcryptSaltModifier;
use crate::core::pipeline::modifiers::bcrypt::bcrypt_verify::BcryptVerifyModifier;
use crate::core::pipeline::modifiers::math::ceil::CeilModifier;
use crate::core::pipeline::modifiers::string::generation::cuid::CUIDModifier;
use crate::core::pipeline::modifiers::logical::r#else::ElseModifier;
use crate::core::pipeline::modifiers::math::floor::FloorModifier;
use crate::core::pipeline::modifiers::logical::r#if::IfModifier;
use crate::core::pipeline::modifiers::datetime::now::NowModifier;
use crate::core::pipeline::modifiers::object::object_value::ObjectValueModifier;
use crate::core::pipeline::modifiers::logical::or::OrModifier;
use crate::core::pipeline::modifiers::logical::r#do::DoModifier;
use crate::core::pipeline::modifiers::string::transform::regex_replace::RegexReplaceModifier;
use crate::core::pipeline::modifiers::string::generation::slug::SlugModifier;
use crate::core::pipeline::modifiers::logical::then::ThenModifier;
use crate::core::pipeline::modifiers::string::generation::uuid::UUIDModifier;


use crate::core::pipeline::modifiers::array::append::AppendModifier;
use crate::core::pipeline::modifiers::array::has_length::{HasLengthModifier, LengthArgument};
use crate::core::pipeline::modifiers::array::prepend::PrependModifier;
use crate::core::pipeline::modifiers::function::callback::{CallbackArgument, CallbackModifier};
use crate::core::pipeline::modifiers::function::compare::{CompareArgument, CompareModifier};
use crate::core::pipeline::modifiers::function::transform::{TransformArgument, TransformModifier};
use crate::core::pipeline::modifiers::function::validate::{ValidateArgument, ValidateModifier};
use crate::core::pipeline::modifiers::logical::any::AnyModifier;
use crate::core::pipeline::modifiers::logical::transform_with::TransformWithModifier;
use crate::core::pipeline::modifiers::logical::validate_with::ValidateWithModifier;
use crate::core::pipeline::modifiers::logical::when_create::WhenCreateModifier;
use crate::core::pipeline::modifiers::logical::when_update::WhenUpdateModifier;
use crate::core::pipeline::modifiers::math::abs::AbsModifier;
use crate::core::pipeline::modifiers::math::add::AddModifier;
use crate::core::pipeline::modifiers::math::divide::DivideModifier;
use crate::core::pipeline::modifiers::math::modular::ModularModifier;
use crate::core::pipeline::modifiers::math::multiply::MultiplyModifier;
use crate::core::pipeline::modifiers::math::subtract::SubtractModifier;
use crate::core::pipeline::modifiers::object::is_instance_of::IsObjectOfModifier;
use crate::core::pipeline::modifiers::object::object_previous_value::ObjectPreviousValueModifier;
use crate::core::pipeline::modifiers::object::object_set_value::ObjectSetValueModifier;
use crate::core::pipeline::modifiers::string::generation::random_digits::RandomDigitsModifier;
use crate::core::pipeline::modifiers::string::validation::is_alphanumeric::IsAlphanumericModifier;
use crate::core::pipeline::modifiers::string::validation::is_email::IsEmailModifier;
use crate::core::pipeline::modifiers::string::validation::is_secure_password::IsSecurePasswordModifier;
use crate::core::pipeline::modifiers::string::validation::regex_match::RegexMatchModifier;
use crate::core::pipeline::modifiers::value::eq::EqModifier;
use crate::core::pipeline::modifiers::value::is_exist::IsExistModifier;
use crate::core::pipeline::modifiers::value::is_false::IsFalseModifier;
use crate::core::pipeline::modifiers::value::is_null::IsNullModifier;
use crate::core::pipeline::modifiers::value::is_true::IsTrueModifier;
use crate::core::pipeline::modifiers::value::neq::NeqModifier;
use crate::core::pipeline::Pipeline;
use crate::core::value::Value;

#[derive(Debug)]
pub struct PipelineBuilder {
    pub modifiers: Vec<Arc<dyn Modifier>>
}

impl PipelineBuilder {

    pub fn new() -> Self {
        return PipelineBuilder {
            modifiers: Vec::new()
        };
    }

    pub(crate) fn has_any_modifier(&self) -> bool {
        self.modifiers.len() > 0
    }

    pub fn abs(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(AbsModifier::new()));
        return self;
    }

    pub fn add(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(AddModifier::new(argument)));
        self
    }

    pub fn subtract(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(SubtractModifier::new(argument)));
        self
    }

    pub fn multiply(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(MultiplyModifier::new(argument)));
        self
    }

    pub fn divide(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(DivideModifier::new(argument)));
        self
    }

    pub fn modular(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(ModularModifier::new(argument)));
        self
    }

    pub fn ceil(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(CeilModifier::new()));
        return self;
    }

    pub fn floor(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(FloorModifier::new()));
        return self;
    }

    pub fn is_alphanumeric(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsAlphanumericModifier::new()));
        return self;
    }

    pub fn is_email(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsEmailModifier::new()));
        return self;
    }

    pub fn regex_match(&mut self, regex: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(RegexMatchModifier::new(regex)));
        return self;
    }

    pub fn regex_replace(&mut self, regex: impl Into<Argument>, substitute: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(RegexReplaceModifier::new(regex, substitute)));
        self
    }

    pub fn random_digits(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(RandomDigitsModifier::new(argument)));
        return self;
    }

    pub fn now(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(NowModifier::new()));
        return self;
    }

    pub fn append(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(AppendModifier::new(argument)));
        self
    }

    pub fn prepend(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(PrependModifier::new(argument)));
        self
    }

    pub fn r#if<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(IfModifier::new(pipeline.build())));
        return self;
    }

    pub fn r#else<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(ElseModifier::new(pipeline.build())));
        return self;
    }

    pub fn then<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(ThenModifier::new(pipeline.build())));
        return self;
    }

    pub fn when_create<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenCreateModifier::new(pipeline.build())));
        return self;
    }

    pub fn when_update<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenUpdateModifier::new(pipeline.build())));
        return self;
    }

    pub fn is_null(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsNullModifier::new()));
        self
    }

    pub fn is_exist(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsExistModifier::new()));
        self
    }

    pub fn is_true(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsTrueModifier::new()));
        self
    }

    pub fn is_false(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsFalseModifier::new()));
        self
    }

    pub fn eq(&mut self, rhs: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(EqModifier::new(rhs)));
        self
    }

    pub fn neq(&mut self, rhs: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(NeqModifier::new(rhs)));
        self
    }

    pub fn object_value(&mut self, key: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(ObjectValueModifier::new(key)));
        self
    }

    pub fn object_previous_value(&mut self, key: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(ObjectPreviousValueModifier::new(key)));
        self
    }

    pub fn object_set_value(&mut self, key: impl Into<Argument>, value: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(ObjectSetValueModifier::new(key, value)));
        self
    }

    pub fn bcrypt_salt(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(BcryptSaltModifier::new()));
        self
    }

    pub fn bcrypt_verify<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(BcryptVerifyModifier::new(pipeline.build())));
        return self;
    }

    pub fn is_secure_password(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsSecurePasswordModifier::new()));
        self
    }

    pub fn has_length(&mut self, len: impl Into<LengthArgument>) -> &mut Self {
        self.modifiers.push(Arc::new(HasLengthModifier::new(len)));
        self
    }

    pub fn all<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        self.modifiers.push(Arc::new(AllModifier::new(build)));
        self
    }

    pub fn any<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        self.modifiers.push(Arc::new(AnyModifier::new(build)));
        self
    }

    pub fn r#do<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        self.modifiers.push(Arc::new(DoModifier::new(build)));
        self
    }

    pub fn or<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        self.modifiers.push(Arc::new(OrModifier::new(build)));
        self
    }

    pub fn and<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        self.modifiers.push(Arc::new(AndModifier::new(build)));
        self
    }

    pub fn is_object_of(&mut self, model: &'static str) -> &mut Self {
        self.modifiers.push(Arc::new(IsObjectOfModifier::new(model)));
        self
    }

    pub fn uuid(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(UUIDModifier::new()));
        self
    }

    pub fn cuid(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(CUIDModifier::new()));
        self
    }

    pub fn slug(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(SlugModifier::new()));
        self
    }

    pub fn transform<T, F>(&mut self, f: F) -> &mut Self where
        T: From<Value> + Into<Value> + Send + Sync + 'static,
        F: TransformArgument<T> + 'static {
        self.modifiers.push(Arc::new(TransformModifier::new(f)));
        self
    }

    pub fn callback<T, F>(&mut self, f: F) -> &mut Self where
        T: From<Value> + Send + Sync + 'static,
        F: CallbackArgument<T> + 'static {
        self.modifiers.push(Arc::new(CallbackModifier::new(f)));
        self
    }

    pub fn validate<T, O, F>(&mut self, f: F) -> &mut Self where
        T: From<Value> + Send + Sync + 'static,
        O: Into<Validity> + Send + Sync + 'static,
        F: ValidateArgument<T, O> + 'static {
        self.modifiers.push(Arc::new(ValidateModifier::new(f)));
        self
    }

    pub fn compare<T, O, F>(&mut self, f: F) -> &mut Self where
        T: From<Value> + Send + Sync + 'static,
        O: Into<Validity> + Send + Sync + 'static,
        F: CompareArgument<T, O> + 'static {
        self.modifiers.push(Arc::new(CompareModifier::new(f)));
        self
    }

    pub fn validate_with<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(ValidateWithModifier::new(pipeline.build())));
        return self;
    }

    pub fn transform_with<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(TransformWithModifier::new(pipeline.build())));
        return self;
    }

    pub(crate) fn build(&self) -> Pipeline {
        Pipeline { modifiers: self.modifiers.clone() }
    }
}

impl Clone for PipelineBuilder {
    fn clone(&self) -> Self {
        PipelineBuilder { modifiers: self.modifiers.clone() }
    }
}

unsafe impl Send for PipelineBuilder {}
unsafe impl Sync for PipelineBuilder {}
