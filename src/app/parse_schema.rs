use std::collections::HashMap;
use dotenvy::dotenv;

use crate::app::app_ctx::AppCtx;
use crate::app::namespace::Namespace;
use crate::core::conf::debug::DebugConf;
use crate::core::conf::test::{Reset, ResetDatasets, ResetMode, TestConf};
use crate::core::connector::conf::ConnectorConf;
use crate::core::field::field::Field;
use crate::core::field::r#type::{FieldType, FieldTypeOwner};
use crate::core::interface::{CustomActionDefinition, InterfaceRef};
use crate::core::model::model::Model;
use crate::core::property::Property;
use crate::gen::interface::client::conf::ClientConf as ClientConf;
use crate::gen::interface::server::conf::EntityConf as EntityConf;
use crate::core::r#enum::{Enum, EnumVariant};
use crate::core::relation::Relation;
use crate::parser::ast::field::{ASTField, ASTFieldClass};
use crate::parser::ast::r#type::Arity;
use crate::seeder::data_set::{DataSet, Group, Record};
use crate::seeder::models::define::define_seeder_models;
use crate::server::conf::ServerConf;
use crate::core::result::Result;
use crate::parser::ast::action::ActionGroupDeclaration;
use crate::parser::ast::data_set::ASTDataSet;
use crate::parser::ast::interface_type::InterfaceType;
use crate::parser::ast::model::ASTModel;
use crate::parser::ast::r#enum::ASTEnum;
use crate::parser::diagnostics::diagnostics::{Diagnostics, DiagnosticsError};
use crate::parser::diagnostics::printer;

fn exit_with_parser_error(diagnostics: &mut Diagnostics, error: DiagnosticsError) {
    diagnostics.insert(error);
    printer::print_diagnostics(diagnostics, true);
    std::process::exit(1);
}

pub(super) fn parse_schema(main: Option<&str>, diagnostics: &mut Diagnostics) -> Result<()> {
    // load env first
    let _ = dotenv();
    let app_ctx = AppCtx::get()?;
    let parser = app_ctx.parser_mut();
    parser.parse(main, diagnostics);
    Ok(())
}

pub(super) fn load_schema(diagnostics: &mut Diagnostics) -> Result<()> {
    let app_ctx = AppCtx::get()?;
    let graph = app_ctx.graph();
    let parser = app_ctx.parser();
    // connector conf
    let connector = parser.connector()?;
    app_ctx.main_namespace_mut().set_connector_conf(Box::new(ConnectorConf {
        provider: connector.provider.unwrap(),
        url: connector.url.as_ref().unwrap().as_str(),
    }));
    // server conf
    let server = parser.server()?;
    app_ctx.main_namespace_mut().set_server_conf(Box::new(ServerConf {
        bind: server.bind.as_ref().unwrap().clone(),
        jwt_secret: server.jwt_secret.as_ref().map(|s| s.as_str()),
        path_prefix: server.path_prefix.as_ref().map(|s| s.as_str()),
    }));
    // debug conf
    if let Some(debug) = parser.debug() {
        app_ctx.main_namespace_mut().set_debug_conf(Box::new(DebugConf {
            log_queries: debug.log_queries,
            log_migrations: debug.log_migrations,
            log_seed_records: debug.log_seed_records,
        }));
    }
    // test conf
    if let Some(_test) = parser.test() {
        app_ctx.main_namespace_mut().set_test_conf(Box::new(TestConf {
            reset: Some(Reset {
                mode: ResetMode::AfterQuery,
                datasets: ResetDatasets::Auto,
            })
        }));
    }
    // entities
    for entity in parser.entities() {
        app_ctx.main_namespace_mut().entities_mut().push(EntityConf {
            name: entity.identifier.as_ref().map(|i| i.name.clone()),
            provider: entity.provider.unwrap(),
            dest: entity.dest.clone().unwrap(),
        })
    }
    // clients
    for client in parser.clients() {
        app_ctx.main_namespace_mut().clients_mut().push(ClientConf {
            name: client.identifier.as_ref().map(|i| i.name.clone()),
            kind: client.provider.unwrap(),
            dest: client.dest.clone().unwrap(),
            package: client.package.unwrap(),
            host: client.host.clone().unwrap(),
            object_name: client.object_name.clone(),
            git_commit: client.git_commit,
        })
    }
    // enums
    install_enums_to_namespace(parser.enums(), AppCtx::get().unwrap().main_namespace_mut());
    // models
    install_models_to_namespace(parser.models(), AppCtx::get().unwrap().main_namespace_mut());
    define_seeder_models();
    // datasets
    install_datasets_to_namespace(parser.data_sets(), AppCtx::get().unwrap().main_namespace_mut());
    // interfaces
    // middlewares
    // action groups
    install_action_groups_to_namespace(parser.action_groups(), AppCtx::get().unwrap().main_namespace_mut())?;
    // static files
    for static_files in parser.static_files() {
        let map = Box::leak(Box::new(static_files.resolved_map.as_ref().unwrap().clone())).as_str();
        let path = Box::leak(Box::new(static_files.resolved_path.as_ref().unwrap().clone())).as_str();
        app_ctx.insert_static_files(path, map)?;
    }
    Ok(())
}

