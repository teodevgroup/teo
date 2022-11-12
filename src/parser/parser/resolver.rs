use std::sync::{Arc, Mutex};
use crate::core::pipeline::argument::{ArgumentRange, FunctionArgument};
use crate::parser::ast::expression::{Expression, RangeExpression};
use crate::parser::ast::source::Source;
use crate::parser::parser::Parser;

pub(crate) struct Resolver { }

impl Resolver {
    pub(crate) fn resolve_parser(parser: &mut Parser) {
        Self::resolve_connector(parser);

    }

    pub(crate) fn resolve_connector(parser: &mut Parser) {
        match &mut parser.connector {
            None => panic!("Connector is not defined."),
            Some(c) => {
                let connector = c.lock().unwrap().as_connector_mut().unwrap();
                let source = parser.get_source_by_id(connector.source_id).unwrap();
                for item in connector.items.iter_mut() {
                    match item.identifier.name.as_str() {
                        "provider" => {
                            let f_arg = Self::resolve_expression(&mut item.expression, source.clone(), parser);

                        },
                        "url" => {
                            let f_arg = Self::resolve_expression(&mut item.expression, source.clone(), parser);
                            f_arg
                        },
                        _ => { panic!("Undefined name '{}' in connector block.", item.identifier.name.as_str())}
                    }
                }
            },
        };
    }

    fn resolve_range(range: &mut RangeExpression, source: Arc<Mutex<Source>>, parser: &mut Parser) {
        let a = Self::resolve_expression(range.expressions.get_mut(0).unwrap(), source.clone(), parser);
        let start = a.as_value().unwrap();
        let b = Self::resolve_expression(range.expressions.get_mut(1).unwrap(), source.clone(), parser);
        let end = b.as_value().unwrap();
        range.resolved = Some(ArgumentRange { closed: range.closed, start: start.clone(), end: end.clone() })
    }

    pub(crate) fn resolve_expression(expression: &mut Expression, source: Arc<Mutex<Source>>, parser: &mut Parser) -> FunctionArgument {
        match expression {
            Expression::Numeric(n) => {
                n.resolve();
                FunctionArgument::ValueArgument(n.resolved.unwrap().clone())
            }
            Expression::String(s) => {
                s.resolve();
                FunctionArgument::ValueArgument(s.resolved.unwrap().clone())
            }
            Expression::Bool(b) => {
                b.resolve();
                FunctionArgument::ValueArgument(b.resolved.unwrap().clone())
            }
            Expression::Null(n) => {
                n.resolve();
                FunctionArgument::ValueArgument(n.resolved.unwrap().clone())
            }
            Expression::EnumChoice(e) => {
                n.resolve();
                FunctionArgument::EnumChoiceArgument(e.resolved.unwrap().as_str().unwrap().to_string())
            }
            Expression::Range(r) => {
                Self::resolve_range(r, source: Arc<Mutex<Source>>, parser: &mut Parser);
                FunctionArgument::RangeArgument(r.resolved.unwrap().clone())
            }
            Expression::Tuple(_) => {}
            Expression::Array(_) => {}
            Expression::Dictionary(_) => {}
            Expression::Path(_) => {}
            Expression::Call(_) => {}
            Expression::Pipeline(_) => {}
        }
    }
}
