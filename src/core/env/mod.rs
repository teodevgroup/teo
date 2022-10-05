use crate::core::action::Action;
use crate::core::action::r#type::ActionType;
use crate::core::env::intent::Intent;
use crate::core::env::position::Position;
use crate::core::env::source::Source;
use crate::core::env::source::Source::CustomCode;

pub(crate) mod source;
pub(crate) mod position;
pub(crate) mod intent;

#[derive(Clone)]
pub(crate) struct Env {
    source: Source,
    trigger: Source,
    intent: Option<Intent>,
    position: Option<Position>,
}

impl Env {

    pub(crate) fn new(source: Source, intent: Intent, position: Position) -> Self {
        Self {
            source: source.clone(),
            trigger: source,
            intent: Some(intent),
            position: Some(position),
        }
    }

    pub(crate) fn custom_code() -> Self {
        Self {
            source: CustomCode,
            trigger: CustomCode,
            intent: None,
            position: None,
        }
    }

    pub(crate) fn nested(&self, intent: Intent, position: Position) -> Self {
        Self {
            source: self.source.clone(),
            trigger: self.trigger.clone(),
            intent: Some(intent),
            position: Some(position),
        }
    }

    pub(crate) fn alter_position(&self, position: Position) -> Self {
        Self {
            source: self.source.clone(),
            trigger: self.trigger.clone(),
            intent: self.intent,
            position: Some(position),
        }
    }

    pub(crate) fn alter_trigger(&self, trigger: Source) -> Self {
        Self {
            source: self.source.clone(),
            trigger,
            intent: self.intent,
            position: self.position,
        }
    }

    pub(crate) fn source(&self) -> &Source {
        &self.source
    }

    pub(crate) fn trigger(&self) -> &Source {
        &self.trigger
    }

    pub(crate) fn intent(&self) -> Option<Intent> {
        self.intent
    }

    pub(crate) fn position(&self) -> Option<Position> {
        self.position
    }
}
