use std::borrow::Cow;
use super::super::types::SortOrder;

#[derive(Debug)]
pub struct EnumDef {
    pub name: &'static str,
    pub variants: Vec<Cow<'static, str>>
}

#[derive(Debug)]
pub struct ColumnDef {
    pub name: Cow<'static, str>,
    pub ty: Cow<'static, str>,
    pub nullable: bool,
    pub default: Option<Cow<'static, str>>,
}

#[derive(Debug, PartialEq)]
pub struct IndexColumnDef {
    pub name: Cow<'static, str>,
    pub order: SortOrder,
}

#[derive(Debug, PartialEq)]
pub struct IndexDef {
    pub name: Cow<'static, str>,
    pub columns: Vec<IndexColumnDef>,
}

#[derive(Debug)]
pub struct TableDef {
    pub name: &'static str,
    pub columns: Vec<ColumnDef>,
    pub indexes: Vec<IndexDef>,
}
