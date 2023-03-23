use std::collections::HashMap;
use std::future::Future;
use futures_util::future::BoxFuture;
use serde_json::{Number, Value};

pub enum Matcher {
    Null,
    Ignore,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Matcher>),
    Object(HashMap<String, Matcher>),
    ValueMatcher(Box<dyn Fn(&Value) -> bool>),
}

#[macro_export(local_inner_macros)]
macro_rules! matcher {
    ($($matcher:tt)+) => {
        matcher_internal!($($matcher)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! matcher_internal {

    (@array [$($elems:expr,)*]) => {
        matcher_internal_vec![$($elems,)*]
    };

    (@array [$($elems:expr),*]) => {
        matcher_internal_vec![$($elems),*]
    };

    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!(null)] $($rest)*)
    };

    (@array [$($elems:expr,)*] ignore $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!(ignore)] $($rest)*)
    };

    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!(true)] $($rest)*)
    };

    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!(false)] $($rest)*)
    };

    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!([$($array)*])] $($rest)*)
    };

    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!({$($map)*})] $($rest)*)
    };

    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!($next),] $($rest)*)
    };

    (@array [$($elems:expr,)*] $last:expr) => {
        matcher_internal!(@array [$($elems,)* matcher_internal!($last)])
    };

    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        matcher_internal!(@array [$($elems,)*] $($rest)*)
    };

    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        matcher_unexpected!($unexpected)
    };

    (@object $object:ident () () ()) => {};

    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        matcher_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        matcher_unexpected!($unexpected);
    };

    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!(null)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: ignore $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!(ignore)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!(true)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!(false)) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!([$($array)*])) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!({$($map)*})) $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!($value)) , $($rest)*);
    };

    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        matcher_internal!(@object $object [$($key)+] (matcher_internal!($value)));
    };

    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        matcher_internal!();
    };

    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        matcher_internal!();
    };

    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        matcher_unexpected!($colon);
    };

    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        matcher_unexpected!($comma);
    };

    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        matcher_expect_expr_comma!($($unexpected)+);
    };

    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        matcher_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    (null) => {
        $crate::lib::matcher::Matcher::Null
    };

    (ignore) => {
        $crate::lib::matcher::Matcher::Ignore
    };

    (true) => {
        $crate::lib::matcher::Matcher::Bool(true)
    };

    (false) => {
        $crate::lib::matcher::Matcher::Bool(false)
    };

    ([]) => {
        $crate::lib::matcher::Matcher::Array(matcher_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::lib::matcher::Matcher::Array(matcher_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::lib::matcher::Matcher::Object(std::collections::HashMap::new())
    };

    ({ $($tt:tt)+ }) => {
        $crate::lib::matcher::Matcher::Object({
            let mut object = std::collections::HashMap::new();
            matcher_internal!(@object object () ($($tt)+) ($($tt)+));
            object
        })
    };

    ($other:expr) => {
        $crate::lib::matcher::to_value($other)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! matcher_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! matcher_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! matcher_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}

pub fn to_value<T: Into<Matcher>>(t: T) -> Matcher {
    t.into()
}

impl From<i32> for Matcher {
    fn from(value: i32) -> Self {
        Matcher::Number(Number::from(value))
    }
}

impl From<i64> for Matcher {
    fn from(value: i64) -> Self {
        Matcher::Number(Number::from(value))
    }
}

impl From<f32> for Matcher {
    fn from(value: f32) -> Self {
        Matcher::Number(Number::from_f64(value as f64).unwrap())
    }
}

impl From<f64> for Matcher {
    fn from(value: f64) -> Self {
        Matcher::Number(Number::from_f64(value as f64).unwrap())
    }
}

impl From<usize> for Matcher {
    fn from(value: usize) -> Self {
        Matcher::Number(Number::from(value))
    }
}

impl From<&str> for Matcher {
    fn from(value: &str) -> Self {
        Matcher::String(value.to_owned())
    }
}

impl From<String> for Matcher {
    fn from(value: String) -> Self {
        Matcher::String(value)
    }
}

impl From<bool> for Matcher {
    fn from(value: bool) -> Self {
        Matcher::Bool(value)
    }
}

impl<T> From<T> for Matcher where T: Fn(&Value) -> bool + 'static {
    fn from(value: T) -> Self {
        Matcher::ValueMatcher(Box::new(value))
    }
}

impl Matcher {

    pub fn is_null(&self) -> bool {
        match self {
            Matcher::Null => true,
            _ => false,
        }
    }

    pub fn is_ignore(&self) -> bool {
        match self {
            Matcher::Ignore => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Matcher::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Matcher::Number(n) => Some(n),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        self.as_number().is_some()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Matcher::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn as_array(&self) -> Option<&Vec<Matcher>> {
        match self {
            Matcher::Array(vec) => Some(vec),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Matcher>> {
        match self {
            Matcher::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }
}
