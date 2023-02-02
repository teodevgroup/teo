use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;

use crate::core::pipeline::modifiers::intent::when_create::WhenCreateModifier;
use crate::core::pipeline::modifiers::intent::when_many_results::WhenManyResultsModifier;
use crate::core::pipeline::modifiers::intent::when_nested_many_results::WhenNestedManyResultsModifier;
use crate::core::pipeline::modifiers::intent::when_nested_single_result::WhenNestedSingleResultModifier;
use crate::core::pipeline::modifiers::intent::when_single_result::WhenSingleResultModifier;
use crate::core::pipeline::modifiers::intent::when_update::WhenUpdateModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn when(args: Vec<Argument>) -> Arc<dyn Modifier> {
    let intent = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap();
    let pipeline = args.get(1).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap();
    match intent {
        "create" => Arc::new(WhenCreateModifier::new(pipeline.clone())),
        "update" => Arc::new(WhenUpdateModifier::new(pipeline.clone())),
        "manyResults" => Arc::new(WhenManyResultsModifier::new(pipeline.clone())),
        "singleResult" => Arc::new(WhenSingleResultModifier::new(pipeline.clone())),
        "nestedManyResults" => Arc::new(WhenNestedManyResultsModifier::new(pipeline.clone())),
        "nestedSingleResult" => Arc::new(WhenNestedSingleResultModifier::new(pipeline.clone())),
        _ => panic!(),
    }
}
