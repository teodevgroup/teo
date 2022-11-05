use crate::core::graph::Graph;

pub(crate) async fn generate_sort_order_cs(_graph: &Graph) -> String {
    format!(r#"namespace Teo {{
    public enum SortOrder {{
        Asc,
        Desc,
    }}
}}
"#)
}