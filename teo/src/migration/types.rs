use std::borrow::Cow;
use super::super::types::SortOrder;

#[derive(Debug)]
pub struct EnumDef {
    pub name: &'static str,
    pub variants: Vec<Cow<'static, str>>
}

#[derive(Debug, PartialEq)]
pub struct ColumnDef<T> {
    pub name: Cow<'static, str>,
    pub ty: T,
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
pub struct TableDef<T> {
    pub name: &'static str,
    pub columns: Vec<ColumnDef<T>>,
    pub indexes: Vec<IndexDef>,
}
