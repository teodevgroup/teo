use std::str::FromStr;
use std::sync::{Arc, Mutex};
use indexmap::map::IndexMap;
use regex::Regex;
use snailquote::unescape;
use crate::core::database::name::DatabaseName;
use crate::core::tson::range::Range;
use crate::parser::ast::expression::{ArrayLiteral, BoolLiteral, DictionaryLiteral, EnumChoiceLiteral, Expression, ExpressionKind, NullishCoalescing, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::parser::ast::source::Source;
use crate::parser::parser::Parser;
use crate::prelude::Value;

pub(crate) struct Resolver { }

impl Resolver {
    pub(crate) fn resolve_parser(parser: &Parser) {
        Self::resolve_connector(parser);
    }

    pub(crate) fn resolve_connector(parser: &Parser) {
        match &parser.connector {
            None => panic!("Connector is not defined."),
            Some(c) => {
                let mut top = c.lock().unwrap();
                let connector = top.as_connector_mut().unwrap();
                let id = c.lock().unwrap().id();
                let source = parser.get_source_by_id(id).unwrap().clone();
                for item in connector.items.iter_mut() {
                    match item.identifier.name.as_str() {
                        "provider" => {
                            let provider = Self::resolve_expression(&mut item.expression, source.clone(), parser);
                            let provider_str = provider.as_raw_enum_choice().unwrap();
                            match provider_str {
                                "sqlite" => connector.provider = Some(DatabaseName::SQLite),
                                "mongo" => connector.provider = Some(DatabaseName::MongoDB),
                                "mysql" => connector.provider = Some(DatabaseName::MySQL),
                                "postgres" => connector.provider = Some(DatabaseName::PostgreSQL),
                                _ => panic!("Unrecognized provider.")
                            }
                        },
                        "url" => {
                            let url = Self::resolve_expression(&mut item.expression, source.clone(), parser);
                            let url_str = url.as_str().unwrap();
                            connector.url = Some(url_str.to_owned());
                        },
                        _ => { panic!("Undefined name '{}' in connector block.", item.identifier.name.as_str())}
                    }
                }
            },
        };
    }

    // Expression
    pub(crate) fn resolve_expression<'a>(expression: &'a mut Expression, source: Arc<Mutex<Source>>, parser: &Parser) -> &'a Value {
        expression.resolved = Some(Self::resolve_expression_kind(&expression.kind, source.clone(), parser));
        expression.resolved.as_ref().unwrap()
    }
    pub(crate) fn resolve_expression_kind(expression: &ExpressionKind, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        match expression {
            ExpressionKind::NullishCoalescing(n) => {
                Self::resolve_nullish_coalescing(n, source.clone(), parser)
            }
            ExpressionKind::NumericLiteral(n) => {
                Self::resolve_numeric_literal(n)
            }
            ExpressionKind::StringLiteral(s) => {
                Self::resolve_string_literal(s)
            }
            ExpressionKind::RegExpLiteral(r) => {
                Self::resolve_regexp_literal(r)
            }
            ExpressionKind::BoolLiteral(b) => {
                Self::resolve_bool_literal(b)
            }
            ExpressionKind::NullLiteral(n) => {
                Self::resolve_null_literal(n)
            }
            ExpressionKind::EnumChoiceLiteral(e) => {
                Self::resolve_enum_choice_literal(e)
            }
            ExpressionKind::RangeLiteral(r) => {
                Self::resolve_range_literal(r, source.clone(), parser)
            }
            ExpressionKind::TupleLiteral(t) => {
                Self::resolve_tuple_literal(t, source.clone(), parser)
            }
            ExpressionKind::ArrayLiteral(a) => {
                Self::resolve_array_literal(a, source.clone(), parser)
            }
            ExpressionKind::DictionaryLiteral(d) => {
                Self::resolve_dictionary_literal(d, source.clone(), parser)
            }
            ExpressionKind::Pipeline(p) => {
                panic!();
                //Self::resolve_pipeline(p, source.clone(), parser)
            }
            ExpressionKind::Identifier(i) => {
                panic!();
                //Self::resolve_identifier(i, source.clone(), parser, None)
            }
            ExpressionKind::ArgumentList(a) => {
                panic!();
                //Self::resolve_argument_list(a, source.clone(), parser)
            }
            ExpressionKind::Subscript(s) => {
                panic!();
                panic!("Subscript cannot appear alone.")
            }
            ExpressionKind::Unit(u) => {
                panic!();
                //Self::resolve_unit(u, source.clone(), parser)
            }
        }
    }

    fn resolve_numeric_literal(n: &NumericLiteral) -> Value {
        let i = i64::from_str(&n.value);
        if i.is_ok() {
            return Value::I64(i.unwrap());
        }
        let i = f64::from_str(&n.value);
        if i.is_ok() {
            return Value::F64(i.unwrap());
        }
        panic!("Cannot resolve numeric value: {}.", &n.value)
    }

    fn resolve_string_literal(s: &StringLiteral) -> Value {
        return Value::String(unescape(&s.value).unwrap());
    }

    fn resolve_regexp_literal(r: &RegExpLiteral) -> Value {
        return Value::RegExp(Regex::new(r.value.as_str()).unwrap())
    }

    fn resolve_bool_literal(b: &BoolLiteral) -> Value {
        match b.value.as_str() {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => panic!("Cannot resolve bool value: {}", &b.value)
        }
    }

    fn resolve_null_literal(_: &NullLiteral) -> Value {
        Value::Null
    }

    fn resolve_enum_choice_literal(e: &EnumChoiceLiteral) -> Value {
        Value::RawEnumChoice(e.value.clone())
    }

    fn resolve_range_literal(range: &RangeLiteral, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        let a = Self::resolve_expression_kind(range.expressions.get(0).unwrap(), source.clone(), parser);
        let start = Box::new(a.clone());
        let b = Self::resolve_expression_kind(range.expressions.get(1).unwrap(), source.clone(), parser);
        let end = Box::new(b.clone());
        Value::Range(Range { closed: range.closed, start, end })
    }

    fn resolve_tuple_literal(tuple: &TupleLiteral, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        let mut resolved = vec![];
        for expression in tuple.expressions.iter() {
            resolved.push(Self::resolve_expression_kind(expression, source.clone(), parser).clone());
        }
        Value::Tuple(resolved)
    }

    fn resolve_array_literal(array: &ArrayLiteral, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        let mut resolved = vec![];
        for expression in array.expressions.iter() {
            resolved.push(Self::resolve_expression_kind(expression, source.clone(), parser).clone());
        }
        Value::Vec(resolved)
    }

    fn resolve_dictionary_literal(dic: &DictionaryLiteral, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        let mut resolved: IndexMap<String, Value> = IndexMap::new();
        for (key, value) in dic.expressions.iter() {
            let k = Self::resolve_expression_kind(key, source.clone(), parser).clone();
            let v = Self::resolve_expression_kind(value, source.clone(), parser).clone();
            resolved.insert(k.as_str().unwrap().to_string(), v);
        }
        Value::IndexMap(resolved)
    }

    fn resolve_nullish_coalescing(n: &NullishCoalescing, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        let mut resolved: Value = Value::Null;
        for e in n.expressions.iter() {
            resolved = Self::resolve_expression_kind(e, source.clone(), parser);
            if !resolved.is_null() {
                return resolved;
            }
        }
        return resolved
    }
}
