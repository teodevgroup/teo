use std::any::TypeId;
use std::path::{Path, PathBuf};
use crate::parser::ast::span::Span;

pub trait DiagnosticsLog {
    fn span(&self) -> &Span;

    fn message(&self) -> &str;

    fn source_path(&self) -> &Path;

    fn into_warning(self) -> DiagnosticsWarning;

    fn into_error(self) -> DiagnosticsError;

    fn is_warning(&self) -> bool;

    fn is_error(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct DiagnosticsError {
    span: Span,
    message: String,
    source_path: PathBuf,
}

impl DiagnosticsLog for DiagnosticsError {
    fn span(&self) -> &Span {
        &self.span
    }

    fn message(&self) -> &str {
        self.message.as_str()
    }

    fn source_path(&self) -> &Path {
        &self.source_path
    }

    fn into_warning(self) -> DiagnosticsWarning {
        unreachable!()
    }

    fn into_error(self) -> DiagnosticsError {
        self
    }

    fn is_warning(&self) -> bool {
        false
    }

    fn is_error(&self) -> bool {
        true
    }
}

impl DiagnosticsLog for &DiagnosticsError {

    fn span(&self) -> &Span {
        &self.span
    }

    fn message(&self) -> &str {
        self.message.as_str()
    }

    fn source_path(&self) -> &Path {
        &self.source_path
    }

    fn into_warning(self) -> DiagnosticsWarning {
        unreachable!()
    }

    fn into_error(self) -> DiagnosticsError {
        self.clone()
    }

    fn is_warning(&self) -> bool {
        false
    }

    fn is_error(&self) -> bool {
        true
    }
}

impl DiagnosticsError {
    pub fn new(span: Span, message: impl Into<String>, source_path: PathBuf) -> Self {
        Self { span, message: message.into(), source_path }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DiagnosticsWarning {
    span: Span,
    message: String,
    source_path: PathBuf,
}

impl DiagnosticsLog for DiagnosticsWarning {
    fn span(&self) -> &Span {
        &self.span
    }

    fn message(&self) -> &str {
        self.message.as_str()
    }

    fn source_path(&self) -> &Path {
        &self.source_path
    }

    fn into_warning(self) -> DiagnosticsWarning {
        self
    }

    fn into_error(self) -> DiagnosticsError {
        unreachable!()
    }

    fn is_warning(&self) -> bool {
        true
    }

    fn is_error(&self) -> bool {
        false
    }
}

impl DiagnosticsLog for &DiagnosticsWarning {
    fn span(&self) -> &Span {
        &self.span
    }

    fn message(&self) -> &str {
        self.message.as_str()
    }

    fn source_path(&self) -> &Path {
        &self.source_path
    }

    fn into_warning(self) -> DiagnosticsWarning {
        self.clone()
    }

    fn into_error(self) -> DiagnosticsError {
        unreachable!()
    }

    fn is_warning(&self) -> bool {
        true
    }

    fn is_error(&self) -> bool {
        false
    }
}

impl DiagnosticsWarning {
    pub fn new(span: Span, message: impl Into<String>, source_path: PathBuf) -> Self {
        Self { span, message: message.into(), source_path }
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

    pub fn warnings(&self) -> &Vec<DiagnosticsWarning> {
        &self.warnings
    }

    pub fn errors(&self) -> &Vec<DiagnosticsError> {
        &self.errors
    }

    pub fn insert<T>(&mut self, item: T) where T: DiagnosticsLog + 'static {
        if TypeId::of::<T>() == TypeId::of::<DiagnosticsWarning>() {
            self.warnings.push(item.into_warning());
        } else if TypeId::of::<T>() == TypeId::of::<DiagnosticsError>() {
            self.errors.push(item.into_error());
        }
    }

    pub fn insert_unparsed_rule(&mut self, span: Span, source_path: PathBuf) {
        self.insert(DiagnosticsError::new(span, "SyntaxError: Unexpected content.", source_path))
    }

    pub fn insert_unresolved_model(&mut self, span: Span, source_path: PathBuf) {
        self.insert(DiagnosticsError::new(span, "ResolvingError: Model is not defined.", source_path))
    }

    pub fn insert_unresolved_enum(&mut self, span: Span, source_path: PathBuf) {
        self.insert(DiagnosticsError::new(span, "ResolvingError: Type is not defined.", source_path))
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}