fn interface_ref_from(type_with_generics: &InterfaceType) -> InterfaceRef {
    InterfaceRef {
        name: type_with_generics.name.name.clone(),
        args: type_with_generics.args.iter().map(|a| {
            interface_ref_from(a)
        }).collect(),
    }
}

fn install_types_to_field_owner<F>(name: &str, field: &mut F, enums: &HashMap<&'static str, Enum>, diagnostics: &mut Diagnostics, ast_field: &ASTField) where F: FieldTypeOwner {
    match name {
        "String" => field.set_field_type(FieldType::String),
        "Bool" => field.set_field_type(FieldType::Bool),
        "Int" | "Int32" => field.set_field_type(FieldType::I32),
        "Int64" => field.set_field_type(FieldType::I64),
        "Float32" => field.set_field_type(FieldType::F32),
        "Float" | "Float64" => field.set_field_type(FieldType::F64),
        "Date" => field.set_field_type(FieldType::Date),
        "DateTime" => field.set_field_type(FieldType::DateTime),
        "Decimal" => field.set_field_type(FieldType::Decimal),
        #[cfg(feature = "data-source-mongodb")]
        "ObjectId" => field.set_field_type(FieldType::ObjectId),
        _ => {
            if let Some(enum_def) = enums.get(name) {
                field.set_field_type(FieldType::Enum(enum_def.clone()))
            } else {
                let source = AppCtx::get().unwrap().parser().get_source(ast_field.source_id);
                diagnostics.insert(DiagnosticsError::new(ast_field.r#type.span, "Unknown type specified", source.path.clone()));
                printer::print_diagnostics_and_exit(diagnostics, true)
            }
        }
    };
}

fn install_enums_to_namespace(enums: Vec<&ASTEnum>, namespace: &mut Namespace) {
    for ast_enum in enums {
        let enum_def = Enum::new(
            ast_enum.identifier.name.as_str(),
            ast_enum.ns_path.clone(),
            None,
            None,
            ast_enum.choices.iter().map(|ast_choice| {
                EnumVariant::new(ast_choice.identifier.name.as_str(), None, None)
            }).collect()
        );
        namespace.add_enum(enum_def).unwrap();
    }
}

fn install_models_to_namespace(models: Vec<&ASTModel>, namespace: &mut Namespace) {
    for ast_model in models {
        let mut model = Model::new(
            ast_model.identifier.name.as_str(),
            ast_model.ns_path.clone(),
            ast_model.comment_block.as_ref().map(|c| c.name()).flatten(),
            ast_model.comment_block.as_ref().map(|c| c.desc()).flatten());
        for ast_decorator in ast_model.decorators.iter() {
            let model_decorator = ast_decorator.accessible.as_ref().unwrap().as_model_decorator().unwrap();
            model_decorator(ast_decorator.get_argument_list(), &mut model);
        }
        for ast_field in ast_model.sorted_fields() {
            match ast_field.field_class {
                ASTFieldClass::Field | ASTFieldClass::DroppedField => {
                    let mut model_field = Field::new(ast_field.identifier.name.as_str());
                    if let Some(comment) = &ast_field.comment_block {
                        if let Some(name) = comment.name.as_ref() {
                            model_field.localized_name = Some(name.to_owned());
                        }
                        if let Some(desc) = comment.desc.as_ref() {
                            model_field.description = Some(desc.to_owned());
                        }
                    }
                    // type
                    match ast_field.r#type.arity {
                        Arity::Scalar => {
                            if ast_field.r#type.item_required {
                                model_field.set_required();
                            } else {
                                model_field.set_optional();
                            }
                            install_types_to_field_owner(&ast_field.r#type.identifier.name, &mut model_field, graph.enums(), diagnostics, ast_field);
                        }
                        Arity::Array => {
                            if ast_field.r#type.collection_required {
                                model_field.set_required();
                            } else {
                                model_field.set_optional();
                            }
                            model_field.field_type = Some(FieldType::Vec(Box::new({
                                let mut inner = Field::new("");
                                if ast_field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                install_types_to_field_owner(&ast_field.r#type.identifier.name, &mut inner, graph.enums(), diagnostics, ast_field);
                                inner
                            })));
                        }
                        Arity::Dictionary => {
                            if ast_field.r#type.collection_required {
                                model_field.set_required();
                            } else {
                                model_field.set_optional();
                            }
                            model_field.field_type = Some(FieldType::HashMap(Box::new({
                                let mut inner = Field::new("");
                                if ast_field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                install_types_to_field_owner(&ast_field.r#type.identifier.name, &mut inner, graph.enums(), diagnostics, ast_field);
                                inner
                            })));
                        }
                    }
                    // decorators
                    for decorator in ast_field.decorators.iter() {
                        let field_decorator = decorator.accessible.as_ref().unwrap().as_field_decorator().unwrap();
                        field_decorator(decorator.get_argument_list(), &mut model_field);
                    }
                    // finalize
                    model_field.finalize();
                    match &ast_field.field_class {
                        ASTFieldClass::DroppedField => {
                            model.add_dropped_field(model_field, ast_field.identifier.name.as_str());
                        }
                        _ => {
                            model.add_field(model_field, ast_field.identifier.name.as_str());
                        }
                    }
                }
                ASTFieldClass::Relation => {
                    let mut model_relation = Relation::new(ast_field.identifier.name.as_str());
                    if let Some(comment) = &ast_field.comment_block {
                        if let Some(name) = comment.name.as_ref() {
                            model_relation.localized_name = Some(name.to_owned());
                        }
                        if let Some(desc) = comment.desc.as_ref() {
                            model_relation.description = Some(desc.to_owned());
                        }
                    }
                    match ast_field.r#type.arity {
                        Arity::Scalar => {
                            if ast_field.r#type.item_required {
                                model_relation.set_required();
                            } else {
                                model_relation.set_optional();
                            }
                            model_relation.set_is_vec(false);
                            model_relation.set_model(vec![ast_field.r#type.identifier.name.clone()]);
                        }
                        Arity::Array => {
                            if !ast_field.r#type.item_required {
                                panic!("Relation cannot have optional items.")
                            }
                            model_relation.set_is_vec(true);
                            model_relation.set_model(vec![ast_field.r#type.identifier.name.clone()]);
                        }
                        Arity::Dictionary => panic!("Relations cannot be dictionary.")
                    }
                    // handle decorators
                    for decorator in ast_field.decorators.iter() {
                        let relation_decorator = decorator.accessible.as_ref().unwrap().as_relation_decorator().unwrap();
                        relation_decorator(decorator.get_argument_list(), &mut model_relation);
                    }
                    model_relation.finalize(model.fields());
                    model.add_relation(model_relation, ast_field.identifier.name.as_str());
                }
                ASTFieldClass::Property => {
                    let mut model_property = Property::new(ast_field.identifier.name.as_str());
                    if let Some(comment) = &ast_field.comment_block {
                        if let Some(name) = comment.name.as_ref() {
                            model_property.localized_name = Some(name.to_owned());
                        }
                        if let Some(desc) = comment.desc.as_ref() {
                            model_property.description = Some(desc.to_owned());
                        }
                    }
                    // type
                    match ast_field.r#type.arity {
                        Arity::Scalar => {
                            if ast_field.r#type.item_required {
                                model_property.set_required();
                            } else {
                                model_property.set_optional();
                            }
                            install_types_to_field_owner(&ast_field.r#type.identifier.name, &mut model_property, graph.enums(), diagnostics, ast_field);
                        }
                        Arity::Array => {
                            if ast_field.r#type.collection_required {
                                model_property.set_required();
                            } else {
                                model_property.set_optional();
                            }
                            model_property.field_type = Some(FieldType::Vec(Box::new({
                                let mut inner = Field::new("");
                                if ast_field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                install_types_to_field_owner(&ast_field.r#type.identifier.name, &mut inner, graph.enums(), diagnostics, ast_field);
                                inner
                            })));
                        }
                        Arity::Dictionary => {
                            if ast_field.r#type.collection_required {
                                model_property.set_required();
                            } else {
                                model_property.set_optional();
                            }
                            model_property.field_type = Some(FieldType::HashMap(Box::new({
                                let mut inner = Field::new("");
                                if ast_field.r#type.item_required {
                                    inner.set_required();
                                } else {
                                    inner.set_optional();
                                }
                                install_types_to_field_owner(&ast_field.r#type.identifier.name, &mut inner, graph.enums(), diagnostics, ast_field);
                                inner
                            })));
                        }
                    }
                    for decorator in ast_field.decorators.iter() {
                        let property_decorator = decorator.accessible.as_ref().unwrap().as_property_decorator().unwrap();
                        property_decorator(decorator.get_argument_list(), &mut model_property);
                    }
                    model_property.finalize()?;
                    model.add_property(model_property, ast_field.identifier.name.as_str());
                }
                ASTFieldClass::Unresolved => unreachable!()
            }
        }
        namespace.add_model(model, ast_model.identifier.name.as_str()).unwrap();
        namespace.model_mut(ast_model.identifier.name.as_str()).unwrap().finalize();
    }
}

