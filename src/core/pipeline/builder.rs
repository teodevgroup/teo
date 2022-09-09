use std::future::Future;
use std::sync::{Arc};
use crate::core::argument::Argument;
use crate::core::modifier::Modifier;
use crate::core::modifiers::abs::AbsModifier;
use crate::core::modifiers::addi::AddIModifier;
use crate::core::modifiers::addf::AddFModifier;
use crate::core::modifiers::all::AllModifier;
use crate::core::modifiers::alnum::AlnumModifier;
use crate::core::modifiers::alpha::AlphaModifier;
use crate::core::modifiers::and::AndModifier;
use crate::core::modifiers::any::AnyModifier;
use crate::core::modifiers::bcrypt_salt::BcryptSaltModifier;
use crate::core::modifiers::bcrypt_verify::BcryptVerifyModifier;
use crate::core::modifiers::ceil::CeilModifier;
use crate::core::modifiers::cuid::CUIDModifier;
use crate::core::modifiers::else_p::ElsePModifier;
use crate::core::modifiers::email::EmailModifier;
use crate::core::modifiers::floor::FloorModifier;
use crate::core::modifiers::if_p::IfPModifier;
use crate::core::modifiers::is_instance_of::IsInstanceOfModifier;
use crate::core::modifiers::is_null::IsNullModifier;
use crate::core::modifiers::is_self::IsSelfModifier;
use crate::core::modifiers::length::LengthModifier;
use crate::core::modifiers::length_between::LengthBetweenModifier;
use crate::core::modifiers::now::NowModifier;
use crate::core::modifiers::object_value::ObjectValueModifier;
use crate::core::modifiers::or::OrModifier;
use crate::core::modifiers::r#do::DoModifier;
use crate::core::modifiers::regex_match::RegexMatchModifier;
use crate::core::modifiers::random_digits::RandomDigitsModifier;
use crate::core::modifiers::regex_replace::RegexReplaceModifier;
use crate::core::modifiers::secure_password::SecurePasswordModifier;
use crate::core::modifiers::slug::SlugModifier;
use crate::core::modifiers::str_append::StrAppendModifier;
use crate::core::modifiers::str_prepend::StrPrependModifier;
use crate::core::modifiers::then_p::ThenPModifier;
use crate::core::modifiers::transform::TransformModifier;
use crate::core::modifiers::uuid::UUIDModifier;

use crate::core::stage::Stage;
use crate::core::object::Object;
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

    pub(crate) async fn process(&self, mut stage: Stage, object: &Object) -> Stage {
        for modifier in &self.modifiers {
            stage = modifier.call(stage.clone(), object).await;
            match stage {
                Stage::Invalid(s) => {
                    return Stage::Invalid(s)
                }
                Stage::Value(v) => {
                    stage = Stage::Value(v);
                }
                Stage::ConditionTrue(v) => {
                    stage = Stage::ConditionTrue(v);
                }
                Stage::ConditionFalse(v) => {
                    stage = Stage::ConditionFalse(v);
                }
            }
        }
        return stage;
    }

    pub fn abs(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(AbsModifier::new()));
        return self;
    }

    pub fn addi(&mut self, addend: i128) -> &mut Self {
        self.modifiers.push(Arc::new(AddIModifier::new(addend)));
        return self;
    }

    pub fn addf(&mut self, addend: f64) -> &mut Self {
        self.modifiers.push(Arc::new(AddFModifier::new(addend)));
        return self;
    }

    pub fn alnum(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(AlnumModifier::new()));
        return self;
    }

    pub fn alpha(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(AlphaModifier::new()));
        return self;
    }

    pub fn ceil(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(CeilModifier::new()));
        return self;
    }

    pub fn floor(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(FloorModifier::new()));
        return self;
    }

    pub fn email(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(EmailModifier::new()));
        return self;
    }

    pub fn now(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(NowModifier::new()));
        return self;
    }

    pub fn random_digits(&mut self, len: usize) -> &mut Self {
        self.modifiers.push(Arc::new(RandomDigitsModifier::new(len)));
        return self;
    }

    pub fn str_append(&mut self, suffix: &'static str) -> &mut Self {
        self.modifiers.push(Arc::new(StrAppendModifier::new(suffix)));
        return self;
    }

    pub fn str_prepend(&mut self, prefix: &'static str) -> &mut Self {
        self.modifiers.push(Arc::new(StrPrependModifier::new(prefix)));
        return self;
    }

    pub fn regex_match(&mut self, regex: &'static str) -> &mut Self {
        self.modifiers.push(Arc::new(RegexMatchModifier::new(regex)));
        return self;
    }

    pub fn regex_replace(&mut self, regex: &'static str, substitute: &'static str) -> &mut Self {
        self.modifiers.push(Arc::new(RegexReplaceModifier::new(regex, substitute)));
        self
    }

    pub fn if_p<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(IfPModifier::new(pipeline.build())));
        return self;
    }

    pub fn else_p<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(ElsePModifier::new(pipeline.build())));
        return self;
    }

    pub fn then_p<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(ThenPModifier::new(pipeline.build())));
        return self;
    }

    pub fn is_null(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsNullModifier::new()));
        self
    }

    pub fn object_value(&mut self, key: &'static str) -> &mut Self {
        self.modifiers.push(Arc::new(ObjectValueModifier::new(key)));
        self
    }

    pub fn bcrypt_salt(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(BcryptSaltModifier::new()));
        self
    }

    pub fn bcrypt_verify(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(BcryptVerifyModifier::new(argument)));
        self
    }

    pub fn secure_password(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(SecurePasswordModifier::new()));
        self
    }

    pub fn length(&mut self, len: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(LengthModifier::new(len)));
        self
    }

    pub fn length_between(&mut self, min: impl Into<Argument>, max: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(LengthBetweenModifier::new(min, max)));
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

    pub fn is_self(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsSelfModifier::new()));
        self
    }

    pub fn is_instance_of(&mut self, model_name: &'static str) -> &mut Self {
        self.modifiers.push(Arc::new(IsInstanceOfModifier::new(model_name)));
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

    pub fn transform<F, I, O, Fut>(&mut self, f: &'static F) -> &mut Self where
        F: Fn(I) -> Fut + Sync + Send + 'static,
        I: From<Value> + Send + Sync,
        O: Into<Value>,
        Fut: Future<Output = O> + Send + Sync {
        self.modifiers.push(Arc::new(TransformModifier::new(f)));
        self
    }

    // pub fn compare<F, I, Fut>(&mut self, f: &'static F) -> &mut Self where
    //     F: Fn(I, I) -> Fut + Sync + Send + 'static,
    //     I: From<Value> + Send + Sync,
    //     Fut: Future<Output = Result<(), ActionError>> + Send + Sync {
    //     self.modifiers.push(Arc::new(CompareModifier::new(f)));
    //     self
    // }

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
