use std::any::TypeId;
use crate::parser::ast::span::Span;

pub trait DiagnosticsLog { }

#[derive(Debug, Clone)]
pub struct DiagnosticsError {
    span: Span,
    message: String,
    source_id: usize,
}

impl DiagnosticsLog for DiagnosticsError { }

impl DiagnosticsError {
    pub fn new(span: Span, message: impl Into<String>, source_id: usize) -> Self {
        Self { span, message, source_id }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiagnosticsWarning {
    span: Span,
    message: String,
    source_id: usize,
}

impl DiagnosticsLog for DiagnosticsWarning { }

impl DiagnosticsWarning {
    pub fn new(span: Span, message: impl Into<String>, source_id: usize) -> Self {
        Self { span, message, source_id }
    }
}

#[derive(Debug)]
pub struct Diagnostics {
    errors: Vec<DiagnosticsError>,
    warnings: Vec<DiagnosticsWarning>,
}

impl Diagnostics {

    pub fn new() -> Diagnostics {
        Diagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn insert<T>(&mut self, item: T) where T: DiagnosticsLog {
        match TypeId::of::<T>() {
            TypeId::of::<DiagnosticsWarning>() => {
                self.warnings.push(item);
            }
            TypeId::of::<DiagnosticsError>() => {
                self.errors.push(item);
            }
            _ => ()
        }
    }

    pub fn insert_unparsed_rule(&mut self, span: Span, message: impl Into<String>, source_id: usize) {
        self.insert(DiagnosticsError::new(span, message.into(), source_id))
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}