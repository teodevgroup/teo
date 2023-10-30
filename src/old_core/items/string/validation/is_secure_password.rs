use async_trait::async_trait;
use regex::Regex;
use crate::core::item::Item;
use crate::core::pipeline::ctx::PipelineCtx;
use crate::core::result::Result;
#[derive(Debug, Clone)]
pub struct IsSecurePasswordItem {
    patterns: Vec<Regex>
}

impl IsSecurePasswordItem {
    pub fn new() -> Self {
        return IsSecurePasswordItem {
            patterns: vec![
                Regex::new(r#"[A-Z]"#).unwrap(),
                Regex::new(r#"[a-z]"#).unwrap(),
                Regex::new(r#"\d"#).unwrap(),
                Regex::new(r#"[!@#$&*`~()\-_+=\[\]{}:;'",<>.?\\|/]"#).unwrap(),
            ]
        };
    }
}

#[async_trait]
impl Item for IsSecurePasswordItem {
    async fn call<'a>(&self, ctx: PipelineCtx<'a>) -> Result<PipelineCtx<'a>> {
        match ctx.value.as_str() {
            Some(s) => {
                for regex in &self.patterns {
                    if !regex.is_match(&s) {
                        return Err(ctx.with_invalid("value is not secure password"));
                    }
                }
                Ok(ctx)
            }
            None => Err(ctx.internal_server_error("isSecurePassword: value is not string"))
        }
    }
}
