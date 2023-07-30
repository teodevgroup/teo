use std::borrow::Borrow;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use key_path::KeyPath;
use maplit::hashmap;
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use crate::app::app_ctx::AppCtx;
use crate::core::action::{Action, CREATE, INTERNAL_AMOUNT, INTERNAL_POSITION, PROGRAM_CODE, SINGLE};
use crate::core::connector::connection::Connection;
use crate::core::initiator::Initiator;
use crate::core::object::Object;
use crate::core::r#enum::Enum;
use crate::core::error::Error;
use crate::core::model::model::Model;
use crate::core::relation::Relation;
use crate::core::result::Result;
use crate::prelude::{Req, Value};

#[derive(ToMut)]
pub struct Graph {
    pub(crate) enums: HashMap<&'static str, Enum>,
    pub(crate) models: HashMap<&'static str, Model>,
}

impl Graph {

    pub(crate) fn new() -> Self {
        Self {
            enums: hashmap!{},
            models: hashmap!{},
        }
    }

    pub(crate) fn add_enum(&self, e: Enum) {
        let mut_self = self.to_mut();
        mut_self.enums.insert(e.name, e);
    }

    pub(crate) fn add_model(&self, m: Model, name: &'static str) {
        let mut_self = self.to_mut();
        mut_self.models.insert(name, m);
    }

    pub fn models(&self) -> Vec<&Model> {
        self.models.values().collect()
    }

    pub fn models_without_teo_internal(&self) -> Vec<&Model> {
        self.models().iter().filter(|m| !m.is_teo_internal()).map(|m| *m).collect()
    }

    // MARK: - Queries

