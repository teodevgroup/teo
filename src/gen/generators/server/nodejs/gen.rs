use async_trait::async_trait;
use askama::Template;

use crate::gen::interface::server::conf::Conf;
use crate::gen::internal::server::ctx::Ctx;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::server::generator::EntityGenerator;
use crate::gen::internal::server::outline::outline::EntityOutline;

use crate::gen::internal::filters;

#[derive(Template)]
#[template(path = "server/nodejs/index.js.jinja", escape = "none")]
pub(self) struct NodeJSIndexJsTemplate<'a> {
    pub(self) outline: &'a EntityOutline<'a>,
    pub(self) conf: &'a Conf,
}

impl<'a> NodeJSIndexJsTemplate<'a> {
    fn new(outline: &'a EntityOutline<'a>, conf: &'a Conf) -> Self {
        Self {
            outline,
            conf,
        }
    }
}

#[derive(Template)]
#[template(path = "server/nodejs/index.d.ts.jinja", escape = "none")]
pub(self) struct NodeJSIndexDTsTemplate<'a> {
    pub(self) outline: &'a EntityOutline<'a>,
    pub(self) conf: &'a Conf,
    pub(self) has_date: bool,
    pub(self) has_decimal: bool,
    pub(self) has_object_id: bool,
    pub(self) shared_interface: String,
}

impl<'a> NodeJSIndexDTsTemplate<'a> {
    fn new(outline: &'a EntityOutline<'a>, conf: &'a Conf) -> Self {
        let has_date = outline.classes.iter().find(|c| c.fields.iter().find(|f| {
            !f.kind.is_relation() &&
                (f.input_field_type.as_ref().contains("DateOnly") ||
                    f.output_field_type.as_ref().contains("DateOnly"))
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
        //let shared_interface = generate_index_d_ts(graph, "teo".to_owned(), true);
        let shared_interface = "".to_string();
        Self {
            outline,
            conf,
            has_date,
            has_decimal,
            has_object_id,
            shared_interface,
        }
    }
}

pub(in crate::gen) struct NodeJSEntityGenerator { }

impl NodeJSEntityGenerator {
    pub(in crate::gen) fn new() -> Self {
        Self { }
    }
}

#[async_trait]
impl EntityGenerator for NodeJSEntityGenerator {
    async fn generate_entity_files(&self, ctx: &Ctx, generator: &FileUtil) -> crate::prelude::Result<()> {
        let template = NodeJSIndexDTsTemplate::new(&ctx.entity_outline, ctx.conf);
        generator.generate_file("index.d.ts", template.render().unwrap()).await?;
        Ok(())
    }
}