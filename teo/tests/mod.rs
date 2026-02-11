use teo_derive::Entity;
use teo::teo_column_type::PostgresColumnType;

#[derive(Entity)]
#[teo(table_name = "User")]
#[teo(index(
    column(name = a, order = "asc"),
    column(name = b),
    name = "my_index"))]
struct User {
    #[teo(column_name = "a", primary)]
    #[teo(postgres(column_type = "text"))]
    a: String,
    #[teo(postgres(column_type = "int"))]
    b: i32,
}
