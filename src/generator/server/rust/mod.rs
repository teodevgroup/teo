use std::collections::HashSet;
use std::sync::Arc;
use async_trait::async_trait;
use inflector::Inflector;
use itertools::Itertools;
use maplit::hashset;
use tokio::fs;
use toml_edit::{Document, value};
use crate::core::app::conf::EntityGeneratorConf;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::model::Model;
use crate::core::r#enum::Enum;
use crate::core::relation::Relation;
use crate::generator::lib::code::Code;
use crate::generator::lib::generator::Generator;
use crate::generator::server::EntityGenerator;
use crate::prelude::Graph;

pub(crate) struct RustEntityGenerator {}

impl RustEntityGenerator {

    pub fn new() -> Self {
        Self { }
    }

    fn relation_getter_type(&self, relation: &Arc<Relation>) -> String {
        let model = relation.model();
        if relation.is_vec() {
            format!("Vec<{}>", model)
        } else if relation.is_optional() {
            format!("Option<{}>", model)
        } else {
            model.to_owned()
        }
    }

    fn setter_type_for_field<T: FieldTypeOwner>(&self, field: &T) -> String {
        if field.is_optional() && !field.field_type().is_string() {
            format!("Option<{}>", self.setter_type_for_field_type(field.field_type(), false))
        } else {
            self.setter_type_for_field_type(field.field_type(), field.is_optional())
        }
    }

    fn getter_type_for_field<T: FieldTypeOwner>(&self, field: &T) -> String {
        if field.is_optional() {
            format!("Option<{}>", self.getter_type_for_field_type(field.field_type()))
        } else {
            self.getter_type_for_field_type(field.field_type())
        }
    }

    fn getter_type_for_field_type(&self, field_type: &FieldType) -> String {
        match field_type {
            FieldType::ObjectId => "ObjectId".to_owned(),
            FieldType::Bool => "bool".to_owned(),
            FieldType::I32 => "i32".to_owned(),
            FieldType::I64 => "i64".to_owned(),
            FieldType::F32 => "f32".to_owned(),
            FieldType::F64 => "f64".to_owned(),
            FieldType::Decimal => panic!("Decimal is not supported yet."),
            FieldType::String => "String".to_owned(),
            FieldType::Date => "NaiveDate".to_owned(),
            FieldType::DateTime => "DateTime<Utc>".to_owned(),
            FieldType::Enum(name) => name.clone(),
            FieldType::Vec(inner) => format!("Vec<{}>", self.getter_type_for_field(inner.as_ref())),
            FieldType::HashMap(inner) => format!("HashMap<String, {}>", self.getter_type_for_field(inner.as_ref())),
            FieldType::BTreeMap(inner) => format!("BTreemap<String, {}>", self.getter_type_for_field(inner.as_ref())),
            FieldType::Object(name) => name.clone(),
        }
    }

    fn setter_type_for_field_type(&self, field_type: &FieldType, optional: bool) -> String {
        match field_type {
            FieldType::String => if optional { "impl Into<Option<String>>".to_owned() } else { "impl Into<String>".to_owned() },
            _ => self.getter_type_for_field_type(field_type)
        }
    }

