use teo_derive::Entity;

#[derive(Entity)]
#[teo(table_name = "User")]
#[teo(index(
    column(name = a, order = "asc"),
    column(name = b),
    name = "my_index"))]
struct User {
    #[teo(column_name = "a", primary, column_type = "d")]
    a: String,
    #[teo(postgres(column_type = "int"))]
    b: i32,
}
