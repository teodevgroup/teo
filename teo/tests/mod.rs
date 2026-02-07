use teo_derive::Entity;

#[derive(Entity)]
#[teo(table_name = "User")]
struct User {
    #[teo(column_name = "a", primary, default = "ss", column_type = "d")]
    a: String,
    b: i32,
}
