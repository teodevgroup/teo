use mysql_async::{params, prelude::{BatchQuery, Query, WithParams}, QueryWithParams};

pub struct MyRecord {
  bool: bool,
  string: String,
  int: i64,
  double: f64,
}

impl MyRecord {
  pub fn new() -> Self {
    Self {
      bool: true,
      string: "String Value".to_owned(),
      int: 1,
      double: 0.1,
    }
  }
}

#[tokio::test]
async fn test_postgres() {
  let record = MyRecord::new();
}

#[tokio::test]
async fn test_mysql() {
  let record = MyRecord::new();
  let pool = mysql_async::Pool::new("mysql://localhost:3307/mydb");
  let mut conn = pool.get_conn().await.unwrap();
  r"CREATE TEMPORARY TABLE myrecord (
    bool bool not null,
    string text not null,
    int integer not null,
    double double not null
  )".ignore(&mut conn).await.unwrap();
  "".with(vec![params! {
    "bool" => true,
  }]);
}

#[tokio::test]
async fn test_sqlite() {
  let record = MyRecord::new();
}

#[tokio::test]
async fn test_mongo() {
  let record = MyRecord::new();
}
