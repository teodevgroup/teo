use std::sync::Arc;
use crate::core::item::Item;
use crate::core::items::logical::all::AllItem;
use crate::core::items::logical::and::AndItem;
use crate::core::items::logical::any::AnyItem;
use crate::core::items::logical::valid::ValidItem;
use crate::core::items::logical::invalid::InvalidItem;
use crate::core::items::logical::not::NotItem;
use crate::core::items::logical::or::OrItem;
use crate::core::items::logical::passed::PassedItem;
use crate::core::items::logical::r#if::IfItem;
use crate::parser::ast::argument::Argument;

pub(crate) fn valid(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(ValidItem::new())
}

pub(crate) fn invalid(_args: &Vec<Argument>) -> Arc<dyn Item> {
    Arc::new(InvalidItem::new())
}

pub(crate) fn if_modifier(args: &Vec<Argument>) -> Arc<dyn Item> {
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
    Arc::new(IfItem::new(cond.clone(), then, r#else))
}


pub(crate) fn all_modifier(args: &Vec<Argument>) -> Arc<dyn Item> {
    if args.len() == 0 {
        panic!("`all` takes at least 1 argument.")
    }
    let mut pipelines = vec![];
    for arg in args.iter() {
        pipelines.push(arg.resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone());
    }
    Arc::new(AllItem::new(pipelines))
}


pub(crate) fn any_modifier(args: &Vec<Argument>) -> Arc<dyn Item> {
    if args.len() == 0 {
        panic!("`any` takes at least 1 argument.")
    }
    let mut pipelines = vec![];
    for arg in args.iter() {
        pipelines.push(arg.resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone());
    }
    Arc::new(AnyItem::new(pipelines))
}

pub(crate) fn not_modifier(args: &Vec<Argument>) -> Arc<dyn Item> {
    if args.len() != 1 {
        panic!("`not` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().clone();
    Arc::new(NotItem::new(value))
}

pub(crate) fn passed(args: &Vec<Argument>) -> Arc<dyn Item> {
    if args.len() != 1 {
        panic!("`passed` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().as_pipeline().unwrap().clone();
    Arc::new(PassedItem::new(value))
}

pub(crate) fn and_modifier(args: &Vec<Argument>) -> Arc<dyn Item> {
    if args.len() != 1 {
        panic!("`and` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().clone();
    Arc::new(AndItem::new(value))
}

pub(crate) fn or_modifier(args: &Vec<Argument>) -> Arc<dyn Item> {
    if args.len() != 1 {
        panic!("`or` takes exactly 1 argument.")
    }
    let value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap().clone();
    Arc::new(OrItem::new(value))
}
