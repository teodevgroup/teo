use teo::teo_derive::{Entity, Schema};

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

#[test]
fn a() {
    println!("good");
}
