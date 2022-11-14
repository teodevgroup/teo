// resolve pipeline as value
pub(crate) async fn resolve(&self, context: Context<'_>) -> Value {
    match self {
        ValueArgument(v) => v.clone(),
        PipelineArgument(p) => p.process(context).await.value,
        _ => panic!("Cannot resolve argument.")
    }
}
