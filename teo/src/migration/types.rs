use std::borrow::Cow;
#[cfg(feature = "mongodb")]
use serde::{Deserialize, Serialize};

use super::super::types::SortOrder;

#[derive(Debug)]
pub struct EnumDef {
    pub name: &'static str,
    pub variants: Vec<Cow<'static, str>>
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "mongodb", derive(Serialize, Deserialize))]
pub struct ColumnDef<T> {
    pub name: Cow<'static, str>,
    pub ty: T,
    pub nullable: bool,
    pub default: Option<Cow<'static, str>>,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "mongodb", derive(Serialize, Deserialize))]
pub struct IndexColumnDef {
    pub name: Cow<'static, str>,
    pub order: SortOrder,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "mongodb", derive(Serialize, Deserialize))]
pub struct IndexDef {
    pub name: Cow<'static, str>,
    pub columns: Vec<IndexColumnDef>,
}

#[derive(Debug)]
#[cfg_attr(feature = "mongodb", derive(Serialize, Deserialize))]
pub struct TableDef<T> {
    pub name: Cow<'static, str>,
    pub columns: Vec<ColumnDef<T>>,
    pub indexes: Vec<IndexDef>,
}
