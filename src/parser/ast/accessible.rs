use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use maplit::hashmap;
use crate::core::field::Field;
use crate::core::model::builder::ModelBuilder;
use crate::core::model::Model;
use crate::core::property::Property;
use crate::core::relation::Relation;
use crate::core::tson::Value;
use crate::parser::ast::argument::Argument;
use crate::parser::ast::entity::Entity;
use crate::parser::std::constants::EnvObject;

pub(crate) type FieldDecorator = fn(args: Vec<Argument>, field: &mut Field);

pub(crate) type RelationDecorator = fn(args: Vec<Argument>, relation: &mut Relation);

pub(crate) type PropertyDecorator = fn(args: Vec<Argument>, property: &mut Property);

pub(crate) type ModelDecorator = fn(args: Vec<Argument>, model: &mut ModelBuilder);

#[derive(Debug, Clone)]
pub(crate) struct Container {
    pub(crate) objects: HashMap<String, Entity>
}

impl Container {
    pub(crate) fn std_global_constants() -> Self {
        Self {
            objects: hashmap!{
                "ENV".to_owned() => Entity::Accessible(Accessible::Env(EnvObject {}))
            }
        }
    }

    pub(crate) fn access_property(&self, name: &str) -> &Entity {
        self.objects.get(name).unwrap()
    }
}

#[derive(Clone)]
pub(crate) enum Accessible {
    FieldDecorator(FieldDecorator),
    RelationDecorator(RelationDecorator),
    PropertyDecorator(PropertyDecorator),
    ModelDecorator(ModelDecorator),
    Container(Container),
    Env(EnvObject),
}

impl Debug for Accessible {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Accessible")
    }
}

impl Accessible {

    pub(crate) fn as_container(&self) -> Option<&Container> {
        match self {
            Accessible::Container(c) => Some(c),
            _ => None,
        }
    }

    pub(crate) fn is_container(&self) -> bool {
        self.as_container().is_some()
    }

    pub(crate) fn as_env(&self) -> Option<&EnvObject> {
        match self {
            Accessible::Env(e) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn is_env(&self) -> bool {
        self.as_env().is_some()
    }

    pub(crate) fn as_field_decorator(&self) -> Option<&FieldDecorator> {
        match self {
            Accessible::FieldDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_relation_decorator(&self) -> Option<&RelationDecorator> {
        match self {
            Accessible::RelationDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_property_decorator(&self) -> Option<&PropertyDecorator> {
        match self {
            Accessible::PropertyDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn as_model_decorator(&self) -> Option<&ModelDecorator> {
        match self {
            Accessible::ModelDecorator(d) => Some(d),
            _ => None,
        }
    }

    pub(crate) fn access_property(&self, name: &str) -> &Entity {
        match self.as_container() {
            Some(c) => c.access_property(name),
            None => panic!("Cannot access property '{}'", name),
        }
    }
}
