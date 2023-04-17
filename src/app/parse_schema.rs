use crate::app::new_app::ctx::AppCtx;
use crate::core::conf::debug::DebugConf;
use crate::core::conf::test::{Reset, ResetDatasets, ResetMode, TestConf};
use crate::core::connector::ConnectorConf;
use crate::core::field::field::Field;
use crate::core::field::r#type::FieldType;
use crate::core::model::Model;
use crate::core::model::model::Model;
use crate::core::property::Property;
use crate::gen::interface::client::conf::Conf;
use crate::gen::interface::server::conf::EntityGeneratorConf;
use crate::parser::parser::parser::ASTParser;
use crate::prelude::Graph;
use crate::core::r#enum::{Enum, EnumVariant};
use crate::core::relation::Relation;
use crate::parser::ast::field::ASTFieldClass;
use crate::parser::ast::r#type::Arity;
use crate::server::conf::ServerConf;
use super::new_app::new_result::Result;

pub(super) fn parse_schema(main: Option<&str>) -> Result<()> {
    let app_ctx = AppCtx::get_mut()?;
    app_ctx.set_parser(Box::new(ASTParser::new(AppCtx::get()?.callbacks())));
    let parser = app_ctx.parser_mut()?;
    parser.parse(main);
    Ok(())
}

