pub(crate) trait ClientGenerator {
    fn generate_main(graph: Graph, client: Client, ) -> ();
    fn generate_accessories(graph: Graph, client: Client) -> ();
}
