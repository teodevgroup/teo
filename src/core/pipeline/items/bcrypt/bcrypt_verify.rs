use async_trait::async_trait;
use bcrypt::verify;
use crate::core::result::Result;
use crate::core::pipeline::item::Item;
use crate::core::pipeline::ctx::Ctx;
use crate::core::pipeline::Pipeline;

#[derive(Debug, Clone)]
pub struct BcryptVerifyItem {
    argument: Pipeline
}

impl BcryptVerifyItem {
    pub fn new(argument: Pipeline) -> Self {
        Self { argument }
    }
}

#[async_trait]
impl Item for BcryptVerifyItem {

    async fn call<'a>(&self, ctx: Ctx<'a>) -> Result<Ctx<'a>> {
        println!("HERE INTO?");
        match ctx.value.as_str() {
            None => {
                println!("HERE MAYBE?");
                Err(ctx.internal_server_error("bcryptVerify: value is not string"))
            }
            Some(string) => {
                println!("HERE FIND>?");
                let argument = self.argument.process(ctx.clone()).await?;
                match argument.as_str() {
                    None => Err(ctx.internal_server_error("bcryptVerify: argument is not string")),
                    Some(hash) => {
                        println!("see hash {}; see ctx value {}", hash, string);
                        if verify(string, hash).unwrap() {
                            Ok(ctx.clone())
                        } else {
                            Err(ctx.with_invalid("value is not correct"))
                        }
                    }
                }
            }
        }
    }
}