    pub async fn find_unique<'a, T: From<Object>>(&'static self, model: &'a str, finder: &'a Value, connection: Option<Arc<dyn Connection>>, req: Option<Req>) -> Result<Option<T>> {
        let connection = connection.unwrap();
        match self.find_unique_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), Initiator::ProgramCode(req), connection).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_first<'a, T: From<Object>>(&'static self, model: &'a str, finder: &'a Value, connection: Option<Arc<dyn Connection>>, req: Option<Req>) -> Result<Option<T>> {
        let connection = connection.unwrap();
        match self.find_first_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), Initiator::ProgramCode(req), connection).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_many<'a, T: From<Object>>(&'static self, model: &'a str, finder: &'a Value, connection: Option<Arc<dyn Connection>>, req: Option<Req>) -> Result<Vec<T>> {
        let connection = connection.unwrap();
        match self.find_many_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), Initiator::ProgramCode(req), connection).await {
            Ok(results) => Ok(results.iter().map(|item| item.clone().into()).collect()),
            Err(err) => Err(err),
        }
    }

    pub(crate) async fn find_unique_internal<'a>(&'static self, model: &'a str, finder: &'a Value, mutation_mode: bool, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Option<Object>> {
        let model = self.model(model)?;
        connection.find_unique(self, model, finder, mutation_mode, action, action_source).await
    }

    pub(crate) async fn find_first_internal<'a>(&'static self, model: &'a str, finder: &'a Value, mutation_mode: bool, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Option<Object>> {
        let model = self.model(model)?;
        let mut finder = finder.as_hashmap().clone().unwrap().clone();
        finder.insert("take".to_string(), 1.into());
        let finder = Value::HashMap(finder);
        let result = connection.find_many(self, model, &finder, mutation_mode, action, action_source).await;
        match result {
            Err(err) => Err(err),
            Ok(retval) => {
                if retval.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(retval.get(0).unwrap().clone()))
                }
            }
        }
    }

    pub(crate) async fn find_many_internal<'a>(&'static self, model: &'a str, finder: &'a Value, mutation_mode: bool, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Vec<Object>> {
        let model = self.model(model)?;
        connection.find_many(self, model, finder, mutation_mode, action, action_source).await
    }

    pub(crate) async fn batch<'a, F, Fut>(&'static self, model: &'a str, finder: &'a Value, action: Action, action_source: Initiator, f: F, connection: Arc<dyn Connection>) -> Result<()> where
        F: Fn(Object) -> Fut,
        Fut: Future<Output = Result<()>> {
        let batch_size: usize = 200;
        let mut index: usize = 0;
        loop {
            let mut batch_finder = finder.clone();
            batch_finder.as_hashmap_mut().unwrap().insert("skip".to_owned(), (index * batch_size).into());
            batch_finder.as_hashmap_mut().unwrap().insert("take".to_owned(), batch_size.into());
            let results = self.find_many_internal(model, &batch_finder, true, action, action_source.clone(), connection.clone()).await?;
            for result in results.iter() {
                f(result.clone()).await?;
            }
            if results.len() < batch_size {
                return Ok(());
            }
            index += 1;
        }
    }

    pub(crate) async fn count<'a>(&'static self, model: &'a str, finder: &'a Value, connection: Option<Arc<dyn Connection>>) -> Result<usize> {
        let connection = connection.unwrap();
        let model = self.model(model)?;
        connection.count(self, model, finder).await
    }

    pub(crate) async fn aggregate<'a>(&'static self, model: &'a str, finder: &'a Value, connection: Option<Arc<dyn Connection>>) -> Result<Value> {
        let connection = connection.unwrap();
        let model = self.model(model)?;
        connection.aggregate(self, model, finder).await
    }

    pub(crate) async fn group_by<'a>(&'static self, model: &'a str, finder: &'a Value, connection: Option<Arc<dyn Connection>>) -> Result<Value> {
        let connection = connection.unwrap();
        let model = self.model(model)?;
        connection.group_by(self, model, finder).await
    }

    // MARK: - Create an object

    pub(crate) fn new_object(&'static self, model: &str, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Object> {
        match self.model(model) {
            Ok(model) => Ok(Object::new(self, model, action, action_source, connection)),
            Err(_) => Err(Error::invalid_operation(format!("Model with name '{model}' is not defined.")))
        }
    }

    pub(crate) async fn new_object_with_teon_and_path<'a>(&'static self, model: &str, initial: &Value, path: &KeyPath<'a>, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Object> {
        let object = self.new_object(model, action, action_source, connection)?;
        object.set_teon_with_path(initial, path).await?;
        Ok(object)
    }

    pub async fn create_object(&'static self, model: &str, initial: impl Borrow<Value>, connection: Option<Arc<dyn Connection>>, req: Option<Req>) -> Result<Object> {
        let connection = connection.unwrap();
        let obj = self.new_object(model, Action::from_u32(PROGRAM_CODE | CREATE | SINGLE | INTERNAL_POSITION), Initiator::ProgramCode(req), connection)?;
        obj.set_teon(initial.borrow()).await?;
        Ok(obj)
    }

    pub(crate) fn model(&self, name: &str) -> Result<&Model> {
        match self.models.get(name) {
            Some(model) => Ok(model),
            None => Err(Error::fatal_message(format!("Model `{}' is not found.", name))),
        }
    }

    pub(crate) fn r#enum(&self, name: &str) -> Option<&Enum> {
        self.enums.get(name)
    }

    pub(crate) fn enums(&self) -> &HashMap<&'static str, Enum> { &self.enums }

    pub(crate) fn enum_values(&self, name: &str) -> Option<&Vec<String>> {
        match self.enums.get(name) {
            Some(e) => Some(e.values()),
            None => None,
        }
    }

    /// Returns the opposite relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph.
    ///
    /// # Return Value
    ///
    /// A tuple of opposite relation's model and opposite relation.
    ///
    pub(crate) fn opposite_relation(&self, relation: &Relation) -> (&Model, Option<&Relation>) {
        let opposite_model = self.model(relation.model()).unwrap();
        let opposite_relation = opposite_model.relations().iter().find(|r| r.fields() == relation.references() && r.references() == relation.fields());
        match opposite_relation {
            Some(relation) => (opposite_model, Some(relation.as_ref())),
            None => (opposite_model, None)
        }
    }

    /// Returns the through relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph. This relation must be a
    /// through relation.
    ///
    /// # Return Value
    ///
    /// A tuple of through relation's model and through model's local relation.
    ///
    pub(crate) fn through_relation(&self, relation: &Relation) -> (&Model, &Relation) {
        let through_model = self.model(relation.through().unwrap()).unwrap();
        let through_local_relation = through_model.relation(relation.local()).unwrap();
        (through_model, through_local_relation)
    }

    /// Returns the through opposite relation of the argument relation.
    ///
    /// # Arguments
    ///
    /// * `relation` - The relation must be of a model of this graph. This relation must be a
    /// through relation.
    ///
    /// # Return Value
    ///
    /// A tuple of through relation's model and through model's foreign relation.
    ///
    pub(crate) fn through_opposite_relation(&self, relation: &Relation) -> (&Model, &Relation) {
        let through_model = self.model(relation.through().unwrap()).unwrap();
        let through_foreign_relation = through_model.relation(relation.foreign()).unwrap();
        (through_model, through_foreign_relation)
    }
}

unsafe impl Send for Graph { }
unsafe impl Sync for Graph { }
