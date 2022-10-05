use crate::core::action::Action;
use crate::core::action::r#type::ActionType;
use crate::core::env::position::Position;
use crate::core::env::source::Source;
use crate::core::env::source::Source::CustomCode;

pub(crate) mod source;
pub(crate) mod position;

#[derive(Clone)]
pub(crate) struct Env {
    source: Source,
    trigger: Source,
    action: Option<ActionType>,
    position: Option<Position>,
}

impl Env {

    pub(crate) fn new(source: Source, action: ActionType, position: Position) -> Self {
        Self {
            source: source.clone(),
            trigger: source,
            action: Some(action),
            position: Some(position),
        }
    }

    pub(crate) fn custom_code() -> Self {
        Self {
            source: CustomCode,
            trigger: CustomCode,
            action: None,
            position: None,
        }
    }

    pub(crate) fn alter_position(&self, position: Position) -> Self {
        Self {
            source: self.source.clone(),
            trigger: self.trigger.clone(),
            action: self.action,
            position: Some(position),
        }
    }

    pub(crate) fn alter_trigger(&self, trigger: Source) -> Self {
        Self {
            source: self.source.clone(),
            trigger,
            action: self.action,
            position: self.position,
        }
    }

    pub(crate) fn source(&self) -> &Source {
        &self.source
    }

    pub(crate) fn trigger(&self) -> &Source {
        &self.trigger
    }

    pub(crate) fn action(&self) -> Option<&ActionType> {
        self.action.as_ref()
    }

    pub(crate) fn position(&self) -> Option<&Position> {
        self.position.as_ref()
    }
}
