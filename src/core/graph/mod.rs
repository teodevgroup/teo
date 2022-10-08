use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use key_path::KeyPath;
use crate::core::graph::builder::GraphBuilder;
use crate::core::connector::Connector;
use crate::core::env::Env;
use crate::core::model::Model;
use crate::core::object::Object;
use crate::core::r#enum::Enum;
use crate::core::error::ActionError;
use crate::core::relation::Relation;
use crate::core::result::ActionResult;
use crate::prelude::Value;

pub mod builder;

#[derive(Clone)]
pub struct Graph {
    inner: Arc<GraphInner>
}

struct GraphInner {
    enums: HashMap<String, Enum>,
    models_vec: Vec<Model>,
    models_map: HashMap<String, Model>,
    url_segment_name_map: HashMap<String, String>,
    connector: Option<Box<dyn Connector>>,
}

impl Graph {

    // MARK: - Create a graph

    pub async fn new<'a, F: Fn(&mut GraphBuilder)>(build: F) -> Graph {
        let mut builder = GraphBuilder::new();
        build(&mut builder);
        let mut graph = GraphInner {
            enums: builder.build_enums(),
            models_vec: Vec::new(),
            models_map: HashMap::new(),
            url_segment_name_map: HashMap::new(),
            connector: None,
        };
        graph.models_vec = builder.models.iter().map(|mb| { mb.build(&builder.connector_builder()) }).collect();
        let mut models_map: HashMap<String, Model> = HashMap::new();
        let mut url_segment_name_map: HashMap<String, String> = HashMap::new();
        for model in graph.models_vec.iter() {
            models_map.insert(model.name().to_owned(), model.clone());
            url_segment_name_map.insert(model.url_segment_name().to_owned(), model.name().to_owned());
        }
        graph.models_map = models_map;
        graph.url_segment_name_map = url_segment_name_map;
        graph.connector = Some(builder.connector_builder().build_connector(&graph.models_vec, builder.reset_database).await);
        Graph { inner: Arc::new(graph) }
    }

    // MARK: - Queries

    pub async fn find_unique(&self, model: &str, finder: &Value, mutation_mode: bool, env: Env) -> ActionResult<Object> {
        let model = self.model(model).unwrap();
        self.connector().find_unique(self, model, finder, mutation_mode, env).await
    }

    pub async fn find_first(&self, model: &str, finder: &Value, mutation_mode: bool, env: Env) -> ActionResult<Object> {
        let model = self.model(model).unwrap();
        let mut finder = finder.as_hashmap().clone().unwrap().clone();
        finder.insert("take".to_string(), 1.into());
        let finder = Value::HashMap(finder);
        let result = self.connector().find_many(self, model, &finder, mutation_mode, env).await;
        match result {
            Err(err) => Err(err),
            Ok(retval) => {
                if retval.is_empty() {
                    Err(ActionError::object_not_found())
                } else {
                    Ok(retval.get(0).unwrap().clone())
                }
            }
        }
    }

    pub async fn find_many(&self, model: &str, finder: &Value, mutation_mode: bool, env: Env) -> ActionResult<Vec<Object>> {
        let model = self.model(model).unwrap();
        self.connector().find_many(self, model, finder, mutation_mode, env).await
    }

    pub async fn batch<F, Fut>(&self, model: &str, finder: &Value, env: Env, f: F) -> ActionResult<()> where
    F: Fn(Object) -> Fut,
    Fut: Future<Output = ActionResult<()>> {
        let batch_size: usize = 200;
        let mut index: usize = 0;
        loop {
            let mut batch_finder = finder.clone();
            batch_finder.as_hashmap_mut().unwrap().insert("skip".to_owned(), (index * batch_size).into());
            batch_finder.as_hashmap_mut().unwrap().insert("take".to_owned(), batch_size.into());
            let results = self.find_many(model, &batch_finder, true, env.clone()).await?;
            for result in results.iter() {
                f(result.clone()).await?;
            }
            if results.len() < batch_size {
                return Ok(());
            }
            index += 1;
        }
    }

    pub async fn count(&self, model: &str, finder: &Value) -> Result<usize, ActionError> {
        let model = self.model(model).unwrap();
        self.connector().count(self, model, finder).await
    }

    pub async fn aggregate(&self, model: &str, finder: &Value) -> Result<Value, ActionError> {
        let model = self.model(model).unwrap();
        self.connector().aggregate(self, model, finder).await
    }

    pub async fn group_by(&self, model: &str, finder: &Value) -> Result<Value, ActionError> {
        let model = self.model(model).unwrap();
        self.connector().group_by(self, model, finder).await
    }

    // MARK: - Create an object

    pub(crate) fn new_object(&self, model: &str, env: Env) -> Result<Object, ActionError> {
        match self.model(model) {
            Some(model) => Ok(Object::new(self, model, env)),
            None => Err(ActionError::invalid_operation(format!("Model with name '{model}' is not defined.")))
        }
    }

    pub(crate) async fn new_object_with_tson_and_path(&self, model: &str, initial: &Value, path: &KeyPath, env: Env) -> Result<Object, ActionError> {
        let object = self.new_object(model, env)?;
        object.set_tson_with_path(initial, path).await?;
        Ok(object)
    }

    pub async fn create_object(&self, model: &str, initial: Value) -> Result<Object, ActionError> {
        let obj = self.new_object(model, Env::custom_code())?;
        obj.set_tson(&initial).await?;
        Ok(obj)
    }

    // MARK: - Getting the connector

    pub(crate) fn connector(&self) -> &dyn Connector {
        match &self.inner.connector {
            Some(c) => { c.as_ref() }
            None => { panic!() }
        }
    }



    pub(crate) fn model(&self, name: &str) -> Option<&Model> {
        self.inner.models_map.get(name)
    }

    pub(crate) fn model_with_url_segment_name(&self, segment_name: &str) -> Option<&Model> {
        match self.inner.url_segment_name_map.get(segment_name) {
            Some(val) => self.model(val),
            None => None
        }
    }

    pub(crate) fn models(&self) -> &Vec<Model> { &self.inner.models_vec }

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
        if let Some(through) = relation.through() {
            let through_model = self.model(through).unwrap();
            self.opposite_relation(through_model.relation(relation.foreign()).unwrap())
        } else {
            let opposite_model = self.model(relation.model()).unwrap();
            let opposite_relation = opposite_model.relations().iter().find(|r| r.fields() == relation.references() && r.references() == relation.fields());
            match opposite_relation {
                Some(relation) => (opposite_model, Some(relation.as_ref())),
                None => (opposite_model, None)
            }
        }
    }
}

unsafe impl Send for Graph { }
unsafe impl Sync for Graph { }
