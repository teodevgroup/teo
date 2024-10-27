use colored::Colorize;
use teo_runtime::app::entrance::Entrance;
use teo_runtime::app::runtime_version::RuntimeVersion;
use teo_result::Result;
use crate::message::info_message;

pub fn server_start_message(port: u16, runtime_version: &RuntimeVersion, entrance: &Entrance, silent: bool) -> Result<()> {
    if silent { return Ok(()) }
    // Introducing
    let teo_version = env!("CARGO_PKG_VERSION");
    let teo = format!("Teo {}", teo_version);
    info_message(format!("{} ({}, {})", teo, runtime_version.to_string(), entrance.to_str()));
    // Listening
    let port_str = format!("{port}").bold();
    info_message(format!("listening on port {}", port_str));
    Ok(())
}