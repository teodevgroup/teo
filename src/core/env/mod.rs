

use crate::core::env::intent::Intent;
use crate::core::env::source::Source;
use crate::core::env::source::Source::CustomCode;

pub(crate) mod source;
pub(crate) mod intent;

#[derive(Clone)]
pub struct Env {
    source: Source,
    trigger: Source,
    intent: Option<Intent>,
}

impl Env {

    pub(crate) fn new(source: Source, intent: Intent) -> Self {
        Self {
            source: source.clone(),
            trigger: source,
            intent: Some(intent),
        }
    }

    pub(crate) fn custom_code() -> Self {
        Self {
            source: CustomCode,
            trigger: CustomCode,
            intent: None,
        }
    }

    pub(crate) fn nested(&self, intent: Intent) -> Self {
        Self {
            source: self.source.clone(),
            trigger: self.trigger.clone(),
            intent: Some(intent),
        }
    }

    pub(crate) fn alter_position(&self) -> Self {
        Self {
            source: self.source.clone(),
            trigger: self.trigger.clone(),
            intent: self.intent,
        }
    }

    pub(crate) fn alter_trigger(&self, trigger: Source) -> Self {
        Self {
            source: self.source.clone(),
            trigger,
            intent: self.intent,
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

}