fn install_datasets_to_namespace(datasets: Vec<&ASTDataSet>, namespace: &mut Namespace) {
    for data_set in datasets {
        let seeder_data_set = DataSet {
            name: data_set.identifier.name.clone(),
            groups: data_set.groups.iter().map(|g| Group {
                name: g.identifier.name.split(".").map(|s| s.to_string()).collect(),
                records: g.records.iter().map(|r| Record {
                    name: r.identifier.name.clone(),
                    value: r.resolved.as_ref().unwrap().clone()
                }).collect(),
            }).collect(),
            autoseed: data_set.auto_seed,
            notrack: data_set.notrack,
        };
        namespace.datasets_mut().push(seeder_data_set);
    }
}

fn install_action_groups_to_namespace(action_groups: Vec<&ActionGroupDeclaration>, namespace: &mut Namespace) -> Result<()> {
    for action_group_dec in action_groups {
        let group = action_group_dec.identifier.name.clone();
        let group_str = Box::leak(Box::new(action_group_dec.identifier.name.clone().clone()));
        for action_dec in action_group_dec.actions.iter() {
            let name = action_dec.identifier.name.clone();
            let name_str = Box::leak(Box::new(name.clone()));
            namespace.add_custom_action_declaration(group_str, name_str, CustomActionDefinition {
                group: group.clone(),
                name,
                input: interface_ref_from(&action_dec.input_type),
                output: interface_ref_from(&action_dec.output_type),
                input_fields: action_dec.resolved_input_shape.as_ref().unwrap().clone(),
            })?;
        }
    }
    Ok(())
}