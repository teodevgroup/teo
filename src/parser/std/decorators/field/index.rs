use crate::core::field::field::{Field, FieldIndex, FieldIndexSettings, Sort};
use crate::parser::ast::argument::Argument;

static VALID_NAMES: [&str; 3] = ["map", "length", "sort"];

static FIELD_INDEX_PRIMARY: u8 = 0;
static FIELD_INDEX_INDEX: u8 = 1;
static FIELD_INDEX_UNIQUE: u8 = 2;

pub(crate) fn id_decorator(args: Vec<Argument>, field: &mut Field) {
    decorator(args, field, FIELD_INDEX_PRIMARY)
}

pub(crate) fn unique_decorator(args: Vec<Argument>, field: &mut Field) {
    decorator(args, field, FIELD_INDEX_UNIQUE)
}

pub(crate) fn index_decorator(args: Vec<Argument>, field: &mut Field) {
    decorator(args, field, FIELD_INDEX_INDEX)
}

fn decorator(args: Vec<Argument>, field: &mut Field, index_kind: u8) {
    if index_kind == FIELD_INDEX_PRIMARY {
        field.primary = true;
    }
    let mut settings = FieldIndexSettings::default();
    for arg in args {
        if arg.name.is_none() {
            panic!("@migration requires argument name.");
        }
        if !VALID_NAMES.contains(&arg.name.as_ref().unwrap().name.as_str()) {
            panic!("Unknown argument name: {}", arg.name.as_ref().unwrap().name.as_str());
        }
        match arg.name.as_ref().unwrap().name.as_str() {
            "sort" => {
                match arg.resolved.as_ref().unwrap().as_value().unwrap().as_raw_enum_choice().unwrap() {
                    "asc" => settings.sort = Sort::Asc,
                    "desc" => settings.sort = Sort::Desc,
                    _ => unreachable!()
                }
            }
            "length" => {
                settings.length = Some(arg.resolved.as_ref().unwrap().as_value().unwrap().as_usize().unwrap());
            }
            "map" => {
                settings.name = Some(arg.resolved.as_ref().unwrap().as_value().unwrap().as_str().unwrap().to_owned())
            }
            _ => unreachable!()
        }
    }
    field.index = Some(match index_kind {
        0 => FieldIndex::Primary(settings),
        2 => FieldIndex::Unique(settings),
        1 => FieldIndex::Index(settings),
        _ => unreachable!()
    });
}
