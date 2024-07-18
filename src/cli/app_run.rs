use teo_runtime::app::App;
use teo_result::Result;
use teo_runtime::schema::load::load_schema::load_schema;
use crate::cli::run::run;

#[async_trait::async_trait]
pub trait AppExt {
    async fn run(&self) -> Result<()>;
    async fn prepare_for_run(&self) -> Result<()>;
    async fn run_without_prepare(&self) -> Result<()>;
}

#[async_trait::async_trait]
impl AppExt for App {

    async fn run(&self) -> Result<()> {
        self.prepare_for_run().await?;
        self.run_without_prepare().await
    }

    async fn prepare_for_run(&self) -> Result<()> {
        load_schema(self.main_namespace(), self.schema(), self.cli().command.ignores_loading()).await?;
        let namespace = Box::into_raw(Box::new(self.main_namespace().build()));
        self.set_compiled_main_namespace(unsafe { &*namespace });
        Ok(())
    }

    async fn run_without_prepare(&self) -> Result<()> {
        run(self).await
    }

}
