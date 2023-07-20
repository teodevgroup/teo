use std::collections::BTreeSet;
use async_trait::async_trait;
use askama::Template;
use maplit::btreeset;
use tokio::fs;
use toml_edit::{Document, value};
use crate::gen::interface::server::conf::Conf;
use crate::gen::internal::server::ctx::Ctx;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::server::generator::EntityGenerator;
use crate::gen::internal::server::outline::outline::EntityOutline;
use crate::prelude::Graph;
use crate::gen::internal::filters;

#[derive(Template)]
#[template(path = "server/rust/mod.rs.jinja", escape = "none")]
pub(self) struct RustMainTemplate<'a> {
    pub(self) outline: &'a EntityOutline<'a>,
    pub(self) conf: &'a Conf,
    pub(self) has_date: bool,
    pub(self) has_datetime: bool,
    pub(self) has_decimal: bool,
    pub(self) has_object_id: bool,
}

impl<'a> RustMainTemplate<'a> {
    fn new(outline: &'a EntityOutline<'a>, conf: &'a Conf) -> Self {
        let has_date = outline.classes.iter().find(|c| c.fields.iter().find(|f| {
            !f.kind.is_relation() &&
                (f.input_field_type.as_ref().contains("NaiveDate") ||
                    f.output_field_type.as_ref().contains("NaiveDate"))
        }).is_some()).is_some();
        let has_datetime = outline.classes.iter().find(|c| c.fields.iter().find(|f| {
            !f.kind.is_relation() &&
                (f.input_field_type.as_ref().contains("DateTime<Utc>") ||
                    f.output_field_type.as_ref().contains("DateTime<Utc>"))
        }).is_some()).is_some();
        let has_decimal = outline.classes.iter().find(|c| c.fields.iter().find(|f| {
            !f.kind.is_relation() &&
                (f.input_field_type.as_ref().contains("BigDecimal") ||
                    f.output_field_type.as_ref().contains("BigDecimal"))
        }).is_some()).is_some();
        let has_object_id = outline.classes.iter().find(|c| c.fields.iter().find(|f| {
            !f.kind.is_relation() &&
                (f.input_field_type.as_ref().contains("ObjectId") ||
                    f.output_field_type.as_ref().contains("ObjectId"))
        }).is_some()).is_some();
        Self {
            outline,
            conf,
            has_date,
            has_datetime,
            has_decimal,
            has_object_id,
        }
    }
}

pub(in crate::gen) struct RustEntityGenerator { }

impl RustEntityGenerator {
    pub(in crate::gen) fn new() -> Self {
        Self { }
    }
}

impl RustEntityGenerator {
    async fn find_and_update_cargo_toml(&self, package_requirements: &BTreeSet<&str>, generator: &FileUtil) {
        let cargo_toml = match generator.find_file_upwards("Cargo.toml") {
            Some(path) => path,
            None => return,
        };
        let toml = fs::read_to_string(&cargo_toml).await.unwrap();
        let mut doc = toml.parse::<Document>().expect("`Cargo.toml' has invalid content");
        let deps = doc.get_mut("dependencies").unwrap();
        if package_requirements.contains(&"chrono") {
            if deps.get("chrono").is_none() {
                deps["chrono"]["version"] = value("0.4.23");
            }
        }
        if package_requirements.contains(&"bson") {
            if deps.get("bson").is_none() {
                deps["bson"]["version"] = value("2.3.0");
            }
        }
        if package_requirements.contains(&"bigdecimal") {
            if deps.get("bigdecimal").is_none() {
                deps["bigdecimal"]["version"] = value("0.3.0");
            }
        }
        fs::write(cargo_toml, doc.to_string()).await.unwrap();
    }
}

#[async_trait]
impl EntityGenerator for RustEntityGenerator {
    async fn generate_entity_files(&self, ctx: &Ctx, generator: &FileUtil) -> crate::prelude::Result<()> {
        let template = RustMainTemplate::new(&ctx.entity_outline, ctx.conf);
        generator.generate_file("mod.rs", template.render().unwrap()).await?;
        // Modify files
        let mut package_requirements = btreeset![];
        if template.has_date || template.has_datetime {
            package_requirements.insert("chrono");
        }
        if template.has_decimal {
            package_requirements.insert("bigdecimal");
        }
        if template.has_object_id {
            package_requirements.insert("bson");
        }
        if !package_requirements.is_empty() {
            self.find_and_update_cargo_toml(&package_requirements, generator).await;
        }
        Ok(())
    }
}