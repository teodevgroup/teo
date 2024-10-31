use teo_runtime::action::Action;
use teo_runtime::handler::action::builtin_action_handler_from_name;
use teo_runtime::handler::Handler;
use teo_runtime::handler::r#match::HandlerMatch;
use teo_runtime::model::Model;
use teo_runtime::namespace::Namespace;

pub(super) enum HandlerFound<'a> {
    Custom(&'a Handler),
    Builtin(&'a Model, Action),
}

pub(super) fn find_handler<'a>(main_namespace: &'static Namespace, match_result: &'a HandlerMatch) -> Option<(&'static Namespace, HandlerFound<'a>)> {
    let mut group = false;
    let dest_namespace = if let Some(d) = main_namespace.namespace_at_path(&match_result.path()) {
        d
    } else if match_result.path().len() > 0 {
        if let Some(d) = main_namespace.namespace_at_path(&match_result.path_without_last()) {
            group = true;
            d
        } else {
            None?
        }
    } else {
        None?
    };
    let handler_resolved = if group {
        if let Some(model) = dest_namespace.models().get(match_result.group_name()) {
            if let Some(group) = dest_namespace.model_handler_groups().get(match_result.group_name()) {
                if let Some(handler) = group.handlers().get(match_result.handler_name()) {
                    (dest_namespace, HandlerFound::Custom(handler))
                } else {
                    if let Some(action) = builtin_action_handler_from_name(match_result.handler_name()) {
                        (dest_namespace, HandlerFound::Builtin(model, action))
                    } else {
                        None?
                    }
                }
            } else {
                if let Some(action) = builtin_action_handler_from_name(match_result.handler_name()) {
                    (dest_namespace, HandlerFound::Builtin(model, action))
                } else {
                    None?
                }
            }
        } else if let Some(group) = dest_namespace.handler_groups().get(match_result.group_name()) {
            if let Some(handler) = group.handlers().get(match_result.handler_name()) {
                (dest_namespace, HandlerFound::Custom(handler))
            } else {
                None?
            }
        } else {
            None?
        }
    } else {
        if let Some(handler) = dest_namespace.handlers().get(match_result.handler_name()) {
            (dest_namespace, HandlerFound::Custom(handler))
        } else {
            None?
        }
    };
    Some(handler_resolved)
}