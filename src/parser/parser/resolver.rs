use std::str::FromStr;
use std::sync::{Arc, Mutex};
use indexmap::map::IndexMap;
use regex::Regex;
use snailquote::unescape;
use crate::core::database::name::DatabaseName;
use crate::core::tson::range::Range;
use crate::parser::ast::accessible::Accessible;
use crate::parser::ast::argument::ArgumentList;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::entity::Entity;
use crate::parser::ast::expression::{ArrayLiteral, BoolLiteral, DictionaryLiteral, EnumChoiceLiteral, Expression, ExpressionKind, NullishCoalescing, NullLiteral, NumericLiteral, RangeLiteral, RegExpLiteral, StringLiteral, TupleLiteral};
use crate::parser::ast::group::Group;
use crate::parser::ast::identifier::Identifier;
use crate::parser::ast::import::Import;
use crate::parser::ast::pipeline::Pipeline;
use crate::parser::ast::reference::{IdReference, Reference};
use crate::parser::ast::source::Source;
use crate::parser::ast::top::Top;
use crate::parser::ast::unit::Unit;
use crate::parser::parser::Parser;
use crate::prelude::Value;

pub(crate) struct Resolver { }

impl Resolver {

    pub(crate) fn resolve_parser(parser: &mut Parser) {
        let main_source = parser.get_source_mut(1);
        Self::resolve_source(parser, main_source);
        for (_, source) in parser.sources.iter_mut() {
            if !source.resolved {
                Self::resolve_source(parser, source);
            }
        }
        parser.resolved = true;
    }

    pub(crate) fn resolve_source(parser: &mut Parser, source: &mut Source) {
        for (item_id, top) in source.tops.iter_mut() {
            match top {
                Top::Import(import) => {
                    Self::resolve_import(parser, source, import);
                }
                Top::Constant(constant) => {
                    Self::resolve_constant(parser, source, constant);
                }
                Top::Enum(r#enum) => {

                }
                Top::Model(model) => {

                }
                Top::Connector(connector) => {

                }
                Top::Generator(generator) => {

                }
                Top::Client(client) => {

                }
                Top::Config(config) => {

                }
            }
        }
        source.resolved = true;
    }

    pub(crate) fn resolve_import(parser: &mut Parser, source: &mut Source, import: &mut Import) {
        let from_source = parser.sources.iter().find(|(source_id, source)| {
            source.path == import.path
        }).unwrap().1;
        import.from_id = Some(from_source.id);
        for (item_id, top) in from_source.tops {
            if top.is_model() {
                let model = top.as_model().unwrap();
                for identifier in import.identifiers.iter() {
                    if identifier.name == model.identifier.name {
                        import.references.insert(identifier.name.clone(), Reference::ModelReference((from_source.id, item_id)));
                    }
                }
            } else if top.is_constant() {
                let constant = top.as_constant().unwrap();
                for identifier in import.identifiers.iter() {
                    if identifier.name == constant.identifier.name {
                        import.references.insert(identifier.name.clone(), Reference::ConstantReference((from_source.id, item_id)));
                    }
                }
            }
        }
    }

    pub(crate) fn resolve_constant(parser: &mut Parser, source: &mut Source, constant: &mut Constant) {
        Self::resolve_expression(parser, source, &mut constant.expression);
        constant.resolved = true;
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

    pub(crate) fn resolve_expression<'a>(parser: &mut Parser, source: &mut Source, expression: &mut Expression) {
        expression.resolved = Some(Self::resolve_expression_kind(parser, source, &expression.kind));
    }

