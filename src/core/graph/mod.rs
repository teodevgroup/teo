use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use key_path::KeyPath;
use to_mut_proc_macro::ToMut;
use to_mut::ToMut;
use crate::core::action::{Action, CREATE, INTERNAL_AMOUNT, INTERNAL_POSITION, PROGRAM_CODE, SINGLE};
use crate::core::action::source::ActionSource;
use crate::core::connector::Connector;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::r#enum::Enum;
use crate::core::error::Error;
use crate::core::relation::Relation;
use crate::core::result::Result;
use crate::prelude::Value;

pub mod builder;

#[derive(Clone, ToMut)]
pub struct Graph {
    inner: Arc<GraphInner>
}

pub(crate) struct GraphInner {
    pub(crate) enums: HashMap<String, Enum>,
    pub(crate) models_vec: Vec<Model>,
    pub(crate) models_map: HashMap<String, Model>,
    pub(crate) connector: Option<Arc<dyn Connector>>,
}

static mut CURRENT: Option<&'static Graph> = None;

impl Graph {

    pub fn models(&self) -> &Vec<Model> {
        self.inner.models_vec.as_ref()
    }

    pub fn current() -> &'static Self {
        unsafe {
            if CURRENT.is_none() {
                panic!("Current graph is accessed before app is initialized.")
            }
            CURRENT.unwrap()
        }
    }

    pub(crate) fn set_current(current: &'static Graph) {
        unsafe {
            CURRENT = Some(current);
        }
    }

    // MARK: - Queries

    pub async fn find_unique<T: From<Object>>(&self, model: &str, finder: &Value) -> Result<T> {
        match self.find_unique_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), ActionSource::ProgramCode).await {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }

    pub async fn find_first<T: From<Object>>(&self, model: &str, finder: &Value) -> Result<T> {
        match self.find_first_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), ActionSource::ProgramCode).await {
            Ok(result) => Ok(result.into()),
            Err(err) => Err(err),
        }
    }

    pub async fn find_many<T: From<Object>>(&self, model: &str, finder: &Value) -> Result<Vec<T>> {
        match self.find_many_internal(model, finder, false, Action::from_u32(PROGRAM_CODE | INTERNAL_AMOUNT | INTERNAL_POSITION), ActionSource::ProgramCode).await {
            Ok(results) => Ok(results.iter().map(|item| item.clone().into()).collect()),
            Err(err) => Err(err),
        }
    }

    pub(crate) async fn find_unique_internal(&self, model: &str, finder: &Value, mutation_mode: bool, action: Action, action_source: ActionSource) -> Result<Object> {
        let model = self.model(model).unwrap();
        self.connector().find_unique(self, model, finder, mutation_mode, action, action_source).await
    }

    pub(crate) async fn find_first_internal(&self, model: &str, finder: &Value, mutation_mode: bool, action: Action, action_source: ActionSource) -> Result<Object> {
        let model = self.model(model).unwrap();
        let mut finder = finder.as_hashmap().clone().unwrap().clone();
        finder.insert("take".to_string(), 1.into());
        let finder = Value::HashMap(finder);
        let result = self.connector().find_many(self, model, &finder, mutation_mode, action, action_source).await;
        match result {
            Err(err) => Err(err),
            Ok(retval) => {
                if retval.is_empty() {
                    Err(Error::object_not_found())
                } else {
                    Ok(retval.get(0).unwrap().clone())
                }
            }
        }
    }

    pub(crate) async fn find_many_internal(&self, model: &str, finder: &Value, mutation_mode: bool, action: Action, action_source: ActionSource) -> Result<Vec<Object>> {
        let model = self.model(model).unwrap();
        self.connector().find_many(self, model, finder, mutation_mode, action, action_source).await
    }

    pub(crate) async fn batch<F, Fut>(&self, model: &str, finder: &Value, action: Action, action_source: ActionSource, f: F) -> Result<()> where
    F: Fn(Object) -> Fut,
    Fut: Future<Output = Result<()>> {
        let batch_size: usize = 200;
        let mut index: usize = 0;
        loop {
            let mut batch_finder = finder.clone();
            batch_finder.as_hashmap_mut().unwrap().insert("skip".to_owned(), (index * batch_size).into());
            batch_finder.as_hashmap_mut().unwrap().insert("take".to_owned(), batch_size.into());
            let results = self.find_many_internal(model, &batch_finder, true, action, action_source.clone()).await?;
            for result in results.iter() {
                f(result.clone()).await?;
            }
            if results.len() < batch_size {
                return Ok(());
            }
            index += 1;
        }
    }

    pub(crate) async fn count(&self, model: &str, finder: &Value) -> Result<usize> {
        let model = self.model(model).unwrap();
        self.connector().count(self, model, finder).await
    }

    pub(crate) async fn aggregate(&self, model: &str, finder: &Value) -> Result<Value> {
        let model = self.model(model).unwrap();
        self.connector().aggregate(self, model, finder).await
    }

    pub(crate) async fn group_by(&self, model: &str, finder: &Value) -> Result<Value> {
        let model = self.model(model).unwrap();
        self.connector().group_by(self, model, finder).await
    }

    // MARK: - Create an object

    pub(crate) fn new_object(&self, model: &str, action: Action, action_source: ActionSource) -> Result<Object> {
        match self.model(model) {
            Some(model) => Ok(Object::new(self, model, action, action_source)),
            None => Err(Error::invalid_operation(format!("Model with name '{model}' is not defined.")))
        }
    }

    pub(crate) async fn new_object_with_tson_and_path<'a>(&self, model: &str, initial: &Value, path: &KeyPath<'a>, action: Action, action_source: ActionSource) -> Result<Object> {
        let object = self.new_object(model, action, action_source)?;
        object.set_teon_with_path(initial, path).await?;
        Ok(object)
    }

    pub async fn create_object(&self, model: &str, initial: impl AsRef<Value>) -> Result<Object> {
        let obj = self.new_object(model, Action::from_u32(PROGRAM_CODE | CREATE | SINGLE | INTERNAL_POSITION), ActionSource::ProgramCode)?;
        obj.set_teon(initial.as_ref()).await?;
        Ok(obj)
    }

    // MARK: - Getting the connector

    pub(crate) fn connector(&self) -> &dyn Connector {
        match &self.inner.connector {
            Some(c) => { c.as_ref() }
            None => { panic!() }
        }
    }

    pub(crate) fn connector_mut(&self) -> &mut dyn Connector {
        match &self.inner.connector {
            Some(c) => {
                let r = c.as_ref();
                let result = unsafe {
                    let d: * const dyn Connector = r;
                    let e: * mut dyn Connector = d as *mut dyn Connector;
                    &mut *e
                };
                result
            }
            None => { panic!() }
        }
    }

    pub(crate) fn model(&self, name: &str) -> Option<&Model> {
        self.inner.models_map.get(name)
    }

    pub(crate) fn r#enum(&self, name: &str) -> Option<&Enum> {
        self.inner.enums.get(name)
    }

    pub(crate) fn enums(&self) -> &HashMap<String, Enum> { &self.inner.enums }

    pub(crate) fn enum_values(&self, name: &str) -> Option<&Vec<String>> {
        match self.inner.enums.get(name) {
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
