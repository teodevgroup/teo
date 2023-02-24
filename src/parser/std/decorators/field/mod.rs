pub(crate) mod id;
pub(crate) mod db;
pub(crate) mod readonly;
pub(crate) mod writeonly;
pub(crate) mod readwrite;
pub(crate) mod internal;
pub(crate) mod write_on_create;
pub(crate) mod write_once;
pub(crate) mod write_nonnull;
pub(crate) mod read_if;
pub(crate) mod write_if;
pub(crate) mod atomic;
pub(crate) mod nonatomic;
pub(crate) mod map;
pub(crate) mod index;
pub(crate) mod unique;
pub(crate) mod present_with;
pub(crate) mod present_without;
pub(crate) mod present_if;
pub(crate) mod r#virtual;
pub(crate) mod record_previous;
pub(crate) mod input_omissible;
pub(crate) mod output_omissible;
pub(crate) mod auto;
pub(crate) mod auto_increment;
pub(crate) mod default;
pub(crate) mod foreign_key;
pub(crate) mod on_set;
pub(crate) mod on_save;
pub(crate) mod on_output;
pub(crate) mod auth_identity;
pub(crate) mod auth_by;
pub(crate) mod queryable;
pub(crate) mod unqueryable;
pub(crate) mod can_read;
pub(crate) mod can_mutate;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::core::database::name::DatabaseName;
use crate::parser::ast::accessible::Accessible;
use crate::parser::std::decorators::field::atomic::{atomic_decorator};
use crate::parser::std::decorators::field::auth_by::auth_by_decorator;
use crate::parser::std::decorators::field::auth_identity::auth_identity_decorator;
use crate::parser::std::decorators::field::auto::auto_decorator;
use crate::parser::std::decorators::field::auto_increment::auto_increment_decorator;
use crate::parser::std::decorators::field::can_mutate::can_mutate_decorator;
use crate::parser::std::decorators::field::can_read::can_read_decorator;
use crate::parser::std::decorators::field::db::db_container;
use crate::parser::std::decorators::field::default::default_decorator;
use crate::parser::std::decorators::field::foreign_key::foreign_key_decorator;
use crate::parser::std::decorators::field::id::{id_decorator};
use crate::parser::std::decorators::field::index::index_decorator;
use crate::parser::std::decorators::field::input_omissible::input_omissible_decorator;
use crate::parser::std::decorators::field::internal::{internal_decorator};
use crate::parser::std::decorators::field::map::map_decorator;
use crate::parser::std::decorators::field::nonatomic::{nonatomic_decorator};
use crate::parser::std::decorators::field::on_output::on_output_decorator;
use crate::parser::std::decorators::field::on_save::on_save_decorator;
use crate::parser::std::decorators::field::on_set::on_set_decorator;
use crate::parser::std::decorators::field::output_omissible::output_omissible_decorator;
use crate::parser::std::decorators::field::present_if::present_if_decorator;
use crate::parser::std::decorators::field::present_with::present_with_decorator;
use crate::parser::std::decorators::field::present_without::present_without_decorator;
use crate::parser::std::decorators::field::queryable::queryable_decorator;
use crate::parser::std::decorators::field::read_if::read_if_decorator;
use crate::parser::std::decorators::field::readonly::{readonly_decorator};
use crate::parser::std::decorators::field::readwrite::{readwrite_decorator};
use crate::parser::std::decorators::field::unique::unique_decorator;
use crate::parser::std::decorators::field::write_if::write_if_decorator;
use crate::parser::std::decorators::field::write_nonnull::write_nonnull_decorator;
use crate::parser::std::decorators::field::write_on_create::write_on_create_decorator;
use crate::parser::std::decorators::field::write_once::write_once_decorator;
use crate::parser::std::decorators::field::writeonly::{writeonly_decorator};
use crate::parser::std::decorators::field::r#virtual::virtual_decorator;
use crate::parser::std::decorators::field::record_previous::record_previous_decorator;
use crate::parser::std::decorators::field::unqueryable::unqueryable_decorator;


pub(crate) struct GlobalFieldDecorators {
    objects: HashMap<String, Accessible>
}

