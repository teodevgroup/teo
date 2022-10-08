/// Construct a `teo::tson::Value` from a Tson literal. This is inspired by serde_json package and
/// bson package.
///
/// ```
/// # use teo::prelude::tson;
/// #
/// let value = tson!{
///     "code": 200,
///     "success": true,
///     "payload": {
///         "features": [
///             "teo",
///             "tson"
///         ]
///     }
/// };
/// ```
///
#[macro_export]
macro_rules! tson {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: tson!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        crate::tson_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        crate::tson_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        tson!(@array [$($elems,)* tson!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        tson!(@array [$($elems,)* tson!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        tson!(@array [$($elems,)* tson!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        tson!(@array [$($elems,)* tson!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        tson!(@array [$($elems,)* tson!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        tson!(@array [$($elems,)* tson!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        tson!(@array [$($elems,)* tson!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        tson!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        tson_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: tson!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        tson!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        tson_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        tson!(@object $object [$($key)+] (tson!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        tson!(@object $object [$($key)+] (tson!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        tson!(@object $object [$($key)+] (tson!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        tson!(@object $object [$($key)+] (tson!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        tson!(@object $object [$($key)+] (tson!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        tson!(@object $object [$($key)+] (tson!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        tson!(@object $object [$($key)+] (tson!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        tson!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        tson!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        tson_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        tson_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        tson!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        tson_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        tson!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: bson!($($bson)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::prelude::Value::Null
    };

    ([]) => {
        $crate::prelude::Value::Vec(vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::prelude::Value::Vec($crate::tson!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::prelude::Value::HashMap(std::collections::HashMap::new())
    };

   ({ $($tt:tt)+ }) => {
        $crate::prelude::Value::HashMap({
            let mut map = std::collections::HashMap::new();
            tson!(@object map () ($($tt)+) ($($tt)+));
            map
        })
   };

    // Any Into<Value> type.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::prelude::Value::from($other)
    };
}

// The tson macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! tson_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! tson_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! tson_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}
