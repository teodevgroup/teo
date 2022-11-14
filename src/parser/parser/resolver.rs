use std::sync::{Arc, Mutex};
use indexmap::map::IndexMap;
use crate::core::tson::range::Range;
use crate::parser::ast::expression::{ArrayExpression, DictionaryExpression, Expression, RangeExpression, TupleExpression};
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
                            let f_arg = Self::resolve_expression(&mut item.expression, source.clone(), parser);

                        },
                        "url" => {
                            let f_arg = Self::resolve_expression(&mut item.expression, source.clone(), parser);
                            // f_arg
                        },
                        _ => { panic!("Undefined name '{}' in connector block.", item.identifier.name.as_str())}
                    }
                }
            },
        };
    }

    fn resolve_range(range: &mut RangeExpression, source: Arc<Mutex<Source>>, parser: &Parser) {
        let a = Self::resolve_expression(range.expressions.get_mut(0).unwrap(), source.clone(), parser);
        let start = a.as_value().unwrap();
        let b = Self::resolve_expression(range.expressions.get_mut(1).unwrap(), source.clone(), parser);
        let end = b.as_value().unwrap();
        range.resolved = Some(Value::Range(Range { closed: range.closed, start, end }));
    }

    fn resolve_tuple(tuple: &mut TupleExpression, source: Arc<Mutex<Source>>, parser: &Parser) {
        let mut resolved = vec![];
        for expression in tuple.expressions.iter_mut() {
            resolved.push(Self::resolve_expression(expression, source.clone(), parser).clone());
        }
        tuple.resolved = Some(Value::Tuple(resolved));
    }

    fn resolve_array(array: &mut ArrayExpression, source: Arc<Mutex<Source>>, parser: &Parser) {
        let mut resolved = vec![];
        for expression in array.expressions.iter_mut() {
            resolved.push(Self::resolve_expression(expression, source.clone(), parser).clone());
        }
        array.resolved = Some(Value::Array(resolved));
    }

    fn resolve_dictionary(dic: &mut DictionaryExpression, source: Arc<Mutex<Source>>, parser: &Parser) {
        let mut resolved: IndexMap<String, Value> = IndexMap::new();
        for (key, value) in dic.expressions.iter_mut() {
            let k = Self::resolve_expression(key, source.clone(), parser).clone();
            let v = Self::resolve_expression(value, source.clone(), parser).clone();

        }
        dic.resolved = Some(Value::Array(resolved));
    }

    pub(crate) fn resolve_expression<'a>(expression: &'a mut Expression, source: Arc<Mutex<Source>>, parser: &Parser) -> &'a Value {
        match expression {
            Expression::Numeric(n) => {
                n.resolve();
                n.resolved.as_ref().unwrap()
            }
            Expression::String(s) => {
                s.resolve();
                s.resolved.as_ref().unwrap()
            }
            Expression::Bool(b) => {
                b.resolve();
                b.resolved.as_ref().unwrap()
            }
            Expression::Null(n) => {
                n.resolve();
                n.resolved.as_ref().unwrap()
            }
            Expression::EnumChoice(e) => {
                e.resolve();
                e.resolved.as_ref().unwrap()
            }
            Expression::Range(r) => {
                Self::resolve_range(r, source.clone(), parser);
                r.resolved.as_ref().unwrap()
            }
            Expression::Tuple(t) => {
                Self::resolve_tuple(t, source.clone(), parser);
                t.resolved.as_ref().unwrap()
            }
            Expression::Array(a) => {
                Self::resolve_array(a, source.clone(), parser);
                a.resolved.as_ref().unwrap()
            }
            Expression::Dictionary(d) => {
                Self::resolve_dictionary(d, source.clone(), parser);
                d.resolved.as_ref().unwrap()
            }
            Expression::Path(p) => {
            }
            Expression::Call(c) => {
            }
            Expression::Pipeline(p) => {
            }
        }
    }
}
