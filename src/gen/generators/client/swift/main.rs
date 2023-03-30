use inflector::Inflector;
use askama::Template;
use crate::core::r#enum::Enum;
use crate::gen::generators::client::swift::types::SwiftTypes;
use crate::gen::interface::client::conf::Conf;
use crate::gen::internal::client::ctx::Ctx;
use crate::gen::internal::client::outline::outline::Outline;
use crate::gen::internal::file_util::FileUtil;
use crate::prelude::Graph;
use crate::gen::internal::filters;

#[derive(Template)]
#[template(path = "client/swift/teo.swift.jinja", escape = "none")]
pub(self) struct SwiftMainTemplate<'a> {
    pub(self) outline: &'a Outline<'a>,
}
pub(super) async fn generate_swift_main<'a>(ctx: &'a Ctx<'a>, file_util: &FileUtil) -> std::io::Result<()> {
    file_util.generate_file("Teo.swift", SwiftMainTemplate {
        outline: &ctx.outline,
    }.render().unwrap()).await
}
