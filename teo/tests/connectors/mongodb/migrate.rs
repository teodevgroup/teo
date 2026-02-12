use teo::{Entity, Schema, migration::r#async::migrate};
use mongodb::{Database, Client};

#[derive(Entity)]
struct User {
    #[teo(primary, auto_increment)]
    id: i32,
    name: String,
    age: i32,
}

#[derive(Schema)]
#[teo(entity(path = User))]
struct Schema;

async fn connect(c: &str, d: &str) -> Database {
    let client = Client::with_uri_str(c).await.unwrap();
    let database = client.database(d);
    database
}

#[tokio::test]
async fn test_migrate() {
    let mut client = connect("mongodb://127.0.0.1:27017", "mydb2").await;
    migrate::<Database, Schema>(&mut client).await.unwrap();
}
