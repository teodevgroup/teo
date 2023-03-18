use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::parser::ast::accessible::{ASTFunctionInstaller, ASTPipelineInstaller};
use crate::parser::std::pipeline::array::append::append;
use crate::parser::std::pipeline::array::get_length::get_length;
use crate::parser::std::pipeline::array::has_length::has_length;
use crate::parser::std::pipeline::array::prepend::prepend;
use crate::parser::std::pipeline::array::reverse::reverse;
use crate::parser::std::pipeline::array::truncate::truncate;
use crate::parser::std::pipeline::bcrypt::bcrypt_salt::bcrypt_salt;
use crate::parser::std::pipeline::bcrypt::bcrypt_verify::bcrypt_verify;
use crate::parser::std::pipeline::custom_function::{custom_callback, custom_compare, custom_transform, custom_validate};
use crate::parser::std::pipeline::datetime::{now, today};
use crate::parser::std::pipeline::debug::print;
use crate::parser::std::pipeline::identity::identity;
use crate::parser::std::pipeline::action::{redirect, when};
use crate::parser::std::pipeline::logical::{all_modifier, and_modifier, any_modifier, if_modifier, invalid, not_modifier, or_modifier, passed, valid};
use crate::parser::std::pipeline::math::{abs, add, cbrt, ceil, divide, floor, max, min, modular, multiply, pow, root, round, sqrt, subtract};
use crate::parser::std::pipeline::number::{is_even, is_odd};
use crate::parser::std::pipeline::object::{assign, ctx_self, is, is_a, object_get, object_previous_value, object_set};
use crate::parser::std::pipeline::query::query_raw;
use crate::parser::std::pipeline::string::generation::{cuid, random_digits, slug, uuid};
use crate::parser::std::pipeline::string::transform::{ellipsis, lowercase, uppercase, pad_end, pad_start, regex_replace, split, trim, to_word_case};
use crate::parser::std::pipeline::string::validation::{has_prefix, has_suffix, is_alphabetic, is_alphanumeric, is_email, is_hex_color, is_numeric, is_prefix_of, is_secure_password, is_suffix_of, regex_match};
use crate::parser::std::pipeline::value::{eq, gt, gte, exists, is_false, is_null, is_true, lt, lte, neq, one_of};
use crate::parser::std::pipeline::vector::{filter, item_at, join, map};

pub(crate) struct GlobalPipelineInstallers {
    objects: HashMap<String, ASTPipelineInstaller>
}

impl Debug for GlobalPipelineInstallers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlobalPipelineInstallers")
    }
}

impl GlobalPipelineInstallers {

