use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::logical::valid::ValidModifier;
use crate::core::pipeline::modifiers::logical::invalid::InvalidModifier;
use crate::core::pipeline::modifiers::logical::r#if::IfModifier;
use crate::parser::ast::argument::Argument;

pub(crate) fn valid(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(ValidModifier::new())
}

pub(crate) fn invalid(_args: Vec<Argument>) -> Arc<dyn Modifier> {
    Arc::new(InvalidModifier::new())
}

pub(crate) fn if_modifier(args: Vec<Argument>) -> Arc<dyn Modifier> {
    if args.len() == 0 {
        panic!("`if` takes at least 1 argument.")
    }
    let arg0 = args.get(0).unwrap();
    if !arg0.name.is_none() && &arg0.name.as_ref().unwrap().name != "cond" {
        panic!("First argument of `if` must be nameless or with name 'cond'.")
    }
    let cond = arg0.resolved.unwrap().as_value().unwrap();
    let mut then = None;
    let mut r#else = None;
    for (index, arg) in args.iter().enumerate() {
        if index != 0 {
            if arg.name.is_none() {
                panic!("Second and third argument of `if` should have name 'then' or 'else'.")
            }
            match arg.name.unwrap().name.as_str() {
                "then" => {
                    then = Some(arg.resolved.unwrap().as_value().unwrap().clone())
                }
                "else" => {
                    r#else = Some(arg.resolved.unwrap().as_value().unwrap().clone())
                }
                _ => panic!("Second and third argument of `if` should have name 'then' or 'else'.")
            }
        }
    }
    Arc::new(IfModifier::new(cond.clone(), then, r#else))
}
