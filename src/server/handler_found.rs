use teo_runtime::action::Action;
use teo_runtime::handler::Handler;
use teo_runtime::model::Model;

pub(super) enum HandlerFound<'a> {
    Custom(&'a Handler),
    Builtin(&'a Model, Action),
}
