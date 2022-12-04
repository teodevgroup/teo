use std::str::FromStr;
use std::sync::{Arc, Mutex};
use indexmap::map::IndexMap;
use regex::Regex;
use snailquote::unescape;
use crate::core::database::name::DatabaseName;
use crate::core::tson::range::Range;
use crate::parser::ast::accessible::Accessible;
use crate::parser::ast::argument::ArgumentList;
use crate::parser::ast::entity::Entity;
use crate::parser::ast::expression::{ArrayLiteral, BoolLiteral, DictionaryLiteral, EnumChoiceLiteral, Expression, ExpressionKind, NullishCoalescing, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::parser::ast::group::Group;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::pipeline::Pipeline;
use crate::parser::ast::reference::{IdReference, Reference};
use crate::parser::ast::source::Source;
use crate::parser::ast::unit::Unit;
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
                            let provider_value = Self::unwrap_into_value_if_needed(provider, source.clone(), parser);
                            let provider_str = provider_value.as_raw_enum_choice().unwrap();
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
                            let url_value = Self::unwrap_into_value_if_needed(url, source.clone(), parser);
                            let url_str = url_value.as_str().unwrap();
                            connector.url = Some(url_str.to_owned());
                        },
                        _ => { panic!("Undefined name '{}' in connector block.", item.identifier.name.as_str())}
                    }
                }
            },
        };
    }

    // Expression

    pub(crate) fn resolve_expression<'a>(expression: &'a mut Expression, source: Arc<Mutex<Source>>, parser: &Parser) -> &'a Entity {
        expression.resolved = Some(Self::resolve_expression_kind(&expression.kind, source.clone(), parser));
        expression.resolved.as_ref().unwrap()
    }

    pub(crate) fn resolve_expression_kind(expression: &ExpressionKind, source: Arc<Mutex<Source>>, parser: &Parser) -> Entity {
        match expression {
            ExpressionKind::Group(g) => {
                Self::resolve_group(g, source.clone(), parser)
            }
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
            ExpressionKind::Identifier(i) => {
                Self::resolve_identifier(i, source.clone(), parser, None)
            }
            ExpressionKind::ArgumentList(a) => {
                panic!("Argument list cannot appear alone.")
            }
            ExpressionKind::Subscript(s) => {
                panic!("Subscript cannot appear alone.")
            }
            ExpressionKind::Unit(u) => {
                Self::resolve_unit(u, source.clone(), parser)
            }
            ExpressionKind::Pipeline(p) => {
                Self::resolve_pipeline(p, source.clone(), parser)
            }
        }
    }

    // identifier

    fn resolve_group(g: &Group, source: Arc<Mutex<Source>>, parser: &Parser) -> Entity {
        Self::resolve_expression_kind(g.expression.as_ref(), source.clone(), parser)
    }

    fn resolve_identifier(i: &Identifier, source: Arc<Mutex<Source>>, parser: &Parser, parent: Option<Accessible>) -> Entity {
        let reference = Self::find_identifier_origin_in_source(i, source.clone(), parser);
        Entity::Reference(reference)
    }

    fn find_identifier_origin_in_source(identifier: &Identifier, source: Arc<Mutex<Source>>, parser: &Parser) -> Reference {
        let s = source.lock().unwrap();
        // test for constant
        for (id, constant) in s.constants.iter() {
            let c = constant.lock().unwrap();
            if &identifier.name == &c.as_constant().unwrap().identifier.name {
                return Reference::ConstantReference(IdReference::new(s.id, c.id()));
            }
        }
        // test for model
        for (id, model) in s.models.iter() {
            let m = model.lock().unwrap();
            if &identifier.name == &m.as_model().unwrap().identifier.name {
                return Reference::ModelReference(IdReference::new(s.id, m.id()));
            }
        }
        // test for import
        for (id, import) in s.imports.iter() {
            let i = import.lock().unwrap();
            let found = i.as_import().unwrap().identifiers.iter().find(|i| i.name == &identifier.name);
            if found.is_some() {
                let source_id = i.as_import().unwrap().source_id;
                let origin_source = parser.get_source_by_id(source_id).unwrap();
                return Self::find_identifier_origin_in_source(identifier, origin_source.clone(), parser);
            }
            if &identifier.name == &i.as_import().unwrap().identifier.name {
                return Reference::ModelReference(IdReference::new(s.id, i.id()));
            }
        }
        panic!("Reference is not found")
    }

    fn resolve_argument_list(a: &ArgumentList, source: Arc<Mutex<Source>>, parser: &Parser) -> Entity {
        panic!()
    }

    fn resolve_unit(u: &Unit, source: Arc<Mutex<Source>>, parser: &Parser) -> Entity {
        panic!()
    }

    fn resolve_pipeline(p: &Pipeline, source: Arc<Mutex<Source>>, parser: &Parser) -> Entity {
        panic!()
    }

    // literals and operators

    fn resolve_numeric_literal(n: &NumericLiteral) -> Entity {
        let i = i64::from_str(&n.value);
        if i.is_ok() {
            return Entity::Value(Value::I64(i.unwrap()));
        }
        let i = f64::from_str(&n.value);
        if i.is_ok() {
            return Entity::Value(Value::F64(i.unwrap()));
        }
        panic!("Cannot resolve numeric value: {}.", n.value.as_str())
    }

    fn resolve_string_literal(s: &StringLiteral) -> Entity {
        return Entity::Value(Value::String(unescape(s.value.as_str()).unwrap()));
    }

    fn resolve_regexp_literal(r: &RegExpLiteral) -> Entity {
        return Entity::Value(Value::RegExp(Regex::new(r.value.as_str()).unwrap()));
    }

    fn resolve_bool_literal(b: &BoolLiteral) -> Entity {
        match b.value.as_str() {
            "true" => Entity::Value(Value::Bool(true)),
            "false" => Entity::Value(Value::Bool(false)),
            _ => panic!("Cannot resolve bool value: {}", b.value.as_str())
        }
    }

    fn resolve_null_literal(_: &NullLiteral) -> Entity {
        Entity::Value(Value::Null)
    }

    fn resolve_enum_choice_literal(e: &EnumChoiceLiteral) -> Entity {
        Entity::Value(Value::RawEnumChoice(e.value.clone()))
    }

    fn resolve_range_literal(range: &RangeLiteral, source: Arc<Mutex<Source>>, parser: &Parser) -> Entity {
        let a = Self::resolve_expression_kind(range.expressions.get(0).unwrap(), source.clone(), parser);
        let start = Box::new(a.clone());
        let b = Self::resolve_expression_kind(range.expressions.get(1).unwrap(), source.clone(), parser);
        let end = Box::new(b.clone());
        Entity::Value(Value::Range(Range { closed: range.closed.clone(), start, end }))
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

    // Unwrap references

    fn constant_with_reference(r: IdReference, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        let source = parser.get_source_by_id(r.source_id).unwrap();
        let v = source.lock().unwrap().get_constant_with_reference(r.item_id).lock().unwrap().as_constant().unwrap().expression.resolved.unwrap().as_value().unwrap();
        v.clone()
    }

    fn unwrap_into_value_if_needed(e: &Entity, source: Arc<Mutex<Source>>, parser: &Parser) -> Value {
        if e.is_value() {
            return e.as_value().unwrap().clone()
        } else if e.is_reference() {
            let r = e.as_reference().unwrap();
            if r.is_constant_ref() {
                return Self::constant_with_reference(r.as_constant_ref().unwrap(), source.clone(), parser);
            } else {
                panic!("Model ref cannot be transformed into value.")
            }
        } else {
            panic!("Cannot unwrap accessible into value.")
        }
    }
}
