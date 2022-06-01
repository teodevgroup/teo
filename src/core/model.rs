use std::collections::{HashMap, HashSet};
use std::ptr::{addr_of, null};
use inflector::Inflector;
use sea_query::IndexType;
use crate::action::action::ActionType;
use crate::core::builders::model_builder::ModelBuilder;
use crate::core::field::{Field, FieldIndex, QueryAbility, Sort};
use crate::core::field::ReadRule::NoRead;
use crate::core::field::Store::{Calculated, Temp};
use crate::core::field::WriteRule::NoWrite;
use crate::core::permission::Permission;


pub enum ModelIndexType {
    Primary,
    Index,
    Unique,
}

pub(crate) struct CompoundIndexItem {
    pub(crate) field_name: String,
    pub(crate) sort: Sort,
    pub(crate) len: Option<usize>,
}

pub(crate) struct CompoundIndex {
    pub(crate) index_type: ModelIndexType,
    pub(crate) name: String,
    pub(crate) items: Vec<CompoundIndexItem>
}

#[derive(Debug)]
pub(crate) struct Model {
    name: &'static str,
    table_name: String,
    url_segment_name: String,
    localized_name: &'static str,
    description: &'static str,
    identity: bool,
    actions: HashSet<ActionType>,
    permission: Option<Permission>,
    fields_vec: Vec<Field>,
    fields_map: HashMap<&'static str, * const Field>,
    primary_field: * const Field,
    index_fields: Vec<* const Field>,
    input_keys: Vec<&'static str>,
    save_keys: Vec<&'static str>,
    output_keys: Vec<&'static str>,
    get_value_keys: Vec<&'static str>,
    query_keys: Vec<&'static str>,
    unique_query_keys: Vec<&'static str>,
    auth_identity_keys: Vec<&'static str>,
    auth_by_keys: Vec<&'static str>,
}

impl Model {

    pub(crate) fn new(builder: &ModelBuilder) -> Self {
        let input_keys = Self::allowed_input_keys(builder);
        let save_keys = Self::allowed_save_keys(builder);
        let output_keys = Self::allowed_output_keys(builder);
        let get_value_keys = Self::get_get_value_keys(builder);
        let query_keys = Self::get_query_keys(builder);
        let unique_query_keys = Self::get_unique_query_keys(builder);
        let auth_identity_keys = Self::get_auth_identity_keys(builder);
        let auth_by_keys = Self::get_auth_by_keys(builder);
        let fields_vec: Vec<Field> = builder.fields.iter().map(|fb| { Field::new(fb) }).collect();
        let mut fields_map: HashMap<&'static str, * const Field> = HashMap::new();
        let mut primary_field: * const Field = null();
        let mut index_fields: Vec<* const Field> = Vec::new();
        for field in fields_vec.iter() {
            let addr = addr_of!(*field);
            fields_map.insert(field.name, addr);
            if field.primary {
                primary_field = addr_of!(*field);
            }
            if field.index != FieldIndex::NoIndex {
                index_fields.push(addr);
            }
        }
        Model {
            name: builder.name,
            table_name: if builder.table_name == "" { builder.name.to_lowercase().to_plural() } else { builder.table_name.to_string() },
            url_segment_name: if builder.url_segment_name == "" { builder.name.to_kebab_case().to_plural() } else { builder.url_segment_name.to_string() },
            localized_name: builder.localized_name,
            description: builder.description,
            identity: builder.identity,
            actions: builder.actions.clone(),
            permission: if let Some(builder) = &builder.permission { Some(builder.build()) } else { None },
            fields_vec,
            fields_map,
            primary_field,
            index_fields,
            input_keys,
            save_keys,
            output_keys,
            get_value_keys,
            query_keys,
            unique_query_keys,
            auth_identity_keys,
            auth_by_keys
        }
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn table_name(&self) -> &String {
        &self.table_name
    }

    pub(crate) fn url_segment_name(&self) -> &String {
        &self.url_segment_name
    }

    pub(crate) fn localized_name(&self) -> &'static str {
        self.localized_name
    }

    pub(crate) fn description(&self) -> &'static str {
        self.description
    }

    pub(crate) fn identity(&self) -> bool {
        self.identity
    }

    pub(crate) fn fields(&self) -> &Vec<Field> {
        return &self.fields_vec
    }

    pub fn field(&self, name: &str) -> &Field {
        unsafe {
            &(**self.fields_map.get(name).unwrap())
        }
    }

    pub(crate) fn primary_field(&self) -> Option<&Field> {
        if self.primary_field == null() {
            None
        } else {
            Some(unsafe { &*self.primary_field })
        }
    }

    pub(crate) fn primary_field_name(&self) -> Option<&'static str> {
        match self.primary_field() {
            Some(field) => Some(field.name),
            None => None
        }
    }

    pub(crate) fn index_fields(&self) -> Vec<&Field> {
        self.index_fields.iter().map(|f| { unsafe { &**f } }).collect()
    }

    pub(crate) fn input_keys(&self) -> &Vec<&'static str> {
        &self.input_keys
    }

    pub(crate) fn save_keys(&self) -> &Vec<&'static str> {
        &self.save_keys
    }

    pub(crate) fn output_keys(&self) -> &Vec<&'static str> {
        &self.output_keys
    }

    pub(crate) fn get_value_keys(&self) -> &Vec<&'static str> {
        &self.get_value_keys
    }

    pub(crate) fn query_keys(&self) -> &Vec<&'static str> {
        &self.query_keys
    }

    pub(crate) fn unique_query_keys(&self) -> &Vec<&'static str> {
        &self.unique_query_keys
    }

    pub(crate) fn auth_identity_keys(&self) -> &Vec<&'static str> { &self.auth_identity_keys }

    pub(crate) fn auth_by_keys(&self) -> &Vec<&'static str> { &self.auth_by_keys }

    fn allowed_input_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.write_rule != NoWrite })
            .map(|f| { f.name })
            .collect()
    }

    fn allowed_save_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.store != Calculated && f.store != Temp })
            .map(|f| { f.name })
            .collect()
    }

    fn allowed_output_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.read_rule != NoRead })
            .map(|f| { f.name })
            .collect()
    }

    pub(crate) fn get_get_value_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .map(|f| { f.name })
            .collect()
    }

    pub(crate) fn get_query_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.query_ability == QueryAbility::Queryable })
            .map(|f| { f.name })
            .collect()
    }

    pub(crate) fn get_unique_query_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.query_ability == QueryAbility::Queryable && (f.index == FieldIndex::Unique || f.primary == true) })
            .map(|f| { f.name })
            .collect()
    }

    pub(crate) fn get_auth_identity_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.auth_identity == true })
            .map(|f| { f.name })
            .collect()
    }

    pub(crate) fn get_auth_by_keys(builder: &ModelBuilder) -> Vec<&'static str> {
        builder.fields.iter()
            .filter(|&f| { f.auth_by == true })
            .map(|f| { f.name })
            .collect()
    }

    pub(crate) fn has_action(&self, action: ActionType) -> bool {
        self.actions.contains(&action)
    }

    pub(crate) fn actions(&self) -> &HashSet<ActionType> {
        &self.actions
    }
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
