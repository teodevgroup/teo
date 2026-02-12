use teo::{Entity, Schema, migration::r#async::migrate};
use tokio_postgres::{self, Client, NoTls};

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

async fn connect(s: &str) -> Client {
    let (client, connection) = tokio_postgres::connect(s, NoTls).await.unwrap();
    tokio::spawn(connection);
    client
}

#[tokio::test]
async fn test_migrate() {
    let mut client = connect("host=localhost port=5432 user=postgres").await;
    migrate::<Client, Schema>(&mut client).await.unwrap();
}
