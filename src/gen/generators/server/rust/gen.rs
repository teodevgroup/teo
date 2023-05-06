use async_trait::async_trait;
use askama::Template;
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

#[async_trait]
impl EntityGenerator for RustEntityGenerator {
    async fn generate_entity_files(&self, ctx: &Ctx, generator: &FileUtil) -> crate::prelude::Result<()> {
        generator.generate_file("mod.rs", RustMainTemplate::new(&ctx.entity_outline, ctx.conf).render().unwrap()).await?;
        Ok(())
    }
}