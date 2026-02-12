use teo::{Entity, Schema, migration::r#async::migrate};
use mysql_async::{self, Conn, Pool, prelude::Queryable};

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

async fn connect(s: &str) -> Conn {
    let pool = Pool::new(s);
    let mut conn = pool.get_conn().await.unwrap();
    conn.exec_drop("abc", ()).await.unwrap();
    conn
}

#[tokio::test]
async fn test_migrate() {
    let mut client = connect("mysql://localhost:3306/databasename").await;
    migrate::<Conn, Schema>(&mut client).await.unwrap();
}
