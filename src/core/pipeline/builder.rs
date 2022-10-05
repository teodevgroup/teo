use std::sync::{Arc};
use crate::core::pipeline::argument::Argument;
use crate::core::pipeline::context::validity::Validity;
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
use crate::core::pipeline::modifiers::array::get_length::GetLengthModifier;
use crate::core::pipeline::modifiers::array::has_length::{HasLengthModifier, LengthArgument};
use crate::core::pipeline::modifiers::array::prepend::PrependModifier;
use crate::core::pipeline::modifiers::array::reverse::ReverseModifier;
use crate::core::pipeline::modifiers::function::perform::{PerformArgument, PerformModifier};
use crate::core::pipeline::modifiers::function::compare::{CompareArgument, CompareModifier};
use crate::core::pipeline::modifiers::function::transform::{TransformArgument, TransformModifier};
use crate::core::pipeline::modifiers::function::validate::{ValidateArgument, ValidateModifier};
use crate::core::pipeline::modifiers::logical::any::AnyModifier;
use crate::core::pipeline::modifiers::logical::not::NotModifier;
use crate::core::pipeline::modifiers::logical::transform_with::TransformWithModifier;
use crate::core::pipeline::modifiers::logical::validate_with::ValidateWithModifier;
use crate::core::pipeline::modifiers::math::abs::AbsModifier;
use crate::core::pipeline::modifiers::math::add::AddModifier;
use crate::core::pipeline::modifiers::math::cbrt::CbrtModifier;
use crate::core::pipeline::modifiers::math::divide::DivideModifier;
use crate::core::pipeline::modifiers::math::max::MaxModifier;
use crate::core::pipeline::modifiers::math::min::MinModifier;
use crate::core::pipeline::modifiers::math::modular::ModularModifier;
use crate::core::pipeline::modifiers::math::multiply::MultiplyModifier;
use crate::core::pipeline::modifiers::math::pow::PowModifier;
use crate::core::pipeline::modifiers::math::root::RootModifier;
use crate::core::pipeline::modifiers::math::round::RoundModifier;
use crate::core::pipeline::modifiers::math::sqrt::SqrtModifier;
use crate::core::pipeline::modifiers::math::subtract::SubtractModifier;
use crate::core::pipeline::modifiers::object::is_instance_of::IsObjectOfModifier;
use crate::core::pipeline::modifiers::object::object_previous_value::ObjectPreviousValueModifier;
use crate::core::pipeline::modifiers::object::object_set_value::ObjectSetValueModifier;
use crate::core::pipeline::modifiers::intent::when_create::WhenCreateModifier;
use crate::core::pipeline::modifiers::intent::when_many_results::WhenManyResultsModifier;
use crate::core::pipeline::modifiers::intent::when_nested_many_results::WhenNestedManyResultsModifier;
use crate::core::pipeline::modifiers::intent::when_nested_single_result::WhenNestedSingleResultModifier;
use crate::core::pipeline::modifiers::intent::when_single_result::WhenSingleResultModifier;
use crate::core::pipeline::modifiers::intent::when_update::WhenUpdateModifier;
use crate::core::pipeline::modifiers::object::get_object::GetObjectModifier;
use crate::core::pipeline::modifiers::string::generation::random_digits::RandomDigitsModifier;
use crate::core::pipeline::modifiers::string::transform::ellipsis::EllipsisModifier;
use crate::core::pipeline::modifiers::string::transform::pad_end::PadEndModifier;
use crate::core::pipeline::modifiers::string::transform::pad_start::PadStartModifier;
use crate::core::pipeline::modifiers::string::transform::split::SplitModifier;
use crate::core::pipeline::modifiers::string::validation::has_prefix::HasPrefixModifier;
use crate::core::pipeline::modifiers::string::validation::has_suffix::HasSuffixModifier;
use crate::core::pipeline::modifiers::string::validation::is_alphabetic::IsAlphabeticModifier;
use crate::core::pipeline::modifiers::string::validation::is_alphanumeric::IsAlphanumericModifier;
use crate::core::pipeline::modifiers::string::validation::is_email::IsEmailModifier;
use crate::core::pipeline::modifiers::string::validation::is_hex_color::IsHexColorModifier;
use crate::core::pipeline::modifiers::string::validation::is_numeric::IsNumericModifier;
use crate::core::pipeline::modifiers::string::validation::is_prefix_of::IsPrefixOfModifier;
use crate::core::pipeline::modifiers::string::validation::is_secure_password::IsSecurePasswordModifier;
use crate::core::pipeline::modifiers::string::validation::is_suffix_of::IsSuffixOfModifier;
use crate::core::pipeline::modifiers::string::validation::regex_match::RegexMatchModifier;
use crate::core::pipeline::modifiers::value::eq::EqModifier;
use crate::core::pipeline::modifiers::value::is_exist::IsExistModifier;
use crate::core::pipeline::modifiers::value::is_false::IsFalseModifier;
use crate::core::pipeline::modifiers::value::is_null::IsNullModifier;
use crate::core::pipeline::modifiers::value::is_true::IsTrueModifier;
use crate::core::pipeline::modifiers::value::neq::NeqModifier;
use crate::core::pipeline::modifiers::value::gt::GtModifier;
use crate::core::pipeline::modifiers::value::gte::GteModifier;
use crate::core::pipeline::modifiers::value::lt::LtModifier;
use crate::core::pipeline::modifiers::value::lte::LteModifier;
use crate::core::pipeline::modifiers::value::one_of::OneOfModifier;
use crate::core::pipeline::modifiers::vector::item_at::ItemAtModifier;
use crate::core::pipeline::modifiers::vector::join::JoinModifier;
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
        self
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
        self
    }

    pub fn floor(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(FloorModifier::new()));
        self
    }
    
    pub fn round(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(RoundModifier::new()));
        self
    }

    pub fn pow(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(PowModifier::new(argument)));
        self
    }

    pub fn root(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(RootModifier::new(argument)));
        self
    }

    pub fn sqrt(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(SqrtModifier::new()));
        self
    }

    pub fn cbrt(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(CbrtModifier::new()));
        self
    }

    pub fn min(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(MinModifier::new(argument)));
        self
    }

    pub fn max(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(MaxModifier::new(argument)));
        self
    }

    pub fn is_alphabetic(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsAlphabeticModifier::new()));
        self
    }

    pub fn is_numeric(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsNumericModifier::new()));
        self
    }

    pub fn is_alphanumeric(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsAlphanumericModifier::new()));
        self
    }

    pub fn is_email(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsEmailModifier::new()));
        self
    }

    pub fn regex_match(&mut self, regex: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(RegexMatchModifier::new(regex)));
        self
    }

    pub fn regex_replace(&mut self, regex: impl Into<Argument>, substitute: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(RegexReplaceModifier::new(regex, substitute)));
        self
    }

    pub fn ellipsis(&mut self, ellipsis: impl Into<String>, length: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(EllipsisModifier::new(ellipsis, length)));
        self
    }

    pub fn pad_start(&mut self, char: char, width: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(PadStartModifier::new(char, width)));
        self
    }

    pub fn pad_end(&mut self, char: char, width: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(PadEndModifier::new(char, width)));
        self
    }

    pub fn random_digits(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(RandomDigitsModifier::new(argument)));
        self
    }

    pub fn now(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(NowModifier::new()));
        self
    }

    pub fn append(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(AppendModifier::new(argument)));
        self
    }

    pub fn prepend(&mut self, argument: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(PrependModifier::new(argument)));
        self
    }

    pub fn not<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(NotModifier::new(pipeline.build())));
        self
    }

    pub fn r#if<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(IfModifier::new(pipeline.build())));
        self
    }

    pub fn r#else<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(ElseModifier::new(pipeline.build())));
        self
    }

    pub fn then<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(ThenModifier::new(pipeline.build())));
        self
    }

    pub fn when_create<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenCreateModifier::new(pipeline.build())));
        self
    }

    pub fn when_update<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenUpdateModifier::new(pipeline.build())));
        self
    }

    pub fn when_many_results<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenManyResultsModifier::new(pipeline.build())));
        self
    }

    pub fn when_single_result<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenSingleResultModifier::new(pipeline.build())));
        self
    }

    pub fn when_nested_many_results<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenNestedManyResultsModifier::new(pipeline.build())));
        self
    }

    pub fn when_nested_single_result<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(WhenNestedSingleResultModifier::new(pipeline.build())));
        self
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

    pub fn gt(&mut self, rhs: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(GtModifier::new(rhs)));
        self
    }

    pub fn gte(&mut self, rhs: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(GteModifier::new(rhs)));
        self
    }

    pub fn lt(&mut self, rhs: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(LtModifier::new(rhs)));
        self
    }

    pub fn lte(&mut self, rhs: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(LteModifier::new(rhs)));
        self
    }

    pub fn one_of(&mut self, choices: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(OneOfModifier::new(choices)));
        self
    }

    pub fn get_object(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(GetObjectModifier::new()));
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
        self
    }

    pub fn is_secure_password(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsSecurePasswordModifier::new()));
        self
    }

    pub fn has_length(&mut self, len: impl Into<LengthArgument>) -> &mut Self {
        self.modifiers.push(Arc::new(HasLengthModifier::new(len)));
        self
    }

    pub fn get_length(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(GetLengthModifier::new()));
        self
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(ReverseModifier::new()));
        self
    }

    pub fn split(&mut self, separator: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(SplitModifier::new(separator)));
        self
    }

    pub fn join(&mut self, separator: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(JoinModifier::new(separator)));
        self
    }

    pub fn item_at(&mut self, index: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(ItemAtModifier::new(index)));
        self
    }

    pub fn has_prefix(&mut self, prefix: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(HasPrefixModifier::new(prefix)));
        self
    }

    pub fn has_suffix(&mut self, prefix: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(HasSuffixModifier::new(prefix)));
        self
    }

    pub fn is_prefix_of(&mut self, prefix: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(IsPrefixOfModifier::new(prefix)));
        self
    }

    pub fn is_suffix_of(&mut self, prefix: impl Into<Argument>) -> &mut Self {
        self.modifiers.push(Arc::new(IsSuffixOfModifier::new(prefix)));
        self
    }

    pub fn is_hex_color(&mut self) -> &mut Self {
        self.modifiers.push(Arc::new(IsHexColorModifier::new()));
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

    pub fn is_object_of(&mut self, model: impl Into<String>) -> &mut Self {
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

    pub fn perform<T, F>(&mut self, f: F) -> &mut Self where
        T: From<Value> + Send + Sync + 'static,
        F: PerformArgument<T> + 'static {
        self.modifiers.push(Arc::new(PerformModifier::new(f)));
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
        self
    }

    pub fn transform_with<F: Fn(&mut PipelineBuilder)>(&mut self, build: F) -> &mut Self {
        let mut pipeline = PipelineBuilder::new();
        build(&mut pipeline);
        self.modifiers.push(Arc::new(TransformWithModifier::new(pipeline.build())));
        self
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
