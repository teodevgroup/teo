use tokio::test;
use teo::core::graph::Graph;


async fn make_mysql_graph() -> &'static Graph {

    let graph = Box::leak(Box::new(Graph::new(|g| {

        g.data_source().mysql("mysql://root:@localhost/teoteotestindex");

        g.model("UniqueIndex", |m| {
            m.field("id", |f| {
                f.primary().readonly().required().i64().auto_increment();
            });
            m.field("unique", |f| {
                f.unique().optional().string();
            });
        });

        g.model("Index", |m| {
            m.field("id", |f| {
                f.primary().readonly().required().i64().auto_increment();
            });
            m.field("index", |f| {
                f.index().optional().string();
            });
        });

        g.host_url("http://example.com");

    }).await));

    graph
}

// #[test]
// async fn unique_value_cannot_have_duplications_on_create() {
//     let graph = make_mysql_graph().await;
//     let object1 = graph.new_object("UniqueIndex");
//     let _ = object1.set_value("unique", Value::String("123".to_string()));
//     let _ = object1.save().await;
//     let object2 = graph.new_object("UniqueIndex");
//     let _ = object2.set_value("unique", Value::String("123".to_string()));
//     let result = object2.save().await;
//     assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("unique"));
// }
//
// #[test]
// async fn unique_value_cannot_have_duplications_on_update() {
//     let graph = make_mysql_graph().await;
//     let object1 = graph.new_object("UniqueIndex");
//     let _ = object1.set_value("unique", Value::String("123".to_string()));
//     let _ = object1.save().await;
//     let object2 = graph.new_object("UniqueIndex");
//     let _ = object2.set_value("unique", Value::String("222".to_string()));
//     let _ = object2.save().await;
//     let _ = object2.set_value("unique", Value::String("123".to_string()));
//     let result = object2.save().await;
//     assert_eq!(result.err().unwrap(), ActionError::unique_value_duplicated("unique"));
// }
//
// #[test]
// async fn unique_value_can_have_duplicated_nulls() {
//     let graph = make_mysql_graph().await;
//     let object1 = graph.new_object("UniqueIndex");
//     let _ = object1.save().await;
//     let object2 = graph.new_object("UniqueIndex");
//     let result = object2.save().await;
//     assert_eq!(result.ok(), None);
// }

#[tokio::test]
async fn index_field_is_indexed() {
    let graph = make_mysql_graph().await;
}
