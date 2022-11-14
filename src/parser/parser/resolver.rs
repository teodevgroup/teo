use std::sync::{Arc, Mutex};
use crate::core::pipeline::argument::{ArgumentRange, ArgumentTuple, FunctionArgument};
use crate::parser::ast::expression::{Expression, RangeExpression, TupleExpression};
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
        range.resolved = Some(ArgumentRange { closed: range.closed, start: start.clone(), end: end.clone() })
    }

    fn resolve_tuple(tuple: &mut TupleExpression, source: Arc<Mutex<Source>>, parser: &Parser) {
        let mut resolved = vec![];
        for expression in tuple.expressions.iter_mut() {
            resolved.push(Self::resolve_expression(expression, source.clone(), parser));
        }
        tuple.resolved = Some(ArgumentTuple { values: resolved });
    }

    pub(crate) fn resolve_expression(expression: &mut Expression, source: Arc<Mutex<Source>>, parser: &Parser) -> FunctionArgument {
        match expression {
            Expression::Numeric(n) => {
                n.resolve();
                FunctionArgument::ValueArgument(n.resolved.as_ref().unwrap().clone())
            }
            Expression::String(s) => {
                s.resolve();
                FunctionArgument::ValueArgument(s.resolved.as_ref().unwrap().clone())
            }
            Expression::Bool(b) => {
                b.resolve();
                FunctionArgument::ValueArgument(b.resolved.as_ref().unwrap().clone())
            }
            Expression::Null(n) => {
                n.resolve();
                FunctionArgument::ValueArgument(n.resolved.as_ref().unwrap().clone())
            }
            Expression::EnumChoice(e) => {
                e.resolve();
                FunctionArgument::EnumChoiceArgument(e.resolved.as_ref().unwrap().as_str().unwrap().to_string())
            }
            Expression::Range(r) => {
                Self::resolve_range(r, source.clone(), parser);
                FunctionArgument::RangeArgument(r.resolved.as_ref().unwrap().clone())
            }
            Expression::Tuple(t) => {
                Self::resolve_tuple(t, source.clone(), parser);
                FunctionArgument::TupleArgument(t.resolved.as_ref().unwrap().clone())
            }
            Expression::Array(_) => {
                FunctionArgument::ValueArgument(Value::Null)
            }
            Expression::Dictionary(_) => {
                FunctionArgument::ValueArgument(Value::Null)
            }
            Expression::Path(_) => {
                FunctionArgument::ValueArgument(Value::Null)
            }
            Expression::Call(_) => {
                FunctionArgument::ValueArgument(Value::Null)
            }
            Expression::Pipeline(_) => {
                FunctionArgument::ValueArgument(Value::Null)
            }
        }
    }
}
