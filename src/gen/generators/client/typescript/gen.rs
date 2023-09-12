use askama::Template;
use async_trait::async_trait;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::generator::Generator;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::filters;
use crate::core::result::Result;
use crate::gen::generators::client::typescript::pkg::package_json::{generate_package_json, update_package_json};
use crate::gen::generators::client::typescript::pkg::src::index_d_ts::generate_index_d_ts;
use crate::gen::generators::client::typescript::pkg::src::index_js::generate_index_js;

#[derive(Template)]
#[template(path = "client/ts/readme.md.jinja", escape = "none")]
pub(self) struct TsReadMeTemplate<'a> {
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/ts/index.js.jinja", escape = "none")]
pub(self) struct TsIndexJsTemplate<'a> {
    pub(self) outline: &'a Outline<'a>,
    pub(self) conf: &'a Conf,
}

#[derive(Template)]
#[template(path = "client/ts/index.d.ts.jinja", escape = "none")]
pub(self) struct TsIndexDTsTemplate<'a> {
    pub(self) outline: &'a Outline<'a>,
    pub(self) conf: &'a Conf,
    pub(self) ts_conf: &'a TsGenerationConf,
}

pub(in crate::gen) struct TsGenerationConf {
    pub(in crate::gen) datetime_input: &'static str,
    pub(in crate::gen) date_input: &'static str,
    pub(in crate::gen) decimal_input: &'static str,
}

impl TsGenerationConf {

    pub(in crate::gen) fn client() -> Self {
        Self {
            date_input: "string",
            datetime_input: "Date | string",
            decimal_input: "Decimal | string"
        }
    }

    pub(in crate::gen) fn server() -> Self {
        Self {
            date_input: "string",
            datetime_input: "Date",
            decimal_input: "Decimal"
        }
    }
}

pub(crate) struct TsClientGenerator { }

impl TsClientGenerator {
    pub(crate) fn new() -> Self { Self { } }
}

#[async_trait]
impl Generator for TsClientGenerator {

    fn module_directory_in_package(&self, _conf: &Conf) -> String {
        return "src".to_owned();
    }

    async fn generate_module_files(&self, _ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.clear_root_directory().await
    }

    async fn generate_package_files(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.ensure_root_directory().await?;
        generator.generate_file(".gitignore", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/client/ts/gitignore"))).await?;
        generator.generate_file("README.md", TsReadMeTemplate { conf: ctx.conf }.render().unwrap()).await?;
        if generator.generate_file_if_not_exist("package.json", generate_package_json(generator.get_base_dir())).await? {
            // if exist, update package.json with a minor version
            let json_data = std::fs::read_to_string(generator.get_file_path("package.json"))
                .expect("Unable to read package.json");
            generator.generate_file("package.json", update_package_json(json_data)).await?;
        }
        Ok(())
    }

    async fn generate_main(&self, ctx: &Ctx, generator: &FileUtil) -> Result<()> {
        generator.generate_file("index.d.ts", generate_index_d_ts(ctx.graph, ctx.conf.object_name.clone(), false)).await?;
        generator.generate_file("index.js", generate_index_js(ctx.graph, ctx.conf).await).await?;
        generator.generate_file("new.index.d.ts", TsIndexDTsTemplate {
            outline: &ctx.outline,
            conf: ctx.conf,
            ts_conf: &TsGenerationConf::client(),
        }.render().unwrap()).await?;
        generator.generate_file("new.index.js", TsIndexJsTemplate { outline: &ctx.outline, conf: ctx.conf }.render().unwrap()).await?;
        Ok(())
    }
}
