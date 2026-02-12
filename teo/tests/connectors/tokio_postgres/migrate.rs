use teo_derive::Entity;

#[derive(Entity)]
struct User {
    #[teo(primary, auto_increment)]
    id: i32,
    name: String,
    age: i32,
}
