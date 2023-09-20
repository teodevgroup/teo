use std::borrow::Borrow;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use key_path::KeyPath;
use crate::app::app_ctx::AppCtx;
use crate::core::action::{Action, CREATE, INTERNAL_AMOUNT, INTERNAL_POSITION, PROGRAM_CODE, SINGLE};
use crate::core::connector::connection::Connection;
use crate::core::initiator::Initiator;
use crate::core::object::Object;
use crate::core::r#enum::Enum;
use crate::core::model::model::Model;
use crate::core::relation::Relation;
use crate::core::result::Result;
use crate::prelude::{Req, Value};

pub struct Graph { }

impl Graph {

    pub(crate) fn new() -> Self {
        Self { }
    }

    pub fn models(&self) -> Vec<&Model> {
        AppCtx::get().unwrap().models()
    }

    pub fn models_without_teo_internal(&self) -> Vec<&Model> {
        AppCtx::get().unwrap().models().iter().filter(|m| !m.is_teo_internal()).map(|m| *m).collect()
    }

    // MARK: - Queries

    pub async fn find_unique<'a, T: From<Object>>(&'static self, model: &'static Model, finder: &'a Value, connection: Arc<dyn Connection>, req: Option<Req>) -> Result<Option<T>> {
        match self.find_unique_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), Initiator::ProgramCode(req), connection).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_first<'a, T: From<Object>>(&'static self, model: &'static Model, finder: &'a Value, connection: Arc<dyn Connection>, req: Option<Req>) -> Result<Option<T>> {
        match self.find_first_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), Initiator::ProgramCode(req), connection).await {
            Ok(result) => match result {
                Some(o) => Ok(Some(o.into())),
                None => Ok(None),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn find_many<'a, T: From<Object>>(&'static self, model: &'static Model, finder: &'a Value, connection: Arc<dyn Connection>, req: Option<Req>) -> Result<Vec<T>> {
        match self.find_many_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), Initiator::ProgramCode(req), connection).await {
            Ok(results) => Ok(results.iter().map(|item| item.clone().into()).collect()),
            Err(err) => Err(err),
        }
    }

    pub(crate) async fn find_unique_internal<'a>(&self, model: &'static Model, finder: &'a Value, mutation_mode: bool, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Option<Object>> {
        connection.find_unique(model, finder, mutation_mode, action, action_source).await
    }

    pub(crate) async fn find_first_internal<'a>(&'static self, model: &'static Model, finder: &'a Value, mutation_mode: bool, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Option<Object>> {
        let mut finder = finder.as_hashmap().clone().unwrap().clone();
        finder.insert("take".to_string(), 1.into());
        let finder = Value::HashMap(finder);
        let result = connection.find_many(model, &finder, mutation_mode, action, action_source).await;
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

    pub(crate) async fn find_many_internal<'a>(&'static self, model: &'static Model, finder: &'a Value, mutation_mode: bool, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Vec<Object>> {
        connection.find_many(model, finder, mutation_mode, action, action_source).await
    }

    pub(crate) async fn batch<'a, F, Fut>(&'static self, model: &'static Model, finder: &'a Value, action: Action, action_source: Initiator, f: F, connection: Arc<dyn Connection>) -> Result<()> where
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

    pub(crate) async fn count<'a>(&'static self, model: &'static Model, finder: &'a Value, connection: Arc<dyn Connection>) -> Result<usize> {
        connection.count(model, finder).await
    }

    pub(crate) async fn aggregate<'a>(&'static self, model: &'static Model, finder: &'a Value, connection: Arc<dyn Connection>) -> Result<Value> {
        connection.aggregate(model, finder).await
    }

    pub(crate) async fn group_by<'a>(&'static self, model: &'static Model, finder: &'a Value, connection: Arc<dyn Connection>) -> Result<Value> {
        connection.group_by(model, finder).await
    }

    // MARK: - Create an object

    pub(crate) fn new_object(&'static self, model: &'static Model, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Object> {
        Ok(Object::new(self, model, action, action_source, connection))
    }

    pub(crate) async fn new_object_with_teon_and_path<'a>(&'static self, model: &'static Model, initial: &Value, path: &KeyPath<'a>, action: Action, action_source: Initiator, connection: Arc<dyn Connection>) -> Result<Object> {
        let object = self.new_object(model, action, action_source, connection)?;
        object.set_teon_with_path(initial, path).await?;
        Ok(object)
    }

    pub async fn create_object(&'static self, model: &'static Model, initial: impl Borrow<Value>, connection: Arc<dyn Connection>, req: Option<Req>) -> Result<Object> {
        let obj = self.new_object(model, Action::from_u32(PROGRAM_CODE | CREATE | SINGLE | INTERNAL_POSITION), Initiator::ProgramCode(req), connection)?;
        obj.set_teon(initial.borrow()).await?;
        Ok(obj)
    }

    pub(crate) fn r#enum(&self, name: &str) -> Option<&Enum> {
        AppCtx::get().unwrap().main_namespace().r#enum(name)
    }

    pub(crate) fn enums(&self) -> &HashMap<&str, Enum> {
        AppCtx::get().unwrap().main_namespace().enums()
    }

    pub(crate) fn enum_values(&self, name: &str) -> Option<&Vec<String>> {
        match AppCtx::get().unwrap().main_namespace().enums().get(name) {
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
        let opposite_model = AppCtx::get().unwrap().model(relation.model_path()).unwrap().unwrap();
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
        let through_model = AppCtx::get().unwrap().model(relation.through_path().unwrap()).unwrap().unwrap();
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
        let through_model = AppCtx::get().unwrap().model(relation.through_path().unwrap()).unwrap().unwrap();
        let through_foreign_relation = through_model.relation(relation.foreign()).unwrap();
        (through_model, through_foreign_relation)
    }
}

unsafe impl Send for Graph { }
unsafe impl Sync for Graph { }
