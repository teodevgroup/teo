use async_trait::async_trait;
use inflector::Inflector;
use crate::core::app::conf::EntityGeneratorConf;
use crate::core::field::Field;
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;
use crate::core::r#enum::Enum;
use crate::generator::lib::code::Code;
use crate::generator::lib::generator::Generator;
use crate::generator::server::EntityGenerator;
use crate::prelude::Graph;

pub(crate) struct RustEntityGenerator {}

impl RustEntityGenerator {

    pub fn new() -> Self {
        Self {}
    }

    fn setter_type_for_field(&self, field: &Field) -> String {
        if field.is_optional() && !field.field_type().is_string() {
            format!("Option<{}>", self.setter_type_for_field_type(field.field_type(), false))
        } else {
            self.setter_type_for_field_type(field.field_type(), field.is_optional())
        }
    }

    fn getter_type_for_field(&self, field: &Field) -> String {
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

    async fn generate_file_for_model(&self, name: String, model: &Model, generator: &Generator) -> std::io::Result<()> {
        let model_name = model.name();
        generator.generate_file(format!("{}.rs", name), Code::new(0, 4, |b| {
            // use lines
            b.line("use std::{collections::HashMap, fmt::{Debug, Display, Formatter}};");
            b.line("use teo::prelude::{Graph, Object, Value, Result};");
            #[cfg(feature = "data-source-mongodb")]
            if model.fields().iter().find(|f| f.field_type().is_object_id()).is_some() {
                b.line("use bson::oid::ObjectId;");
            }
            let mut chrono_requirements = vec![];
            if model.fields().iter().find(|f| f.field_type().is_date()).is_some() {
                chrono_requirements.push("NaiveDate");
            }
            if model.fields().iter().find(|f| f.field_type().is_datetime()).is_some() {
                chrono_requirements.push("DateTime");
                chrono_requirements.push("Utc");
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
            b.line("");
            // struct and impl
            b.block(format!("pub struct {model_name} {{"), |b| {
                b.line("inner: Object");
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
                b.line(format!(r#"pub async fn new(values: Value) -> Self {{
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

    pub async fn save(&self) -> Result<()> {{
        self.inner.save().await
    }}"#));
                b.line("");
                // field getters and setters
                for field in model.fields() {
                    let field_method_name = field.name.to_snake_case();
                    b.block(format!("pub fn {}(&self) -> {} {{", &field_method_name, self.getter_type_for_field(field.as_ref())), |b| {
                        if field.field_type().is_enum() && field.is_optional() {
                            b.block(format!("match self.inner.get(\"{}\").unwrap() {{", field.name()), |b| {
                                b.line("Value::Null => None,");
                                b.line("Value::String(s) => Some(Sex::from_str(&s).unwrap()),");
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
                // properties
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
        Ok(())
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
        for model in graph.models() {
            let name = model.name().to_snake_case();
            names.push(name.clone());
            self.generate_file_for_model(name, model, generator).await?;
        }
        self.generate_mod_rs(names, generator).await?;
        Ok(())
    }
}
