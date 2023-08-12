use std::collections::HashMap;
use crate::core::r#enum::Enum;

#[derive(Debug)]
pub struct InterfaceRef {
    pub name: String,
    pub args: Vec<InterfaceRef>,
}

#[derive(Debug)]
pub struct InterfaceDefinition {
    pub name: String,
    pub args: Vec<String>,
    pub extends: Vec<InterfaceRef>,
}

#[derive(Debug)]
pub struct InterfaceField {
    pub name: String,
    pub kind: InterfaceRef,
}

#[derive(Debug)]
pub struct CustomActionDefinition {
    pub group: String,
    pub name: String,
    pub input: InterfaceRef,
    pub output: InterfaceRef,
    pub input_fields: ResolvedInterfaceField,
}

#[derive(Debug, Clone)]
pub struct ResolvedInterfaceField {
    pub field_type: ResolvedInterfaceFieldType,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub enum ResolvedInterfaceFieldType {
    ObjectId,
    Bool,
    I32,
    I64,
    F32,
    F64,
    Decimal,
    String,
    Date,
    DateTime,
    Enum(Enum),
    Vec(Box<ResolvedInterfaceField>),
    HashMap(Box<ResolvedInterfaceField>),
    Shape(HashMap<String, ResolvedInterfaceField>),
}

impl ResolvedInterfaceFieldType {
    pub fn optional(self, optional: bool) -> ResolvedInterfaceField {
        ResolvedInterfaceField { field_type: self, optional }
    }
}