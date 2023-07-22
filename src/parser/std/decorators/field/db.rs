use crate::core::database::name::DatabaseName;
use crate::core::database::r#type::DatabaseType;
use crate::core::field::field::Field;
use crate::parser::ast::accessible::{FieldDecorator};
use crate::parser::ast::argument::Argument;
use crate::prelude::Value;

pub(crate) fn db_decorator(database_name: DatabaseName) -> FieldDecorator {
    match database_name {
        DatabaseName::MySQL => {
            mysql_db_types_decorator
        }
        DatabaseName::PostgreSQL => {
            postgresql_db_types_decorator
        }
        DatabaseName::SQLite => {
            sqlite_db_types_decorator
        }
        DatabaseName::MongoDB => {
            mongodb_db_types_decorator
        }
    }
}

pub(crate) fn mysql_db_types_decorator(args: &Vec<Argument>, field: &mut Field) {
    let arg_value = args.get(0).unwrap().resolved.as_ref().unwrap().as_value().unwrap();
    let (name, args) = match arg_value {
        Value::RawEnumChoice(name, args) => {
            (name, args)
        }
        _ => panic!("Argument to @db decorator should be enum member.")
    };
    match name.as_str() {
        "text" => field.database_type = Some(DatabaseType::Text { m: None, n: None, c: None }),
        "varChar" => {
            let first_arg = args.as_ref().unwrap().get(0).unwrap();
            field.database_type = Some(DatabaseType::VarChar { m: first_arg.1.as_usize().unwrap() as u16, n: None, c: None })
        }
        "char" => {
            let first_arg = args.as_ref().unwrap().get(0).unwrap();
            field.database_type = Some(DatabaseType::Char { m: Some(first_arg.1.as_usize().unwrap() as u8), n: None, c: None })
        }
        "mediumText" => field.database_type = Some(DatabaseType::MediumText { n: None, c: None }),
        "longText" => field.database_type = Some(DatabaseType::LongText { n: None, c: None }),
        "int" => field.database_type = Some(DatabaseType::Int),
        "smallInt" => field.database_type = Some(DatabaseType::SmallInt { m: None, u: false }),
        "unsignedSmallInt" => field.database_type = Some(DatabaseType::SmallInt { m: None, u: true }),
        "mediumInt" => field.database_type = Some(DatabaseType::MediumInt { m: None, u: false }),
        "unsignedMediumInt" => field.database_type = Some(DatabaseType::MediumInt { m: None, u: true }),
        "bigInt" => field.database_type = Some(DatabaseType::BigInt { m: None, u: false }),
        "unsignedBigInt" => field.database_type = Some(DatabaseType::BigInt { m: None, u: true }),
        "float" => field.database_type = Some(DatabaseType::Float { m: None, d: None }),
        "double" => field.database_type = Some(DatabaseType::Double { m: None, d: None }),
        _ => panic!("Unrecognized db column type: {name} in @db decorator."),
    }
}

pub(crate) fn postgresql_db_types_decorator(args: &Vec<Argument>, field: &mut Field) {

}

pub(crate) fn sqlite_db_types_decorator(args: &Vec<Argument>, field: &mut Field) {

}

pub(crate) fn mongodb_db_types_decorator(args: &Vec<Argument>, field: &mut Field) {

}