// use crate::{model::Model, select::Select};

// pub struct User {
//     id: i32,
//     name: String,
//     age: i32,
// }

// impl Model for User {

//     fn table_name(&self) -> &'static str {
//         "User"
//     }

//     fn table_name_quoted(&self) -> &'static str {
//         "'User'"
//     }
// }

// mod columns {

//     use crate::{row::Row, select::Select, user::UserNameSelect};

//     pub struct Id;

//     impl Select for Id {
//         type Value = i32;
//         fn extract(row: &Row) -> Self::Value {
//             7
//         }
//     }

//     pub struct Name;

//     impl Select for Name {
//         type Value = String;
//         fn extract(_row: Row) -> Self::Value {
//             "".to_owned()
//         }
//     }

//     impl UserNameSelect for Name { }

//     pub struct Age;

//     impl Select for Age {
//         type Value = i32;
//         fn extract(_row: &Row) -> Self::Value {
//             0
//         }
//     }
// }

// impl UserIdAccess for User {
//     #[inline]
//     fn id(&self) -> i32 {
//         self.id
//     }
// }

// impl UserNameAccess for User {
//     #[inline]
//     fn name(&self) -> &str {
//         &self.name
//     }
// }

// impl UserAgeAccess for User {
//     #[inline]
//     fn age(&self) -> i32 {
//         self.age
//     }
// }

// impl<T0, T1> UserIdAccess for Pick<User, (columns::Id, T0, T1)> {
//     fn id(&self) -> i32 {
//         self.0.0
//     }
// }

// impl<T0, T1> UserAgeAccess for Pick<User, (T0, columns::Age, T1)> {
//     fn age(&self) -> i32 {
//         self.0.1
//     }
// }

// impl<T0, T1> UserAgeAccess for Pick<User, (columns::Age, T0, T1)> {
//     fn age(&self) -> i32 {
//         self.0.0
//     }
// }

// impl<T0> UserAgeAccess for Pick<User, (columns::Age, T0)> where T0: Select {
//     fn age(&self) -> i32 {
//         self.0.0
//     }
// }