pub(super) fn load_schema() -> Result<()> {
    let app_ctx = AppCtx::get_mut()?;
    let parser = app_ctx.parser()?;
    // connector conf
    let connector = parser.connector()?;
    app_ctx.set_connector_conf(Box::new(ConnectorConf {
        provider: connector.provider.unwrap(),
        url: connector.url.as_ref().unwrap().as_str(),
    }));
    // server conf
    let server = parser.server()?;
    app_ctx.set_server_conf(Box::new(ServerConf {
        bind: server.bind.as_ref().unwrap().clone(),
        jwt_secret: server.jwt_secret.map(|s| s.as_str()),
        path_prefix: server.path_prefix.map(|s| s.as_str()),
    }));
    // debug conf
    if let Some(debug) = parser.debug() {
        app_ctx.set_debug_conf(Box::new(DebugConf {
            log_queries: debug.log_queries,
            log_migrations: debug.log_migrations,
            log_seed_records: debug.log_seed_records,
        }));
    }
    // test conf
    if let Some(_test) = parser.test() {
        app_ctx.set_test_conf(Box::new(TestConf {
            reset: Some(Reset {
                mode: ResetMode::AfterQuery,
                datasets: ResetDatasets::Auto,
            })
        }));
    }
    // entities
    for entity in parser.entities() {
        app_ctx.entities_mut().push(EntityGeneratorConf {
            name: entity.identifier.as_ref().map(|i| i.name.clone()),
            provider: entity.provider.unwrap(),
            dest: entity.dest.clone().unwrap(),
        })
    }
    // clients
    for client in parser.clients() {
        app_ctx.clients_mut().push(Conf {
            name: client.identifier.as_ref().map(|i| i.name.clone()),
            kind: client.provider.unwrap(),
            dest: client.dest.clone().unwrap(),
            package: client.package.unwrap(),
            host: client.host.clone().unwrap(),
            object_name: client.object_name.clone(),
            git_commit: client.git_commit,
        })
    }
    app_ctx.set_graph(Box::new(Graph::new()));
    let graph = app_ctx.graph_mut()?;
    // enums
    for ast_enum in parser.enums() {
        let enum_def = Enum::new(
            ast_enum.identifier.name.as_str(),
            None,
            None,
            ast_enum.choices.iter().map(|ast_choice| {
                EnumVariant::new(ast_choice.identifier.name.as_str(), None, None)
            }).collect()
        );
        graph.add_enum(enum_def);
    }
    // models
    for ast_model in parser.models() {
        let mut model = Model::new(
            ast_model.identifier.name.as_str(),
            ast_model.comment_block.map(|c|c.name.map(|n|n.as_str())).flatten(),
            ast_model.comment_block.map(|c|c.desc.map(|n|n.as_str())).flatten());
        for ast_decorator in ast_model.decorators.iter() {
            let model_decorator = ast_decorator.accessible.as_ref().unwrap().as_model_decorator().unwrap();
            model_decorator(ast_decorator.get_argument_list(), &mut model);
        }
        for ast_field in ast_model.fields.iter() {
            match ast_field.field_class {
                ASTFieldClass::Field | ASTFieldClass::DroppedField => {
                    let mut model_field = Field::new(field.identifier.name.as_str().to_owned());
                    if let Some(comment) = &field.comment_block {
                        if let Some(name) = comment.name.as_ref() {
                            model_field.localized_name = Some(name.to_owned());
                        }
                        if let Some(desc) = comment.desc.as_ref() {
                            model_field.description = Some(desc.to_owned());
                        }
                    }
                    // type
                    match field.r#type.arity {
                        Arity::Scalar => {
                            if field.r#type.item_required {
                                model_field.set_required();
                            } else {
                                model_field.set_optional();
                            }
                            Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut model_field, &enums);
                        }
                        Arity::Array => {
                            if field.r#type.collection_required {
                                model_field.set_required();
                            } else {
                                model_field.set_optional();
                            }
                            model_field.field_type = Some(FieldType::Vec(Box::new({
                                let mut inner = Field::new("".to_owned());
                                if field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                Self::install_types_to_field_owner(&field.r#type.identifier.name, &mut inner, &enums);
                                inner
                            })));
                        }
                        Arity::Dictionary => {
                            if field.r#type.collection_required {
                                model_field.set_required();
                            } else {
                                model_field.set_optional();
                            }
                            model_field.field_type = Some(FieldType::HashMap(Box::new({
                                let mut inner = Field::new("".to_owned());
                                if field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner, &enums);
                                inner
                            })));
                        }
                    }
                    // decorators
                    for decorator in field.decorators.iter() {
                        let field_decorator = decorator.accessible.as_ref().unwrap().as_field_decorator().unwrap();
                        field_decorator(decorator.get_argument_list(), &mut model_field);
                    }
                    match &field.field_class {
                        FieldClass::DroppedField => {
                            model.add_dropped_field(model_field);
                        }
                        _ => {
                            model.add_field(model_field);
                        }
                    }
                }
                ASTFieldClass::Relation => {
                    let mut model_relation = Relation::new(field.identifier.name.as_str().to_owned());
                    if let Some(comment) = &field.comment_block {
                        if let Some(name) = comment.name.as_ref() {
                            model_relation.localized_name = Some(name.to_owned());
                        }
                        if let Some(desc) = comment.desc.as_ref() {
                            model_relation.description = Some(desc.to_owned());
                        }
                    }
                    match field.r#type.arity {
                        Arity::Scalar => {
                            if field.r#type.item_required {
                                model_relation.set_required();
                            } else {
                                model_relation.set_optional();
                            }
                            model_relation.set_is_vec(false);
                            model_relation.set_model(field.r#type.identifier.name.clone());
                        }
                        Arity::Array => {
                            if !field.r#type.item_required {
                                panic!("Relation cannot have optional items.")
                            }
                            model_relation.set_is_vec(true);
                            model_relation.set_model(field.r#type.identifier.name.clone());
                        }
                        Arity::Dictionary => panic!("Relations cannot be dictionary.")
                    }
                    // handle decorators
                    for decorator in field.decorators.iter() {
                        let relation_decorator = decorator.accessible.as_ref().unwrap().as_relation_decorator().unwrap();
                        relation_decorator(decorator.get_argument_list(), &mut model_relation);
                    }
                    model.add_relation(model_relation);
                }
                ASTFieldClass::Property => {
                    let mut model_property = Property::new(field.identifier.name.clone());
                    if let Some(comment) = &field.comment_block {
                        if let Some(name) = comment.name.as_ref() {
                            model_property.localized_name = Some(name.to_owned());
                        }
                        if let Some(desc) = comment.desc.as_ref() {
                            model_property.description = Some(desc.to_owned());
                        }
                    }
                    // type
                    match field.r#type.arity {
                        Arity::Scalar => {
                            if field.r#type.item_required {
                                model_property.set_required();
                            } else {
                                model_property.set_optional();
                            }
                            Self::install_types_to_property_builder(&field.r#type.identifier.name, &mut model_property, &enums);
                        }
                        Arity::Array => {
                            if field.r#type.collection_required {
                                model_property.set_required();
                            } else {
                                model_property.set_optional();
                            }
                            model_property.field_type = Some(FieldType::Vec(Box::new({
                                let mut inner = Field::new("".to_owned());
                                if field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner, &enums);
                                inner
                            })));
                        }
                        Arity::Dictionary => {
                            if field.r#type.collection_required {
                                model_property.set_required();
                            } else {
                                model_property.set_optional();
                            }
                            model_property.field_type = Some(FieldType::HashMap(Box::new({
                                let mut inner = Field::new("".to_owned());
                                if field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                Self::install_types_to_field_builder(&field.r#type.identifier.name, &mut inner, &enums);
                                inner
                            })));
                        }
                    }
                    for decorator in field.decorators.iter() {
                        let property_decorator = decorator.accessible.as_ref().unwrap().as_property_decorator().unwrap();
                        property_decorator(decorator.get_argument_list(), &mut model_property);
                    }
                    model.add_property(model_property);
                }
                ASTFieldClass::Unresolved => unreachable!()
            }
        }
    }
    Ok(())
}