    pub(crate) fn resolve_expression_kind(parser: &mut Parser, source: &mut Source, expression_kind: &ExpressionKind) -> Entity {
        match expression_kind {
            ExpressionKind::Group(group) => {
                Self::resolve_group(parser, source, group)
            }
            ExpressionKind::NullishCoalescing(nullish_coalescing) => {
                Self::resolve_nullish_coalescing(parser, source, nullish_coalescing)
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
            ExpressionKind::RangeLiteral(range_literal) => {
                Self::resolve_range_literal(parser, source, range_literal)
            }
            ExpressionKind::TupleLiteral(tuple_literal) => {
                Self::resolve_tuple_literal(parser, source, tuple_literal)
            }
            ExpressionKind::ArrayLiteral(array_literal) => {
                Self::resolve_array_literal(parser, source, array_literal)
            }
            ExpressionKind::DictionaryLiteral(dictionary_literal) => {
                Self::resolve_dictionary_literal(parser, source, dictionary_literal)
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

    fn resolve_group(parser: &mut Parser, source: &mut Source, group: &Group) -> Entity {
        Self::resolve_expression_kind(parser, source, group.expression.as_ref())
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
            let found = i.as_import().unwrap().identifiers.iter().find(|i| &i.name == &identifier.name);
            if found.is_some() {
                let source_id = i.as_import().unwrap().source_id;
                let origin_source = parser.get_source_by_id(source_id).unwrap();
                return Self::find_identifier_origin_in_source(identifier, origin_source.clone(), parser);
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

    fn resolve_range_literal(parser: &mut Parser, source: &mut Source, range_literal: &RangeLiteral) -> Entity {
        let a = Self::resolve_expression_kind(parser, source, range_literal.expressions.get(0).unwrap());
        let a_v = Self::unwrap_into_value_if_needed(&a, source, parser);
        let start = Box::new(a_v);
        let b = Self::resolve_expression_kind(parser, source, range_literal.expressions.get(1).unwrap());
        let b_v = Self::unwrap_into_value_if_needed(&b, source, parser);
        let end = Box::new(b_v);
        Entity::Value(Value::Range(Range { closed: range_literal.closed.clone(), start, end }))
    }

    fn resolve_tuple_literal(parser: &mut Parser, source: &mut Source, tuple_literal: &TupleLiteral) -> Entity {
        let mut resolved = vec![];
        for expression in tuple_literal.expressions.iter() {
            let e = Self::resolve_expression_kind(parser, source, expression);
            let v = Self::unwrap_into_value_if_needed(&e, source, parser);
            resolved.push(v);
        }
        Entity::Value(Value::Tuple(resolved))
    }

    fn resolve_array_literal(parser: &mut Parser, source: &mut Source, array_literal: &ArrayLiteral) -> Entity {
        let mut resolved = vec![];
        for expression in array.expressions.iter() {
            let e = Self::resolve_expression_kind(parser, source, expression);
            let v = Self::unwrap_into_value_if_needed(&e, source, parser);
            resolved.push(v);
        }
        Entity::Value(Value::Vec(resolved))
    }

    fn resolve_dictionary_literal(parser: &mut Parser, source: &mut Source, dic: &DictionaryLiteral) -> Entity {
        let mut resolved: IndexMap<String, Value> = IndexMap::new();
        for (key, value) in dic.expressions.iter() {
            let k = Self::resolve_expression_kind(parser, source, key);
            let k = Self::unwrap_into_value_if_needed(&k, source, parser);
            let v = Self::resolve_expression_kind(parser, source, value);
            let v = Self::unwrap_into_value_if_needed(&v, source, parser);
            resolved.insert(k.as_str().unwrap().to_string(), v);
        }
        Entity::Value(Value::IndexMap(resolved))
    }

    fn resolve_nullish_coalescing(parser: &mut Parser, source: &mut Source, nullish_coalescing: &NullishCoalescing) -> Entity {
        let mut resolved = Entity::Value(Value::Null);
        for e in nullish_coalescing.expressions.iter() {
            resolved = Self::resolve_expression_kind(parser, source, e);
            if !resolved.is_null() {
                return resolved;
            }
        }
        return resolved
    }

    // Unwrap references

    fn constant_with_reference(r: IdReference, source: &Source, parser: &Parser) -> Value {
        let source = parser.get_source_by_id(r.source_id).unwrap();
        let c = source.lock().unwrap().get_constant_with_reference(r.item_id).clone();
        let v = c.lock().unwrap().as_constant().unwrap().expression.resolved.unwrap().as_value().unwrap();
        v.clone()
    }

    fn unwrap_into_value_if_needed(e: &Entity, source: &Source, parser: &Parser) -> Value {
        if e.is_value() {
            return e.as_value().unwrap().clone()
        } else if e.is_reference() {
            let r = e.as_reference().unwrap();
            if r.is_constant_ref() {
                return Self::constant_with_reference(r.as_constant_ref().unwrap(), source, parser);
            } else {
                panic!("Model ref cannot be transformed into value.")
            }
        } else {
            panic!("Cannot unwrap accessible into value.")
        }
    }
}