impl Debug for GlobalFieldDecorators {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlobalFieldDecorators")
    }
}

impl GlobalFieldDecorators {

    pub(crate) fn new(database_name: DatabaseName) -> Self {
        let mut objects: HashMap<String, Accessible> = HashMap::new();
        objects.insert("db".to_owned(), Accessible::Container(db_container(database_name)));
        objects.insert("id".to_owned(), Accessible::FieldDecorator(id_decorator));
        objects.insert("map".to_owned(), Accessible::FieldDecorator(map_decorator));
        objects.insert("unique".to_owned(), Accessible::FieldDecorator(unique_decorator));
        objects.insert("index".to_owned(), Accessible::FieldDecorator(index_decorator));
        objects.insert("internal".to_owned(), Accessible::FieldDecorator(internal_decorator));
        objects.insert("readonly".to_owned(), Accessible::FieldDecorator(readonly_decorator));
        objects.insert("writeonly".to_owned(), Accessible::FieldDecorator(writeonly_decorator));
        objects.insert("readwrite".to_owned(), Accessible::FieldDecorator(readwrite_decorator));
        objects.insert("writeNonnull".to_owned(), Accessible::FieldDecorator(write_nonnull_decorator));
        objects.insert("writeOnCreate".to_owned(), Accessible::FieldDecorator(write_on_create_decorator));
        objects.insert("writeOnce".to_owned(), Accessible::FieldDecorator(write_once_decorator));
        objects.insert("readIf".to_owned(), Accessible::FieldDecorator(read_if_decorator));
        objects.insert("writeIf".to_owned(), Accessible::FieldDecorator(write_if_decorator));
        objects.insert("atomic".to_owned(), Accessible::FieldDecorator(atomic_decorator));
        objects.insert("nonatomic".to_owned(), Accessible::FieldDecorator(nonatomic_decorator));
        objects.insert("virtual".to_owned(), Accessible::FieldDecorator(virtual_decorator));
        objects.insert("presentWith".to_owned(), Accessible::FieldDecorator(present_with_decorator));
        objects.insert("presentWithout".to_owned(), Accessible::FieldDecorator(present_without_decorator));
        objects.insert("presentIf".to_owned(), Accessible::FieldDecorator(present_if_decorator));
        objects.insert("recordPrevious".to_owned(), Accessible::FieldDecorator(record_previous_decorator));
        objects.insert("inputOmissible".to_owned(), Accessible::FieldDecorator(input_omissible_decorator));
        objects.insert("outputOmissible".to_owned(), Accessible::FieldDecorator(output_omissible_decorator));
        objects.insert("auto".to_owned(), Accessible::FieldDecorator(auto_decorator));
        objects.insert("autoIncrement".to_owned(), Accessible::FieldDecorator(auto_increment_decorator));
        objects.insert("default".to_owned(), Accessible::FieldDecorator(default_decorator));
        objects.insert("foreignKey".to_owned(), Accessible::FieldDecorator(foreign_key_decorator));
        objects.insert("onSet".to_owned(), Accessible::FieldDecorator(on_set_decorator));
        objects.insert("onSave".to_owned(), Accessible::FieldDecorator(on_save_decorator));
        objects.insert("onOutput".to_owned(), Accessible::FieldDecorator(on_output_decorator));
        objects.insert("identity".to_owned(), Accessible::FieldDecorator(auth_identity_decorator));
        objects.insert("identityChecker".to_owned(), Accessible::FieldDecorator(auth_by_decorator));
        objects.insert("queryable".to_owned(), Accessible::FieldDecorator(queryable_decorator));
        objects.insert("unqueryable".to_owned(), Accessible::FieldDecorator(unqueryable_decorator));
        objects.insert("can_mutate".to_owned(), Accessible::FieldDecorator(can_mutate_decorator));
        objects.insert("can_read".to_owned(), Accessible::FieldDecorator(can_read_decorator));
        Self { objects }
    }

    pub(crate) fn get(&self, key: &str) -> &Accessible {
        match self.objects.get(key) {
            Some(o) => o,
            None => panic!("Object with key '{}' is not found.", key),
        }
    }
}
