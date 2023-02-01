use std::sync::Arc;
use crate::core::pipeline::modifier::Modifier;
use crate::core::pipeline::modifiers::logical::all::AllModifier;
use crate::core::pipeline::modifiers::logical::and::AndModifier;
use crate::core::pipeline::modifiers::logical::any::AnyModifier;
use crate::core::pipeline::modifiers::logical::valid::ValidModifier;
use crate::core::pipeline::modifiers::logical::invalid::InvalidModifier;
use crate::core::pipeline::modifiers::logical::not::NotModifier;
use crate::core::pipeline::modifiers::logical::or::OrModifier;
use crate::core::pipeline::modifiers::logical::passed::PassedModifier;
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
    let cond = arg0.resolved.as_ref().unwrap().as_value().unwrap();
    let mut then = None;
    let mut r#else = None;
    for (index, arg) in args.iter().enumerate() {
        if index != 0 {
            if arg.name.is_none() {
                panic!("Second and third argument of `if` should have name 'then' or 'else'.")
            }
            match arg.name.as_ref().unwrap().name.as_str() {
                "then" => {
                    then = Some(arg.resolved.as_ref().unwrap().as_value().unwrap().clone())
                }
                "else" => {
                    r#else = Some(arg.resolved.as_ref().unwrap().as_value().unwrap().clone())
                }
                _ => panic!("Second and third argument of `if` should have name 'then' or 'else'.")
            }
        }
    }
    Arc::new(IfModifier::new(cond.clone(), then, r#else))
}


pub(crate) fn all_modifier(args: Vec<Argument>) -> Arc<dyn Modifier> {
    if args.len() == 0 {
        panic!("`all` takes at least 1 argument.")
    }
    let mut pipelines = vec![];
    for arg in args.iter() {
        pipelines.push(arg.resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone());
    }
    Arc::new(AllModifier::new(pipelines))
}


pub(crate) fn any_modifier(args: Vec<Argument>) -> Arc<dyn Modifier> {
    if args.len() == 0 {
        panic!("`any` takes at least 1 argument.")
    }
    let mut pipelines = vec![];
    for arg in args.iter() {
        pipelines.push(arg.resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone());
    }
    Arc::new(AnyModifier::new(pipelines))
}

pub(crate) fn not_modifier(args: Vec<Argument>) -> Arc<dyn Modifier> {
    if args.len() != 1 {
        panic!("`not` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().clone();
    Arc::new(NotModifier::new(value))
}

pub(crate) fn passed(args: Vec<Argument>) -> Arc<dyn Modifier> {
    if args.len() != 1 {
        panic!("`passed` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone();
    Arc::new(PassedModifier::new(value))
}

pub(crate) fn and_modifier(args: Vec<Argument>) -> Arc<dyn Modifier> {
    if args.len() != 1 {
        panic!("`and` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().clone();
    Arc::new(AndModifier::new(value))
}

pub(crate) fn or_modifier(args: Vec<Argument>) -> Arc<dyn Modifier> {
    if args.len() != 1 {
        panic!("`or` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().clone();
    Arc::new(OrModifier::new(value))
}