    async fn generate_file_for_model(&self, name: String, model: &Model, generator: &Generator) -> std::io::Result<HashSet<&str>> {
        let mut package_requirements = hashset![];
        let model_name = model.name();
        generator.generate_file(format!("{}.rs", name), Code::new(0, 4, |b| {
            // use lines
            b.line("use std::{collections::HashMap, fmt::{Debug, Display, Formatter}};");
            b.line("use teo::prelude::{Graph, Object, Value, Result};");
            #[cfg(feature = "data-source-mongodb")]
            if model.fields().iter().find(|f| f.field_type().is_object_id()).is_some() {
                b.line("use bson::oid::ObjectId;");
                package_requirements.insert("bson");
            }
            let mut chrono_requirements = vec![];
            if model.fields().iter().find(|f| f.field_type().is_date()).is_some() {
                chrono_requirements.push("NaiveDate");
                package_requirements.insert("chrono");
            }
            if model.fields().iter().find(|f| f.field_type().is_datetime()).is_some() {
                chrono_requirements.push("DateTime");
                chrono_requirements.push("Utc");
                package_requirements.insert("chrono");
            }
            match chrono_requirements.len() {
                0 => (),
                1 => b.line(format!("use chrono::prelude::{};", chrono_requirements.get(0).unwrap())),
                _ => b.line(format!("use chrono::prelude::{{{}}};", chrono_requirements.join(", "))),
            }
            let mut enum_names = vec![];
            let mut has_optional_enum = false;
            model.fields().iter().for_each(|f| {
               match f.field_type() {
                   FieldType::Enum(name) => {
                       enum_names.push(name);
                       if f.is_optional() {
                           has_optional_enum = true;
                       }
                   },
                   _ => (),
               }
            });
            if has_optional_enum {
                b.line("use std::str::FromStr;");
            }
            for enum_name in enum_names {
                b.line(format!("use super::{}::{};", enum_name.to_snake_case(), enum_name));
            }
            let relation_uses: Vec<&str> = model.relations().iter().map(|relation| {
                relation.model()
            }).collect();
            for model_name in relation_uses {
                b.line(format!("use super::{}::{};", model_name.to_snake_case(), model_name));
            }
            b.line("");
            // struct and impl
            b.block(format!("pub struct {model_name} {{"), |b| {
                b.line("pub(super) inner: Object");
            }, "}");
            b.line("");
            b.block(format!("impl {model_name} {{"), |b| {
                b.line("");
                b.line(format!(r#"pub async fn find_many(query: &Value) -> Result<Vec<{model_name}>> {{
        Graph::current().find_many("{model_name}", query).await
    }}

    pub async fn find_unique(query: &Value) -> Result<{model_name}> {{
        Graph::current().find_unique("{model_name}", query).await
    }}

    pub async fn find_first(query: &Value) -> Result<{model_name}> {{
        Graph::current().find_first("{model_name}", query).await
    }}"#));
                b.line("");
                b.line(format!(r#"pub async fn new(values: impl AsRef<Value>) -> Self {{
        Self {{
            inner: Graph::current().create_object("{model_name}", values).await.unwrap(),
        }}
    }}"#));
                b.line("");
                b.line(format!(r#"pub async fn default() -> Self {{
        Self {{
            inner: Graph::current().create_object("{model_name}", Value::HashMap(HashMap::new())).await.unwrap(),
        }}
    }}

    pub fn is_new(&self) -> bool {{
        self.inner.is_new()
    }}

    pub fn is_modified(&self) -> bool {{
        self.inner.is_modified()
    }}

    pub async fn set(&self, values: impl AsRef<Value>) -> Result<()> {{
        self.inner.set_teon(values.as_ref()).await
    }}

    pub async fn update(&self, values: impl AsRef<Value>) -> Result<()> {{
        self.inner.update_teon(values.as_ref()).await
    }}

    pub async fn save(&self) -> Result<()> {{
        self.inner.save().await
    }}

    pub async fn delete(&self) -> Result<()> {{
        self.inner.delete().await
    }}"#));
                b.line("");
                // field getters and setters
                for field in model.fields() {
                    let field_method_name = field.name.to_snake_case();
                    b.block(format!("pub fn {}(&self) -> {} {{", &field_method_name, self.getter_type_for_field(field.as_ref())), |b| {
                        if field.field_type().is_enum() && field.is_optional() {
                            let enum_name = field.field_type().enum_name();
                            b.block(format!("match self.inner.get(\"{}\").unwrap() {{", field.name()), |b| {
                                b.line("Value::Null => None,");
                                b.line(format!("Value::String(s) => Some({}::from_str(&s).unwrap()),", enum_name));
                                b.line("_ => panic!(),");
                            }, "}");
                        } else {
                            b.line(format!("self.inner.get(\"{}\").unwrap()", field.name()));
                        }
                    }, "}");
                    b.line("");
                    b.block(format!("pub fn set_{}(&self, new_value: {}) {{", &field_method_name, self.setter_type_for_field(field.as_ref())), |b| {
                        if field.field_type().is_enum() && field.is_optional() {
                            b.block(format!("self.inner.set(\"{}\", match new_value {{", field.name()), |b| {
                                b.line("Some(v) => v.into(),");
                                b.line("None => Value::Null,");
                            }, "}).unwrap();");
                        } else if field.field_type().is_string() {
                            b.line(format!("self.inner.set(\"{}\", new_value.into()).unwrap();", field.name()));
                        } else {
                            b.line(format!("self.inner.set(\"{}\", new_value).unwrap();", field.name()));
                        }
                    }, "}");
                    b.line("");
                }
                // relations
                for relation in model.relations() {
                    let relation_name = relation.name();
                    let relation_method_name = relation_name.to_snake_case();
                    let model_name = relation.model();
                    if relation.is_vec() {
                        b.block(format!("pub async fn {}(&self, find_many_input: impl AsRef<Value>) -> {} {{", &relation_method_name, self.relation_getter_type(relation)), |b| {
                            b.line(format!("let objects = self.inner.force_get_relation_objects(\"{}\", find_many_input.as_ref()).await.unwrap();", relation_name));
                            b.line(format!("objects.iter().map(|o| {} {{ inner: o.clone() }}).collect()", model_name));
                        }, "}");
                        b.empty_line();
                        b.block(format!("pub fn set_{}(&self, {}: {}) {{", &relation_method_name, &relation_method_name, self.relation_getter_type(relation)), |b| {
                            b.line(format!("let objects = {}.iter().map(|o| o.inner.clone()).collect();", &relation_method_name));
                            b.line(format!("self.inner.force_set_relation_objects(\"{}\", objects)", relation_name));
                        }, "}");
                        b.empty_line();
                        b.block(format!("pub fn add_to_{}(&self, {}: {}) {{", &relation_method_name, &relation_method_name, self.relation_getter_type(relation)), |b| {
                            b.line(format!("let objects = {}.iter().map(|o| o.inner.clone()).collect();", &relation_method_name));
                            b.line(format!("self.inner.force_add_relation_objects(\"{}\", objects)", relation_name));
                        }, "}");
                        b.empty_line();
                        b.block(format!("pub fn remove_from_{}(&self, {}: {}) {{", &relation_method_name, &relation_method_name, self.relation_getter_type(relation)), |b| {
                            b.line(format!("let objects = {}.iter().map(|o| o.inner.clone()).collect();", &relation_method_name));
                            b.line(format!("self.inner.force_remove_relation_objects(\"{}\", objects)", relation_name));
                        }, "}");
                        b.empty_line();
                    } else {
                        b.block(format!("pub async fn {}(&self) -> {} {{", &relation_method_name, self.relation_getter_type(relation)), |b| {
                            b.line(format!("let object = self.inner.force_get_relation_object(\"{}\").await.unwrap();", relation_name));
                            if relation.is_optional() {
                                b.block("match object {", |b| {
                                    b.line(format!("Some(object) => Some({} {{ inner: object }}),", model_name));
                                    b.line(format!("None => None,"));
                                }, "}");
                            } else {
                                b.line(format!("{} {{ inner: object.unwrap() }}", model_name));
                            }
                        }, "}");
                        b.empty_line();
                        b.block(format!("pub fn set_{}(&self, {}: {}) {{", &relation_method_name, &relation_method_name, self.relation_getter_type(relation)), |b| {
                            b.line(format!("self.inner.force_set_relation_object(\"{}\", {})", relation_name, if relation.is_optional() {
                                format!("{}.map(|o| o.inner.clone())", relation_method_name)
                            } else {
                                format!("Some({}.inner.clone())", &relation_method_name)
                            }));
                        }, "}");
                        b.empty_line();
                    }
                }
                // properties
                for property in model.properties() {
                    let property_name = property.name();
                    let property_method_name = property.name.to_snake_case();
                    if property.getter.is_some() {
                        b.block(format!("pub async fn {}(&self) -> {} {{", &property_method_name, self.getter_type_for_field(property.as_ref())), |b| {
                            if property.field_type().is_enum() && property.is_optional() {
                                let enum_name = property.field_type().enum_name();
                                b.block(format!("match self.inner.get_property(\"{}\").await.unwrap() {{", property.name()), |b| {
                                    b.line("Value::Null => None,");
                                    b.line(format!("Value::String(s) => Some({}::from_str(&s).unwrap()),", enum_name));
                                    b.line("_ => panic!(),");
                                }, "}");
                            } else {
                                b.line(format!("self.inner.get_property(\"{}\").await.unwrap()", property.name()));
                            }
                        }, "}");
                        b.line("");
                    }
                    if property.setter.is_some() {
                        b.block(format!("pub async fn set_{}(&self, new_value: {}) {{", &property_method_name, self.setter_type_for_field(property.as_ref())), |b| {
                            if property.field_type().is_enum() && property.is_optional() {
                                b.block(format!("self.inner.set_property(\"{}\", match new_value {{", property.name()), |b| {
                                    b.line("Some(v) => v.into(),");
                                    b.line("None => Value::Null,");
                                }, "}).await.unwrap();");
                            } else if property.field_type().is_string() {
                                b.line(format!("self.inner.set_property(\"{}\", new_value.into()).await.unwrap();", property_name));
                            } else {
                                b.line(format!("self.inner.set_property(\"{}\", new_value).await.unwrap();", property_name));
                            }
                        }, "}");
                        b.line("");
                    }
                }
            }, "}");
            // shared traits
            b.line(format!(r#"
impl Into<Object> for {model_name} {{
    fn into(self) -> Object {{
        self.inner.clone()
    }}
}}

impl From<Object> for {model_name} {{
    fn from(value: Object) -> Self {{
        Self {{ inner: value }}
    }}
}}

impl Into<Value> for {model_name} {{
    fn into(self) -> Value {{
        Value::Object(self.into())
    }}
}}

impl From<Value> for {model_name} {{
    fn from(value: Value) -> Self {{
        Self::from(value.as_object().unwrap().clone())
    }}
}}

impl Debug for {model_name} {{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
        Debug::fmt(&self.inner, f)
    }}
}}

impl Display for {model_name} {{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
        Display::fmt(&self.inner, f)
    }}
}}
"#));
        }).to_string()).await?;
        Ok(package_requirements)
    }

    async fn generate_file_for_enum(&self, name: String, e: &Enum, generator: &Generator) -> std::io::Result<()> {
        let enum_name = e.name();
        generator.generate_file(format!("{name}.rs"), Code::new(0, 4, |b| {
            // use lines
            b.line("use std::str::FromStr;");
            b.line("use teo::prelude::{Value, Error};");
            b.empty_line();
            b.line("#[derive(Eq, PartialEq, Copy, Clone, Debug)]");
            b.block(format!("pub enum {} {{", enum_name), |b| {
                for choice in e.choices() {
                    b.line(format!("{},", choice.name()));
                }
            }, "}");
            b.empty_line();
            b.block(format!("impl ToString for {enum_name} {{"), |b| {
                b.block("fn to_string(&self) -> String {", |b| {
                    b.block("match self {", |b| {
                        for choice in e.choices() {
                            b.line(format!("{enum_name}::{} => \"{}\".to_string(),", choice.name(), choice.name()));
                        }
                    }, "}");
                }, "}");
            }, "}");
            b.empty_line();
            b.block(format!("impl FromStr for {enum_name} {{"), |b| {
                b.line("type Err = Error;");
                b.empty_line();
                b.block("fn from_str(s: &str) -> Result<Self, Self::Err> {", |b| {
                    b.block("match s {", |b| {
                        for choice in e.choices() {
                            b.line(format!("\"{}\" => Ok({enum_name}::{}),", choice.name(), choice.name()));
                        }
                        b.line(format!("_ => Err(Error::custom_error(format!(\"Cannot convert value '{{}}' to `{enum_name}'.\", s))),"));
                    }, "}");
                }, "}");
            }, "}");
            b.empty_line();
            b.line(format!(r#"impl Into<Value> for {enum_name} {{
    fn into(self) -> Value {{
        Value::String(self.to_string())
    }}
}}

impl From<Value> for {enum_name} {{
    fn from(value: Value) -> Self {{
        Self::from_str(value.as_str().unwrap()).unwrap()
    }}
}}"#));
        }).to_string()).await?;
        Ok(())
    }

    async fn generate_mod_rs(&self, names: Vec<String>, generator: &Generator) -> std::io::Result<()> {
        generator.generate_file("mod.rs", Code::new(0, 4, |b| {
            for name in names.iter() {
                b.line(format!("pub mod {};", name));
            }
        }).to_string()).await?;
        Ok(())
    }

    async fn find_and_update_cargo_toml(&self, package_requirements: Vec<&str>, generator: &Generator) {
        let cargo_toml = match generator.find_file_upwards("Cargo.toml") {
            Some(path) => path,
            None => return,
        };
        let toml = fs::read_to_string(&cargo_toml).await.unwrap();
        let mut doc = toml.parse::<Document>().expect("`Cargo.toml' has invalid content");
        let deps = doc.get_mut("dependencies").unwrap();
        if package_requirements.contains(&"chrono") {
            deps["chrono"]["version"] = value("0.4.23");
        }
        if package_requirements.contains(&"bson") {
            deps["bson"]["version"] = value("2.3.0");
        }
        fs::write(cargo_toml, doc.to_string()).await.unwrap();
    }
}

#[async_trait]
impl EntityGenerator for RustEntityGenerator {
    async fn generate_entity_files(&self, graph: &Graph, _conf: &EntityGeneratorConf, generator: &Generator) -> std::io::Result<()> {
        let mut names: Vec<String> = vec![];
        for (name, e) in graph.enums() {
            let name = name.to_snake_case();
            names.push(name.clone());
            self.generate_file_for_enum(name, e, generator).await?;
        }
        let mut package_requirements = vec![];
        for model in graph.models() {
            let name = model.name().to_snake_case();
            names.push(name.clone());
            let result = self.generate_file_for_model(name, model, generator).await?;
            for item in result {
                if !package_requirements.contains(&item) {
                    package_requirements.push(item);
                }
            }
        }
        self.generate_mod_rs(names, generator).await?;
        if package_requirements.len() > 0 {
            self.find_and_update_cargo_toml(package_requirements, generator).await;
        }
        Ok(())
    }
}