    pub(crate) fn new() -> Self {
        let mut objects: HashMap<String, ASTPipelineInstaller> = HashMap::new();
        // action
        objects.insert("redirect".to_owned(), redirect);
        objects.insert("when".to_owned(), when);
        // array
        objects.insert("append".to_owned(), append);
        objects.insert("prepend".to_owned(), prepend);
        objects.insert("getLength".to_owned(), get_length);
        objects.insert("hasLength".to_owned(), has_length);
        objects.insert("reverse".to_owned(), reverse);
        objects.insert("truncate".to_owned(), truncate);
        // bcrypt
        objects.insert("bcryptSalt".to_owned(), bcrypt_salt);
        objects.insert("bcryptVerify".to_owned(), bcrypt_verify);
        // datetime
        objects.insert("now".to_owned(), now);
        objects.insert("today".to_owned(), today);
        // debug
        objects.insert("print".to_owned(), print);
        // identity
        objects.insert("identity".to_owned(), identity);
        // logical
        objects.insert("valid".to_owned(), valid);
        objects.insert("invalid".to_owned(), invalid);
        objects.insert("if".to_owned(), if_modifier);
        objects.insert("all".to_owned(), all_modifier);
        objects.insert("any".to_owned(), any_modifier);
        objects.insert("not".to_owned(), not_modifier);
        objects.insert("passed".to_owned(), passed);
        objects.insert("and".to_owned(), and_modifier);
        objects.insert("or".to_owned(), or_modifier);
        // math
        objects.insert("abs".to_owned(), abs);
        objects.insert("add".to_owned(), add);
        objects.insert("cbrt".to_owned(), cbrt);
        objects.insert("ceil".to_owned(), ceil);
        objects.insert("div".to_owned(), divide);
        objects.insert("floor".to_owned(), floor);
        objects.insert("max".to_owned(), max);
        objects.insert("min".to_owned(), min);
        objects.insert("mod".to_owned(), modular);
        objects.insert("mul".to_owned(), multiply);
        objects.insert("pow".to_owned(), pow);
        objects.insert("root".to_owned(), root);
        objects.insert("round".to_owned(), round);
        objects.insert("sqrt".to_owned(), sqrt);
        objects.insert("sub".to_owned(), subtract);
        // number
        objects.insert("isEven".to_owned(), is_even);
        objects.insert("isOdd".to_owned(), is_odd);
        // object
        objects.insert("self".to_owned(), ctx_self);
        objects.insert("get".to_owned(), object_get);
        objects.insert("set".to_owned(), object_set);
        objects.insert("assign".to_owned(), assign);
        objects.insert("previous".to_owned(), object_previous_value);
        objects.insert("isA".to_owned(), is_a);
        objects.insert("is".to_owned(), is);
        // string generation
        objects.insert("cuid".to_owned(), cuid);
        objects.insert("randomDigits".to_owned(), random_digits);
        objects.insert("slug".to_owned(), slug);
        objects.insert("uuid".to_owned(), uuid);
        // string transform
        objects.insert("ellipsis".to_owned(), ellipsis);
        objects.insert("padEnd".to_owned(), pad_end);
        objects.insert("padStart".to_owned(), pad_start);
        objects.insert("regexReplace".to_owned(), regex_replace);
        objects.insert("split".to_owned(), split);
        objects.insert("trim".to_owned(), trim);
        objects.insert("toWordCase".to_owned(), to_word_case);
        objects.insert("lowercase".to_owned(), lowercase);
        objects.insert("uppercase".to_owned(), uppercase);
        // string validation
        objects.insert("hasPrefix".to_owned(), has_prefix);
        objects.insert("hasSuffix".to_owned(), has_suffix);
        objects.insert("isAlphabetic".to_owned(), is_alphabetic);
        objects.insert("isAlphanumeric".to_owned(), is_alphanumeric);
        objects.insert("isEmail".to_owned(), is_email);
        objects.insert("isHexColor".to_owned(), is_hex_color);
        objects.insert("isNumeric".to_owned(), is_numeric);
        objects.insert("isPrefixOf".to_owned(), is_prefix_of);
        objects.insert("isSecurePassword".to_owned(), is_secure_password);
        objects.insert("isSuffixOf".to_owned(), is_suffix_of);
        objects.insert("regexMatch".to_owned(), regex_match);
        // value
        objects.insert("eq".to_owned(), eq);
        objects.insert("gt".to_owned(), gt);
        objects.insert("gte".to_owned(), gte);
        objects.insert("exists".to_owned(), exists);
        objects.insert("isFalse".to_owned(), is_false);
        objects.insert("isNull".to_owned(), is_null);
        objects.insert("isTrue".to_owned(), is_true);
        objects.insert("lt".to_owned(), lt);
        objects.insert("lte".to_owned(), lte);
        objects.insert("neq".to_owned(), neq);
        objects.insert("oneOf".to_owned(), one_of);
        // vector
        objects.insert("join".to_owned(), join);
        objects.insert("at".to_owned(), item_at);
        objects.insert("filter".to_owned(), filter);
        objects.insert("map".to_owned(), map);
        // query
        objects.insert("queryRaw".to_owned(), query_raw);
        Self { objects }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&ASTPipelineInstaller> {
        self.objects.get(key)
    }
}

pub(crate) struct GlobalFunctionInstallers {
    objects: HashMap<String, ASTFunctionInstaller>
}

impl Debug for GlobalFunctionInstallers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("GlobalFunctionInstallers")
    }
}

impl GlobalFunctionInstallers {

    pub(crate) fn new() -> Self {
        let mut objects: HashMap<String, ASTFunctionInstaller> = HashMap::new();
        // array
        objects.insert("transform".to_owned(), custom_transform);
        objects.insert("validate".to_owned(), custom_validate);
        objects.insert("callback".to_owned(), custom_callback);
        objects.insert("compare".to_owned(), custom_compare);
        Self { objects }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&ASTFunctionInstaller> {
        self.objects.get(key)
    }
}
