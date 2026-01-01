pub struct Row { }

pub trait Select {
    type Value;

    fn extract(row: &Row) -> Self::Value;
}

impl<T0> Select for (T0,) where T0: Select {

    type Value = (T0::Value,);

    fn extract(row: &Row) -> Self::Value {
        (T0::extract(row),)
    }
}

impl<T0, T1> Select for (T0, T1) where T0: Select, T1: Select {
    type Value = (T0::Value, T1::Value);
    fn extract(row: &Row) -> Self::Value {
        (T0::extract(row),T1::extract(row))
    }
}

fn select<T>(pick: T) -> T::Value where T: Select {
    let row = Row { };
    T::extract(&row)
}

mod columns {
    use crate::select::Select;

    struct Name;

    impl Select for Name {
        type Value = String;
        fn extract(_row: &super::Row) -> Self::Value {
            "".to_owned()
        }
    }

    struct Age;

    impl Select for Age {
        type Value = i32;
        fn extract(_row: &super::Row) -> Self::Value {
            0
        }
    }
}
