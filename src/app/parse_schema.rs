use crate::app::new_app::ctx::AppCtx;
use crate::core::conf::debug::DebugConf;
use crate::core::conf::test::{Reset, ResetDatasets, ResetMode, TestConf};
use crate::core::connector::ConnectorConf;
use crate::gen::interface::client::conf::Conf;
use crate::gen::interface::server::conf::EntityGeneratorConf;
use crate::parser::parser::parser::ASTParser;
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

    // enums
    for e in parser.enums() {
        app_ctx.gr
    }

    Ok(())
}